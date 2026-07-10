# 对象解析层（core 层）

> 范围：从 io 层的 `BundleFile.bytes` 解析 SerializedFile（header / types[含内嵌
> typetree] / objects / ref_types），并把每个对象解码——codegen'd typed struct 或
> TypeTree 解释器，含 fallback。
> 状态：**本文档描述已落地的实现**（与 `src/serialized_file.rs` + `src/typetree/` +
> `src/object_parser.rs` + `crates/generated/` 一致）。
> 上游：[ADR-0001](adr/0001-codegen-from-typetree-dumps.md)（codegen 决策）、
> [`io-layer.md`](io-layer.md)（本层输入）、[`codegen.md`](codegen.md)（typed struct 来源）。

---

## 1. 核心决策：Typed Codegen + TypeTree 解释器

| 对象类型 | 解析路径 | 数据来源 |
|---|---|---|
| MonoBehaviour (classID=114) | **TypeTree 解释器** | AB 内嵌 TypeTree |
| 其他原生类型 | **codegen'd typed parser** | TPK（离线烤进 Rust 代码） |
| typed parse 失败 | **回退到 TypeTree 解释器** | AB 内嵌 TypeTree（如果有） |

**不做 TPK vs AB TypeTree 比较**。直接尝试 typed parse，失败就回退。简单、无额外开销。

```
SerializedFile
  │  每个 object: (class_id, version, data: &[u8], ab_type_tree: Option<&TypeNode>)
  ▼
parse_object(sf, obj, refs)
├── class_id == 114 (MonoBehaviour)
│     → interpret(data, ab_tree, ver, endian, refs)        直接走 TypeTree
└── 其他
      → try_typed(class_id, data, ver, endian)
          ├─ Ok(typed)  → ParsedObject::Typed(UnityObject)
          └─ Err        → 有 ab_tree?
                            ├─ 是 → interpret(...)          fallback
                            └─ 否 → 返回原始错误
```

**为什么用 `&[u8]` 而不是 `&mut R: Read`**：fallback 需要在 typed parse 失败后重新从头
解析。用 `&[u8]` 切片，两条路径各自创建 Cursor，互不干扰。typed parse 失败后 `data`
切片完好，直接喂给 interpret——zero-cost fallback。

---

## 2. 流水线

```
load(path) → Bundle { files: Vec<BundleFile{ name, bytes }> }     [io 层]
   │
   ▼  对每个 BundleFile（.assets / CAB-*）；跳过 .resS
SerializedFile::parse(bytes)                                       [§3]
   │  BE prologue → endian 字节 → 切 endian → 读 types[+typetree] / objects / ref_types
   ▼
SerializedFile { version, unity_version, endian, types, ref_types, objects }
   │  每个 SerializedType.node = 内嵌 TypeNode 树
   ▼
parse_object(&sf, obj, &refs)                                      [§5]
   │  dispatch：MonoBehaviour → TypeTree；其他 → typed → fallback
   ▼
ParsedObject { Typed(UnityObject) | Dynamic(Value) }
```

`.resS` 是 streaming-resource 伴生文件（纹理/音频流数据），不是 SerializedFile，跳过
（见 `src/main.rs`）。

---

## 3. SerializedFile 二进制格式

实现位置：`src/serialized_file.rs::parse`。prologue 是**大端**，读到 endian 字节后
`cursor.set_endian(...)` 切换。

**已用 `char_002_amiya.ab` 探针实测确认**：version=17、file_size=57416（吻合总字节）、
data_offset=46832、enable_type_tree=1、endian 为 LE。

### 3.1 Header（参考 UnityPy `SerializedFile.py:254-269`）

```
── prologue（大端 4×u32）──
  u32 metadata_size
  u32 file_size
  u32 version            // format version（现代 17-22）
  u32 data_offset
── version >= 9 ──
  u8  endian             // 0=LE,1=BE  ← set_endian，后续元数据用它
  u8[3] reserved
  ── version >= 22 (LargeFilesSupport，覆盖前三个值）──
     u32 metadata_size ; i64 file_size ; i64 data_offset ; i64 unknown
── version < 9 ── seek(file_size - metadata_size) ; 读 endian 字节
```

