# 对象解析设计：Typed Codegen + TypeTree 解释器

> ⛔ **历史归档（2026-07-09）** —— 本文的 dispatch + fallback 设计**已全部落地**
> （原「待实现 ⬜」清单已完成）。权威现状见 [`object-layer.md`](object-layer.md)。
> 本文保留仅供追溯。

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

### 3.1 TypeKind — 类型分类 enum

替代字符串比较。原语比较 = 整数比较；PPtr/Struct 带关联 String。

```rust
/// TypeTree 节点的类型分类。
///
/// `Array/Vector/Map` 不单独成变体——它们结构同构（集合节点有唯一 `type="Array"`
/// 子节点，该子节点固定 `[size:int, data:元素]`），靠 `TypeNode::is_sequence` 判别。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeKind {
    // ── 原语（无关联数据）──
    Bool,
    U8,            // UInt8, char, unsigned char
    I8,            // SInt8, signed char
    U16,           // UInt16, ushort, unsigned short
    I16,           // SInt16, short
    U32,           // UInt32, uint, unsigned int
    I32,           // SInt32, int, Type*, EntityId
    U64,           // UInt64, FileSize, unsigned long long
    I64,           // SInt64, long long
    F32,           // float
    F64,           // double

    // ── 特殊容器 ──
    String,        // string, Utf8String — size + char[] + align
    TypelessData,  // size + u8[]
    Pair,          // first + second（两个子节点）

    // ── 引用 ──
    PPtr(String),  // target type name, e.g. "GameObject"

    // ── 用户 struct（含 vector/staticvector/set/map/Array，靠 is_sequence 区分）──
    Struct(String), // struct name, e.g. "Vector3f"、"Array"
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
```

### 3.2 TypeNode — 统一节点

```rust
/// 归一化的 TypeTree 节点（嵌套树）。
/// TPK 节点池和 AB 扁平列表都转换为此格式。
#[derive(Debug, Clone)]
pub struct TypeNode {
    pub kind: TypeKind,
    pub name: String,         // 字段名："m_Width", "Array", "data"
    pub byte_size: i32,       // -1 = 变长
    pub version: i16,         // 节点 schema 版本
    pub meta_flag: u32,       // 0x4000 = align
    pub children: Vec<TypeNode>,
}

const ALIGN_MASK: u32 = 0x4000;

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
```

`type_flags` 不保留在 `TypeNode` 里——解释器用 `is_sequence()` 判断集合，不需要它。
不做 `PartialEq/Eq` 派生（树比较当前无需求）。

### 3.3 从 AB-embedded TypeTree 构建

AB TypeTree 是**扁平节点数组**，用 `level: u8` 表达树深度。

```rust
/// AB TypeTree 的原始扁平节点（从 SerializedFile 二进制读出）。
/// 字段全部保留（含 index / ref_type_hash），即便解释器当前不消费它们。
#[derive(Debug, Clone)]
pub struct BlobNode {
    pub version: i16,
    pub level: u8,
    pub type_flags: u8,
    pub type_str_offset: u32,  // bit31=0 本地 buffer, bit31=1 CommonString
    pub name_str_offset: u32,
    pub byte_size: i32,
    pub index: i32,
    pub meta_flag: u32,
    pub ref_type_hash: Option<u64>,  // version>=19 才读
}

/// 读 blob 格式 TypeTree。返回 (扁平节点列表, 字符串缓冲)。
/// 单节点磁盘布局 `{endian}hBBIIiii[Q]`（version>=19 加 u64 ref_type_hash），
/// 定长 24（或 32）字节。对应 UnityPy `_get_blob_node_struct`。
pub fn parse_blob(r: &mut Cursor, file_ver: u32) -> Result<(Vec<BlobNode>, Vec<u8>)> { /* … */ }

/// 把扁平节点列表按 `level` 还原成 `TypeNode` 嵌套树（递归）。
/// `string_buffer` 是 AB TypeTree 自带的字符串缓冲区；
/// bit31=1 的偏移走全局 CommonString 表（见 §3.4）。
pub fn build_tree(flat: &[BlobNode], str_buf: &[u8]) -> Result<TypeNode> { /* … */ }
```

