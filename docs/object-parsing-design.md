# 对象解析设计：Typed Codegen + TypeTree 解释器

> 范围：定义 SerializedFile 对象的解析架构——何时用 codegen'd typed parser，何时用
> TypeTree 解释器，如何回退。
> 状态：设计稿（替代 `codegen-design.md` §4-§7 的 parse 路径部分）
> 上游：[ADR-0001](adr/0001-codegen-from-typetree-dumps.md)、
> [`codegen-implementation.md`](codegen-implementation.md)（codegen 现状）

---

## 1. 核心决策

| 对象类型 | 解析路径 | 数据来源 |
|---|---|---|
| MonoBehaviour (classID=114) | **TypeTree 解释器** | AB 内嵌 TypeTree |
| 其他原生类型 | **codegen'd typed parser** | TPK（离线烤进 Rust 代码） |
| typed parse 失败 | **回退到 TypeTree 解释器** | AB 内嵌 TypeTree（如果有） |

**不做 TPK vs AB TypeTree 比较**。直接尝试 typed parse，失败就回退。简单、无额外开销。

---

## 2. 架构总览

```
SerializedFile
  │
  │  每个 object: (class_id, version, data: &[u8], ab_type_tree: Option<TypeNode>)
  │
  ▼
┌──────────────────────────────────────────┐
│ dispatch(class_id, data, ver, ab_tt)      │
├──────────┬───────────────────────────────┤
│          │                               │
│ class_id │ 路径                           │
│ == 114   │ → interpret(data, ab_tt, ver)  │  MonoBehaviour: 直接走 TypeTree
│          │                               │
│ 其他     │ → typed_parse(class_id, data)  │  先尝试 codegen'd
│          │   ├─ Ok(obj)  → 返回           │
│          │   └─ Err      → fallback       │
│          │       ├─ 有 ab_tt              │
│          │       │  → interpret(data,ab_tt)│  回退到 TypeTree
│          │       └─ 无 ab_tt              │
│          │          → 返回原始错误         │
└──────────┴───────────────────────────────┘
        │                          │
        ▼                          ▼
  TypedStruct                Value (动态值树)
  (Texture2D, Mesh, ...)     (Vec<(String, Value)>)
```

**为什么用 `&[u8]` 而不是 `&mut R: Read`**：

fallback 需要在 typed parse 失败后重新从头解析。用 `&[u8]` 切片，两条路径各自创建 cursor，
互不干扰。如果用 `&mut R`，typed parse 失败后 reader 位置已乱，无法回退。

---

## 3. TypeNode 设计

### 3.1 TypeName — 类型分类 enum

替代字符串比较。原语比较 = 整数比较；PPtr/Struct 带关联 String。

