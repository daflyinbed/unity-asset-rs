# SerializedFile 层 + 纯 AB TypeTree 解释器

> 范围：从 `BundleFile.bytes` 解析 SerializedFile（header/types[含内嵌
> typetree]/objects），并用内嵌 typetree 把对象解码成动态 `Value`。
> 路线：UnityPy 式纯解释器——**typetree 是布局唯一真相**，不做 codegen/TPK/
> fallback/MonoBehaviour 特判。
> 上游：[ADR-0003](adr/0003-file-loading-and-decompression-layer.md)（已实现的
> bundle 解包层）、`object-parsing-design.md`（codegen 路线，搁置）。

---

## 1. 流水线

```
load(path) → Bundle { files: Vec<BundleFile{ name, bytes }> }     [已有]
   │
   ▼  对每个 BundleFile（.assets / CAB-*）
SerializedFile::parse(bytes)                                        [本文 §2]
   │  BE prologue → endian 字节 → 切 endian → 读 types[+typetree] / objects
   ▼
SerializedFile { version, unity_version, endian, types, ref_types, objects }
   │  每个 SerializedType.node = 内嵌 TypeNode 树
   ▼
SerializedFile::parse_object(i)                                     [本文 §4]
   │  obj.data + types[obj.type_idx].node + endian + refs
   ▼
typetree::interpret() → Value                                       [本文 §3]
```

---

## 2. SerializedFile 二进制格式

**已用 `char_002_amiya.ab` 探针实测确认**：prologue（前 4×u32）是**大端**，
version=17、file_size=57416（吻合总字节）、data_offset=46832、enable_type_tree=1。

### 2.1 Header（参考 UnityPy `SerializedFile.py:254-269`）

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

### 2.2 元数据（用切换后的 endian；fixture 为 LE）

```
ver>=7  cstr unity_version          // "2017.4.39f1"
ver>=8  i32  target_platform        // BuildTarget
ver>=13 bool enable_type_tree
i32 type_count ; SerializedType × count        // §2.3
7<=ver<14 i32 big_id_enabled
i32 object_count ; ObjectInfo × count          // §2.4
ver>=11 i32 script_count ; LocalSerializedObjectIdentifier × count
i32 external_count ; FileIdentifier × count
ver>=20 i32 ref_type_count ; SerializedType(is_ref=true) × count
ver>=5  cstr user_info
```

### 2.3 SerializedType（参考 `SerializedFile.py:112-149`）

```
i32 class_id
ver>=16 bool is_stripped
ver>=17 i16 script_type_index
ver>=13:
  条件 C = (is_ref && script_type_index>=0) || (ver<16 && class_id<0) || (ver>=16 && class_id==114)
  if C: 16 字节 script_id (Hash128)
  16 字节 old_type_hash (Hash128)
enable_type_tree:
  ver>=12 || ver==10: parse_blob()          // §3.2
  else:               (legacy 递归格式，未实现)
ver>=21:
  is_ref: cstr class_name, cstr namespace, cstr assembly_name   // ref 必需
  else:   i32[] type_dependencies
```

### 2.4 ObjectInfo（参考 `ObjectReader.from_reader:67-130`）

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

对象实际字节：`data[byte_start .. byte_start+byte_size]`（byte_start 已含 data_offset）。

---

## 3. TypeTree 模块

### 3.1 TypeKind / TypeNode

`Array/Vector/Map` **不进 TypeKind**——Unity 把所有集合编码成相同结构
（集合节点有唯一 `type="Array"` 子节点，该子节点固定 `[size:int, data:元素]`），
解码算法唯一，靠结构判别（`children[0].kind == Struct("Array")`），与 UnityPy
`TypeTreeHelper.py:208` 一致。`map` 只是元素为 `pair` 的 vector，零特殊代码。

