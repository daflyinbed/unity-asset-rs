# Codegen 设计：TPK → Rust 结构体与 Parser

> 范围：`crates/codegen` 的内部架构、TPK → Rust 的数据流、生成代码的最终形态。
> 状态：目标设计（反映当前实现 + 最新决策）
> 上游：[`object-parsing-design.md`](object-parsing-design.md)（dispatch + fallback 架构）

---

## 1. 概述

codegen 是一个**离线工具**（`crates/codegen`，bin），从 `type_tree.tpk` 生成
`crates/generated/` 下的 Rust 源码（提交进仓库，独立编译缓存）。

```
type_tree.tpk ──→ crates/codegen ──→ crates/generated/src/*.rs
                   (6 pass)            (~8万行，1300+ 文件)
```

### 输入

`type_tree.tpk`（AssetRipper 出品）。brotli 压缩后 ~186KB。
含 1395 个 Unity 版本的 TypeTree 数据，覆盖 412 个 classID。

### 输出

- **结构体定义**：每个具体 class 一个 `pub struct`（扁平字段）+ `impl Parse`
- **分派 enum**：`enum UnityObject` + `enum ClassIDType`

### 不生成的

| 不做什么 | 原因 |
|---|---|
| MonoBehaviour (classID=114) | 脚本字段不在 TPK 里；运行时走 TypeTree 解释器 |
| 抽象父类（Object, NamedObject, Texture, ...） | TPK 里它们的 root 为空；字段已平铺到子类 |
| `parent` 字段 / 父类 enum | 不需要——TPK 的 release_root 已含全部继承链字段 |
| enum（TextureFormat 等） | TPK 不含 enum 信息；整数字段一律 `i32` |
| dispatch / fallback 逻辑 | 在主 crate `object_parser.rs` 里 |
| 写入路径 | MVP 只做 parse |

---

## 2. 生成代码的最终形态

### 2.1 一个 class（Texture2D）

```rust
// crates/generated/src/classes/texture_2_d.rs
//! 自动生成。请勿手动修改。

use unity_core::PPtr;
use crate::subclasses::gl_texture_settings::GlTextureSettings;
use crate::subclasses::streaming_info::StreamingInfo;

#[derive(Debug, Clone)]
pub struct Texture2D {
    // 所有字段直接平铺——无 parent 字段
    pub m_name: String,
    pub m_width: i32,
    pub m_height: i32,
    pub m_forced_fallback_format: Option<i32>,   // ver >= 2017.3
    pub m_texture_format: i32,
    pub m_mip_map: Option<bool>,                 // ver >= 3.4
    pub m_is_readable: bool,
    pub image_data: Vec<u8>,                     // TypelessData
    pub m_stream_data: Option<StreamingInfo>,    // ver >= 5.3
    // ... 共约 30 字段
}

impl unity_core::Parse for Texture2D {
    fn parse<R: unity_core::EndianReader>(
        r: &mut R,
        ver: &unity_core::UnityVersion,
    ) -> Result<Self, R::Error> {
        // 所有字段按 TPK release_root 顺序直接读
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_width = r.read_i32()?;
        let m_height = r.read_i32()?;
        let m_forced_fallback_format = if *ver >= unity_core::UnityVersion::new(2017, 3, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_texture_format = r.read_i32()?;
        // ...
        let image_data = {
            let n = r.read_i32()? as usize;
            r.read_bytes(n)?
        };
        let m_stream_data = if *ver >= unity_core::UnityVersion::new(5, 3, 0) {
            Some(StreamingInfo::parse(r, ver)?)
        } else {
            None
        };

        Ok(Texture2D {
            m_name,
            m_width,
            m_height,
            m_forced_fallback_format,
            m_texture_format,
            image_data,
            m_stream_data,
            // ...
        })
    }
}
```

**关键特征**：

- **无 parent 字段**：所有继承链字段直接在类上。TPK 的 release_root 已含全部字段。
- **无抽象父类**：不生成 `Object` / `NamedObject` / `Texture` 等。它们在 TPK 里 root 为空。
- **版本门控**：跨版本差异字段用 `Option<T>` + `if *ver >= V`。
- **对齐**：`r.align(4)?` 由 `meta_flag & (0x4000 | 0x8000)` 决定，codegen 烤死。
- **用户访问**：`tex.m_name`（直接，不用 chain）。