```rust
/// Unity TypeTree 节点的类型分类。
/// 从 TPK 和 AB-embedded 两个来源的原始字符串统一构造。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeName {
    // ── 原语（无关联数据）──
    Bool,
    U8,            // UInt8, char, unsigned char
    I8,            // SInt8, signed char
    U16,           // UInt16, ushort, unsigned short
    I16,           // SInt16, short
    U32,           // UInt32, uint, unsigned int
    I32,           // SInt32, int, Type*, EntityId
    U64,           // UInt64, unsigned long long, FileSize
    I64,           // SInt64, long long
    F32,           // float
    F64,           // double

    // ── 特殊容器 ──
    String,        // string, Utf8String — size + char[] + align
    TypelessData,  // size + u8[]
    Array,         // [size, data]
    Vector,        // vector, staticvector, set — 包装一个 Array
    Map,           // map — Array of pair
    Pair,          // first + second

    // ── 引用 ──
    PPtr(String),  // target type name, e.g. "GameObject"

    // ── 用户 struct ──
    Struct(String), // struct name, e.g. "Vector3f"
}

impl TypeName {
    /// 从 Unity 原始类型字符串构造（含归一化）。
    /// TPK 字符串和 AB 字符串都走这个函数，保证两端产出相同。
    pub fn from_unity(s: &str) -> Self {
        match s {
            // 归一化别名
            "short"            => Self::I16,
            "ushort"           => Self::U16,
            "int"              => Self::I32,
            "uint"             => Self::U32,
            "long long"        => Self::I64,
            "unsigned short"   => Self::U16,
            "unsigned int"     => Self::U32,
            "unsigned long long" => Self::U64,
            "unsigned char"    => Self::U8,
            "signed char"      => Self::I8,
            "Utf8String"       => Self::String,
            // 已规范名
            "bool"             => Self::Bool,
            "UInt8" | "char"   => Self::U8,
            "SInt8"            => Self::I8,
            "UInt16"           => Self::U16,
            "SInt16"           => Self::I16,
            "UInt32"           => Self::U32,
            "SInt32" | "Type*" | "EntityId" => Self::I32,
            "UInt64" | "FileSize" => Self::U64,
            "SInt64"           => Self::I64,
            "float"            => Self::F32,
            "double"           => Self::F64,
            "string"           => Self::String,
            "TypelessData"     => Self::TypelessData,
            "Array"            => Self::Array,
            "vector" | "staticvector" | "set" => Self::Vector,
            "map"              => Self::Map,
            "pair"             => Self::Pair,
            // PPtr<T> 提取 target
            s if s.starts_with("PPtr<") && s.ends_with('>') => {
                let inner = &s[5..s.len()-1];
                let target = inner.strip_prefix('$').unwrap_or(inner);
                Self::PPtr(target.to_owned())
            }
            // 其余 = 用户 struct
            other => Self::Struct(other.to_owned()),
        }
    }
}
```

### 3.2 TypeNode — 统一节点

```rust
/// 归一化的 TypeTree 节点。
/// TPK 节点池和 AB 扁平列表都转换为此格式。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeNode {
    pub type_name: TypeName,
    pub name: String,         // 字段名："m_Width", "Array", "data"
    pub byte_size: i32,       // -1 = 变长
    pub version: i16,         // 节点 schema 版本
    pub type_flags: u8,       // bit0 = Array/TypelessData
    pub meta_flag: u32,       // 0x4000 = align, 0x8000 = child align
    pub children: Vec<TypeNode>,
}
```

`PartialEq` 递归比较全树——每个字段 + 子树。用于测试和调试。

### 3.3 从 AB-embedded TypeTree 构建

AB TypeTree 是**扁平节点数组**，用 `level: u8` 表达树深度。

```rust
/// AB TypeTree 的原始扁平节点（从 SerializedFile 二进制读出）。
pub struct AbRawNode {
    pub version: u16,
    pub level: u8,
    pub type_flags: u8,
    pub type_str_offset: u32,  // bit31=0 本地 buffer, bit31=1 CommonString
    pub name_str_offset: u32,
    pub byte_size: i32,
    pub meta_flag: u32,
    // index, ref_type_hash 丢弃（解析不需要）
}

impl TypeNode {
    /// 从 AB 扁平节点列表构建 TypeNode 树。
    ///
    /// `string_buffer` 是 AB TypeTree 自带的字符串缓冲区。
    /// `common` 是 Unity 全局 CommonString 表（~113 条硬编码）。
    pub fn from_ab(
        nodes: &[AbRawNode],
        string_buffer: &[u8],
        common: &CommonStringTable,
    ) -> Option<Self> {
        if nodes.is_empty() { return None; }
        let mut idx = 0;
        Self::build_ab_tree(nodes, &mut idx, 0, string_buffer, common)
    }

    fn build_ab_tree(
        nodes: &[AbRawNode],
        idx: &mut usize,
        expected_level: u8,
        str_buf: &[u8],
        common: &CommonStringTable,
    ) -> Option<Self> {
        if *idx >= nodes.len() { return None; }
        let raw = &nodes[*idx];
        if raw.level != expected_level { return None; }
        *idx += 1;

        let type_name = TypeName::from_unity(&resolve_str(raw.type_str_offset, str_buf, common));
        let name = resolve_str(raw.name_str_offset, str_buf, common);

        let mut children = Vec::new();
        while *idx < nodes.len() && nodes[*idx].level > expected_level {
            children.push(Self::build_ab_tree(nodes, idx, expected_level + 1, str_buf, common)?);
        }

        Some(Self {
            type_name,
            name,
            byte_size: raw.byte_size,
            version: raw.version as i16,
            type_flags: raw.type_flags,
            meta_flag: raw.meta_flag,
            children,
        })
    }
}

/// 解析 AB 字符串偏移。
/// bit31=0: null-terminated string in local buffer
/// bit31=1: index into CommonString table
fn resolve_str(offset: u32, buf: &[u8], common: &CommonStringTable) -> String {
    if offset & 0x8000_0000 == 0 {
        let start = offset as usize;
        let end = buf[start..].iter().position(|&b| b == 0)
            .map(|p| start + p)
            .unwrap_or(buf.len());
        String::from_utf8_lossy(&buf[start..end]).into()
    } else {
        common.get(offset & !0x8000_0000).to_owned()
    }
}
```