### 3.2 元数据（用切换后的 endian）

```
ver>=7  cstr unity_version          // "2017.4.39f1"
ver>=8  i32  target_platform        // BuildTarget
ver>=13 bool enable_type_tree
i32 type_count ; SerializedType × count        // §3.3
7<=ver<14 i32 big_id_enabled
i32 object_count ; ObjectInfo × count          // §3.4
ver>=11 i32 script_count ; LocalSerializedObjectIdentifier × count
i32 external_count ; FileIdentifier × count
ver>=20 i32 ref_type_count ; SerializedType(is_ref=true) × count
```

### 3.3 SerializedType（参考 `SerializedFile.py:112-149`）

```
i32 class_id
ver>=16 bool is_stripped
ver>=17 i16 script_type_index
ver>=13:
  条件 C = (is_ref && script_type_index>=0) || (ver<16 && class_id<0) || (ver>=16 && class_id==114)
  if C: 16 字节 script_id (Hash128)
  16 字节 old_type_hash (Hash128)
enable_type_tree:
  ver>=12 || ver==10: parse_blob()          // §4.2
  else:               (legacy 递归格式，未实现 → SerializedFile 错误)
ver>=21:
  is_ref: cstr class_name, cstr namespace, cstr assembly_name   // ref 必需
  else:   i32[] type_dependencies
```

### 3.4 ObjectInfo（参考 `ObjectReader.from_reader:67-130`）

```
path_id:  big_id→i64 ; ver<14→i32 ; else { align(4); i64 }
byte_start: ver>=22→i64 else u32 ; += data_offset
byte_size: u32
type_id: i32
ver<16: u16 class_id ; 按 type_id 在 types 里查 type_idx
ver>=16: type_idx = type_id ; class_id = types[type_id].class_id
ver<11: u16 is_destroyed
11<=ver<17: i16 script_type_index
ver==15||16: u8 is_stripped
```

对象实际字节：`data[byte_start .. byte_start+byte_size]`（`byte_start` 已含 `data_offset`）。

### 3.5 SerializedFile 结构

```rust
pub struct SerializedFile {
    pub version: u32,                 // format version（现代 17-22）
    pub unity_version: String,
    pub unity_version_parsed: UnityVersion,
    pub endian: Endian,
    pub types: Vec<SerializedType>,   // 可能带内嵌 typetree 根节点
    pub ref_types: Vec<SerializedType>,  // [SerializeReference]，version >= 20
    pub objects: Vec<ObjectInfo>,
    data: Bytes,                      // 原始字节（对象切片来源）
}

pub struct SerializedType {
    pub class_id: i32,
    pub script_type_index: Option<i16>,
    pub node: Option<TypeNode>,       // 内嵌 typetree 根节点
    pub ref_class_name: Option<String>,
    pub ref_namespace: Option<String>,
    pub ref_assembly: Option<String>,
}

pub struct ObjectInfo {
    pub path_id: i64,
    pub byte_start: u64,              // 已含 data_offset
    pub byte_size: u32,
    pub type_id: i32,
    pub type_idx: usize,              // 指向 SerializedFile.types 的索引
}

impl ObjectInfo {
    pub fn class_id(&self, sf: &SerializedFile) -> i32;
    pub fn data<'a>(&self, sf: &'a SerializedFile) -> &'a [u8];
}

impl SerializedFile {
    pub fn parse(bytes: Bytes) -> Result<SerializedFile>;
    pub fn parse_object(&self, idx: usize) -> Result<Option<Value>>;  // 用内嵌 typetree
    pub fn ref_type_table(&self) -> RefTypeTable;
}
```

---

## 4. TypeTree 模块

实现位置：`src/typetree/`（`mod.rs` + `blob.rs` + `interpret.rs` + `common_string.rs`）。