### 2.2 父类变化的 class（AudioClip，原 23 例）

**不再生成 parent enum**。TPK 的每个版本变体都已有完整的扁平字段列表。
p05 字段并集自动处理：旧版本（继承 NamedObject）没有的字段，
在新版本（继承 SampleClip）出现 → 自动得到 `Since(5,0,0)` 门控。

```rust
#[derive(Debug, Clone)]
pub struct AudioClip {
    pub m_name: String,                    // NamedObject/SampleClip 都有
    pub m_compressed: Option<bool>,        // SampleClip 才有 → ver >= 5.0
    pub m_audio_data: Vec<u8>,             // AudioClip 自己
    // ...
}

impl unity_core::Parse for AudioClip {
    fn parse<R: unity_core::EndianReader>(r: &mut R, ver: &unity_core::UnityVersion)
        -> Result<Self, R::Error>
    {
        let m_name = r.read_aligned_string()?;
        let m_compressed = if *ver >= unity_core::UnityVersion::new(5, 0, 0) {
            Some(r.read_bool()?)
        } else { None };
        let m_audio_data = { /* ... */ };
        // ...
        Ok(AudioClip { m_name, m_compressed, m_audio_data })
    }
}
```

### 2.3 enum UnityObject（分派）

```rust
// crates/generated/src/unity_object.rs
#[derive(Debug, Clone)]
pub enum UnityObject {
    GameObject(GameObject),
    Transform(Transform),
    Texture2D(Texture2D),
    Mesh(Mesh),
    // ...
    // ★ 不含 MonoBehaviour（主 crate 拦截 classID=114 走 TypeTree）
    // ★ 不含抽象类（Object, Texture, NamedObject 等）
}
```

### 2.4 基础设施（lib.rs）

`crates/generated` 依赖 `unity-core`，自身只加 `ClassIDType` + `UnityObject` + 模块声明：

```rust
// crates/generated/src/lib.rs
pub mod classes;
pub mod subclasses;
mod unity_object;
mod class_id;

pub use class_id::ClassIDType;
pub use unity_object::UnityObject;
// Parse / EndianReader / UnityVersion / PPtr 都从 unity_core re-export
pub use unity_core::{EndianReader, Parse, UnityVersion, PPtr};
```

---

## 3. 共享 crate：unity-core

三个 crate（`src/`主 crate、`crates/generated`、`crates/codegen`）共用的类型抽到一个
极小的共享 crate，避免重复定义和类型不匹配。

### 3.1 依赖关系

```
crates/unity-core/              # 零外部依赖，~150 行
     ↑          ↑
     │          │
     │     crates/generated/    # 依赖 unity-core
     │          ↑
     │          │
     │     src/ (主 crate)       # 依赖 unity-core + generated
     │
crates/codegen/                 # 不依赖 unity-core（只生成引用它的代码）
```

### 3.2 内容

```
crates/unity-core/
├── Cargo.toml                  # 零依赖
└── src/
    ├── lib.rs
    ├── version.rs              # UnityVersion
    ├── reader.rs               # Endian enum + EndianReader trait + Parse trait
    └── pptr.rs                 # PPtr struct
```

```rust
// crates/unity-core/src/version.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnityVersion {
    pub major: u16,
    pub minor: u16,
    pub build: u16,
}

impl UnityVersion {
    pub const fn new(major: u16, minor: u16, build: u16) -> Self { ... }
    pub fn parse(s: &str) -> Self { ... }  // "2017.4.39f1" → (2017, 4, 39)
}
```

```rust
// crates/unity-core/src/reader.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Endian { Little, Big }

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
}

pub trait Parse: Sized {
    fn parse<R: EndianReader>(r: &mut R, ver: &UnityVersion) -> Result<Self, R::Error>;
}
```

```rust
// crates/unity-core/src/pptr.rs
#[derive(Debug, Clone)]
pub struct PPtr {
    pub m_file_id: i32,
    pub m_path_id: i64,
}
```

### 3.3 各 crate 如何使用