### 3.4 CommonString 表

Unity 硬编码的 ~113 个全局共享类型/字段名。AB TypeTree 用它去重常见字符串。

```rust
/// 从 AssetRipper CommonString.cs 抄录的硬编码表。
pub struct CommonStringTable;

static COMMON: &[(u32, &str)] = &[
    (0, ""),     (1, "AABB"),   (49, "Array"),  (222, "SInt32"),
    (840, "string"), (894, "TypelessData"),
    // ... 完整 113 条
];

impl CommonStringTable {
    pub fn get(&self, offset: u32) -> &'static str {
        COMMON.iter()
            .find(|(k, _)| *k == offset)
            .map(|(_, v)| *v)
            .unwrap_or("")
    }
}
```

### 3.5 从 TPK 构建（codegen 用）

codegen 从 TPK 读取时也转换成 `TypeNode`，但这条路径只用于离线 codegen，不在运行时：

```rust
impl TypeNode {
    /// 从 TPK 节点池递归展开（图→树）。codegen 离线用。
    pub fn from_tpk(
        node_idx: u16,
        pool: &[TpkUnityNode],
        strings: &[String],
    ) -> Self {
        let node = &pool[node_idx as usize];
        let raw_type = strings[node.type_name_index as usize].as_str();
        let raw_name = strings[node.name_index as usize].as_str();
        Self {
            type_name: TypeName::from_unity(raw_type),
            name: raw_name.to_owned(),
            byte_size: node.byte_size,
            version: node.node_version,
            type_flags: node.type_flags,
            meta_flag: node.meta_flag,
            children: node.sub_nodes.iter()
                .map(|&i| Self::from_tpk(i, pool, strings))
                .collect(),
        }
    }
}
```

---

## 4. Value + 解释器

### 4.1 Value — 动态值树

MonoBehaviour 和 fallback 路径的输出类型。

```rust
#[derive(Debug, Clone)]
pub enum Value {
    Bool(bool),
    I8(i8),  U8(u8),
    I16(i16), U16(u16),
    I32(i32), U32(u32),
    I64(i64), U64(u64),
    F32(f32), F64(f64),
    String(String),
    Bytes(Vec<u8>),
    Array(Vec<Value>),
    Struct(Vec<(String, Value)>),  // 保序字段列表
    PPtr(PPtrValue),
}

#[derive(Debug, Clone)]
pub struct PPtrValue {
    pub file_id: i32,
    pub path_id: i64,
}
```

### 4.2 interpret — 递归走 TypeNode 读字节