`build_subtree` 内部按 `level` 递归：当前节点 `level == expected_level`，随后所有
`level > expected_level` 的连续节点归为子树。类型名/字段名通过 `resolve_str` 解析：

```rust
/// 解析 AB 字符串偏移：bit31=0 本地 buffer null-terminated；bit31=1 CommonString 索引。
fn resolve_str(offset: u32, str_buf: &[u8]) -> String {
    if offset & 0x8000_0000 == 0 {
        let start = offset as usize;
        let end = str_buf[start..].iter().position(|&b| b == 0)
            .map(|p| start + p).unwrap_or(str_buf.len());
        String::from_utf8_lossy(&str_buf[start..end]).into_owned()
    } else {
        common_string::get(offset & 0x7FFF_FFFF).to_owned()
    }
}
```

### 3.4 CommonString 表

Unity 硬编码的 ~113 个全局共享类型/字段名。AB TypeTree 用它去重常见字符串。
实现为模块级自由函数 + 静态数组，不需要封装成 struct：

```rust
/// 按偏移取共享字符串；未命中返回空串。
pub fn get(offset: u32) -> &'static str {
    TABLE.iter()
        .find(|(k, _)| *k == offset)
        .map(|(_, v)| *v)
        .unwrap_or("")
}

static TABLE: &[(u32, &str)] = &[
    (0, "AABB"),   (5, "AnimationClip"),   (19, "AnimationCurve"),
    (34, "AnimationState"),  (49, "Array"),  (55, "Base"),
    // ... 完整 113 条，从 AssetRipper CommonString.cs 抄录
];
```

注意 offset 0 对应 `"AABB"`（非空串）——与 AssetRipper 一致。

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
            kind: TypeKind::from_unity(raw_type),
            name: raw_name.to_owned(),
            byte_size: node.byte_size,
            version: node.node_version,
            meta_flag: node.meta_flag,
            children: node.sub_nodes.iter()
                .map(|&i| Self::from_tpk(i, pool, strings))
                .collect(),
        }
    }
}
```

`type_flags` 不存进 `TypeNode`（解释器用 `is_sequence()` 判集合，不需要它）。

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
    Pair(Box<Value>, Box<Value>),
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

解释器用具体 `Cursor`（不用泛型 trait），`refs` 是 `[SerializeReference]` 引用类型查表。
走完后校验 `cursor.pos() == data.len()`，不匹配报 `TypeTreeSizeMismatch`（版本不匹配的强信号）。

```rust
/// 解释器上下文（Copy，因为只含不可变引用 + 一个 bool 标志）。
#[derive(Clone, Copy)]
struct Ctx<'a> {
    ver: &'a UnityVersion,
    refs: &'a RefTypeTable,
    has_registry: bool,
}

/// TypeTree 解释器：按 node 的 TypeNode 树递归读字节。
/// 用于 MonoBehaviour 和 typed parse 失败时的回退。
pub fn interpret(
    data: &[u8],
    root: &TypeNode,
    ver: &UnityVersion,
    endian: Endian,
    refs: &RefTypeTable,
) -> Result<Value> {
    let mut r = Cursor::new(data, endian);
    let ctx = Ctx { ver, refs, has_registry: false };
    let v = walk(&mut r, root, ctx)?;
    if r.pos() != data.len() {
        return Err(TypeTreeSizeMismatchSnafu { expected: data.len(), actual: r.pos() }.build());
    }
    Ok(v)
}