### 4.1 TypeKind / TypeNode

`Array/Vector/Map` **不进 TypeKind**——Unity 把所有集合编码成相同结构（集合节点有唯一
`type="Array"` 子节点，该子节点固定 `[size:int, data:元素]`），解码算法唯一，靠结构
判别（`TypeNode::is_sequence()`），与 UnityPy `TypeTreeHelper.py:208` 一致。`map` 只是
元素为 `pair` 的 vector。

```rust
pub enum TypeKind {
    Bool, U8, I8, U16, I16, U32, I32, U64, I64, F32, F64,
    String,         // i32 len + utf8 + align(4)
    TypelessData,   // i32 len + u8[]
    Pair,
    PPtr(String),   // PPtr<T> → target（去 $ 前缀）
    Struct(String), // 其余（含 "vector"/"map"/"Array" 包装器）
}

pub struct TypeNode {
    pub kind: TypeKind,
    pub name: String,
    pub byte_size: i32,       // -1 = 变长
    pub version: i16,         // 节点 schema 版本
    pub meta_flag: u32,       // 0x4000 = align
    pub children: Vec<TypeNode>,
}

impl TypeNode {
    pub fn is_sequence(&self) -> bool;   // children[0].kind == Struct("Array")
    pub fn is_aligned(&self) -> bool;
}
```

`TypeKind::from_unity` 归一化别名表：`SInt32/int/Type*/EntityId→I32`、
`unsigned int/UInt32→U32`、`long long/SInt64→I64`、`PPtr<…>→PPtr` 等（见
`src/typetree/mod.rs`）。

### 4.2 blob 读取（参考 `TypeTreeNode.py:111-154`）

实现位置：`src/typetree/blob.rs`。

单节点磁盘布局 `{endian}hBBIIiii[Q]`（ver>=19 加 `u64 ref_type_hash`），定长 24（或 32）
字节。先读 `i32 node_count` + `i32 str_buf_size`，再批量读节点 + 字符串缓冲。

```rust
pub struct BlobNode {
    pub version: i16, pub level: u8, pub type_flags: u8,
    pub type_str_offset: u32, pub name_str_offset: u32,
    pub byte_size: i32, pub index: i32, pub meta_flag: u32,
    pub ref_type_hash: Option<u64>,  // version>=19 才读
}

pub fn parse_blob(r: &mut Cursor, file_ver: u32) -> Result<(Vec<BlobNode>, Vec<u8>)>;
pub fn build_tree(flat: &[BlobNode], str_buf: &[u8]) -> Result<TypeNode>;
```

`build_subtree` 按 `level` 递归：当前节点 `level == expected_level`，随后所有
`level > expected_level` 的连续节点归为子树。字符串偏移 `resolve_str`：`bit31=0`→本地
buffer null-terminated；`bit31=1`→CommonString 表索引（§4.4）。

### 4.3 interpret（参考 `TypeTreeHelper.read_value:182-263`）

实现位置：`src/typetree/interpret.rs`。

```rust
pub fn interpret(
    data: &[u8], root: &TypeNode, ver: &UnityVersion, endian: Endian, refs: &RefTypeTable,
) -> Result<Value>;
```

按 `TypeKind` 分派：原语直读；`String`/`TypelessData` 带 len；`Pair` 读两子节点；
`PPtr` 读 `file_id:i32` + `path_id`（`ver>=5`→i64 else i32）；`Struct` 若 `is_sequence()`
读 size+N 元素，否则 `read_struct` 遍历子节点。每节点末尾 `meta_flag & 0x4000` →
`align(4)`；Array 包装器的 meta_flag 会向上传染给集合节点。

**边界**：
- `ReferencedObject`（Unity 2021+ `[SerializeReference]`）：`read_struct` 中遇到
  `Struct("ReferencedObjectData")` 子节点时，用已读的 `type{class,ns,asm}` 兄弟字段在
  `RefTypeTable` 查到目标节点再走它（`extract_ref_type`）。
