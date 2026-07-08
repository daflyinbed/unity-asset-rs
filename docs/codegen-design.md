# Codegen 设计：从 TPK 生成类型化 Rust 结构与 Parser

> ⛔ **历史归档（2026-07-09）** —— 本文档描述的**嵌套 `parent` 模型**（`tex.parent.parent.m_name`）
> 在实现中已被**扁平 struct、无 parent 字段**取代（TPK release_root 已含全部继承链字段）。
> 权威现状见 [`codegen.md`](codegen.md)。本文保留仅供追溯，**请勿据此实现**。

> 范围：定义 `crates/codegen` 的内部架构、生成代码的最终形态、命名规范，以及 TPK → Rust 的端到端数据流。
> 状态：设计稿（对应 ADR-0001 的实施细化）
> 上游决策：[ADR-0001](adr/0001-codegen-from-typetree-dumps.md)（对象解析与代码生成决策）、[ADR-0003](adr/0003-file-loading-and-decompression-layer.md)（io 层架构，本层在其之上）

---

## 1. 概述

本文档覆盖"从 `type_tree.tpk` 到 `crates/generated/src/` 下可编译的 Rust 代码"的整条生成管线。

**输入**：
- `type_tree.tpk`（AssetRipper 出品，跨版本去重的 TypeTree 二进制容器，brotli/lz4/lzma 压缩后 ~187KB）

整数字段一律生成 `i32`，**不生成 enum**（TPK 本身不含 enum 信息；用户需要枚举语义时自己在调用端 cast）。这样 codegen 输入端**只依赖 TPK 单一来源**，零外部依赖。

**输出**：
- `crates/generated/src/` 下约 4–5 万行 Rust 源码（提交进仓库，独立编译缓存）
- 366 个有字段的 Unity 原生 class 全覆盖

**核心设计取舍**（详见 ADR-0001）：
- 版本差异 = `Option<T>` + `if ver >= V`（**不**用 per-version 变体）
- 继承 = 嵌套 `parent` 字段（**不**平铺祖先字段到子类）
- enum = **不做**（整数字段一律 `i32`，保持纯 TPK 输入）
- 字段命名 = **Rust 风格**，非 Unity 原样（详见 §2）
- 输出 = Rust 源码到独立 crate（**不**用 build.rs，**不**仿 AssetRipper 的 IL 注入）
- 读写范围 = **只生成 parse 路径**（写入留待后续）

---

## 2. 命名规范（重点）

Unity TypeTree 里的标识符是 C# 风格（`m_Width`、`ColorRGBA`、`AABB`）。生成到 Rust 时**强制转换**，让生成代码符合 Rust 习惯，不被 Unity 命名污染。

### 2.1 字段名：`m_TextureFormat` → `m_texture_format`

**保留** `m_` / `k_` 前缀（Unity 原生约定，便于对照官方文档），仅做 snake_case 化。

| 规则 | Unity 原名 | Rust 生成名 |
|---|---|---|
| 保留前缀 + snake_case 化 | `m_Width` | `m_width` |
| 保留前缀 + PascalCase 转 snake_case | `m_TextureFormat` | `m_texture_format` |
| 保留前缀 + 首字母小写 | `m_GameObject` | `m_game_object` |
| 无前缀，已是 snake_case 保留 | `image_data` | `image_data` |
| 无前缀，全大写缩写转小写 | `RGB` | `rgb` |
| 数字后缀保留 | `m_BlendType0` | `m_blend_type_0` |
| 关键字冲突用 `r#` 转义 | `type` | `r#type` |
| 含连字符 / 空格（罕见）| `Type vs.` | `type_vs`（sanitize 非标识符字符）|
| 空名 / 容器节点名（`Array` / `data` / `size`）| — | 不生成字段（容器节点展开为 `Vec<T>`）|

**算法**（伪代码）：
```
fn to_rust_field_name(unity_name: &str) -> String {
    // 1. 保留 m_ / k_ 前缀
    // 2. snake_case 化（m_GameObject → m_game_object；GameObject → game_object）
    let s = to_snake_case(unity_name);
    // 3. 关键字冲突用 r# 转义（保留原意，比加 _ 后缀更符合 Rust 习惯）
    if is_rust_keyword_or_reserved(&s) { format!("r#{s}") } else { s }
}
```

