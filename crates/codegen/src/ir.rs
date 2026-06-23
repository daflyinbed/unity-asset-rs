//! 中间表示：从 TPK 解包到 EmitClass 集合的数据结构。
//!
//! 参考设计：`docs/codegen-design.md` §6.2。

use std::collections::BTreeMap;

// ────────────────────────────────────────────────────────────────────
// NodeType 分类（参考 AssetRipper `UniversalNode.cs:47-80`）
// ────────────────────────────────────────────────────────────────────

/// TypeTree 节点的种类。判定靠 `(TypeName 字符串, 子树形态)` 二元组，
/// **不**靠字符串 tokenizer（PPtr 例外，靠 `PPtr<` 前缀）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NodeType {
    // 原语
    Boolean,
    UInt8,
    Int8,
    UInt16,
    Int16,
    UInt32,
    Int32,
    UInt64,
    Int64,
    Single,
    Double,
    // 容器
    String,
    TypelessData,
    Array,
    Vector,
    Map,
    Pair,
    PPtr,
    /// 自定义 struct（引用 subclass）
    Type,
}

impl NodeType {
    /// 调用前应已跑过 p01 类型归一化（`int → SInt32` 等）。
    pub fn classify(type_name: &str, sub_nodes: &[u16]) -> Self {
        if sub_nodes.is_empty() {
            match type_name {
                "bool" => Self::Boolean,
                "char" | "UInt8" => Self::UInt8,
                "SInt8" => Self::Int8,
                "SInt16" => Self::Int16,
                "UInt16" => Self::UInt16,
                "SInt32" | "Type*" | "EntityId" => Self::Int32,
                "UInt32" => Self::UInt32,
                "SInt64" => Self::Int64,
                "UInt64" | "FileSize" => Self::UInt64,
                "float" => Self::Single,
                "double" => Self::Double,
                _ => Self::Type, // 罕见：无子节点的 struct
            }
        } else {
            if type_name.starts_with("PPtr<") {
                return Self::PPtr;
            }
            match type_name {
                "Array" => Self::Array,
                "vector" | "staticvector" | "set" => Self::Vector,
                "map" => Self::Map,
                "pair" => Self::Pair,
                "TypelessData" => Self::TypelessData,
                "string" => Self::String,
                _ => Self::Type,
            }
        }
    }

    pub fn is_primitive(self) -> bool {
        matches!(
            self,
            Self::Boolean
                | Self::UInt8
                | Self::Int8
                | Self::UInt16
                | Self::Int16
                | Self::UInt32
                | Self::Int32
                | Self::UInt64
                | Self::Int64
                | Self::Single
                | Self::Double
        )
    }
}

// ────────────────────────────────────────────────────────────────────
// RustType：生成代码里的字段类型
// ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RustType {
    Bool,
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    F32,
    F64,
    String,
    /// TypelessData：变长 `Vec<u8>`（size 前缀）
    Bytes,
    /// `Vec<T>`
    List(Box<RustType>),
    /// `(K, V)` 元组（map 的元素）
    Pair(Box<RustType>, Box<RustType>),
    /// `PPtr<T>`，target_class 已 Rust 化
    Pptr { target_class: String },
    /// 引用另一个 EmitClass（subclass）
    Subclass(String),
    /// 版本门控字段（外层包 Option）
    Option(Box<RustType>),
}

impl RustType {
    pub fn to_rust_src(&self) -> String {
        match self {
            Self::Bool => "bool".into(),
            Self::I8 => "i8".into(),
            Self::U8 => "u8".into(),
            Self::I16 => "i16".into(),
            Self::U16 => "u16".into(),
            Self::I32 => "i32".into(),
            Self::U32 => "u32".into(),
            Self::I64 => "i64".into(),
            Self::U64 => "u64".into(),
            Self::F32 => "f32".into(),
            Self::F64 => "f64".into(),
            Self::String => "String".into(),
            Self::Bytes => "Vec<u8>".into(),
            Self::List(t) => format!("Vec<{}>", t.to_rust_src()),
            Self::Pair(k, v) => format!("({}, {})", k.to_rust_src(), v.to_rust_src()),
            Self::Pptr { target_class: _ } => "PPtr".into(),
            Self::Subclass(name) => name.clone(),
            Self::Option(t) => format!("Option<{}>", t.to_rust_src()),
        }
    }
}

// ────────────────────────────────────────────────────────────────────
// EmitClass：codegen 内部状态（pass 之间传递）
// ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ClassKey {
    /// Unity 主 class，按 classID 索引。
    Main(i32),
    /// 内嵌 struct（subclass），按 Unity type_name 索引。
    Subclass(String),
}

#[derive(Debug, Clone)]
pub struct EmitClass {
    pub key: ClassKey,
    /// Rust 化后的类型名（`Texture2D`、`Vector3f`）
    pub name: String,
    /// 原始 Unity 名（`Texture2D`、`Vector3f`）
    pub unity_name: String,
    pub parent: ParentSpec,
    pub own_fields: Vec<EmitField>,
    pub is_abstract: bool,
    pub is_mono_behaviour: bool,
}

impl EmitClass {
    pub fn class_id(&self) -> Option<i32> {
        if let ClassKey::Main(id) = self.key {
            Some(id)
        } else {
            None
        }
    }
    pub fn is_main(&self) -> bool {
        matches!(self.key, ClassKey::Main(_))
    }
}

#[derive(Debug, Clone)]
pub enum ParentSpec {
    /// 顶层（无父类）
    Root,
    /// 所有版本里父类都相同（389/412）
    Stable(String),
    /// 父类跨版本变化（23/412）—— 生成 `{Class}Parent` enum
    Variable {
        enum_name: String,
        /// `[(版本门控, 父类 Rust 名)]`，按版本升序
        branches: Vec<(VersionGate, String)>,
    },
}

#[derive(Debug, Clone)]
pub struct EmitField {
    pub rust_name: String,
    pub unity_name: String,
    pub rust_type: RustType,
    pub version_gate: Option<VersionGate>,
    pub align_after: bool,
    pub unity_meta_flag: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VersionGate {
    /// 全版本都有
    Always,
    /// `ver >= (major, minor, build)`
    Since(u16, u16, u16),
    /// `(major1, minor1, build1) <= ver < (major2, minor2, build2)`
    Range {
        from: (u16, u16, u16),
        to: (u16, u16, u16),
    },
}

impl VersionGate {
    pub fn since(major: u16, minor: u16, build: u16) -> Self {
        Self::Since(major, minor, build)
    }
}

/// 整个 pass 管线的最终产物。
pub struct Pipeline {
    pub classes: BTreeMap<ClassKey, EmitClass>,
    /// 所有出现过的 PPtr 目标 class（用于 emit PPtr 引用）
    pub pptr_targets: std::collections::BTreeSet<String>,
    /// 所有 Unity 版本（按升序），用于计算 version_gate
    pub all_versions: Vec<crate::tpk::UnityVersion>,
}
