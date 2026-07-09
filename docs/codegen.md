# Codegen：从 TPK 生成类型化 Rust 结构与 Parser

> 范围：`crates/codegen` 的架构、TPK → Rust 数据流、生成代码的最终形态。
> 状态：**本文档描述已落地的实现**（与 `crates/codegen/src/` + `crates/generated/src/` 一致）。
> 上游：[ADR-0001](adr/0001-codegen-from-typetree-dumps.md)（codegen 决策，含修订附录）、[`object-layer.md`](object-layer.md)（生成产物的运行时 dispatch）。

---

## 1. 概述

codegen 是一个**离线工具**（`crates/codegen`，bin + lib），从 `type_tree.tpk`
（AssetRipper 出品，brotli 压缩后约 186KB）生成 `crates/generated/` 下的 Rust 源码
（提交进仓库，独立编译缓存）。**它不链接 `unity-core` 或 `generated`**——只 emit 引用
它们的源码字符串。

```
type_tree.tpk ──→ crates/codegen (p00–p06 + emit) ──→ crates/generated/src/*.rs
                   2433 行手写                          ~9.4 万行 / 940 文件
```

### 输入

`type_tree.tpk`：跨版本去重的 TypeTree 二进制容器，含 1395 个 Unity 版本
（3.4 → 6000.x）、412 个 classID 的 TypeTree 数据。

### 输出

- **结构体定义**：每个具体 class 一个扁平 `pub struct` + `impl Parse`
- **分派 enum**：`enum UnityObject` + `enum ClassIDType`
- **版本枚举**：字段类型跨版本变化时生成 `enum XxxVariant` + `impl Parse`

### 四条设计约束（硬性）

1. **只读 release root**。editor root 不参与生成（用户拿到的 .bundle 99% 是 release）。
   例外：p02 抽 subclass 时扫整个 `node_pool`（否则 editor-only 的 subclass 会漏，见 §12.3）。
2. **每个 class 只生成一个 struct**。跨版本差异用 `Option<T>` 字段表达；版本门控写在
   parse 函数里；版本间相同的读取逻辑只写一遍。
3. **不处理 MonoBehaviour（classID=114）**。脚本字段不在 TPK 里；运行时由主 crate 用
   AB 内嵌 TypeTree 解释器处理（见 `object-layer.md`）。
4. **不生成父类，只生成子类**。TPK 里抽象父类 `release_root` 为空、字段已平铺到具体
   子类；codegen 不再生成父类 struct、也不再生成 `parent` 字段。

### 不生成的

| 不做什么 | 原因 |
|---|---|
| MonoBehaviour (classID=114) | 脚本字段不在 TPK；运行时走 TypeTree 解释器 |
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

use crate::PPtr;
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

impl crate::Parse for Texture2D {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        // 所有字段按 TPK release_root 顺序直接读
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_width = r.read_i32()?;
        let m_height = r.read_i32()?;
        let m_forced_fallback_format = if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            Some(r.read_i32()?)
        } else { None };
        let m_texture_format = r.read_i32()?;
        // ...
        let image_data = { let n = r.read_i32()? as usize; r.read_bytes(n)? };
        let m_stream_data = if *ver >= crate::UnityVersion::new(5, 3, 0) {
            Some(StreamingInfo::parse(r, ver)?)
        } else { None };

        Ok(Texture2D { m_name, m_width, m_height, /* ... */ })
    }
}
```

**关键特征**：

- **无 parent 字段**：所有继承链字段直接在类上。TPK 的 release_root 已含全部字段。
- **无抽象父类**：不生成 `Object` / `NamedObject` / `Texture` 等。
- **版本门控**：跨版本差异字段用 `Option<T>` + `if *ver >= V`。
- **对齐**：`r.align(4)?` 由 `meta_flag & (0x4000 | 0x8000)` 决定，codegen 烤死。
- **用户访问**：`tex.m_name`（直接，不用 chain）。

### 2.2 父类变化的 class（AudioClip，原 23 例）

**不再生成 parent enum**。TPK 的每个版本变体都已有完整的扁平字段列表。p05 字段并集
自动处理：旧版本（继承 NamedObject）没有的字段在新版本（继承 SampleClip）出现 →
自动得到 `Since(5,0,0)` 门控。

```rust
#[derive(Debug, Clone)]
pub struct AudioClip {
    pub m_name: String,                    // NamedObject/SampleClip 都有
    pub m_compressed: Option<bool>,        // SampleClip 才有 → ver >= 5.0
    pub m_audio_data: Vec<u8>,             // AudioClip 自己
    // ...
}
```

### 2.3 字段类型跨版本变化：版本枚举（VersionedEnum）

同一字段在不同版本里出现**不同 `RustType` 或不同 `align_after`** 时（如
`Mesh.m_shapes` 旧版为 `Vec<MeshBlendShape>`、新版为 `Vec<MeshBlendShapeVertex>`），
不再「取最新版本类型」，而是为该字段生成一个**版本枚举**：

```rust
// crates/generated/src/classes/game_object.rs
pub enum GameObjectMComponentVariant {
    Default(Vec<(i32, PPtr)>),
    V5_5(Vec<ComponentPair>),
    V2017_1(Vec<ComponentPair>),
}