**为什么保留 `m_`**：
- Unity 字段名是公开 API 契约的一部分，对照 [Unity Scripting API](https://docs.unity3d.com/ScriptReference/) 文档时需要原样前缀
- 调试时一眼能识别"这是 Unity 序列化字段"vs"这是 Rust 端自定义字段"
- `r#` 转义比 `_` 后缀更精确（`r#type` 保留了原名 `type`，`type_` 改了名）

### 2.2 类型名（class / struct / enum）：`ColorRGBA` → `ColorRgba`

| 规则 | Unity 原名 | Rust 生成名 |
|---|---|---|
| PascalCase 保留 | `Texture2D` | `Texture2D`（数字保留）|
| 全大写缩写展开为 `Xxx Yyy`（遵循 clippy `upper_case_acronyms`）| `AABB` | `Aabb` |
| 同上 | `GUID` | `Guid` |
| 同上 | `ColorRGBA` | `ColorRgba` |
| 同上 | `PPtr` | `PPtr`（**例外**，见下）|
| 数字 + 后缀保留 | `Vector3f` | `Vector3f` |
| `StreamingInfo` | `StreamingInfo` | `StreamingInfo`（无变化）|

**PPtr 例外**：`PPtr<T>` 是 generic 容器，全大写首字母 + 缩写组合，但它是用户天天键入的核心类型，统一转成 `Pptr` 反而怪。保留 `PPtr`，在类型上加 `#[allow(non_camel_case_types)]`。

**算法**：
```
fn to_rust_type_name(unity_name: &str) -> String {
    // 1. 拆分缩写：连续 ≥2 个大写字母后跟小写/数字 → 拆出
    //    AABB → [AABB], ColorRGBA → [Color, RGBA], GUID → [GUID]
    // 2. 每段：首字母大写，其余小写
    //    AABB → Aabb, RGBA → Rgba, GUID → Guid
    // 3. 拼接
    // 例外白名单：PPtr（保留原样 + 加 #[allow]）
}
```

### 2.3 enum 字段：一律 `i32`，不生成 enum

**不做 enum**。Unity TypeTree 里所有看起来是 enum 的字段（`m_TextureFormat`、`m_BuildTarget` 等）在磁盘上都是 `i32`，生成器一律 emit `i32`：

```rust
// crates/generated/src/classes/texture_2d.rs
pub struct Texture2D {
    pub m_texture_format: i32,    // 不是 TextureFormat，是 i32
    // ...
}
```

用户需要枚举语义时自己在调用端 cast：
```rust
// 用户侧（自己定义 enum 或从其它库引入）
let format = MyTextureFormat::try_from(tex.m_texture_format)?;
```

**理由**：TPK 本身不含 enum 信息（AssetRipper 从 Unity 官方文档 XML + 引擎 DLL 反射抽取，工程量大且 Linux 上跑不通 NativeEnumExtractor 的 PDB 解析）。保持 codegen 输入端**只依赖 TPK**，单一来源、零外部依赖。

### 2.4 父类 enum 命名

父类跨版本变化的 23 个 class，生成 `{ClassName}Parent` 枚举：

```rust
pub enum AudioClipParent {
    NamedObject(NamedObject),
    SampleClip(SampleClip),
}
```

### 2.5 模块 / 文件命名

| 类型 | 文件 / 模块 |
|---|---|
| Unity class | `classes/{snake_case_name}.rs`（如 `classes/texture_2d.rs`）|
| Unity struct（subclass）| `subclasses/{snake_case_name}.rs` |
| PPtr | `pptr.rs` |
| `enum UnityObject` 分派 | `unity_object.rs` |
| `enum ClassIDType` | `class_id.rs` |

`{snake_case_name}` 由 PascalCase 类型名转来：`Texture2D` → `texture_2d`、`ColorRgba` → `color_rgba`、`Aabb` → `aabb`。

---

## 3. 数据流总览

```
输入        type_tree.tpk ──→ crates/codegen (bin) ─→ crates/generated/src/*.rs
                              (8 个顺序 pass)        (4–5 万行，提交)
                                                          ↑
                                                          └── 依赖 ── crates/unity-asset (lib + bin)
                                                                       (为 SerializedFile parse 提供 reader impl)
```

**codegen 是离线工具**：库维护者在 TPK 升级或 codegen 逻辑改动时手动跑一次，产物提交。它不参与 `crates/unity-asset` / `crates/generated` 的构建依赖图。

**`crates/generated` 是 pure-data crate**：无外部依赖（甚至不依赖 `bytes`），只暴露数据 struct + `impl Parse` + 一个 `EndianReader` trait。这样它独立编译一次后 cargo 缓存其 artifact，修改 `crates/unity-asset` 手写代码时不必重编这 ~5 万行。

---

## 4. 生成代码形态

### 4.1 一个 class 的完整样子（Texture2D，稳定父类）

```rust
// crates/generated/src/classes/texture_2d.rs

use crate::parse::{EndianReader, Parse};
use crate::pptr::PPtr;
use crate::subclasses::{StreamingInfo, GUID};
use unity_asset_primitives::UnityVersion;   // 来自一个极小的 primitives crate

/// Texture2D —— classID = 28。
///
/// 继承链：Texture2D ← Texture ← NamedObject ← Object。
/// 跨版本字段并集；门控字段用 Option<T>。
#[derive(Debug, Clone)]
pub struct Texture2D {
    /// 父类字段（嵌套，不自展平）。
    pub parent: Texture,
    // ── 本类字段 ──
    pub m_width: i32,
    pub m_height: i32,
    pub m_complete_image_size: Option<i32>,         // ver >= 5.2
    pub m_texture_format: i32,                      // 整数（不生成 TextureFormat enum）
    pub m_mip_map: bool,
    pub m_mip_count: Option<i32>,                   // ver >= 5.2
    pub m_is_readable: Option<bool>,                // ver >= 2017.1
    pub m_streaming_mipmaps: Option<bool>,          // ver >= 2018.2
    pub image_data: Vec<u8>,                        // TypelessData（无 m_ 前缀）
    pub m_stream_data: Option<StreamingInfo>,       // ver >= 5.3
    // ... 共约 30 个本类字段
}

impl Parse for Texture2D {
    fn parse<R: EndianReader>(
        r: &mut R,
        ver: &UnityVersion,
    ) -> Result<Self, R::Error> {
        // 先解父类（Unity 字节流顺序天然匹配）
        let parent = Texture::parse(r, ver)?;

        // 本类字段，按版本门控
        let m_width = r.read_i32()?;
        let m_height = r.read_i32()?;
        let m_complete_image_size = if *ver >= UnityVersion::new(5, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_texture_format = r.read_i32()?;
        let m_mip_map = r.read_bool()?;
        r.align(4)?;                              // ← MetaFlag 0x4000 触发的对齐
        let m_mip_count = if *ver >= UnityVersion::new(5, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        // ...
        let image_data = {
            let n = r.read_i32()? as usize;
            r.read_bytes(n)?
        };
        let m_stream_data = if *ver >= UnityVersion::new(5, 3, 0) {
            Some(StreamingInfo::parse(r, ver)?)
        } else {
            None
        };

        Ok(Texture2D {
            parent,
            m_width,
            m_height,
            m_complete_image_size,
            m_texture_format,
            m_mip_map,
            m_mip_count,
            image_data,
            m_stream_data,
            // ...
        })
    }
}
```

**访问祖父字段**：`tex2d.parent.parent.m_name`（`Texture.parent: NamedObject`，`NamedObject.m_name: String`）。MVP **不生成 convenience accessor**，用户自己 chain。

### 4.2 父类变化的 class（AudioClip，23 例之一）

```rust
// crates/generated/src/classes/audio_clip.rs

/// 自动生成：父类跨版本变化时，每个历史父类一个分支。
#[derive(Debug, Clone)]
pub enum AudioClipParent {
    /// ver < 5.0.0
    NamedObject(NamedObject),
    /// ver >= 5.0.0
    SampleClip(SampleClip),
}

#[derive(Debug, Clone)]
pub struct AudioClip {
    pub parent: AudioClipParent,
    // 本类字段
    pub m_audio_data: Vec<u8>,
    // ...
}

impl Parse for AudioClip {
    fn parse<R: EndianReader>(
        r: &mut R,
        ver: &UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = if *ver < UnityVersion::new(5, 0, 0) {
            AudioClipParent::NamedObject(NamedObject::parse(r, ver)?)
        } else {
            AudioClipParent::SampleClip(SampleClip::parse(r, ver)?)
        };
        let m_audio_data = {
            let n = r.read_i32()? as usize;
            r.read_bytes(n)?
        };
        // ...
        Ok(AudioClip { parent, m_audio_data /*, ... */ })
    }
}
```

**访问**：
```rust
let name = match &audio_clip.parent {
    AudioClipParent::NamedObject(n) => &n.m_name,
    AudioClipParent::SampleClip(s)  => &s.parent.m_name,
};
```

### 4.3 顶层（无父类）

```rust
// crates/generated/src/classes/named_object.rs（如果 NamedObject 在 TPK 里 base 为空）
#[derive(Debug, Clone)]
pub struct NamedObject {
    // 无 parent 字段
    pub m_name: String,
}

impl Parse for NamedObject {
    fn parse<R: EndianReader>(r: &mut R, _ver: &UnityVersion) -> Result<Self, R::Error> {
        let m_name = r.read_aligned_string()?;
        Ok(NamedObject { m_name })
    }
}
```

实际 TPK 里 `Object` / `UnityEngineObject` 才是真正顶层（base 为空），`NamedObject` 是它的子类。

### 4.4 抽象类

TPK 标记 `is_abstract` 的 ~46 个 class（`Object`、`Texture`、`NamedObject`、`EditorExtension` 等）：
- **生成 struct + impl Parse**（子类 `parent.parse()` 会调用）
- **不加入 `enum UnityObject` 分派表**（磁盘上不独立出现）

### 4.5 `enum UnityObject` 分派

```rust
// crates/generated/src/unity_object.rs
use crate::classes::*;

#[derive(Debug, Clone)]
pub enum UnityObject {
    GameObject(GameObject),
    Texture2D(Texture2D),
    Mesh(Mesh),
    Transform(Transform),
    MonoBehaviour(MonoBehaviour),
    // ... 共 ~320 个变体（366 个 class 减去抽象类）
}

impl UnityObject {
    /// 按 classID 分派。由 SerializedFile 解析时调用。
    pub fn parse<R: EndianReader>(
        class_id: i32,
        r: &mut R,
        ver: &UnityVersion,
    ) -> Result<Self, R::Error> {
        match class_id {
            1   => Ok(Self::GameObject(GameObject::parse(r, ver)?)),
            4   => Ok(Self::Transform(Transform::parse(r, ver)?)),
            28  => Ok(Self::Texture2D(Texture2D::parse(r, ver)?)),
            43  => Ok(Self::Mesh(Mesh::parse(r, ver)?)),
            114 => Ok(Self::MonoBehaviour(MonoBehaviour::parse(r, ver)?)),
            // ... codegen 完整生成
            id  => Err(R::Error::unknown_class_id(id)),
        }
    }
}
```

### 4.6 PPtr

```rust
// crates/generated/src/pptr.rs
use crate::parse::{EndianReader, Parse};
use unity_asset_primitives::UnityVersion;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct PPtr<T> {
    pub m_file_id: i32,
    pub m_path_id: i64,                           // ver >= 5 是 i64；老版本 i32（生成器按版本 emit）
    _phantom: PhantomData<T>,
}

impl<T> PPtr<T> {
    pub fn parse<R: EndianReader>(
        r: &mut R,
        ver: &UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_file_id = r.read_i32()?;
        let m_path_id = if *ver >= UnityVersion::new(5, 0, 0) {
            r.read_i64()?
        } else {
            r.read_i32()? as i64
        };
        Ok(PPtr { m_file_id, m_path_id, _phantom: PhantomData })
    }
}
```

PPtr 是 generic + PhantomData，不依赖具体类型 T 的 `Parse` 实现（磁盘上 PPtr 永远只是 file_id + path_id 两个整数）。

### 4.7 MonoBehaviour（特殊）

MonoBehaviour 的用户脚本字段不在 TPK 里。生成最小骨架：

```rust
#[derive(Debug, Clone)]
pub struct MonoBehaviour {
    pub parent: Behaviour,                        // ← 嵌套，照常继承
    pub token: Vec<u8>,                           // 用户脚本字段，原样保留
}
```

用户字段靠运行时 TypeTree 解释器（`crates/unity-asset` 的另一个模块，**不在 codegen 范围内**）从 SerializedFile 内嵌的 TypeTree 解析 `token` 切片。这与本设计正交。

---

## 5. 字段类型映射表（NodeType → Rust）

照搬 AssetRipper `UniversalNode.cs:47-80` 的 NodeType 分类（**不**靠字符串 tokenizer，靠 `(TypeName, 子树形态)` 二元组判定）：

| TypeTree NodeType | 子树形态 | Rust 类型 | parse 调用 |
|---|---|---|---|
| `bool` | 叶 | `bool` | `r.read_bool()?` |
| `char` / `UInt8` | 叶 | `u8` | `r.read_u8()?` |
| `SInt8` | 叶 | `i8` | `r.read_i8()?` |
| `short` / `SInt16` | 叶 | `i16` | `r.read_i16()?` |
| `ushort` / `UInt16` / `unsigned short` | 叶 | `u16` | `r.read_u16()?` |
| `int` / `SInt32` / `Type*` / `EntityId` | 叶 | `i32` | `r.read_i32()?` |
| `uint` / `UInt32` / `unsigned int` | 叶 | `u32` | `r.read_u32()?` |
| `SInt64` / `long long` | 叶 | `i64` | `r.read_i64()?` |
| `UInt64` / `FileSize` / `unsigned long long` | 叶 | `u64` | `r.read_u64()?` |
| `float` | 叶 | `f32` | `r.read_f32()?` |
| `double` | 叶 | `f64` | `r.read_f64()?` |
| `string` / `Utf8String` | 非叶，Array→char | `String` | `r.read_aligned_string()?` * |
| `TypelessData` | 非叶，Array→UInt8 | `Vec<u8>` | `{ let n=r.read_i32()? as usize; r.read_bytes(n)? }` |
| `Array`（其它） | size + data 子节点 | `Vec<T>` | `{ let n=r.read_i32()?; (0..n).map(\|_\| T::parse(r, ver)).collect() }` |
| `vector` / `staticvector` / `set` | 1 个 Array 子节点 | `Vec<T>` | 同 Array（生成器展开后等价）|
| `map` | Array → pair | `Vec<(K, V)>` † | 见下 |
| `pair` | first + second | `(K, V)`（**具名 subclass** 替代元组）† | `PairStruct::parse(r, ver)?` |
| `PPtr<X>` | 2 叶 file_id + path_id | `PPtr<X>` | `PPtr::parse(r, ver)?` |
| 其它非叶（struct） | 任意 | 具名 subclass struct | `XxxStruct::parse(r, ver)?` |

\* `string` 在 ver >= 5 是 aligned string（4 字节对齐），ver < 5 是 size-prefixed char[]。生成器按版本 emit 不同代码。

† `map` / `pair` **不生成元组**，而是生成具名 subclass（`ColorPair`、`KeyValuePair_String_Texture2D` 之类），便于字段名有意义（`first` / `second` 太抽象）。map 的 Rust 类型是 `Vec<PairStruct>`，**不**是 `HashMap`（Unity 的 map 不保证去重，且字段顺序在磁盘上是有意义的）。

### 5.1 版本门控字段

| 版本范围形态 | 字段类型 | parse 代码 |
|---|---|---|
| 全版本 | `T` | `let x = r.read_i32()?;` |
| `ver >= V` | `Option<T>` | `let x = if *ver >= V { Some(r.read_i32()?) } else { None };` |
| `V1 <= ver < V2` | `Option<T>` | `let x = if *ver >= V1 && *ver < V2 { Some(...) } else { None };` |
| 不连续 | `Option<T>` | `let x = if matches!(*ver, v if GATE_SET.contains(&v)) { Some(...) } else { None };`（罕见，首版可 panic）|

### 5.2 对齐

节点的 `meta_flag` 位：

| 位 | 名 | 含义 |
|---|---|---|
| `0x4000` | `AlignBytes` | 该字段后对齐到 4 字节 |
| `0x8000` | `AnyChildUsesAlignBytes` | 子树里某处需对齐 |

**生成规则**：节点 `meta_flag & (0x4000 | 0x8000) != 0` → 该字段 parse 调用后 emit `r.align(4)?;`。完全信任 MetaFlag（AssetRipper 同样做法），不额外启发式。

---

## 6. Codegen 内部架构

### 6.1 crate 结构

```
crates/codegen/
├── Cargo.toml
└── src/
    ├── main.rs                      # CLI: --tpk X --enums Y --native-enums Z -o OUT_DIR
    ├── lib.rs                       # 库形态（供测试）
    ├── tpk/                         # TPK 二进制读取（小端序，~250 行）
    │   ├── mod.rs
    │   ├── file.rs                  # TpkFile: magic 0x2A4B5054 + header + 解压
    │   ├── type_tree_blob.rs        # TpkTypeTreeBlob: CreationTime + Versions + ClassInformation + CommonString + NodeBuffer + StringBuffer
    │   ├── compression.rs           # lz4_flex / brotli / lzma-rs 解压
    │   └── string_buffer.rs         # .NET 7-bit varint + UTF-8
    ├── ir/                          # 中间表示
    │   ├── mod.rs
    │   ├── unity_class.rs           # ResolvedClass（节点树展开后）
    │   ├── unity_node.rs            # RawNode（节点池原始元素）
    │   └── version.rs               # UnityVersion 解析 + 偏序比较
    ├── pass/                        # 顺序 pass 管线（8 个）
    │   ├── mod.rs
    │   ├── p00_load_tpk.rs
    │   ├── p01_normalize_types.rs
    │   ├── p02_extract_subclasses.rs
    │   ├── p03_extract_pptrs.rs
    │   ├── p04_resolve_inheritance.rs
    │   ├── p05_merge_fields.rs      # ★ 字段并集 + 版本门控推断
    │   ├── p06_plan_alignment.rs
    │   └── p07_emit.rs              # 用 quote + prettyplease 生成 .rs
    ├── emit/                        # 代码发射辅助
    │   ├── mod.rs
    │   ├── rust_type.rs             # IR NodeType → TokenStream
    │   ├── parse_body.rs            # 字段 parse 调用 → TokenStream
    │   └── names.rs                 # §2 命名规则实现
    └── tests.rs                     # 黄金测试（Texture2D 等样本）
```

依赖：`lz4_flex`、`brotli`、`lzma-rs`、`proc-macro2` + `quote` + `syn`、`prettyplease`。

### 6.2 IR 数据结构

```rust
// 解包后的 TypeTree（TPK 镜像）
pub struct TpkTypeTree {
    pub versions: Vec<UnityVersion>,                          // 升序
    pub classes: HashMap<i32, Vec<ClassAtVersion>>,           // classID → 按版本分段
    pub string_pool: Vec<String>,
    pub node_pool: Vec<RawNode>,
}

pub struct ClassAtVersion {
    pub range: VersionRange,                                  // [start, end)
    pub class: Option<ResolvedClass>,                        // None = 该区间不存在
}

pub struct ResolvedClass {
    pub name: String,
    pub base_name: Option<String>,
    pub is_abstract: bool,
    pub release_root: Option<NodeRef>,                       // 节点池索引展开后的树
    pub editor_root: Option<NodeRef>,
}

pub struct RawNode {
    pub type_name: String,
    pub name: String,
    pub byte_size: i32,
    pub node_version: i16,
    pub type_flags: u8,
    pub meta_flag: u32,
    pub sub_nodes: Vec<usize>,
}

// Codegen 内部状态（pass 之间传递）
pub struct EmitClass {
    pub class_id: i32,
    pub name: String,                                         // 已 Rust 化（Texture2D）
    pub parent: ParentSpec,
    pub own_fields: Vec<EmitField>,                           // 仅本类字段
    pub is_abstract: bool,
    pub is_mono_behaviour: bool,
}

pub enum ParentSpec {
    Root,
    Stable(String),                                           // 稳定父类 Rust 名
    Variable {
        enum_name: String,                                    // AudioClipParent
        branches: Vec<(VersionGate, String)>,                 // [(< 5.0.0, NamedObject), (>= 5.0.0, SampleClip)]
    },
}

pub struct EmitField {
    pub rust_name: String,                                    // width
    pub rust_type: RustType,                                  // i32 / Option<...> / Vec<u8> ...
    pub version_gate: Option<VersionGate>,
    pub align_after: bool,
}

pub enum RustType {
    Bool, I8, U8, I16, U16, I32, U32, I64, U64, F32, F64,
    String,
    Bytes,                                                    // TypelessData
    List(Box<RustType>),
    Pair(Box<RustType>, Box<RustType>),                       // map 的元素
    PPtr { target_class: String },
    Subclass(String),
    Option(Box<RustType>),
}
```

### 6.3 Pass 系统（顺序管线）

仿 AssetRipper 但大幅简化（AssetRipper 50+ pass，Rust 端 8 个够用）：

| # | Pass | 职责 | 关键逻辑 |
|---|---|---|---|
| 00 | `p00_load_tpk` | bytes → `TpkTypeTree` | 二进制反序列化 + 解压 + 索引展开。UnityVersion 8 字节拆位（major/minor/build/type/type_number）。.NET 7-bit varint 解码字符串 |
| 01 | `p01_normalize_types` | `RawNode.type_name` 归一化 | 应用别名表（`SInt32 → int`、`unsigned int → uint` 等 ~10 对），照搬 AssetRipper `UniversalNode.cs:131-143` |
| 02 | `p02_extract_subclasses` | 内嵌 struct 抽出 | 扫所有节点的子树，凡 `NodeType::Type`（非 primitive / 非 vector / 非 PPtr）的子树抽成独立 subclass。同形态（节点树 hash 相同）只生成一份 |
| 03 | `p03_extract_pptrs` | PPtr 注册 | 所有 `PPtr<X>` 映射到 `PPtr<X>`（generic phantom）|
| 04 | `p04_resolve_inheritance` | 填 `EmitClass.parent` | 见 §7.1 算法 |
| 05 | `p05_merge_fields` ★ | 本类 `own_fields` 并集 + 门控 | 见 §7.2 算法 |
| 06 | `p06_plan_alignment` | 填 `EmitField.align_after` | `meta_flag & (0x4000 \| 0x8000) != 0` → `true` |
| 07 | `p07_emit` | `Vec<EmitClass>` → Rust 源文件 | `quote!` 生成 TokenStream，`prettyplease::unparse` 格式化，写文件 |

Pass 之间无依赖图，纯顺序（与 AssetRipper 一致）。

---

## 7. 关键算法

### 7.1 Pass 04：继承链解析

```
对每个 classID:
    bases = { vc.base_name for vc in all_version_variants if vc.base_name.is_some() }

    match bases.len() {
        0 => ParentSpec::Root,
        1 => ParentSpec::Stable(to_rust_type_name(bases[0])),
        _ => {
            // 1. 按版本分段切，每段记 base_name
            sections = [(range, base) for (range, vc) in version_variants if vc.base_name.is_some()]
            
            // 2. ★ 合并相邻同 base 的段
            //    （TPK 因别的字段变化产生多个分段，但 base 没变；合掉避免 enum 分支虚高）
            merged = coalesce_adjacent_same_base(sections)
            
            // 3. 转 VersionGate + Rust 类型名
            branches = merged.iter().map(|(range, base)| {
                (range_to_version_gate(range), to_rust_type_name(base))
            }).collect()
            
            ParentSpec::Variable {
                enum_name: format!("{}Parent", class_name),
                branches,
            }
        }
    }
```

`coalesce_adjacent_same_base` 直接搬 AssetRipper `Pass005_SplitAbstractClasses.cs:192-214` `GetMergedSections`。

### 7.2 Pass 05：字段并集 + 版本门控（核心）

**输入**：一个 class 的所有版本变体（`Vec<ClassAtVersion>`，每个含一棵 release_root 节点树）
**输出**：`Vec<EmitField>`

```
# 1. 展平每个版本的节点树，丢弃中间容器节点（Array / vector / pair / map 本身）
flat_i = flatten_with_typetype(root_i) for i in versions
# flat_i: [(field_name, node_type, byte_size, meta_flag, stream_position)]

# 2. 收集所有版本里出现的字段名并集
all_field_names = ∪_i { field.name for field in flat_i }

# 3. 对每个字段，计算"出现的版本集合"
for name in all_field_names:
    present_ranges = ∪_i { range_i | name ∈ flat_i }
    gate = compute_version_gate(present_ranges, all_versions)
    # gate ∈ { Always, Since(V), Range(V1..V2), Disjoint(...) }
    # Disjoint 极罕见，首版可 panic 或落 matches!

# 4. 字段排序：按"在大多数版本里出现的 stream 位置"中位数
fields_sorted = sort_by_median_stream_position(all_field_names)

# 5. 类型一致性检查
for name in all_field_names:
    types = { node_type(name in flat_i) for i where name ∈ flat_i }
    if len(types) > 1:
        # ADR-0001 §15 提到的 814 个"类型变化"
        # 绝大多数是 SInt32↔int（p01 已归一化），真冲突（如 path_id: i32→i64）落 Option + 版本 match
        emit_version_dispath(...)

# 6. 生成 EmitField（rust_name 经 §2.1 转换；rust_type 经 §5 映射 + Option 包裹）
```

**版本门控 emit 规则**：

| `gate` 形态 | 字段类型 | parse 代码 |
|---|---|---|
| `Always` | `T` | `let x = r.read_i32()?;` |
| `Since(V)` | `Option<T>` | `let x = if *ver >= V { Some(r.read_i32()?) } else { None };` |
| `Range(V1..V2)` | `Option<T>` | `let x = if *ver >= V1 && *ver < V2 { Some(...) } else { None };` |
| `Disjoint(...)` | `Option<T>` | `let x = if matches!(*ver, v if GATE_SET.contains(&v)) { Some(...) } else { None };` |

### 7.3 Pass 07：代码发射

用 `quote!` + `proc-macro2`，**不**用字符串拼接（避免转义噩梦）。

```rust
fn emit_class(c: &EmitClass) -> TokenStream {
    let name = format_ident!("{}", c.name);
    let parent_field = match &c.parent {
        ParentSpec::Root => quote! {},
        ParentSpec::Stable(p) => {
            let p_ident = format_ident!("{}", p);
            quote! { pub parent: #p_ident, }
        }
        ParentSpec::Variable { enum_name, .. } => {
            let e_ident = format_ident!("{}", enum_name);
            quote! { pub parent: #e_ident, }
        }
    };
    let field_decls = c.own_fields.iter().map(|f| {
        let n = format_ident!("{}", f.rust_name);
        let t = emit_rust_type(&f.rust_type);
        quote! { pub #n: #t }
    });
    let parse_parent = emit_parse_parent(&c.parent);
    let parse_fields = c.own_fields.iter().map(emit_field_parse);
    let construct_fields = c.own_fields.iter().map(|f| {
        let n = format_ident!("{}", f.rust_name);
        quote! { #n }
    });
    let parent_enum_def = emit_parent_enum_if_variable(&c.parent);

    quote! {
        #parent_enum_def

        #[derive(Debug, Clone)]
        pub struct #name {
            #parent_field
            #(#field_decls,)*
        }

        impl crate::Parse for #name {
            fn parse<R: crate::EndianReader>(
                r: &mut R,
                ver: &unity_asset_primitives::UnityVersion,
            ) -> Result<Self, R::Error> {
                #parse_parent
                #(#parse_fields)*
                Ok(#name {
                    parent_fields_init
                    #(#construct_fields,)*
                })
            }
        }
    }
}
```

`emit_parent_enum_if_variable` 在 `ParentSpec::Variable` 时生成 enum 定义，紧贴 struct 之前。

---

## 8. Reader trait 与 primitives crate

由于 `crates/generated` 必须无依赖（独立编译缓存），但 parse 代码需要 `EndianReader` trait + `UnityVersion` 类型，需要一个小 primitives crate：

```
crates/unity-asset-primitives/        # 极小，无依赖
└── src/
    ├── lib.rs
    ├── version.rs                    # UnityVersion（major/minor/build + 偏序比较）
    └── ...
```

`crates/generated` 定义 `EndianReader` trait 自己（因为它和 `Parse` 紧耦合），依赖 `unity-asset-primitives` 拿 `UnityVersion`。`crates/unity-asset` 为现有的 bundle Cursor（待扩成 endian + seek + 全原语）impl 这个 trait，作为 SerializedFile parse 的 reader。

```rust
// crates/generated/src/parse.rs
use unity_asset_primitives::UnityVersion;

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
    fn endian(&self) -> crate::Endian;
}

pub trait Parse: Sized {
    fn parse<R: EndianReader>(r: &mut R, ver: &UnityVersion) -> Result<Self, R::Error>;
}
```

> **未来重构**：当 codegen 多 crate 化时（pyo3 binding 等），`EndianReader` 和 `UnityVersion` 都可重抽到独立底层 crate。当前阶段保留在 `crates/generated` 内 + `unity-asset-primitives` 即可。

---

## 9. 与现有架构的关系

```
┌─────────────────────────────────────────────────────┐
│ crates/codegen (bin, 离线工具)                       │
│  ↓ 跑一次，提交产物                                  │
└─────────────────────────────────────────────────────┘
                        ↓
┌─────────────────────────────────────────────────────┐
│ crates/generated (lib, pure data)                    │
│  - struct Texture2D, Mesh, ... (~320 个)              │
│  - enum UnityObject 分派                              │
│  - trait Parse / EndianReader                         │
│  依赖: unity-asset-primitives                        │
└─────────────────────────────────────────────────────┘
                        ↑ 依赖
┌─────────────────────────────────────────────────────┐
│ crates/unity-asset (lib + bin)                       │
│  - 现有: bundle.rs / decompress.rs / error.rs        │
│  - 新增: serialized_file.rs (用 generated 的 Parse)    │
│  - 新增: world.rs (聚合 objects + resources)          │
│  - 为 Cursor impl generated::EndianReader            │
│  依赖: generated, unity-asset-primitives, bytes, ... │
└─────────────────────────────────────────────────────┘
```

与 [file-loading-layer.md](file-loading-layer.md) §8 的 crate 拆分表一致，仅多了 `crates/unity-asset-primitives` 一个极小 crate（§8 表格里没列，需补充）。

---

## 10. 实施步骤（增量交付）

| 阶段 | 产出 | 验证 |
|---|---|---|
| **D1**：TPK reader | `crates/codegen/src/tpk/*`，跑通读 `type_tree.tpk`，打印前 10 个 class 的字段树 | 对照 AssetRipper 解出的 InfoJson，字段 byte_size / flags 完全一致 |
| **D2**：IR + p01-p04 | `EmitClass` 含 parent + own_fields；命名规则实现 | 打印 Texture2D 的 EmitClass，人工核对 ancestors / fields / Rust 名 |
| **D3**：p05（核心）+ p07 | Texture2D 单类 emit 成 Rust | cargo 编译 `crates/generated`，对照 UnityPy 解同一 .bundle 的 Texture2D 字段值 |
| **D4**：p06 + 全量 emit | 366 class 全 emit，`crates/generated` 完整 | 编译通过；用 `example/char_002_amiya.ab` 解出所有对象，字段值与 UnityPy 对比 |
| **D5**：MonoBehaviour 兜底 | 最小骨架（继承照常 + `token: Vec<u8>`）；运行时 TypeTree 解释器作为 `crates/unity-asset` 的独立模块 | MonoBehaviour 字段头对齐 |
| **D6**：CI / 重构流程 | 文档化"TPK 升级 → 重跑 codegen → 提交"流程；`.gitattributes` 标 `linguist-generated` | 跑一遍新 TPK |

D1-D3 是核心工作量（约占 70%）。D4 的全量 emit 后会暴露很多边角 bug（罕见字段类型、奇怪继承、editor-only 异常），需要逐个解。

---

## 11. 风险与未决

| 项 | 状态 | 说明 |
|---|---|---|
| MonoBehaviour 用户字段 | 必做 | TPK 不含。靠运行时 TypeTree 解释器（ADR-0001 备选 A 的局部应用）。属 codegen 之外、`crates/unity-asset` 内的另一模块。D6 阶段同步实现 |
| editor vs release | 已决定 | 首版只读 release_root（用户拿到的 .bundle 99% 是 release）。editor-only 字段在 release build 里不出现。如用户加载编辑器构建会漏字段，需要时再加 editor root 支持 |
| 字符串 sanitize | 已规定 | §2 命名规则覆盖所有已知情形（`m_` 前缀、缩写、关键字冲突、PPtr 例外）。`crates/codegen/src/emit/names.rs` 实现一份 sanitize 函数 + 黄金测试 |
| 生成代码量 | 已估算 | 366 class × 平均 30 字段 ≈ 11k 字段，每字段 parse 行 ~3 行 → parse 函数 ~33k 行；struct 声明 ~7k 行；subclass + UnityObject 分派 ~5k 行。**合计约 4.5–5 万行**，是 ADR-0001 §51 估算的 2.5 倍。`.gitattributes` 标 `linguist-generated` 让 GitHub diff 默认折叠 |
| 类型真冲突 | 待 D5 暴露 | ADR-0001 §15 提到 814 个"类型变化"，绝大多数是命名归一化（p01 已处理）。真冲突（如 `m_PathID: i32 → i64`）落到 `Option<T>` + 版本 match，D5 阶段逐个解决 |
| enum 字段全 i32 | 已决定 | 用户需要 enum 语义（TextureFormat/BuildTarget 等）时自己在调用端 cast。MVP 不做，未来需要高频 enum 时可手写 ~20 个补充进 `crates/generated/src/enums.rs`，与 codegen 解耦 |

---

## 12. 与 ADR-0001 的差异

本设计在 ADR-0001 基础上做了若干细化和调整，列出差异：

| 维度 | ADR-0001 原文 | 本设计 | 理由 |
|---|---|---|---|
| 继承 | "组合复用 `parent: ConcreteType`"（§24）| 嵌套 parent + 父类 enum（23 个变化分支）| 平铺会丢失类型层次；父类变化用 enum 精确表达 |
| enum | "类型归一化表消除命名差异"（§26），未提 enum 来源 | **不做 enum**，整数字段一律 `i32` | TPK 不含 enum；保持纯 TPK 输入，零外部依赖 |
| 输出 | "通过 `build.rs` 在 `OUT_DIR` 编译"（§83）| 提交到 `crates/generated/src/` 独立 crate | `file-loading-layer.md` §8 已更新为独立 crate；独立编译缓存收益 |
| Reader trait | 未提 | 定义在 `crates/generated/src/parse.rs`，依赖 `unity-asset-primitives` | `crates/generated` 必须无外部依赖，trait 自含 |
| Arena | "bumpalo 分配"（§27）| 首版 `Vec<UnityObject>`（owned）| `file-loading-layer.md` §4.3 已更新；避免 `'arena` 生命周期污染 API |

后续 ADR-0001 应按本设计修订。