fn walk(r: &mut Cursor, node: &TypeNode, ctx: Ctx) -> Result<Value> {
    let mut align = node.meta_flag & ALIGN_MASK != 0;

    let v = match &node.kind {
        TypeKind::Bool  => Value::Bool(r.read_bool()?),
        TypeKind::U8    => Value::U8(r.read_u8()?),
        TypeKind::I8    => Value::I8(r.read_i8()?),
        TypeKind::U16   => Value::U16(r.read_u16("u16")?),
        TypeKind::I16   => Value::I16(r.read_i16("i16")?),
        TypeKind::U32   => Value::U32(r.read_u32("u32")?),
        TypeKind::I32   => Value::I32(r.read_i32("i32")?),
        TypeKind::U64   => Value::U64(r.read_u64("u64")?),
        TypeKind::I64   => Value::I64(r.read_i64("i64")?),
        TypeKind::F32   => Value::F32(r.read_f32("f32")?),
        TypeKind::F64   => Value::F64(r.read_f64("f64")?),

        TypeKind::String => {
            let n = r.read_i32("string len")? as usize;
            let b = r.read_bytes(n, "string data")?;
            let s = String::from_utf8_lossy(b).into_owned();
            r.align(4);                         // ← 字符串数据后无条件对齐到 4
            Value::String(s)
        }
        TypeKind::TypelessData => {
            let n = r.read_i32("typeless len")? as usize;
            Value::Bytes(r.read_bytes(n, "typeless data")?.to_vec())
        }

        // pair → 两个子节点
        TypeKind::Pair => {
            let first = walk(r, &node.children[0], ctx)?;
            let second = walk(r, &node.children[1], ctx)?;
            Value::Pair(Box::new(first), Box::new(second))
        }

        TypeKind::PPtr(_) => {
            let file_id = r.read_i32("pptr file_id")?;
            let path_id = if *ctx.ver >= UnityVersion::new(5, 0, 0) {
                r.read_i64("pptr path_id")?
            } else {
                r.read_i32("pptr path_id")? as i64
            };
            Value::PPtr(PPtrValue { file_id, path_id })
        }

        TypeKind::Struct(_) => {
            if node.is_sequence() {
                // vector / staticvector / set / map / Array：[size, N×element]
                let array_wrapper = &node.children[0];   // "Array" 子节点
                if array_wrapper.meta_flag & ALIGN_MASK != 0 {
                    align = true;
                }
                let size = r.read_i32("array size")?;
                let elem = &array_wrapper.children[1];   // [0]=size, [1]=data
                let mut out = Vec::with_capacity((size as usize).min(4096));
                for _ in 0..size {
                    out.push(walk(r, elem, ctx)?);
                }
                Value::Array(out)
            } else {
                read_struct(r, node, ctx)?
            }
        }
    };

    if align {
        r.align(4);
    }
    Ok(v)
}
```

`read_struct` 遍历子节点逐个 `walk`，产出 `Value::Struct(Vec<(String, Value)>)`；
其中对 `ManagedReferencesRegistry`（同一对象只读一次）和
`ReferencedObjectData`（用已读的 `type{class,ns,asm}` 兄弟字段查 `refs`）做特判。

**对齐说明**：对齐来源有两个——①`meta_flag & 0x4000`（节点自身/集合的 Array 子节点）；
②String 数据后无条件对齐到 4（Unity 字符串固定如此）。递归调用天然保证子节点对齐在
子节点自己的 `walk` 里处理。

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
let (flat, str_buf) = parse_blob(&mut cursor, sf.version)?;
let tree = build_tree(&flat, &str_buf)?;
let value = interpret(data, &tree, ver, endian, &refs)?;

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
/// `ab_type_tree` 是 AB 内嵌 TypeTree（已构建好的 `TypeNode` 根节点，如果有）。
pub fn parse_object(
    data: &[u8],
    class_id: i32,
    ver: &UnityVersion,
    endian: Endian,
    refs: &RefTypeTable,
    ab_type_tree: Option<&TypeNode>,
) -> Result<ParsedObject, ParseError> {

    // ── MonoBehaviour：直接走 TypeTree ──
    if class_id == 114 {
        let tree = ab_type_tree.ok_or(ParseError::MonoBehaviourWithoutTypeTree)?;
        let value = interpret(data, tree, ver, endian, refs)?;
        return Ok(ParsedObject::Dynamic(value));
    }

    // ── 其他类型：先试 typed parse ──
    match crate::generated::parse_typed(class_id, data, ver, endian) {
        Ok(typed) => Ok(ParsedObject::Typed(typed)),
        Err(typed_err) => {
            // 回退到 TypeTree 解释器
            if let Some(tree) = ab_type_tree {
                log::warn!(
                    "classID {class_id}: typed parse failed ({typed_err}), \
                     falling back to TypeTree interpreter"
                );
                let value = interpret(data, tree, ver, endian, refs)?;
                Ok(ParsedObject::Dynamic(value))
            } else {
                // AB 没有 TypeTree，无法回退
                Err(typed_err)
            }
        }
    }
}
```