- `ManagedReferencesRegistry`：同一对象只读一次（`Ctx.has_registry` 标志）。
- **校验**：走完 root 后 `cursor.pos() == data.len()`，否则报 `TypeTreeSizeMismatch`
  （版本不匹配的强信号）。

**对齐说明**：对齐来源有两个——①`meta_flag & 0x4000`（节点自身/集合的 Array 子节点）；
②String 数据后无条件对齐到 4（Unity 字符串固定如此）。递归调用天然保证子节点对齐在
子节点自己的 `walk` 里处理。

### 4.4 CommonString

实现位置：`src/typetree/common_string.rs`。硬编码 113 条 `(u32 offset, &'static str)`，
从 AssetRipper `CommonString.cs` 抄录。

```rust
pub fn get(offset: u32) -> &'static str;  // 未命中返回空串
```

注意 offset 0 对应 `"AABB"`（非空串）——与 AssetRipper 一致。

### 4.5 Value + PPtrValue + RefTypeTable

```rust
pub enum Value {
    Bool(bool), I8(i8), U8(u8), I16(i16), U16(u16),
    I32(i32), U32(u32), I64(i64), U64(u64), F32(f32), F64(f64),
    String(String), Bytes(Vec<u8>),
    Pair(Box<Value>, Box<Value>),
    Array(Vec<Value>),              // vector/map/set
    Struct(Vec<(String, Value)>),   // 保序字段
    PPtr(PPtrValue),
}

pub struct PPtrValue { pub file_id: i32, pub path_id: i64 }

pub struct RefTypeTable { /* Vec<(class, ns, asm, TypeNode)> */ }
impl RefTypeTable {
    pub fn empty() -> Self;
    pub fn push(&mut self, class: String, ns: String, asm: String, node: TypeNode);
    pub fn lookup(&self, class: &str, ns: &str, asm: &str) -> Option<&TypeNode>;
}
```

`RefTypeTable` 由 `SerializedFile::ref_type_table()` 从 `ref_types` 构建。

---

## 5. dispatch：`object_parser.rs`（已实现）

实现位置：`src/object_parser.rs`。这是 typed parse 和 TypeTree 解释器两套路径的汇合点。

```rust
pub enum ParsedObject {
    Typed(UnityObject),          // codegen'd typed struct（Texture2D, Mesh, ...）
    Dynamic(Value),              // typed parse 失败回退到 TypeTree
    MonoBehaviour(MonoBehaviour), // classID=114：typed shell + 动态 structure
}

pub fn parse_object(
    sf: &SerializedFile,
    obj: &ObjectInfo,
    refs: &RefTypeTable,
) -> Result<ParsedObject>;
```

### 5.1 dispatch 逻辑

1. 取 `data = obj.data(sf)`、`class_id`、`ver`、`endian`、`ab_tree`（该 type 的内嵌
   TypeNode）。
2. **MonoBehaviour（class_id == 114）**：走 `interpret_mono_behaviour`——shell（引擎
   基类字段）typed 投影 + structure（用户脚本字段）动态 `Value` 分段。无 `ab_tree` →
   `Error::MonoBehaviourWithoutTypeTree`。详见 §8。
3. **其他类型**：`try_typed`：
   - `ClassIDType::from_i32(class_id)` 不认识 → `UnknownClassId`。
   - `UnityObject::parse(class, &mut Cursor, ver)` 失败 → fallback 到 TypeTree（有
     `ab_tree` 时）；无 `ab_tree` 则返回原始错误。

### 5.2 try_typed

```rust
fn try_typed(class_id: i32, data: &[u8], ver: &UnityVersion, endian: Endian)
    -> Result<UnityObject>
{
    let class = ClassIDType::from_i32(class_id).ok_or(Error::UnknownClassId { id: class_id })?;
    let mut cursor = Cursor::new(data, endian);
    let obj = UnityObject::parse(class, &mut cursor, ver)?;
    Ok(obj)
}
```