```rust
/// TypeTree 解释器：按 node 的 TypeNode 树递归读字节。
/// 用于 MonoBehaviour 和 typed parse 失败时的回退。
pub fn interpret(
    data: &[u8],
    node: &TypeNode,
    ver: &UnityVersion,
    endian: Endian,
) -> Result<Value, ParseError> {
    let mut r = Cursor::new(data, endian);
    interpret_node(&mut r, node, ver)
}

fn interpret_node<R: EndianReader>(
    r: &mut R,
    node: &TypeNode,
    ver: &UnityVersion,
) -> Result<Value, R::Error> {
    let value = match &node.type_name {
        TypeName::Bool  => Value::Bool(r.read_bool()?),
        TypeName::U8    => Value::U8(r.read_u8()?),
        TypeName::I8    => Value::I8(r.read_i8()?),
        TypeName::U16   => Value::U16(r.read_u16()?),
        TypeName::I16   => Value::I16(r.read_i16()?),
        TypeName::U32   => Value::U32(r.read_u32()?),
        TypeName::I32   => Value::I32(r.read_i32()?),
        TypeName::U64   => Value::U64(r.read_u64()?),
        TypeName::I64   => Value::I64(r.read_i64()?),
        TypeName::F32   => Value::F32(r.read_f32()?),
        TypeName::F64   => Value::F64(r.read_f64()?),

        TypeName::String => {
            let n = r.read_i32()? as usize;
            let bytes = r.read_bytes(n)?;
            Value::String(String::from_utf8_lossy(&bytes).into())
        }

        TypeName::TypelessData => {
            let n = r.read_i32()? as usize;
            Value::Bytes(r.read_bytes(n)?)
        }

        // vector / staticvector / set / map → 包一层 Array
        TypeName::Vector | TypeName::Map => {
            let array_node = &node.children[0]; // "Array" 子节点
            read_array(r, array_node, ver)?
        }

        TypeName::Array => {
            read_array(r, node, ver)?
        }

        TypeName::PPtr(_) => {
            let file_id = r.read_i32()?;
            let path_id = if *ver >= UnityVersion::new(5, 0, 0) {
                r.read_i64()?
            } else {
                r.read_i32()? as i64
            };
            Value::PPtr(PPtrValue { file_id, path_id })
        }

        // pair / Struct → 读子节点
        TypeName::Pair | TypeName::Struct(_) => {
            let mut fields = Vec::with_capacity(node.children.len());
            for child in &node.children {
                let name = child.name.clone();
                let value = interpret_node(r, child, ver)?;
                fields.push((name, value));
            }
            Value::Struct(fields)
        }
    };

    // 对齐：meta_flag bit 0x4000
    if node.meta_flag & 0x4000 != 0 {
        r.align(4)?;
    }

    Ok(value)
}

/// Array 节点 = [size:int, data:element_type]
fn read_array<R: EndianReader>(
    r: &mut R,
    array_node: &TypeNode,
    ver: &UnityVersion,
) -> Result<Value, R::Error> {
    let count = r.read_i32()?;
    // Array 的 children: [0]=size(int), [1]=data(元素类型)
    let elem_node = &array_node.children[1];
    let mut elements = Vec::with_capacity(count.min(1024) as usize);
    for _ in 0..count {
        elements.push(interpret_node(r, elem_node, ver)?);
    }
    Ok(Value::Array(elements))
}
```

**对齐说明**：每个 `interpret_node` 调用结束后检查自己的 `meta_flag & 0x4000`。
递归调用天然保证子节点的对齐在子节点自己的 interpret 里处理。

---

## 5. MonoBehaviour 处理

MonoBehaviour **不生成 codegen'd struct**。始终走 TypeTree 解释器。

### 5.1 为什么

MonoBehaviour 的用户脚本字段不在 TPK 里（TPK 只有引擎基类字段）。
用户脚本字段在 SerializedFile 的 AB-embedded TypeTree 里。
对 MonoBehaviour 做 codegen 没有意义——脚本字段每次都不同。

### 5.2 输出

MonoBehaviour 解析产出一个 `Value::Struct`，包含基类字段 + 脚本字段，全部来自 AB TypeTree：