### 6.3 ParsedObject — 统一返回类型

```rust
/// 解析结果：typed struct 或动态 Value。
pub enum ParsedObject {
    /// codegen'd typed struct（大多数原生类型）
    Typed(UnityObject),
    /// TypeTree 解释器产出的动态值（MonoBehaviour + fallback）
    Dynamic(Value),
}
```

用户 match：

```rust
match parse_object(data, class_id, ver, endian, &refs, ab_tree)? {
    ParsedObject::Typed(UnityObject::Texture2D(tex)) => {
        println!("{}x{} {}", tex.m_width, tex.m_height, tex.m_name);
    }
    ParsedObject::Dynamic(value) => {
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

## 7. 现状与目标

### 7.1 已实现（✅）

| 模块 | 位置 | 说明 |
|---|---|---|
| TypeTree 解释器 | `src/typetree/` | `TypeKind` + `TypeNode` + `Value` + `interpret()`（5 参，含 `&RefTypeTable`）+ `blob.rs` + `common_string.rs` + `RefTypeTable` |
| SerializedFile 解析 | `src/serialized_file.rs` | 读 header / types / objects / ref_types，每个 type 可带内嵌 TypeNode |
| Cursor | `src/cursor.rs` | 大/小端切换游标，`read_*` + `align` |
| UnityVersion | `src/version.rs` | `{ major, minor, patch }` + `parse("2017.4.39f1")` |
| Bundle 解包 | `src/bundle.rs` + `src/decompress.rs` | UnityFS + LZ4 |
| codegen | `crates/codegen/` | TPK reader + 7 pass + emit |
| generated structs | `crates/generated/` | ~412 class + ~903 subclass，含 `Parse` trait + `EndianReader` trait + `UnityObject` enum |

### 7.2 待实现（⬜）

| 改动 | 位置 | 说明 |
|---|---|---|
| **抽 unity-core crate** | `crates/unity-core/` | 共享 `UnityVersion` + `EndianReader` + `Parse` + `PPtr`；src/generated 都依赖它 |
| **移除 MonoBehaviour + parent** | `crates/generated/` + `crates/codegen/` | 不生成 MB struct；不生成 parent 字段/enum/抽象类空壳 |
| **impl EndianReader for Cursor** | `src/cursor.rs` | `impl unity_core::EndianReader for Cursor` |
| **`object_parser.rs`** | `src/object_parser.rs` | dispatch：MonoBehaviour→TypeTree，其他→typed，失败→fallback |
| **依赖 generated + unity-core** | `Cargo.toml` | 主 crate 加两个 dependency |

---

## 8. Parse 签名：不改

之前的设计稿提议把 `Parse` trait 从 `&mut R` 改为 `&[u8]`。**不需要**——fallback 用 `&[u8]` 切片各自创建 Cursor 即可：

```rust
// generated crate 的 Parse trait 来自 unity-core，签名不变
// unity_core::Parse: fn parse<R: EndianReader>(r: &mut R, ver: &UnityVersion) -> Result<Self, R::Error>

// 主 crate 的 dispatch：两条路径各自 new Cursor
fn try_typed(class_id, data: &[u8], ver, endian) -> Result<UnityObject, Error> {
    let mut cursor = Cursor::new(data, endian);  // Cursor 实现 unity_core::EndianReader
    UnityObject::parse(class_id, &mut cursor, ver)  // ver 已是 unity_core::UnityVersion
    //   ↑ 失败时丢弃 cursor，data 切片完好
}