impl crate::Parse for GameObjectMComponentVariant {
    fn parse<R: crate::EndianReader>(rdr: &mut R, ver: &crate::UnityVersion) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = /* 读 Vec<ComponentPair> */;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else if *ver >= crate::UnityVersion::new(5, 5, 0) {
            Ok(Self::V5_5(/* 读 Vec<ComponentPair> */))
        } else {
            Ok(Self::Default(/* 读 Vec<(i32, PPtr)> */))
        }
    }
}
```

- 枚举名：`{ClassName}{FieldNamePascal}Variant`（如 `GameObjectMComponentVariant`）。
- 变体命名按版本号：`V5_6`、`V2017_4`；build 非 0 时附加 build 号（`V2017_4_39`），
  避免同 major.minor 不同 build 的变体重名。`Always` 变体命名为 `Default`。
- **宿主 class 不对 `VersionedEnum` 字段额外对齐**——`emit` 检测到该类型时跳过 align，
  完全交给枚举的 `Parse` 实现。
- 详见 §5.4。

### 2.4 enum UnityObject（分派）

```rust
// crates/generated/src/unity_object.rs
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

### 2.5 基础设施（lib.rs）

`crates/generated` 依赖 `unity-core`，自身只加 `ClassIDType` + `UnityObject` + 模块
声明，共享原语从 `unity_core` re-export：

```rust
// crates/generated/src/lib.rs
pub mod classes;
pub mod subclasses;
mod unity_object;
mod class_id;

pub use class_id::ClassIDType;
pub use unity_object::UnityObject;
// Parse / EndianReader / UnityVersion / PPtr / Endian 都从 unity_core re-export
pub use unity_core::{Endian, EndianReader, Parse, PPtr, UnityVersion};
```

codegen 生成的代码里统一用 `crate::Parse` / `crate::EndianReader` / `crate::PPtr` /
`crate::UnityVersion` 引用，经 lib.rs 的 re-export 解析到 `unity_core`。

---

## 3. 共享 crate：unity-core

三个 crate（主 crate `unity-asset-rs`、`crates/generated`、`crates/codegen`）共用的
类型抽到一个极小的共享 crate（`crates/unity-core`，约 150 行，零外部依赖），避免
重复定义和类型不匹配。详见 [`object-layer.md`](object-layer.md) §6。

### 依赖关系

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

- `crates/generated`：`use unity_core::{EndianReader, Parse, UnityVersion, PPtr}`；每个
  struct `impl unity_core::Parse`（生成代码里写作 `impl crate::Parse`）。
- `src/`（主 crate）：`impl unity_core::EndianReader for Cursor`；调
  `UnityObject::parse()` 时传 `&unity_core::UnityVersion`。
- `crates/codegen`：emit 时生成 `crate::Parse` 等引用；**自身不 link unity-core**。

---

## 4. 命名规则

实现位置：`crates/codegen/src/names.rs`。

### 4.1 字段名（`to_rust_field_name`）

```
Unity 原名              →  Rust 字段名
─────────────────────────────────────────────
m_Width                 →  m_width              (保留 m_ 前缀)
m_TextureFormat         →  m_texture_format
m_GameObject            →  m_game_object
image_data              →  image_data           (已是 snake_case)
image data              →  image_data           (空格 → 下划线)
Type vs.                →  type_vs              (非标识符字符 → 下划线)
type                    →  r#type               (关键字用 r# 转义)
4_3                     →  f_4_3                (首字符数字 → 加 f_ 前缀)
m_RGBColor              →  m_rgb_color          (heck 处理连续大写)
```