| crate | 用法 |
|---|---|
| `crates/generated` | `use unity_core::{EndianReader, Parse, UnityVersion, PPtr}`; 每个 struct `impl unity_core::Parse` |
| `src/`（主 crate） | `impl unity_core::EndianReader for Cursor`; 调 `UnityObject::parse()` 时传 `&unity_core::UnityVersion` |
| `crates/codegen` | emit 时生成 `unity_core::Parse` / `unity_core::EndianReader` 等引用；自身不 link unity-core |

---

## 4. 命名规则

### 4.1 字段名

| Unity 原名 | Rust 名 | 规则 |
|---|---|---|
| `m_Width` | `m_width` | 保留 `m_`，heck ToSnakeCase |
| `m_TextureFormat` | `m_texture_format` | 同上 |
| `m_GameObject` | `m_game_object` | 同上 |
| `image_data` | `image_data` | 已是 snake_case |
| `image data` | `image_data` | 空格 → 下划线 |
| `Type vs.` | `type_vs` | 非标识符字符 → 下划线 |
| `type` | `r#type` | Rust 关键字 → `r#` 转义 |
| `4_3` | `f_4_3` | 首字符数字 → 加 `f_` 前缀 |

实现：`heck::ToSnakeCase` + 关键字检测 + 数字前缀处理。

### 4.2 类型名

| Unity 原名 | Rust 名 | 规则 |
|---|---|---|
| `Texture2D` | `Texture2D` | PascalCase 保留，数字保留 |
| `ColorRGBA` | `ColorRgba` | 连续大写缩写规则化 |
| `AABB` | `Aabb` | 同上 |
| `GUID` | `Guid` | 同上 |
| `Vector3f` | `Vector3f` | 数字 + 字母保留 |
| `float` | `float_` | Rust 原始类型名 → 加 `_` 后缀 |

### 4.3 模块 / 文件命名

| 类型 | 路径 |
|---|---|
| Unity class | `classes/{snake_name}.rs`（`texture_2_d.rs`） |
| Unity subclass | `subclasses/{snake_name}.rs` |
| `enum UnityObject` | `unity_object.rs` |
| `enum ClassIDType` | `class_id.rs` |

---

## 5. 字段类型映射

TPK 节点的 `(type_name, 子树形态)` 二元组 → Rust 类型 + parse 代码：

| TypeTree 类型 | Rust 类型 | parse 代码 |
|---|---|---|
| `bool` | `bool` | `r.read_bool()?` |
| `UInt8` / `char` | `u8` | `r.read_u8()?` |
| `SInt8` | `i8` | `r.read_i8()?` |
| `SInt16` / `short` | `i16` | `r.read_i16()?` |
| `UInt16` / `ushort` | `u16` | `r.read_u16()?` |
| `SInt32` / `int` | `i32` | `r.read_i32()?` |
| `UInt32` / `uint` | `u32` | `r.read_u32()?` |
| `SInt64` / `long long` | `i64` | `r.read_i64()?` |
| `UInt64` / `FileSize` | `u64` | `r.read_u64()?` |
| `float` | `f32` | `r.read_f32()?` |
| `double` | `f64` | `r.read_f64()?` |
| `string` | `String` | `r.read_aligned_string()?` |
| `TypelessData` | `Vec<u8>` | `{ let n = r.read_i32()? as usize; r.read_bytes(n)? }` |
| `Array` / `vector` / `staticvector` / `set` | `Vec<T>` | `{ let n = r.read_i32()?; (0..n).map(...) }` |
| `map` | `Vec<PairStruct>` | 同 Array，元素是 pair subclass |
| `pair` | 具名 subclass | `PairStruct::parse(r, ver)?` |
| `PPtr<X>` | `PPtr` | `PPtr::parse(r, ver)?`（非 generic） |
| 其它非叶（struct） | 具名 subclass | `XxxStruct::parse(r, ver)?` |

**map 用 `Vec<PairStruct>` 而非 `HashMap`**：Unity 的 map 不保证去重，字段顺序有意义。

### 版本门控

| 形态 | 字段类型 | parse 代码 |
|---|---|---|
| 全版本 | `T` | `let x = r.read_i32()?;` |
| `ver >= V` | `Option<T>` | `let x = if *ver >= V { Some(r.read_i32()?) } else { None };` |

### 对齐

`meta_flag & (0x4000 | 0x8000) != 0` → parse 调用后 emit `r.align(4)?;`。

