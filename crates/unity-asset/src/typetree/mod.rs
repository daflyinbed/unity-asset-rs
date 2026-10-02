//! TypeTree 解释器：从 AB 内嵌 typetree 读出节点树，并按树把对象字节解码成 `Value`。
//!
//! 参考实现：UnityPy `helpers/TypeTreeNode.py` + `helpers/TypeTreeHelper.py`。

pub mod blob;
mod common_string;
mod interpret;

pub use blob::{BlobNode, build_tree, parse_blob};
pub use interpret::{interpret, interpret_mono_behaviour};

const ALIGN_MASK: u32 = 0x4000;

/// TypeTree 节点的类型分类。
///
/// `Array/Vector/Map` 不单独成变体——它们结构同构（集合节点有唯一 `type="Array"`
/// 子节点，该子节点固定 `[size:int, data:元素]`），靠 `TypeNode::is_sequence` 判别。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeKind {
    Bool,
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
    F32,
    F64,
    String,
    TypelessData,
    Pair,
    PPtr(String),
    Struct(String),
}

impl TypeKind {
    /// 从 Unity 原始类型字符串构造（含归一化别名）。
    pub fn from_unity(s: &str) -> Self {
        match s {
            "bool" => Self::Bool,
            "UInt8" | "char" | "unsigned char" => Self::U8,
            "SInt8" | "signed char" => Self::I8,
            "UInt16" | "ushort" | "unsigned short" => Self::U16,
            "SInt16" | "short" => Self::I16,
            "UInt32" | "uint" | "unsigned int" => Self::U32,
            "SInt32" | "int" | "Type*" | "EntityId" => Self::I32,
            "UInt64" | "FileSize" | "unsigned long long" => Self::U64,
            "SInt64" | "long long" => Self::I64,
            "float" => Self::F32,
            "double" => Self::F64,
            "string" | "Utf8String" => Self::String,
            "TypelessData" => Self::TypelessData,
            "pair" => Self::Pair,
            s if s.starts_with("PPtr<") && s.ends_with('>') => {
                let inner = &s[5..s.len() - 1];
                let target = inner.strip_prefix('$').unwrap_or(inner);
                Self::PPtr(target.to_owned())
            }
            // 其余（含 "vector"/"staticvector"/"set"/"map"/"Array"）按 struct 处理
            other => Self::Struct(other.to_owned()),
        }
    }
}

/// 归一化的 TypeTree 节点（嵌套树）。
#[derive(Debug, Clone)]
pub struct TypeNode {
    pub kind: TypeKind,
    pub name: String,
    pub byte_size: i32,
    pub version: i16,
    pub meta_flag: u32,
    pub children: Vec<TypeNode>,
}

impl TypeNode {
    /// 是否为集合（vector/staticvector/set/map/Array）——children[0] 是 `Array` 包装器。
    pub fn is_sequence(&self) -> bool {
        matches!(
            self.children.first(),
            Some(c) if matches!(&c.kind, TypeKind::Struct(s) if s == "Array")
        )
    }

    pub fn is_aligned(&self) -> bool {
        self.meta_flag & ALIGN_MASK != 0
    }
}

/// 动态值树与 `PPtrValue` 定义在 unity-core（generated 的 codegen 发射代码
/// 也要引用它们），此处 re-export 维持 `crate::typetree::Value` 既有路径。
pub use unity_core::{PPtrValue, Value};

/// `[SerializeReference]` 引用类型查表（由 SerializedFile.ref_types 构建）。
#[derive(Default)]
pub struct RefTypeTable {
    entries: Vec<(String, String, String, TypeNode)>,
}

impl RefTypeTable {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn push(&mut self, class: String, ns: String, asm: String, node: TypeNode) {
        self.entries.push((class, ns, asm, node));
    }

    pub fn lookup(&self, class: &str, ns: &str, asm: &str) -> Option<&TypeNode> {
        self.entries
            .iter()
            .find(|(c, n, a, _)| c == class && n == ns && a == asm)
            .map(|(_, _, _, node)| node)
    }
}