**算法**：
1. `heck::ToSnakeCase` 转 snake_case（保留 `m_`/`k_` 前缀，处理空格/点等非标识符字符）。
2. 首字符是数字 → 加 `f_` 前缀。
3. 命中 Rust 关键字 → 加 `r#` 转义。

### 4.2 类型名（`to_rust_type_name`）

```
Unity 原名              →  Rust 类型名
─────────────────────────────────────────────
Texture2D               →  Texture2D            (PascalCase 保留)
Vector3f                →  Vector3f             (数字 + 字母保留)
ColorRGBA               →  ColorRgba            (缩写规则化)
AABB                    →  Aabb
GUID                    →  Guid
PPtr<Component>         →  PPtr<Component>      (PPtr 例外，原样保留)
float                   →  float_               (Rust 原始类型名加 _ 后缀)
void                    →  void_
```

**算法**：
1. `PPtr<X>` 原样返回。
2. `collapse_acronyms` 处理连续大写字母（AABB→Aabb）。
3. 命中 Rust 原始类型/关键字 → 加 `_` 后缀（struct 名不能用 `r#`）。

### 4.3 模块 / 文件命名

| 类型 | 路径 |
|---|---|
| Unity class | `classes/{snake_name}.rs`（`texture_2_d.rs`） |
| Unity subclass | `subclasses/{snake_name}.rs` |
| `enum UnityObject` | `unity_object.rs` |
| `enum ClassIDType` | `class_id.rs` |

---

## 5. Pipeline（p00–p06 + emit）

实现位置：`crates/codegen/src/pipeline.rs`。**纯顺序管线，无依赖图**（与 AssetRipper
一致）。`run_pipeline` 产出 `Pipeline { classes, pptr_targets, all_versions }`，再由
`emit.rs::emit_all` 落盘。

| # | Pass | 函数 | 职责 |
|---|---|---|---|
| 00 | 过滤引擎类型 | `p00_filter_engine_types` | 移除 classID 100000-100011（`int`/`bool`/`float`/`Vector3f`/... 注册了 classID 但从不序列化，对齐 AssetRipper `Pass000`） |
| 00b | 合并 moved groups | `p00b_merge_moved_groups` | 把被 Unity 移动过 classID 的类合并到新 ID（194→238 NavMesh、197→258 LightProbes、1011→319 AvatarMask、327→329 VideoClip），对齐 AssetRipper `Pass001`（见 §12.7） |
| 01 | 类型归一化 | `p01_normalize_types` | 字符串池别名归一化：`int→SInt32`、`short→SInt16` 等 ~10 对 |
| —  | 重命名 | `rename::run` ★ | 同名异构 type 拆分 + string→PropertyName 特判 + 纯字段重命名归一（对应 AssetRipper `Pass002`，见 §12.8/§12.9） |
| 02 | 抽 subclass | `p02_extract_subclasses` | 扫整个 `node_pool`，`NodeType::Type` 的 type_name 注册为 subclass（排除非抽象 main class 名字，见 §12.3） |
| 03 | 收集 PPtr 目标 | `p03_collect_pptr_targets` | 收集所有 `PPtr<X>` 的 target（记录用；PPtr 仍非 generic） |
| 04 | 创建 class 骨架 | `p04_resolve_inheritance` | 每个 main class 建 `EmitClass`（名字 + `is_abstract`/`is_mono_behaviour`）；**跳过抽象类**；**跳过 MonoBehaviour**；重名加 `_ID` 后缀 |
| 05 | 字段并集 + 版本门控 | `p05_merge_fields` ★ | **核心**：字段并集 + 版本门控 + VersionedEnum + version_layouts（见 §5.1–5.4） |
| 06 | 对齐（no-op） | `p06_plan_alignment` | **占位 no-op**——对齐已在 `flatten_root`/`compute_align_after` 阶段算完；保留编号便于和文档对照 |
| 07 | emit | `emit::emit_all`（emit.rs） | `Vec<EmitClass>` → `.rs` 文件（字符串拼接，不用 `quote!`） |