```rust
pub enum TypeKind {
    Bool, U8, I8, U16, I16, U32, I32, U64, I64, F32, F64,
    String,         // i32 len + utf8 + align(4)
    TypelessData,   // i32 len + u8[]
    Pair,
    PPtr(String),   // PPtr<T> → target
    Struct(String), // 其余（含 "vector"/"map"/"Array" 包装器）
}
pub struct TypeNode {
    pub kind: TypeKind, pub name: String, pub byte_size: i32,
    pub version: i16, pub meta_flag: u32, pub children: Vec<TypeNode>,
}
impl TypeNode {
    pub fn is_sequence(&self) -> bool { /* children[0].kind == Struct("Array") */ }
}
```

`TypeKind::from_unity` 归一化别名表：`SInt32/int/Type*/EntityId→I32`、
`unsigned int/UInt32→U32`、`long long/SInt64→I64`、`PPtr<…>→PPtr` 等。

### 3.2 blob 读取（参考 `TypeTreeNode.py:111-154`）

单节点磁盘布局 `{endian}hBBIIiii[Q]`（ver>=19 加 `u64 ref_type_hash`），
定长 24（或 32）字节。先读 `i32 node_count` + `i32 str_buf_size`，再批量读节点 +
字符串缓冲。字符串偏移 `bit31=0`→本地 buffer null-terminated；`bit31=1`→
CommonString 表索引（§3.4）。扁平节点按 `level` 用递归还原成嵌套树。

### 3.3 interpret（参考 `TypeTreeHelper.read_value:182-263`）

```rust
pub fn interpret(data, root, ver, endian, refs) -> Result<Value>
```

按 `TypeKind` 分派：原语直读；`String`/`TypelessData` 带 len；`Pair` 读两子节点；
`PPtr` 读 `file_id:i32` + `path_id`（`ver>=5`→i64 else i32）；`Struct` 若
`is_sequence()` 读 size+N 元素，否则遍历子节点。每节点末尾 `meta_flag & 0x4000`
→ `align(4)`；Array 包装器的 meta_flag 会向上传染给集合节点。

**边界**：
- `ReferencedObject`（Unity 2021+ `[SerializeReference]`）：Struct 子节点中遇到
  `Struct("ReferencedObjectData")` 时，用已读的 `type{class,ns,asm}` 兄弟字段在
  `RefTypeTable` 查到目标节点再走它。
- `ManagedReferencesRegistry`：同一对象只读一次（`has_registry` 标志）。
- **校验**：走完 root 后 `cursor.pos() == data.len()`，否则报
  `TypeTreeSizeMismatch`（版本不匹配强信号）。

### 3.4 CommonString

`typetree/common_string.rs` 硬编码 113 条 `(u32 offset, &'static str)`，从
AssetRipper `CommonString.cs` 抄录。

### 3.5 Value

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
```

---

## 4. 对象解码入口

```rust
impl SerializedFile {
    pub fn parse_object(&self, i: usize) -> Result<Option<Value>> {
        let obj = &self.objects[i];
        let node = self.types.get(obj.type_idx).and_then(|t| t.node.as_ref());
        let refs = self.ref_type_table();
        interpret(obj.data(self), node?, &self.unity_version_parsed, self.endian, &refs).map(Some)
    }
}
```

---

## 5. 已完成的基建

- `src/cursor.rs`：公共可切-endian 游标（原 bundle 私有 Cursor 提升 + endian）。
- `src/error.rs`：所有 `reason` 用 `String`（原 `&'static str`）；新增
  `SerializedFile`/`TypeTreeSizeMismatch`/`NegativeArrayLen`/`RefTypeNotFound`/
  `EmptyTypeTree`/`TypeTree` 变体。

## 6. 模块结构

```
src/
├── bundle.rs            (已有)
├── decompress.rs        (已有)
├── cursor.rs            (已有，公共游标)
├── error.rs             (已有 + 新变体)
├── version.rs           ★ UnityVersion 解析
├── serialized_file.rs   ★ §2
├── typetree/
│   ├── mod.rs           ★ TypeKind/TypeNode/Value/RefTypeTable/from_unity
│   ├── common_string.rs ★ 113 条共享字符串
│   ├── blob.rs          ★ parse_blob + build_tree
│   └── interpret.rs     ★ interpret 递归走树
├── lib.rs
└── main.rs              (demo)
```