两条路径各自 `Cursor::new(data, endian)` 创建独立 cursor。typed parse 失败后 `data`
切片完好，直接喂给 `interpret`。**zero-cost fallback**。

### 5.3 Fallback 触发条件

| 原因 | typed parse | AB TypeTree 回退 |
|---|---|---|
| 未知 class_id（不在 codegen 里） | 直接 Err | ✅ 如果 AB 有 TypeTree |
| Unity 版本不在 TPK 覆盖范围 | 字段门控错（读到 EOF / 多余字节）| ✅ |
| 引擎魔改（如明日方舟定制 Unity） | 字段 layout 不匹配 | ✅ |
| 数据损坏 | Err | 也会 Err（数据真坏了）|

### 5.4 用户调用端

```rust
let bundle = load("char_002_amiya.ab")?;
for bf in &bundle.files {
    if bf.name.ends_with(".resS") { continue; }   // 跳过 streaming resource
    let sf = SerializedFile::parse(bf.bytes.clone())?;
    let refs = sf.ref_type_table();
    for obj in &sf.objects {
        match parse_object(&sf, obj, &refs)? {
            ParsedObject::Typed(UnityObject::Texture2D(tex)) => {
                println!("{}x{} {}", tex.m_width, tex.m_height, tex.m_name);
            }
            ParsedObject::Typed(UnityObject::Mesh(mesh)) => { /* ... */ }
            ParsedObject::MonoBehaviour(mb) => {
                // typed shell：m_game_object / m_enabled / m_script / m_name
                println!("MonoBehaviour: {}", mb.m_name);
                // 动态 structure：用户脚本字段，按名字 pattern-match
                if let Value::Struct(fields) = &mb.structure {
                    for (name, val) in fields { println!("  {name}: {val:?}"); }
                }
            }
            ParsedObject::Dynamic(value) => {
                // typed parse 失败回退到 TypeTree
                if let Value::Struct(fields) = value {
                    for (name, val) in fields { println!("  {name}: {val:?}"); }
                }
            }
            _ => {}
        }
    }
}
```

完整示例见 `src/main.rs`：`list` 子命令对 bundle 里全部对象跑 parse，标注
typed/monobehaviour/dynamic/失败并统计汇总；`object` 子命令 dump 单个对象的字段。

---

## 6. 共享 crate：unity-core

实现位置：`crates/unity-core/`（~150 行，零外部依赖）。主 crate、generated、codegen 共
用，消除类型重复定义。

```rust
// crates/unity-core/src/reader.rs
pub enum Endian { Little, Big }

/// 字节序 reader 抽象。生成代码通过此 trait 读字节，
/// 实际实现由主 crate 的 Cursor 提供（impl EndianReader for Cursor）。
pub trait EndianReader {
    type Error;
    fn read_bool(&mut self) -> Result<bool, Self::Error>;
    fn read_i8(&mut self) -> Result<i8, Self::Error>;
    fn read_u8(&mut self) -> Result<u8, Self::Error>;
    fn read_i16(&mut self) -> Result<i16, Self::Error>;
    fn read_u16(&mut self) -> Result<u16, Self::Error>;
    fn read_i32(&mut self) -> Result<i32, Self::Error>;
    fn read_u32(&mut self) -> Result<u32, Self::Error>;
    fn read_i64(&mut self) -> Result<i64, Self::Error>;
    fn read_u64(&mut self) -> Result<u64, Self::Error>;
    fn read_f32(&mut self) -> Result<f32, Self::Error>;
    fn read_f64(&mut self) -> Result<f64, Self::Error>;
    fn read_bytes(&mut self, n: usize) -> Result<Vec<u8>, Self::Error>;
    fn read_aligned_string(&mut self) -> Result<String, Self::Error>;
    fn align(&mut self, n: usize) -> Result<(), Self::Error>;
    fn make_error(&self, msg: &str) -> Self::Error;  // 生成代码不支持的 class_id 用
}

/// 解析 trait。每个生成的 struct 实现它。
pub trait Parse: Sized {
    fn parse<R: EndianReader>(r: &mut R, ver: &UnityVersion) -> Result<Self, R::Error>;
}
```