### 5.1 p05 字段并集 + 版本门控（核心）

```
输入：一个 class 的所有版本变体，每个含一棵 release_root 节点树

1. 展平每个版本的 root 子节点 → flat_fields_i
   （root 的直接子节点 = 该 class 的全部字段，含继承链）
2. 收集所有版本里出现的字段名并集（按 rust_name 去重）
3. 对每个字段：
   - 计算 present_versions
   - gate = if 全版本都有 { Always } else { Since(earliest) }
   - rust_type = if Always { T } else { Option<T> }
4. 按首次出现位置排序（保持字节流顺序）
5. 检测字段顺序跨版本是否兼容（merged_order_compatible）：
   - 兼容 → version_layouts 为空，emit 用线性 parse + per-field Option 门控
   - 不兼容 → 各版本真实字段顺序存入 version_layouts，emit 用 version-gated 分段（见 §5.2）
```

**不需要扣祖先字段**：没有 parent，所有字段直接在类上。
**不需要处理父类变化**：p05 的版本并集自动处理——不同父类的字段差异体现为 VersionGate。

### 5.2 字段顺序不兼容：version_layouts + 锚点分段

反例：某字段 `density` 在 v5.3 首次出现，且被插入到旧字段 `material` **之前**。
- 旧版本：`[m_width, material]`
- 新版本：`[m_width, density, material]`

按合并顺序读会把 `density` 字节当 `material`，字节错位。故必须检测顺序兼容性
（`merged_order_compatible` + `presence_is_monotonic`）；不兼容时用 `version_layouts`
记录每个版本区间真实字段顺序，emit 时按版本分段读取（锚点在所有版本顺序一致 → 只读一次，
变化的段才 version-gate）。

### 5.3 字段类型或对齐跨版本变化：VersionedEnum

当同一字段在不同版本里出现不同 `RustType` **或**不同 `align_after` 时，生成版本枚举
（见 §2.3 示例）。

> **为什么 align_after 变化也要 version-gate？** 同一类型在不同版本可能有不同对齐需求。
> 若统一取最新版本的 align，老版本会多读/少读对齐字节，导致后续字段错位。因此
> `(类型, align_after)` 任一变化都拆成新变体。

#### 构建规则（`build_versioned_enum`）

- 按 `(RustType, align_after)` 分组——类型不变但 `align_after` 变了也拆新变体。
- 每变体 `align_after` 取该版本区间首次出现版本的 `meta_flag` 计算。
- 枚举的 `Parse` 里按版本降序判断，实际选中的变体读完后按需 `r.align(4)?`。
- **宿主 class 不对 `VersionedEnum` 字段额外对齐**。

#### 与字段增减的交互

- 字段全版本都有但类型变化 → struct 字段类型为 `VersionedEnum`（非 `Option`）。
- 字段只在部分版本出现且类型变化 → `Option<VersionedEnum>`，先判字段是否存在，再判版本。

### 5.4 collect_imports

`emit.rs` 生成 `use` 语句时，`VersionedEnum.variants` 中每个变体的实际 `RustType`
都必须扫描，否则类型变化涉及 subclass 时会漏 import。详见 §12.2。

---

## 6. IR（中间表示）

实现位置：`crates/codegen/src/ir.rs`。

```rust
pub enum NodeType {   // TypeTree 节点种类，靠 (type_name, 子树形态) 判定
    Boolean, UInt8, Int8, UInt16, Int16, UInt32, Int32, UInt64, Int64,
    Single, Double, String, TypelessData, Array, Vector, Map, Pair, PPtr, Type,
}

pub enum RustType {
    Bool, I8, U8, I16, U16, I32, U32, I64, U64, F32, F64,
    String, Bytes,
    List(Box<RustType>),
    Pair(Box<RustType>, Box<RustType>),
    Pptr { target_class: String },     // 仍非 generic，但保留 target 信息
    Subclass(String),
    Option(Box<RustType>),
    VersionedEnum { name: String, variants: Vec<EnumVariant> },  // 跨版本类型变化
    Box(Box<RustType>),                // 打破 VersionedEnum 变体里的递归类型循环
                                       // （如 Clip → OffsetPtr → OffsetPtrDataVariant → Clip）
}

pub struct EnumVariant {
    pub gate: VersionGate,
    pub rust_type: RustType,
    pub align_after: bool,
}

pub enum VersionGate { Always, Since(u16,u16,u16), Range { from, to } }

pub enum ClassKey { Main(i32), Subclass(String) }

pub struct EmitClass {
    pub key: ClassKey,
    pub name: String,             // Rust 化类型名
    pub unity_name: String,       // 原始 Unity 名
    pub own_fields: Vec<EmitField>,
    pub is_abstract: bool,        // true → 不生成 struct
    pub version_layouts: Vec<VersionLayout>,  // 字段顺序跨版本不兼容时非空
}

pub struct VersionLayout {
    pub since: (u16, u16, u16),   // 该布局适用起始版本
    pub fields: Vec<EmitField>,   // 该版本的 own_fields（按字节流顺序）
}
```