---

## 6. Codegen 内部架构

### 6.1 crate 结构

```
crates/codegen/
├── Cargo.toml
├── tests/
│   ├── fixtures/type_tree.tpk
│   └── tpk_read.rs
├── examples/
│   └── explain_inheritance.rs
└── src/
    ├── main.rs                      # CLI: <tpk_path> <out_dir>
    ├── lib.rs
    ├── tpk/                         # TPK 二进制读取（小端）
    │   ├── mod.rs                   #   TpkFile + TpkTypeTree + TpkUnityNode
    │   ├── reader.rs                #   LeReader
    │   └── compression.rs           #   LZ4 / LZMA / Brotli
    ├── ir.rs                        # IR: NodeType, RustType, EmitClass, EmitField
    ├── names.rs                     # 命名转换
    ├── pipeline.rs                  # Pass 系统（p01-p05）
    └── emit.rs                      # 代码发射
```

依赖：`lz4_flex`、`brotli`、`lzma-rs`、`heck`、`thiserror`、`log`。
**不依赖** `unity-core`（只生成引用它的代码字符串）。

### 6.2 IR 数据结构

```rust
pub enum NodeType {
    Boolean, UInt8, Int8, UInt16, Int16, UInt32, Int32, UInt64, Int64,
    Single, Double,
    String, TypelessData, Array, Vector, Map, Pair, PPtr, Type,
}

pub enum RustType {
    Bool, I8, U8, I16, U16, I32, U32, I64, U64, F32, F64,
    String, Bytes,
    List(Box<RustType>),
    Pair(Box<RustType>, Box<RustType>),
    Pptr { target_class: String },
    Subclass(String),
    Option(Box<RustType>),
}

pub enum ClassKey {
    Main(i32),
    Subclass(String),
}

/// 一个待生成的 class。
pub struct EmitClass {
    pub key: ClassKey,
    pub name: String,             // Rust 化类型名
    pub unity_name: String,       // 原始 Unity 名
    pub fields: Vec<EmitField>,   // 全部字段（含继承链，平铺）
    pub is_abstract: bool,        // true → 不生成 struct
    pub is_mono_behaviour: bool,  // true → 不生成 struct
}

/// ★ 无 ParentSpec——不生成 parent 字段。
/// ★ 无 Stable/Variable——不生成 parent enum。
/// 跨版本父类变化的字段自动由 fields 的 VersionGate 处理。

pub struct EmitField {
    pub rust_name: String,
    pub unity_name: String,
    pub rust_type: RustType,
    pub version_gate: Option<VersionGate>,
    pub align_after: bool,
    pub unity_meta_flag: u32,
}

pub enum VersionGate {
    Always,
    Since(u16, u16, u16),
}

pub struct Pipeline {
    pub classes: BTreeMap<ClassKey, EmitClass>,
    pub pptr_targets: BTreeSet<String>,
    pub all_versions: Vec<crate::tpk::UnityVersion>,
}
```

### 6.3 Pass 系统（6 个）

| # | Pass | 职责 |
|---|---|---|
| 01 | `normalize_types` | 类型名归一化（`int → SInt32` 等 ~10 对别名） |
| 02 | `extract_subclasses` | 扫所有节点，`NodeType::Type` 的子树抽成 subclass |
| 03 | `collect_pptr_targets` | 收集 `PPtr<X>` 目标（记录用，PPtr 非 generic） |
| 04 | `create_class_skeletons` | 为每个 main class 创建 EmitClass；重名加 `_ID`；标记 abstract / mono_behaviour |
| 05 | `merge_fields` ★ | **核心**：字段并集 + 版本门控（见 §6.4） |
| 06 | `plan_alignment` | `meta_flag & (0x4000 \| 0x8000)` → `align_after` |
| 07 | `emit` | `Vec<EmitClass>` → .rs 文件 |

**与旧设计的差异**：

- 旧 p04（`resolve_inheritance`）计算 `ParentSpec`（Root/Stable/Variable）。
  新 p04（`create_class_skeletons`）**不算继承**——直接创建空壳 EmitClass，
  字段由 p05 填充。无 `ParentSpec`、无 parent enum。