`crates/unity-core/src/version.rs` 的 `UnityVersion { major, minor, build }` 带
`parse("2017.4.39f1")` + 偏序比较；`src/pptr.rs` 的 `PPtr { m_file_id, m_path_id }`
非 generic（`m_path_id` 永远按 i64 读，老版本分支留给未来）。

### 6.1 Cursor 桥接

主 crate 的 `Cursor`（`src/cursor.rs`）`impl unity_core::EndianReader for Cursor`，
把各 `read_*` 委托给 Cursor 现有方法。这是 generated 代码能读字节的实际入口。

`Cursor` 本身：可切-endian、带边界检查、`read_*` + `align` + `read_cstr` +
`read_aligned_string`。io 层用 `Endian::Big`（UnityFS 大端），SerializedFile 读到 endian
字节后切换，TypeTree 解释器继承 SerializedFile 的 endian。

---

## 7. 公共 API

主 crate `src/lib.rs` 暴露的对象解析相关类型：

```rust
pub use serialized_file::{ObjectInfo, SerializedFile, SerializedType};
pub use typetree::{build_tree, interpret, parse_blob, BlobNode, PPtrValue,
                   RefTypeTable, TypeKind, TypeNode, Value};
pub use unity_core::{Endian, PPtr, UnityVersion};
pub use generated::{ClassIDType, UnityObject};
pub use object_parser::{parse_object, ParsedObject};
```

- `SerializedFile::parse` / `parse_object(idx)` / `ref_type_table()` — SerializedFile 解析。
- `parse_object(sf, obj, refs)` / `ParsedObject` — dispatch 入口。
- `interpret(data, root, ver, endian, refs)` / `Value` — TypeTree 解释器（可独立使用）。
- `UnityObject` / `ClassIDType` — 来自 `crates/generated`（见 [`codegen.md`](codegen.md)）。

io 层入口（`load` / `load_bytes` / `load_with` / `load_bytes_with` / `Bundle` /
`BundleFile` / `Registry` / `Decompressor*`）见 [`io-layer.md`](io-layer.md)。

---

## 8. MonoBehaviour 处理

MonoBehaviour **不生成 codegen'd struct**（见 [`codegen.md`](codegen.md) §1 设计约束 3），
始终走 TypeTree 解释器。原因：用户脚本字段不在 TPK 里（TPK 只有引擎基类字段），脚本字段
每次都不同；AB 内嵌 TypeTree 是其唯一来源。

### 8.1 shell + structure 分段（对齐 AssetRipper）

参考 AssetRipper 的壳子/结构分离设计（见
`docs/assetripper-monobehaviour-handling.md`）：MonoBehaviour = **shell**（引擎基类
字段，typed）+ **structure**（用户脚本字段，动态）。

- **shell**：`m_GameObject` / `m_Enabled` / `m_Script` / `m_Name` / `m_EditorClassIdentifier`
  （可选）。投影成 typed `MonoBehaviour` struct 字段（`PPtrValue`/`bool`/`String`）。
- **structure**：TypeTree 里 shell 之后的剩余字段，保持动态 `Value`（`Value::Struct`），
  调用方按名字 pattern-match 访问。

实现：`interpret_mono_behaviour`（`src/typetree/interpret.rs`）在 TypeTree 节点层找
shell/structure 分界（`find_mono_behaviour_split`：顺序匹配引擎字段名集合，首个不匹配
节点即 structure 起点），用共享 `Cursor` 分两段 `read_children`——对齐 AssetRipper
`FindStartingIndexForMonoBehaviour` + `SerializableStructure.Read`。TypeTree 路径下
MonoScript **不参与**布局推理——布局已在 TypeTree 里；`m_script` 仅作后续解引用到
MonoScript 资产的寻址入口。