> **无 `ParentSpec` / 无 `is_mono_behaviour`**：`EmitClass` 没有 parent 字段；
> MonoBehaviour 根本不会被加入 `classes`（p04 直接跳过 classID=114）。这是与早期
> 设计（嵌套 parent 模型）最大的差异。

---

## 7. 字段类型映射表

TPK 节点的 `(type_name, 子树形态)` → Rust 类型 + parse 代码（`node_to_rust_type`）：

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

### 对齐

某字段 parse 后是否 emit `rdr.align(4)?;` 由 `compute_align_after` 决定（与运行时
解释器 `src/typetree/interpret.rs` 的 `walk` 及 AssetRipper `Pass100_FillReadMethods.cs`
对齐口径一致）：

- **非容器字段**（标量 / struct / PPtr / string）：检查自身 `meta_flag & 0x4000`
- **vector / map / Array**：检查**容器节点自身**与 **Array wrapper 子节点**（`sub_nodes[0]`）
  **任一**带 `0x4000`（即 `node.AlignBytes || arrayNode.AlignBytes`）

| 位 | 名 | 含义 |
|---|---|---|
| `0x4000` | `AlignBytes` | 该字段后对齐到 4 字节 |
| `0x8000` | `AnyChildUsesAlignBytes` | 子树里某处需对齐（**不参与** align 判定，仅元信息） |

**只认 `0x4000`，不用 `0x8000`**：`0x8000` 仅表示「后代某处有对齐」，不代表此字段本身
读完后要对齐。AssetRipper 真正的 release parser（`Pass100`，经 `UniversalNode.AlignBytes`）
也只用 `0x4000`；`0x8000` 仅在 JsonSerializer 工具（`SerializablePrimitive.cs`）出现。

> 容器字段的 `node || wrapper` 是曾遗漏的坑（历史版本只查 wrapper，漏掉容器自身带
> `0x4000` 的 108 个真实节点，如 `m_Normals`(0x4001)、`m_AnimatorEvents`(0xc001)），
> 已修正。

---

## 8. TPK 二进制格式（简述）

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

解压后：`creationTime(i64)` + `versions[](u64×N)` + `classes[](变长)` + `commonString`
+ `nodeBuffer` + `stringBuffer`。

每个 `TpkUnityNode`：`typeName(u16)` + `name(u16)` + `byteSize(i32)` + `version(i16)` +
`typeFlags(u8)` + `metaFlag(u32)` + `subNodes(u16[])`。不存 `Level` / `Index`（树结构
由 subNodes 表达）。详见 `crates/codegen/src/tpk/`。

---

## 9. 输出目录布局

```
crates/generated/src/
├── lib.rs                    # use unity_core::*; + 模块声明
├── unity_object.rs           # enum UnityObject + parse dispatch
├── class_id.rs               # enum ClassIDType（含 MonoBehaviour=114，但 dispatch 不含）
├── classes/
│   ├── mod.rs
│   ├── game_object.rs        # struct GameObject { m_name, ... } + impl Parse
│   ├── texture_2_d.rs
│   ├── audio_clip.rs
│   └── ...                   # 366 个文件（不含抽象类、不含 MonoBehaviour）
└── subclasses/
    ├── mod.rs
    ├── vector3f.rs
    ├── streaming_info.rs
    └── ...                   # 571 个文件
```