fn try_dynamic(data: &[u8], tree: &TypeNode, ver, endian) -> Result<Value, Error> {
    interpret(data, tree, ver, endian, &refs)
    //   ↑ interpret 内部自己 new Cursor，不依赖上面的 cursor
}
```

`data: &[u8]` 是不可变引用，typed parse 和 interpret 各自消费自己的 Cursor。**zero-cost fallback**。
UnityVersion 不再需要桥接——主 crate 和 generated 共用 `unity_core::UnityVersion`。

---

## 9. 最终 Crate 结构

```
unity-asset-rs/                         # 主 crate（bin + lib）
├── Cargo.toml                          # deps: bytes, lz4, snafu, unity-core, generated
├── src/
│   ├── lib.rs                          # 公开 API
│   ├── bundle.rs                       # ✅ UnityFS 解包
│   ├── cursor.rs                       # ✅ Cursor + Endian  →  ⬜ +impl EndianReader
│   ├── decompress.rs                   # ✅ 解压注册表
│   ├── error.rs                        # ✅ 错误类型
│   ├── serialized_file.rs              # ✅ SerializedFile → objects + types + ref_types
│   ├── version.rs                      # 删除（改用 unity_core::UnityVersion）
│   ├── object_parser.rs                # ⬜ dispatch + fallback（唯一新增文件）
│   └── typetree/                       # ✅ TypeTree 解释器（已完成）
│       ├── mod.rs                      #    TypeKind, TypeNode, Value, PPtrValue, RefTypeTable
│       ├── blob.rs                     #    AB TypeTree blob → 扁平节点 → 树
│       ├── common_string.rs            #    113 条 CommonString 硬编码表
│       └── interpret.rs                #    interpret() 递归走树
│
└── crates/
    ├── unity-core/                     # ⬜ 新增：共享 crate（~150 行，零依赖）
    │   └── src/
    │       ├── lib.rs                  #    re-export
    │       ├── version.rs              #    UnityVersion { major, minor, build } + parse()
    │       ├── reader.rs               #    Endian, EndianReader trait, Parse trait
    │       └── pptr.rs                 #    PPtr { m_file_id, m_path_id }
    │
    ├── codegen/                        # 离线工具（微调：去掉 parent 逻辑）
    │   └── src/                        #    TPK reader + pipeline（无 ParentSpec）+ emit
    │
    └── generated/                      # ⬜ 微调：移除 MonoBehaviour + parent + 抽象类空壳
        ├── Cargo.toml                  #    deps: unity-core
        └── src/
            ├── lib.rs                  #    use unity_core::*; + 模块声明
            ├── class_id.rs             #    enum ClassIDType
            ├── unity_object.rs         #    enum UnityObject（无 MB，无抽象类）
            ├── classes/                #    扁平 struct（无 parent 字段）
            └── subclasses/             #    Vector3f, StreamingInfo, ...
```

### 依赖关系

```
crates/unity-core     ←──零依赖──  (~150 行)
       ↑          ↑
       │          │
       │   crates/generated       (depends on unity-core)
       │          ↑
       │          │
       │   src/ (主 crate)         (depends on unity-core + generated)
       │
crates/codegen                   (不依赖 unity-core，只生成引用它的代码)
```

---

## 10. object_parser.rs — dispatch 设计

这是主 crate 里**唯一新增**的文件，负责把 SerializedFile 的每个 object 路由到正确的 parser。

```rust
//! src/object_parser.rs
//! 对象解析 dispatch：MonoBehaviour → TypeTree，其他 → typed，失败 → fallback。

use crate::cursor::{Cursor, Endian};
use crate::error::{Result, Error};
use crate::serialized_file::{ObjectInfo, SerializedFile};
use crate::typetree::{self, interpret, RefTypeTable, TypeNode, Value};
use crate::version::UnityVersion;
use generated::{ClassIDType, UnityObject};

/// 解析结果。
pub enum ParsedObject {
    /// codegen'd typed struct（Texture2D, Mesh, ...）
    Typed(UnityObject),
    /// TypeTree 解释器产出（MonoBehaviour 或 fallback）
    Dynamic(Value),
}