```rust
pub struct MonoBehaviour {
    pub m_game_object: PPtrValue,
    pub m_enabled: bool,
    pub m_script: PPtrValue,
    pub m_name: String,
    pub m_editor_class_identifier: Option<String>,
    pub structure: Value,   // 用户脚本字段（动态）
}
```

### 8.2 错误处理

- 无 AB TypeTree → `Error::MonoBehaviourWithoutTypeTree`（非 TypeTree 路径 = 未来工作，
  需 MonoScript + 托管 DLL 反射还原布局，本次未实现）。
- shell 字段缺失/类型错（`m_GameObject`/`m_Script`）→ `Error::MonoBehaviourShell`
  （强信号，shell 必须存在）。`m_Enabled`/`m_Name`/`m_EditorClassIdentifier` 缺失时
  取默认值（false / 空串 / None）。

### 8.3 Environment 访问

```rust
// 按 class_id == 114 过滤（codegen 未生成 ClassIDType::MonoBehaviour，不走 objects_of_class）
for o in env.mono_behaviours() {
    let mb = o.as_mono_behaviour().unwrap();
    // m_script 已实现 PPtrLike，可直接解引用到 MonoScript：
    let script = env.resolve_from(o, &mb.m_script);
}
```

---

## 9. 错误类型

对象解析相关错误见 `src/error.rs`：

- `SerializedFile { reason }` — SerializedFile 解析失败。
- `TypeTreeSizeMismatch { expected, actual }` — 解释器走完后 pos 校验不通过（版本不匹配强信号）。
- `NegativeArrayLen { len }` — 数组长度 < 0。
- `RefTypeNotFound { class, ns, asm }` — SerializeReference 查表未命中。
- `EmptyTypeTree` — 空 typetree。
- `TypeTree { reason }` — typetree 其它错误。
- `TypedParse { reason }` — generated typed parse 失败。
- `UnknownClassId { id }` — class_id 不在 `ClassIDType`。
- `MonoBehaviourWithoutTypeTree` — MonoBehaviour 无内嵌 TypeTree。
- `MonoBehaviourShell { field, reason }` — MonoBehaviour shell 字段缺失或类型错。

---

## 10. 路线图（未实现）

| 项 | 设计要点 | 状态 |
|---|---|---|
| ~~**World 聚合层**~~ → **Environment 聚合层** | eager parse 全部对象 + `.resS` resources + PPtr 解引用。已落地为 `Environment`，详见 [`environment-layer.md`](environment-layer.md) | ✅ |
| ~~**PPtr 解引用**~~ | `Environment::resolve(source_collection, file_id, path_id)`，按 `dependencies[file_id]` → target collection → `assets[path_id]`。已落地 | ✅ |
| ~~**streaming_data**~~ | `Environment::streaming_data(&StreamingInfo)` 取零拷贝 `Bytes` slice。已落地 | ✅ |
| **bumpalo arena** | 对象 arena 分配。当前对象按需 parse、owned。附带收益：TypeTree 解释器 `interpret` 的递归 `Value` 返回值可改为 arena 指针（类似 oxc 的 box+arena），从设计上缩栈，根治 debug build 栈帧膨胀（当前靠 `Cargo.toml` 的 `opt-level=1` 缓解） | ⬜ |
| **跨 bundle Environment** | 多 bundle 聚合 + 磁盘依赖加载。当前 `Environment` 单 bundle 范围 | ⬜ |

---

## 11. 与其他文档的关系

| 文档 | 角色 |
|---|---|
| **本文档** | 对象解析层**现状权威**（SerializedFile + TypeTree + dispatch + fallback，已落地） |
| [ADR-0001](adr/0001-codegen-from-typetree-dumps.md) | typed struct 决策（含修订附录） |
| [`io-layer.md`](io-layer.md) | 本层输入（`BundleFile.bytes`） |
| [`codegen.md`](codegen.md) | typed struct 来源（generated crate） |
| [`environment-layer.md`](environment-layer.md) | 聚合层（`Environment` + PPtr 解引用 + streaming_data） |

冲突时以本文档为准。