实测规模：**940 个 .rs 文件 / 93,544 行**（366 main class + 571 subclass + 3 个特殊
文件 + 2 个 mod.rs）。标 `linguist-generated=true`（`.gitattributes`），GitHub diff
默认折叠。

---

## 10. 如何运行

```bash
# 重新生成（TPK 升级或 codegen 逻辑改动后）
rm -rf crates/generated/src && mkdir -p crates/generated/src
cargo run -p unity-asset-codegen -- \
    crates/codegen/tests/fixtures/type_tree.tpk \
    crates/generated/src

# 编译验证
cargo build --workspace
cargo test --workspace

# 只编译/测试 codegen 自身
cargo build -p unity-asset-codegen
cargo test  -p unity-asset-codegen          # TPK reader 黄金测试
cargo clippy -p unity-asset-codegen          # 应零警告
cargo run    -p unity-asset-codegen --example explain_inheritance  # 查看 TPK 继承链
```

生成后务必 `cargo build --workspace` 确认产物可编译。Edition `2024` → Rust ≥ 1.85。

### 用不同压缩格式的 TPK

AssetRipper 提供 4 种压缩（None / LZ4 / LZMA / Brotli），都支持。下载：

```
https://nightly.link/AssetRipper/Tpk/workflows/type_tree_tpk/master/<compression>_file.zip
```

其中 `<compression>` ∈ {`uncompressed`, `lz4`, `lzma`, `brotli`}。测试用 brotli（最小）。

---

## 11. 已知限制

| 项 | 现状 | 影响 |
|---|---|---|
| PPtr 非 generic | 所有 `PPtr<X>` 都生成 `PPtr` | 无类型区分；因 TPK 里有些 target（如 PrefabInstance）被改名/移除，generic 会要占位 struct |
| `VersionGate` 支持 Since/Range | 不支持 Disjoint | 罕见「消失后重现」字段会落到 Since，老版本可能多读字节 |
| emit 字符串拼接 | 无语法校验 | 边界 bug 容易引入（fixture 测试 + 主 crate `verify_codegen` 实跑覆盖主要形态） |
| generated crate warnings | `#![allow(...)]` 在 crate 根统一静音 | codegen 已知副产物（多余括号 / 未用版本参数 / 版本门控默认值覆盖 / 含数字类型名） |
| 只读 release | editor build 的字段会丢失 | — |
| **旧版 AudioClip 流式音频（pre-5.0）** | 生成代码无条件按内联数组读 `m_AudioData`（`i32 count + bytes + align`） | pre-5.0 且 `m_Stream==2`（Streaming）时，磁盘实际是 `u32 size; u32 offset;` 指向外部 `.resS`，typed parse 会把 `size` 当 count 导致后续全错位/越界。这是**值相关**特殊 case（对应 AssetRipper `Pass505_FixOldAudioClip` + `AudioClipHelper.ReadOldByteArray`），无法靠 TypeTree 变换解决，且 TypeTree 回退路径同样读错（两条路径都失败）。彻底修复需 `.resS` streaming_data 支持（见 §13 路线图）。example 的 Arknights bundle 不触发（AudioClip 均 5.0+ 或非流式）。 |
| **string 字段无条件 align（正确，非 bug）** | codegen（`emit.rs` 用 `read_aligned_string`）与运行时解释器（`interpret.rs`）都在读完 string 后**无条件** `align(4)` | 这是**正确**行为：AssetRipper `EndianSpanReaderExtensions.ReadUtf8StringAligned` 注释明说 "Alignment after strings has happened since 2.1.0"，即磁盘上所有 string 都被 padding 到 4 字节，与 `0x4000` 标志无关。曾疑为 bug（TPK 中 924/954 个 string 节点未标 0x4000），但实测把 align 改为条件式会让 `verify_codegen` 从 0 失败暴涨到 340 失败（1955 个无 0x4000 的 string 节点实际被遍历）。**不要改成条件 align。** |

---

## 12. 已踩到、但此前设计未显式覆盖的坑

实现过程中遇到并修复的 corner case（顺序即发现顺序）：