```rust
// 解析 MonoBehaviour
let ab_tree = TypeNode::from_ab(ab_nodes, str_buf, &common)?;
let value = interpret(data, &ab_tree, ver, endian)?;

// value 是 Value::Struct，例如：
// Value::Struct([
//     ("m_GameObject", Value::PPtr(...)),
//     ("m_Enabled", Value::Bool(true)),
//     ("m_Script", Value::PPtr(...)),
//     ("m_Name", Value::String("MyComponent")),
//     ("m_EditorClassIdentifier", Value::String("")),
//     ("m_Speed", Value::F32(3.14)),        // ← 用户脚本字段
//     ("targets", Value::Array([...)),       // ← 用户脚本字段
// ])
```

### 5.3 没有 AB TypeTree 时

如果 AB 没有内嵌 TypeTree（极少见，MonoBehaviour 正常都有），无法解析脚本字段。
返回错误。

---

## 6. Fallback 机制

### 6.1 触发条件

typed parse 返回 `Err`。常见原因：

| 原因 | typed parse | AB TypeTree 回退 |
|---|---|---|
| 未知 class_id（不在 codegen 里） | 直接 Err | ✅ 如果 AB 有 TypeTree |
| Unity 版本不在 TPK 覆盖范围 | 字段门控错（读到 EOF / 多余字节）| ✅ |
| 引擎魔改（如明日方舟定制 Unity） | 字段 layout 不匹配 | ✅ |
| 数据损坏 | Err | 也会 Err（数据真坏了）|

### 6.2 实现

```rust
/// 解析一个 SerializedFile object。
///
/// `data` 是该 object 的原始字节（从 SerializedFile 的 block info 解出）。
/// `ab_type_tree` 是 AB 内嵌 TypeTree（如果有）。
pub fn parse_object(
    data: &[u8],
    class_id: i32,
    ver: &UnityVersion,
    endian: Endian,
    ab_type_tree: Option<(&[AbRawNode], &[u8])>,  // (nodes, string_buffer)
) -> Result<ObjectValue, ParseError> {

    // ── MonoBehaviour：直接走 TypeTree ──
    if class_id == 114 {
        let (nodes, str_buf) = ab_type_tree
            .ok_or(ParseError::MonoBehaviourWithoutTypeTree)?;
        let tree = TypeNode::from_ab(nodes, str_buf, &CommonStringTable)
            .ok_or(ParseError::EmptyTypeTree)?;
        let value = interpret(data, &tree, ver, endian)?;
        return Ok(ObjectValue::Dynamic(value));
    }

    // ── 其他类型：先试 typed parse ──
    match crate::generated::parse_typed(class_id, data, ver, endian) {
        Ok(typed) => Ok(ObjectValue::Typed(typed)),
        Err(typed_err) => {
            // 回退到 TypeTree 解释器
            if let Some((nodes, str_buf)) = ab_type_tree {
                log::warn!(
                    "classID {class_id}: typed parse failed ({typed_err}), \
                     falling back to TypeTree interpreter"
                );
                let tree = TypeNode::from_ab(nodes, str_buf, &CommonStringTable)
                    .ok_or(ParseError::EmptyTypeTree)?;
                let value = interpret(data, &tree, ver, endian)?;
                Ok(ObjectValue::Dynamic(value))
            } else {
                // AB 没有 TypeTree，无法回退
                Err(typed_err)
            }
        }
    }
}
```

### 6.3 ObjectValue — 统一返回类型

```rust
/// 解析结果：typed struct 或动态 Value。
pub enum ObjectValue {
    /// codegen'd typed struct（大多数原生类型）
    Typed(UnityObject),
    /// TypeTree 解释器产出的动态值（MonoBehaviour + fallback）
    Dynamic(Value),
}
```

用户 match：