- TPK 的 release_root 已含全部继承链字段（扁平），p05 直接取 root 的子节点作为字段列表。
- 抽象类（`is_abstract=true`）和 MonoBehaviour（`is_mono_behaviour=true`）不生成 struct。

### 6.4 Pass 05：字段并集 + 版本门控（核心算法）

```
输入：一个 class 的所有版本变体，每个含一棵 release_root 节点树

1. 展平每个版本的 root 子节点 → flat_fields_i
   （root 的直接子节点 = 该 class 的全部字段，含继承链）
2. 收集所有版本里出现的字段名并集（按 rust_name 去重）
3. 对每个字段：
   - 计算 present_versions
   - gate = if 全版本都有 { Always } else { Since(最早出现版本) }
   - rust_type = if Always { T } else { Option<T> }
4. 按首次出现位置排序（保持字节流顺序）
```

**不需要扣祖先字段**：没有 parent，所有字段直接在类上。
**不需要处理父类变化**：p05 的版本并集自动处理——不同父类的字段差异体现为 VersionGate。

---

## 7. TPK 二进制格式（简述）

```
TPK 文件（全程小端）:
  u32 magic = 0x2A4B5054 ('T' 'P' 'K' '*')
  u8  version = 1
  u8  compression (0=None, 1=LZ4, 2=LZMA, 3=Brotli)
  u8  dataType = 0 (TypeTree)
  u8  reserved; u32 reserved
  i32 compressedSize; i32 decompressedSize
  [u8; compressedSize] payload
```

解压后：`creationTime(i64)` + `versions[](u64×N)` + `classes[](变长)` + `commonString` + `nodeBuffer` + `stringBuffer`。

每个 `TpkUnityNode`：`typeName(u16)` + `name(u16)` + `byteSize(i32)` + `version(i16)` +
`typeFlags(u8)` + `metaFlag(u32)` + `subNodes(u16[])`。
不存 `Level` / `Index`（树结构由 subNodes 表达）。

---

## 8. 输出目录布局

```
crates/generated/src/
├── lib.rs                    # use unity_core::*; + 模块声明
├── unity_object.rs           # enum UnityObject + parse dispatch
├── class_id.rs               # enum ClassIDType（含 MonoBehaviour=114，但 dispatch 不含）
├── classes/
│   ├── mod.rs
│   ├── game_object.rs        # struct GameObject { m_name, ... } + impl Parse
│   ├── texture2_d.rs         # struct Texture2D { m_name, m_width, ... }
│   ├── audio_clip.rs         # struct AudioClip { m_name, m_compressed: Option, ... }
│   ├── ...                   # ~360 个文件（不含抽象类、不含 MonoBehaviour）
│   └── (无 texture.rs, named_object.rs, object.rs — 抽象类不生成)
└── subclasses/
    ├── mod.rs
    ├── vector3f.rs           # struct Vector3f { x: f32, y: f32, z: f32 }
    ├── streaming_info.rs
    └── ...                   # ~900 个文件
```

---

## 9. 如何运行

```bash
# 重新生成
rm -rf crates/generated/src
mkdir -p crates/generated/src
cargo run -p unity-asset-codegen -- \
    crates/codegen/tests/fixtures/type_tree.tpk \
    crates/generated/src

# 编译验证
cargo build --workspace
cargo test --workspace
```

---

## 10. 已知限制

| 项 | 现状 | 影响 |
|---|---|---|
| PPtr 非 generic | 所有 PPtr 同类型 | 无类型区分 |
| VersionGate 只有 Since | 不支持 Range / Disjoint | 罕见字段（消失后重现）会误判 |
| subclass 无版本并集 | 取第一次出现的子树 | 同 subclass 跨版本变化时会错 |
| emit 字符串拼接 | 无语法校验 | 边界 bug（测试覆盖主要形态） |

---

## 11. 与其他文档的关系

| 文档 | 角色 |
|---|---|
| **本文档** | codegen 目标设计（扁平 struct + parser） |
| [`codegen-implementation.md`](codegen-implementation.md) | codegen 当前实现记录（详细 pass / corner case） |
| [`codegen-design.md`](codegen-design.md) | 原始设计稿（**已过时**，保留作历史参考） |
| [`object-parsing-design.md`](object-parsing-design.md) | dispatch + fallback 架构（主 crate 侧） |

冲突时以本文档为准。