1. **字段名带空格**（`image data`）→ heck snake_case 处理
2. **`Option<Subclass>` 没 import** → `collect_type_imports` 递归
3. **align 时机错**（gated 字段在 if 外 align）→ 重构 emit 让 align 在 if 内
4. **closure 内 `?` 错** → field_read_expr 改为返回 `Result<T, E>` 表达式，调用方加 `?`
5. **`PPtr<PrefabInstance>` 引用不存在 class** → PPtr 改非 generic
6. **VideoClip class ID 复用同名**（`VideoClip=327` 和 `329`）→ 重名加 `_<classID>` 后缀
7. **字段名 `4_3` 以数字开头** → 加 `f_` 前缀
8. **scrollposition_x 重复**（unity_name 不同但 rust_name 相同）→ 按 rust_name 去重
9. **class 名 `float` / `void`**（Rust 关键字）→ 加 `_` 后缀（`float_` / `void_`）
10. **Variable parent enum 重复 variant** → （已废弃，扁平化后无 parent）

### 12.1 顺序重排分支里的 `align_after` 必须按版本取

version-gated parse 分支里读字段时：`rust_type` 取 merged `own_map`（与 struct 字段
类型一致，避免编译错误），但 **`align_after` 必须取该版本 layout 里的值**，不能取 merged
值。反例：`CircleCollider2D.m_used_by_effector` 在 v5.6 无 `align_after`、在 v2023.1
有；若 v5.6 分支用 merged 的 `align_after=true` 会多读 4 字节。

> 版本枚举字段不受此规则影响：其 `align_after` 完全由枚举 `Parse` 实现处理。

### 12.2 `collect_imports` 必须扫版本枚举变体

`emit.rs` 生成 `use` 语句时，`VersionedEnum.variants` 中每个变体的实际 `RustType`
都必须扫描，否则类型变化涉及 subclass 时会漏 import。`collect_type_imports` 递归扫描。

### 12.3 p02 抽 subclass 实际扫整个 `node_pool` 而非只扫 release_root

设计约束 1 说「只读 release root」，但 subclass 抽取无法只读 release root：某些
subclass（如 `PackingParameters`）只出现在 editor 树的 `SpriteAtlasEditorData` 里，
release root 里没有；若只扫 release root 会漏掉，导致引用它们的 class 编译失败。
因此 p02 扫整个 `node_pool`，但排除 main class 名字以避免同名文件重复。

> 这是「只读 release」约束的已知例外。

### 12.4 TPK 版本去重：同 (major, minor, build) 不同 type 后缀

TPK 的 `TpkClassInformation.classes` 按 u64 原始值升序。Unity 版本 u64 编码含
`version_type`（Alpha/Beta/Final/...）位，故同 `(major, minor, build)` 但不同 type
后缀（如 `2021.1.0b1` 和 `2021.1.0f1`）会作为相邻但独立的条目存在。

若不去重，`p05` 会把两个条目都纳入 → `build_versioned_enum` 按 `(类型, align_after)`
分组时可能生成同名变体（`V2021_1` 重复定义），`version_layouts` 出现 `since` 相同的
分支。**修复**：遍历 `info.classes` 时，若当前条目 `(major, minor, build)` 与上一条
相同则替换（取后者，即更接近 final 的版本）。

### 12.5 版本枚举变体命名的稳定性

变体名按版本号命名（`V5_6`、`V2017_4`；build 非 0 附加 build 号 `V2017_4_39`）；
`Always` 变体命名为 `Default`。避免用 `Legacy`/`Modern`（「现代」是相对概念，TPK
更新后含义会变）。

### 12.6 classID 100000-100011 的过滤

TPK 里 classID 100000-100011（`int`/`bool`/`float`/`Vector3f`/...）注册为 class 但
从不作为独立对象序列化，且 `release_root=None`。必须在 p00 过滤，否则生成空 struct
+ 空 parser，引用它们的类（如 `Transform` 引用 `Vector3f`）会读 0 字节，导致后续字段
错位。

### 12.7 合并 moved groups（p00b，对应 AssetRipper Pass001）

Unity 在版本演进里把若干同名类挪到了新 classID：旧 ID 在某版本起变 null/废弃，新 ID
从该版本起接管。映射（对齐 `Pass001_MergeMovedGroups.cs` 的 `Changes`）：

| 新 ID | 旧 ID | 类 |
|---|---|---|
| 238 | 194 | NavMesh → NavMeshData |
| 258 | 197 | LightProbes |
| 319 | 1011 | AvatarMask |
| 329 | 327 | VideoClip |