```rust
match parse_object(data, class_id, ver, endian, ab_tt)? {
    ObjectValue::Typed(UnityObject::Texture2D(tex)) => {
        println!("{}x{} {}", tex.m_width, tex.m_height, tex.m_name);
    }
    ObjectValue::Dynamic(value) => {
        // MonoBehaviour 或 fallback
        if let Value::Struct(fields) = value {
            for (name, val) in fields {
                println!("{name}: {val:?}");
            }
        }
    }
}
```

---

## 7. Codegen 输出（变化点）

### 7.1 不变的

- 扁平 struct + 版本门控（`Option<T>` + `if ver >= V`）
- 对齐烤死（`meta_flag & 0x4000`）
- 字段类型映射（`SInt32 → i32` 等）
- 命名规则（`to_rust_field_name` / `to_rust_type_name`）

### 7.2 变化的

| 项 | 旧 | 新 |
|---|---|---|
| MonoBehaviour | 生成 typed struct（空壳） | **不生成**，从 `enum UnityObject` 排除 |
| `enum UnityObject` | 含 MonoBehaviour 变体 | **移除** MonoBehaviour 变体 |
| Parse trait | `fn parse(&mut R, ver)` | `fn parse(data: &[u8], ver, endian)` — 用 `&[u8]` 支持 fallback |

### 7.3 Parse 签名变化

**旧**（`&mut R`，无法回退）：
```rust
pub trait Parse: Sized {
    fn parse<R: EndianReader>(r: &mut R, ver: &UnityVersion) -> Result<Self, R::Error>;
}
```

**新**（`&[u8]`，支持回退）：
```rust
pub trait Parse: Sized {
    fn parse(data: &[u8], ver: &UnityVersion, endian: Endian) -> Result<Self, ParseError>;
}
```

每个生成函数内部创建 cursor，从 `data` 开头读。fallback 时用同一个 `data` 重新创建 cursor。

---

## 8. SerializedFile 集成

SerializedFile 解析后，每个 object 有：

```
ObjectInfo {
    class_id: i32,
    data: &[u8],            // 解压后的对象字节
    type_tree: Option<TypeTree>,  // AB 内嵌的 TypeTree（如果有）
}
```

集成流程：

```rust
let bundle = load("x.bundle")?;
let serialized = SerializedFile::parse(&bundle.files[0].data)?;

for obj in &serialized.objects {
    let class_id = obj.class_id;
    let ver = &serialized.version;
    let endian = serialized.endian;
    let ab_tt = obj.type_tree.as_ref()
        .map(|tt| (tt.nodes.as_slice(), tt.string_buffer.as_slice()));

    match parse_object(obj.data, class_id, ver, endian, ab_tt) {
        Ok(ObjectValue::Typed(typed)) => { /* Texture2D, Mesh, ... */ }
        Ok(ObjectValue::Dynamic(value)) => { /* MonoBehaviour / fallback */ }
        Err(e) => log::error!("Failed to parse classID {class_id}: {e}"),
    }
}
```

### AB TypeTree 存在性

| 构建类型 | 原生类型 TypeTree | MonoBehaviour TypeTree |
|---|---|---|
| Editor build | ✅ 有 | ✅ 有 |
| Release build（默认） | ❌ 通常被 strip | ✅ 始终有 |
| Release + `BuildOptions.StripTypeTree` | ❌ | ❌（极少见）|

所以：
- 原生类型：正常走 typed parse（不依赖 AB TypeTree）
- MonoBehaviour：正常走 TypeTree（AB TypeTree 存在）
- Fallback：只在 editor build 或未被 strip 的 release build 中生效

---

## 9. Crate 结构