/// 解析单个对象。
///
/// 由 `SerializedFile::iter_objects()` 的调用方逐个调用。
pub fn parse_object(
    sf: &SerializedFile,
    obj: &ObjectInfo,
    refs: &RefTypeTable,
) -> Result<ParsedObject> {
    let data = obj.data(sf);
    let class_id = obj.class_id(sf);
    let ver = &sf.unity_version_parsed;
    let endian = sf.endian;

    // 获取该对象类型的 AB 内嵌 TypeTree（如果有）
    let ab_tree = sf.types.get(obj.type_idx)
        .and_then(|t| t.node.as_ref());

    // ── MonoBehaviour (classID=114)：直接走 TypeTree ──
    if class_id == 114 {
        let tree = ab_tree.ok_or(Error::MonoBehaviourWithoutTypeTree)?;
        let value = interpret(data, tree, ver, endian, refs)?;
        return Ok(ParsedObject::Dynamic(value));
    }

    // ── 其他类型：先试 typed parse ──
    match try_typed(class_id, data, ver, endian) {
        Ok(typed) => Ok(ParsedObject::Typed(typed)),
        Err(typed_err) => {
            // 回退到 TypeTree
            if let Some(tree) = ab_tree {
                log::warn!("classID {class_id}: typed parse failed, falling back to TypeTree");
                let value = interpret(data, tree, ver, endian, refs)?;
                Ok(ParsedObject::Dynamic(value))
            } else {
                Err(typed_err)
            }
        }
    }
}

/// 尝试 codegen'd typed parse。内部创建 Cursor。
fn try_typed(
    class_id: i32,
    data: &[u8],
    ver: &UnityVersion,
    endian: Endian,
) -> Result<UnityObject> {
    let class = ClassIDType::from_i32(class_id)
        .ok_or(Error::UnknownClassId { id: class_id })?;

    let mut cursor = Cursor::new(data, endian);
    let ver_g = generated::UnityVersion::new(ver.major, ver.minor, ver.patch);
    let obj = UnityObject::parse(class, &mut cursor, &ver_g)
        .map_err(|e| Error::TypedParse { class_id, source: Box::new(e) })?;
    Ok(obj)
}
```

### 用户调用端

```rust
let bundle = load("char_002_amiya.ab")?;
let sf = SerializedFile::parse(&bundle.files[0].bytes, &bundle.files[0].name)?;
let refs = sf.ref_type_table();