`p00b_merge_moved_groups` 把旧 ID 的早期版本条目并入新 ID（归并排序 + 同版本去重，
新 ID 接管），旧 ID 从 `blob.classes` 移除。NavMesh(194) 有特殊预处理：合并前先去掉
末尾的 `[5.3]null` + `[5.0]NavMeshObsolete`，补 `[5.0]null`（对齐 `Pass001.cs:25-30`）。

合并时还需**统一旧条目的类名为新 ID 规范名**（取新 ID 最新版本的类名），否则 p04
用最早非 null 条目取名会拿到旧名（如 "NavMesh" 而非 "NavMeshData"）。

### 12.8 string → PropertyName 特判（对应 AssetRipper Pass002 ChangeStringToUtf8String）

绝大多数 `string` 节点的唯一子节点是 `Array`（正常 string：size + bytes + align），
`NodeType::String` 正确处理。但 TPK 里有 5 个 `string` 节点的子节点**不是 Array**：

- 子节点是 `SInt32`/`int`（4 字节 CRC 哈希）→ 旧 PropertyName 布局（ExposedReferenceTable
  2017.1+、ExtensionPropertyValue 2017.2+）
- 子节点是 `string`（嵌套真 string）→ 新 PropertyName 布局（ExposedReferenceTable 2018.4+）

`rename_one` 对这类节点按 AssetRipper `ChangeStringToUtf8String` 逻辑重命名 type_name：
`SInt32` 子节点 → `PropertyName32`（subclass 读一个 `i32` + align），`string` 子节点 →
`PropertyNameNested`（subclass 读一个 aligned_string）。p02 把它们当独立 subclass 抽取，
p05 对 ExposedReferenceTable.m_References 生成 VersionedEnum（旧版/新版两个变体）。

若不处理，`NodeType::classify` 一律当 `String` → `read_aligned_string()`，读错字节。

### 12.9 纯字段重命名归一（对应 AssetRipper Pass002 DoSecondaryRenaming 字段规则）

`rename_one` 末尾补了 AssetRipper Pass002 的纯字段重命名规则（按父 type 分发 + 通用
前缀规则），如 `m_TextureFormat→m_Format`、`m_ObjectHideFlags→m_HideFlags`、
`m_PrefabParentObject→m_CorrespondingSourceObject`、`m_Dst→m_Destination` 等。这些虽
不影响字节布局，但让跨版本的同字段不同名（如旧版 `m_PrefabParentObject` / 新版
`m_CorrespondingSourceObject`）被 p05 按 `rust_name` 合并为单一字段，而非拆成两个
`Option<T>`。

**rename 必须遍历整个 node_pool**（而非只从 blob.classes 的 release/editor root 遍历）：
p02 扫整个 node_pool 抽 subclass（含孤儿子类，如被 MergeMovedGroups 移除的 class 的
release root 仍会被 p02 当 subclass 抽取），故 rename 也须覆盖每个节点，否则孤儿子类
的字段重命名会漏。

---

## 13. 路线图（未实现）

| 优先级 | 项 | 说明 |
|---|---|---|
| 低 | PPtr 改回 generic + forward decl | 为 missing target 生成占位 struct；代价是用户需处理 |
| 低 | 用 `quote!` 替换字符串拼接 | 提高可维护性、加语法校验，但增依赖 |
| 低 | VersionGate 支持 Disjoint | 处理「消失后重现」字段 |

---

## 14. 与其他文档的关系

| 文档 | 角色 |
|---|---|
| **本文档** | codegen **现状权威**（扁平 struct + parser，已落地） |
| [ADR-0001](adr/0001-codegen-from-typetree-dumps.md) | codegen 决策（含修订附录，记录与实现的偏差） |
| [`object-layer.md`](object-layer.md) | 生成产物的运行时 dispatch（typed → fallback） |

冲突时以本文档为准。

## 15. 参考

- AssetRipper TPK 仓库：`https://github.com/AssetRipper/Tpk`
- AssetRipper AssemblyDumper：`https://github.com/AssetRipper/AssetRipper/tree/master/Source/AssetRipper.AssemblyDumper`
- TPK 黄金测试数字（1395 版本 / 412 class / 首版本 3.4）见 ADR-0001 §9