```
unity-asset-rs/
├── crates/
│   ├── codegen/              # 离线工具（不变）
│   ├── generated/            # codegen 产物（微调：移除 MonoBehaviour）
│   │   └── src/
│   │       ├── lib.rs        # Parse trait, EndianReader, PPtr, UnityVersion
│   │       ├── unity_object.rs  # enum UnityObject（无 MonoBehaviour 变体）
│   │       └── classes/      # typed structs
│   └── ...
└── src/
    ├── lib.rs
    ├── bundle.rs             # UnityFS 解包（已有）
    ├── decompress.rs         # 解压（已有）
    ├── serialized_file.rs    # ★ 新增：SerializedFile 解析
    ├── object_parser.rs      # ★ 新增：dispatch + fallback
    └── typetree/             # ★ 新增：TypeTree 解释器
        ├── mod.rs            # TypeNode, TypeName, Value, PPtrValue
        ├── interpret.rs      # interpret() 递归走树
        ├── from_ab.rs        # AB 扁平节点 → TypeNode 树
        └── common_string.rs  # CommonString 硬编码表
```

`typetree/` 模块是纯运行时代码（不在 codegen 里），依赖：
- `TypeName` / `TypeNode` — 数据结构
- `EndianReader` trait（从 generated crate 复用，或独立定义）
- `UnityVersion`（同上）

---

## 10. 实施步骤

| 阶段 | 产出 | 验证 |
|---|---|---|
| **P1** | `typetree/` 模块：TypeName + TypeNode + from_ab + CommonString | 单元测试：从 AB 读出 TypeNode 树 |
| **P2** | `interpret()` + Value | 单元测试：给 TypeNode + bytes → Value，对照 UnityPy |
| **P3** | `object_parser.rs`：dispatch + fallback | MonoBehaviour 解出 Value::Struct |
| **P4** | `serialized_file.rs`：SerializedFile 解析 | 从 .bundle 提取 object list + AB TypeTree |
| **P5** | 接入 `crates/generated`：typed parse | 原生类型解出 typed struct |
| **P6** | 端到端验证 | 用 `char_002_amiya.ab` 解出全部对象，字段值对比 UnityPy |

P1-P2 是核心工作量（~60%）。P4 的 SerializedFile 解析涉及 Unity 的 metadata 格式（big-endian header + serialized type info + object info table），有一定复杂度。

---

## 11. 与 codegen-design.md 的关系

| 维度 | codegen-design.md | 本文档 |
|---|---|---|
| parse 路径 | 只有 codegen'd typed parse | typed + TypeTree 解释器 + fallback |
| MonoBehaviour | 最小骨架 + token 兜底 | **不 codegen**，直接走 TypeTree |
| TPK 在 runtime | 不需要 | 不需要（相同） |
| AB TypeTree | 提及但未设计 | **完整设计**（from_ab + CommonString + interpret） |
| Value | 不存在 | 新增（MonoBehaviour + fallback 的输出） |
| Parse 签名 | `&mut R` | `&[u8]`（支持 fallback） |
| 比较逻辑 | — | **不做**（直接 try + fallback） |

`codegen-design.md` 仍然有效，但 parse 路径部分（§4 的 parse 代码形态、§7.2 算法）
以本文档为准。codegen 的 struct 生成逻辑（命名、字段并集、版本门控）不变。

---

## 12. 设计取舍记录

### 为什么不做 TypeTree 比较？

之前的讨论中考虑过：比较 AB TypeTree 和 TPK TypeTree，一致走 typed，不一致走 dynamic。
最终放弃，原因：

1. **复杂度**：需要加载 TPK 到 runtime，做树比较，处理 prefix match（MonoBehaviour）
2. **收益低**：typed parse 失败的情况很少（版本覆盖足够 + 引擎不魔改时不会失败）
3. **Fallback 更简单**：try-catch 一行代码，无额外数据结构
4. **性能**：不需要每次都比较两棵树

### 为什么 MonoBehaviour 不 codegen？

即使 codegen 了基类字段，脚本字段仍然需要 TypeTree 解释器。两套机制（codegen + interpret）
共存增加复杂度。统一用 TypeTree 解释器，一步到位。

### 为什么用 `&[u8]` 而不是 `&mut R`？

Fallback 需要在 typed parse 失败后从头重试。`&[u8]` 不可变引用让两条路径各自创建 cursor，
天然支持重试。`&mut R` 失败后 reader 位置已乱，无法回退。