for obj in &sf.objects {
    match parse_object(&sf, obj, &refs)? {
        ParsedObject::Typed(UnityObject::Texture2D(tex)) => {
            println!("{}x{} {}", tex.m_width, tex.m_height, tex.m_name);
        }
        ParsedObject::Typed(UnityObject::Mesh(mesh)) => { /* ... */ }
        ParsedObject::Dynamic(value) => {
            // MonoBehaviour 或 fallback
            if let Value::Struct(fields) = value {
                for (name, val) in &fields {
                    println!("  {name}: {val:?}");
                }
            }
        }
        _ => {}
    }
}
```

---

## 11. 具体改动清单

### 11.1 `crates/unity-core/` — 新增共享 crate

抽出 `UnityVersion` + `Endian` + `EndianReader` trait + `Parse` trait + `PPtr`。
主 crate 的 `src/version.rs` 删除，改用 `unity_core::UnityVersion`。
generated 的 `lib.rs` 改为 `pub use unity_core::*;`。

### 11.2 `crates/generated/` + `crates/codegen/` — 移除 MonoBehaviour + parent

| 文件 | 改动 |
|---|---|
| `crates/codegen/src/ir.rs` | 删除 `ParentSpec` enum；`EmitClass` 删除 `parent` 字段，`own_fields` 改名 `fields` |
| `crates/codegen/src/pipeline.rs` | p04 不算继承（改名 `create_class_skeletons`）；p05 不变（字段已扁平） |
| `crates/codegen/src/emit.rs` | 不 emit parent 字段 / parent enum / parent parse 调用；跳过 `is_abstract`/`is_mono_behaviour` |
| `src/unity_object.rs` | 删除 MonoBehaviour variant + match arm；删除抽象类 variant |
| `src/classes/mono_behaviour.rs` | 删除 |
| `src/classes/object.rs` `named_object.rs` `texture.rs` 等 | 删除（抽象类空壳不生成） |
| `src/class_id.rs` | 保留 `MonoBehaviour = 114`（运行时识别用） |

### 11.3 `src/cursor.rs` — impl EndianReader

```rust
impl unity_core::EndianReader for Cursor<'_> {
    type Error = crate::error::Error;
    // ... 各 read 方法委托给 Cursor 现有方法 ...
}
```

### 11.4 `src/object_parser.rs` — 新增

见 §10 的完整代码。约 80 行。

### 11.5 `Cargo.toml` — 加依赖

```toml
[dependencies]
unity-core = { path = "crates/unity-core" }
generated = { path = "crates/generated" }
```

---

## 12. 与 codegen-design.md / codegen-implementation.md 的关系

| 维度 | codegen-design.md | codegen-implementation.md | 本文档 |
|---|---|---|---|
| 角色 | 原始设计稿 | 当前实现记录 | **parse 路径的目标设计** |
| Parse trait | `&mut R: EndianReader` | 同左 | **不改**（保持 `&mut R`） |
| MonoBehaviour | 最小骨架 + token | 生成了空壳 struct | **不生成**，从 codegen 排除 |
| dispatch | 在 generated 里 | 在 generated `unity_object.rs` | **移到主 crate** `object_parser.rs` |
| TypeTree 解释器 | 未涉及 | 未涉及 | ✅ 已实现 `src/typetree/` |
| fallback | 不存在 | 不存在 | try typed → fail → try TypeTree |

**codegen 的 struct 生成逻辑**（命名、字段并集、版本门控、对齐）不受影响。
唯一变化：codegen 跳过 MonoBehaviour，不再为其生成 struct。

---

## 13. 实施步骤（增量）

| 步骤 | 改动 | 验证 |
|---|---|---|
| **S1** | `crates/unity-core/` 新建（UnityVersion + traits + PPtr） | 编译通过 |
| **S2** | `crates/generated/` 改依赖 unity-core + 去掉重复定义 | `cargo build -p generated` 通过 |
| **S3** | codegen 去 parent（ir.rs + pipeline.rs + emit.rs）+ 去 MB + 重跑 | `cargo build --workspace` 通过 |
| **S4** | `src/cursor.rs` 加 `impl unity_core::EndianReader for Cursor` | 编译通过 |
| **S5** | `Cargo.toml` 加 unity-core + generated 依赖 | `cargo build` 通过 |
| **S6** | `src/object_parser.rs`：dispatch + fallback | 单元测试（mock data） |
| **S7** | 端到端：`char_002_amiya.ab` | 全部 object 解出，字段值对比 UnityPy |

---

## 14. 设计取舍记录

### 为什么不做 TypeTree 比较？

考虑过比较 AB TypeTree 和 TPK TypeTree，一致走 typed，不一致走 dynamic。放弃：

1. **复杂度**：需要加载 TPK 到 runtime + 树比较 + prefix match
2. **收益低**：typed parse 失败的情况很少
3. **Fallback 更简单**：try-catch，无额外数据结构

### 为什么 MonoBehaviour 不 codegen？

脚本字段不在 TPK 里。即使 codegen 基类字段，脚本字段仍需 TypeTree 解释器。
统一用 TypeTree，一步到位。

### 为什么不改 Parse trait 签名？

`&mut R: EndianReader` 和 `&[u8]` 可以共存：dispatch 层持有 `&[u8]`，
两条路径各自 `Cursor::new(data, endian)` 创建独立 cursor。
typed parse 失败后，`data` 切片完好，直接喂给 interpret。不需要改任何生成代码。

### 为什么 dispatch 在主 crate 而不在 generated？

generated 是纯数据 crate（struct + Parse impl），不依赖 TypeTree 解释器。
dispatch 需要 import 两侧（generated 的 UnityObject + typetree 的 interpret），
放主 crate 最自然。generated 只依赖 unity-core，保持独立编译缓存。

### 为什么抽 unity-core？

之前 UnityVersion 有两份（主 crate `{major,minor,patch}` 和 generated `{major,minor,build}`），
EndianReader trait 也在 generated 里定义、主 crate 需要 impl。抽出 unity-core 后：
- 消除重复定义和类型不匹配
- generated 只依赖 unity-core（~150 行零依赖），编译缓存更稳定
- 主 crate 依赖 unity-core + generated，职责清晰
