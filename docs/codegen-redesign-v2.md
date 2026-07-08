# Codegen Redesign v2：扁平单 struct + 版本门控

> 范围：`crates/codegen` 重新设计，明确约束下从 `type_tree.tpk` 生成 `crates/generated`。
> 状态：设计稿（对 `codegen-target-design.md` 的收紧与简化）
> 上游：`codegen-target-design.md`（目标设计）、`codegen-implementation.md`（当前实现）

---

## 1. 设计约束

本次重新设计在以下四条硬性约束下进行：

1. **只读 release root**。editor root 不参与生成。
2. **每个 class 只生成一个 struct**。跨版本差异用 `Option<T>` 字段表达；parse 函数里用 `ver` 控制是否读取；版本间完全相同的读取逻辑只写一遍。
3. **不处理 MonoBehaviour**。classID 114 不在生成范围内；运行时由主 crate 用 bundle 内嵌 TypeTree 解释器处理。
4. **不生成父类，只生成子类**。TPK 里抽象父类 `release_root` 为空，没有字段；所有字段已平铺到具体子类，codegen 不再生成父类 struct，也不再生成 `parent` 字段。

---

## 2. 端到端数据流

```text
type_tree.tpk
    │
    ▼
TpkFile::read_from_path  →  TpkTypeTree
    │
    ▼
run_pipeline (7 pass)
    │
    ▼
Pipeline { classes: BTreeMap<ClassKey, EmitClass>, ... }
    │
    ▼
emit::emit_all
    │
    ▼
crates/generated/src/
    ├── lib.rs
    ├── unity_object.rs      (enum UnityObject + parse dispatch)
    ├── class_id.rs            (enum ClassIDType)
    ├── classes/               (~350 个非抽象非 MonoBehaviour main class)
    └── subclasses/            (~900 个内嵌 struct)
```

---

## 3. Pipeline（8 个 pass）

| # | Pass | 职责 |
|---|---|---|
| 00 | `load_tpk` | 读 TPK 头、解压、反序列化 blob。产出 `TpkTypeTree`。 |
| 01 | `normalize_types` | 字符串池别名归一化：`int→SInt32`、`short→SInt16` 等。 |
| 02 | `extract_subclasses` | 扫描 `node_pool` 中所有 `NodeType::Type` 节点，注册为 subclass；排除 main class 名字。 |
| 03 | `collect_pptr_targets` | 收集所有 `PPtr<X>` 的 target 名（记录用；PPtr 仍非 generic）。 |
| 04 | `create_class_skeletons` | 为每个 main class 创建 `EmitClass`；**跳过抽象类**；**跳过 MonoBehaviour (classID=114)**；重名 class 加 `_ID` 后缀。 |
| 05 | `merge_fields` ★ | **核心**：字段并集 + 版本门控推断 + 字段顺序兼容性检测。 |
| 06 | `plan_alignment` | `meta_flag & (0x4000 \| 0x8000) != 0` → `align_after=true`。 |
| 07 | `emit` | `Vec<EmitClass>` → `.rs` 文件。 |

---

## 4. 中间表示（IR）

```rust
pub enum ClassKey {
    Main(i32),
    Subclass(String),
}

pub struct EmitClass {
    pub key: ClassKey,
    pub name: String,              // Rust 类型名
    pub unity_name: String,         // 原始 Unity 名
    pub own_fields: Vec<EmitField>, // 全部字段（已平铺，含继承链）
    pub is_abstract: bool,          // true → 不生成
    // 注意：不再保留 is_mono_behaviour，因为 MonoBehaviour 根本不会被加入 classes
}

pub struct EmitField {
    pub rust_name: String,
    pub unity_name: String,
    pub rust_type: RustType,
    pub version_gate: VersionGate,
    pub align_after: bool,
    pub unity_meta_flag: u32,
}

pub enum VersionGate {
    Always,                        // 所有版本都有
    Since(u16, u16, u16),          // ver >= (major, minor, build)
    Range { from, to },            // 保留扩展，首版不强制生成
}

pub enum RustType {
    Bool, I8, U8, I16, U16, I32, U32, I64, U64, F32, F64,
    String, Bytes,
    List(Box<RustType>),
    Pair(Box<RustType>, Box<RustType>),
    Pptr { target_class: String }, // 仍非 generic，但保留 target 信息
    Subclass(String),
    VersionedEnum {
        name: String,                         // 枚举类型名，全局唯一
        variants: Vec<EnumVariant>,           // 每个版本区间对应的变体
    },
    Option(Box<RustType>),
}

/// 版本枚举的一个变体：某版本区间对应的实际类型 + 对齐。
pub struct EnumVariant {
    pub gate: VersionGate,
    pub rust_type: RustType,
    pub align_after: bool,
}

pub struct Pipeline {
    pub classes: BTreeMap<ClassKey, EmitClass>,
    pub pptr_targets: BTreeSet<String>,
    pub all_versions: Vec<UnityVersion>,
}
```

---

## 5. Pass 05：字段并集 + 版本门控（核心）

### 5.1 输入输出

输入：一个 main class 的 `TpkClassInformation`，即按版本分段的 `(UnityVersion, Option<TpkUnityClass>)` 列表。

输出：
- `own_fields: Vec<EmitField>`：该 class 的 struct 字段并集。
- `version_layouts: Vec<VersionLayout>`：当字段顺序跨版本不兼容时，记录每个版本区间的真实字段顺序。

### 5.2 步骤

```text
1. 对每个版本变体：
   - 取 cls.release_root（editor root 不处理）
   - 递归展平 root 的直接子节点 → Vec<FlatField>
   - 字段已包含继承链（TPK 已平铺），无需扣祖先字段

2. 收集所有版本里出现过的字段名并集（按 rust_name 去重）

3. 对每个字段：
   - present_versions = 该字段出现的版本集合
   - 若在所有版本都出现 → VersionGate::Always，基础类型为 T
   - 否则 → VersionGate::Since(最早出现的版本)，基础类型为 T
   - **检测该字段在所有版本中的 `(RustType, align_after)` 是否一致**
     - 若一致 → `rust_type = T`
     - 若不一致 → 生成一个**版本枚举**（versioned enum），`rust_type = VersionedEnum(...)`，详见 5.4

4. 按「所有版本中的首次出现位置」排序，得到合并字段顺序 `merged_order`

5. 检测 merged_order 是否兼容每个版本的实际字段顺序：
   - 对每个版本实际顺序，取各字段在 merged_order 中的下标序列
   - 若下标序列严格递增 → 兼容
   - 否则 → 存在字段插入/重排，需要 version_layouts

6. 若兼容：version_layouts 为空，emit 用线性 parse + per-field Option 门控
   若不兼容：把各版本的真实字段顺序存入 version_layouts，emit 用分段锚点或全分支
```

### 5.3 为什么需要顺序兼容性检测

反例：某字段 `density` 在 v5.3 首次出现，并且被插入到旧字段 `material` 之前。

- 旧版本字段顺序：`[m_width, material]`
- 新版本字段顺序：`[m_width, density, material]`

若按 merged_order（按首次出现位置）读：`[m_width, material, density]`，则在 v5.3+ 会先把 `density` 的字节当成 `material` 读，字节错位。

因此必须检测顺序兼容性。不兼容时退化为 version_layouts 分支。

---

### 5.4 字段类型或对齐跨版本变化：生成版本枚举

当同一个字段在不同版本里出现不同的 `RustType` **或**不同的 `align_after` 时（如 `Mesh.m_shapes` 旧版为 `Vec<MeshBlendShape>`、新版为 `Vec<MeshBlendShapeVertex>`；又如 `SpriteRenderer.m_receive_shadows` 在 v5.4+ 类型为 `u8` 但 align_after 在 v2018.1 后消失），不再「取最新版本类型」，而是为该字段生成一个**版本枚举**。

> **为什么 align_after 变化也要 version-gate？** 同一类型在不同版本可能有不同的对齐需求。若统一取最新版本的 align，老版本会多读/少读对齐字节，导致后续字段错位。因此 `(类型, align_after)` 任一变化都拆成新变体。

#### 枚举定义

```rust
pub enum RustType {
    // ... 原语、容器 ...
    VersionedEnum {
        name: String,                         // 枚举类型名，全局唯一
        variants: Vec<EnumVariant>,           // 每个版本区间对应的变体
    },
    Option(Box<RustType>),
}

/// 版本枚举的一个变体：某版本区间对应的实际类型 + 对齐。
pub struct EnumVariant {
    pub gate: VersionGate,
    pub rust_type: RustType,
    /// 该变体读完后是否需要 align(4)。由该版本区间首次出现的 meta_flag 计算。
    pub align_after: bool,
}
```

- 枚举名：`{ClassName}{FieldNamePascal}Variant`（如 `MeshMShapesVariant`）。若名字过长或冲突，可哈希化。
- 枚举随宿主 class 文件一起生成，作为同文件的 `pub enum`。
- 变体命名：按版本号命名，如 `V5_6`、`V2017_4`；build 非 0 时附加 build 号（如 `V2017_4_39`），避免同 major.minor 不同 build 的变体重名。`Always` 变体命名为 `Default`。

#### 版本枚举的 VersionGate

版本枚举的 `variants` 按 `VersionGate` 区分。例如：

```rust
variants: [
    EnumVariant { gate: VersionGate::Always,      rust_type: List(Box::new(Subclass("MeshBlendShape".into()))),      align_after: true },
    EnumVariant { gate: VersionGate::Since(5,6,0), rust_type: List(Box::new(Subclass("MeshBlendShapeVertex".into()))), align_after: false },
]
```

> 注意：这里 `VersionGate::Always` 表示「在旧版本里用这个类型」；`Since(5,6,0)` 表示从该版本起类型变化。实际 emit 时按版本降序判断。

#### 与字段增减的交互

- 若字段在所有版本都有但类型变化 → struct 字段类型为 `VersionedEnum`（非 `Option`）。
- 若字段只在部分版本出现且类型变化 → struct 字段类型为 `Option<VersionedEnum>`，先判断字段是否存在，再判断该版本用哪个枚举变体。

#### 对 imports 的影响

`collect_imports` 必须扫描 `VersionedEnum.variants` 中每个变体的实际 `RustType`，否则类型变化涉及 subclass 时会漏 import。

---

## 6. Emit：单 struct + 版本控制 parse

### 6.1 简单情况（字段顺序兼容）

```rust
#[derive(Debug, Clone)]
pub struct Texture2D {
    pub m_name: String,
    pub m_width: i32,
    pub m_height: i32,
    pub m_forced_fallback_format: Option<i32>,
    // ...
}

impl crate::Parse for Texture2D {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = r.read_aligned_string()?;
        let m_width = r.read_i32()?;
        let m_height = r.read_i32()?;
        let m_forced_fallback_format = if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        // ...
        Ok(Texture2D { m_name, m_width, m_height, m_forced_fallback_format })
    }
}
```

**特征**：每个字段只读一次；公共字段没有任何版本分支；`Option` 字段只在自己的版本阈值处分支。

### 6.2 复杂情况（字段顺序不兼容）

当 version_layouts 非空时，使用**锚点分段**策略：

1. 求所有版本字段序列的**最长公共子序列（LCS）**作为锚点。
2. 锚点在所有版本里顺序一致 → 线性读一次。
3. 锚点之间的「段」才可能版本变化 → 只对段做 version-gate，且只在段变化的版本点分支。
4. 同一段在不同版本字段顺序相同 → 合并分支，不重复写。

```rust
// 伪代码：假设锚点是 a、c，段 0 在 V2 后插入 b，段 1 在 V3 后插入 d
let mut a = None;
let mut b = None;
let mut c = None;
let mut d = None;

// slot 0：前缀段
if *ver >= crate::UnityVersion::new(V2_MAJ, V2_MIN, V2_BUILD) {
    b = Some(r.read_i32()?);
}
a = Some(r.read_i32()?); // 锚点 a

// slot 1：中间段
if *ver >= crate::UnityVersion::new(V3_MAJ, V3_MIN, V3_BUILD) {
    d = Some(r.read_i32()?);
}
c = Some(r.read_i32()?); // 锚点 c

// ...

Ok(X { a: a.unwrap(), b, c: c.unwrap(), d })
```

**关键点**：锚点只读一次，变化段只在自己变化的版本处分支，公共路径绝不重复。

### 6.3 字段类型版本枚举

当某个字段的 `RustType` 为 `VersionedEnum` 时，宿主 class 文件里需要同时生成：

1. 枚举定义 `pub enum {EnumName} { Default(T1), V5_6(T2), ... }`。
2. 该枚举的 `impl crate::Parse`，按版本降序判断用哪个变体。

#### 生成示例

```rust
// Mesh.rs 内生成
pub enum MeshMShapesVariant {
    Default(Vec<MeshBlendShape>),
    V5_6(Vec<MeshBlendShapeVertex>),
}

impl crate::Parse for MeshMShapesVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        // 新版优先判断
        if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { MeshBlendShapeVertex::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::V5_6(v))
        } else {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { MeshBlendShape::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}
```

#### 在宿主 class 中使用

```rust
pub struct Mesh {
    pub m_shapes: MeshMShapesVariant,   // 全版本都有但类型变化
    pub m_skin: Option<MeshMSkinVariant>, // 部分版本才有且类型变化
    // ...
}

// parse 里无需再次判断版本，直接调用子类型 Parse
let m_shapes = MeshMShapesVariant::parse(r, ver)?;
let m_skin = if *ver >= crate::UnityVersion::new(...) {
    Some(MeshMSkinVariant::parse(r, ver)?)
} else {
    None
};
```

#### 对齐处理

版本枚举的每个变体可能有不同的 `align_after` 需求（例如旧类型读完需要对齐，新类型读完不需要）。因此：

- 在 `p05_merge_fields` 里，按 `(RustType, align_after)` 分组——类型不变但 `align_after` 变了也拆成新变体。
- 每个变体的 `align_after` 取该版本区间首次出现版本的 `meta_flag` 计算。
- 枚举的 `Parse` 实现里，根据实际选中的变体，在该变体读完后执行 `r.align(4)?`（若该变体需要）。
- **宿主 class 不对 `VersionedEnum` 字段额外对齐**——`emit_field_parse` 和 `set_field_none` 检测到 `VersionedEnum` 类型时跳过 align，完全交给枚举的 `Parse`。

### 6.4 构造 Self

简单情况直接用变量名构造。复杂情况所有字段先声明为 `let mut x = None;`，读完后再 `unwrap()` 那些 `Always` 字段、`Option` 保持原样。

```rust
Ok(X {
    always_field: always_field.unwrap(),
    gated_field,        // Option<T>
})
```

---

## 7. MonoBehaviour 不处理

### 7.1 p04 直接跳过

```rust
for (class_id, info) in &blob.classes {
    if class_id == 114 {
        continue; // MonoBehaviour 不生成
    }
    // ... 创建 EmitClass
}
```

### 7.2 对输出的影响

- `classes/mono_behaviour.rs` 不存在。
- `enum UnityObject` 没有 `MonoBehaviour` 变体。
- `enum ClassIDType` 可以保留 `MonoBehaviour = 114`（只是标识），但 `UnityObject::parse` 的 match 里没有对应 arm。
- 运行时主 crate 遇到 classID=114 直接走 bundle 内嵌 TypeTree 解释器，不调用 generated 的 parser。

### 7.3 为什么可以跳过

MonoBehaviour 的序列化字段分为两部分：
- 引擎字段（`m_GameObject`、`m_Enabled`、`m_Script`、`m_Name`）——TPK 里有，但意义不大；
- 用户脚本字段——完全不在 TPK 里，只能靠运行时 bundle 内嵌 TypeTree 解析。

既然用户脚本字段必须运行时解释，何必在 codegen 里生成一个半成品的 `MonoBehaviour` struct？直接跳过，让运行时统一走 TypeTree 解释器。

---

## 8. 不生成父类

### 8.1 抽象父类直接跳过

TPK 中 `is_abstract=true` 的 class（如 `Object`、`NamedObject`、`Texture`、`Behaviour`）`release_root` 为空，没有可读取字段。`p04` 标记 `is_abstract=true` 后直接跳过，不生成文件、不进入 `UnityObject` 分派。

### 8.2 不生成 parent 字段

每个具体 class 的 `release_root` 已经包含整条继承链的字段，因此 struct 上不需要 `parent` 字段。例如 `Texture2D` 直接持有 `m_name`（来自 `NamedObject`）、`m_Width` 等，而不是 `parent: Texture`。

### 8.3 父类变化的 class

如 `AudioClip` 在旧版本继承 `NamedObject`，新版本继承 `SampleClip`。TPK 的 `AudioClip.release_root` 在不同版本已经分别包含对应父类字段。`p05` 的字段并集会自动处理：旧版本有的字段新版本没有 → `Option<T>`；新版本有的字段旧版本没有 → `Option<T>`。无需生成父类 enum。

---

## 9. 输出目录布局

```text
crates/generated/src/
├── lib.rs                    # re-export Parse / EndianReader / UnityVersion / PPtr
├── unity_object.rs           # enum UnityObject + parse dispatch
│                             #   不含 MonoBehaviour
│                             #   不含抽象父类
├── class_id.rs               # enum ClassIDType
│                             #   可保留 MonoBehaviour=114 作为标识
├── classes/
│   ├── mod.rs
│   ├── game_object.rs        # struct GameObject { m_name, m_component, ... }
│   ├── transform.rs          # struct Transform { m_game_object, m_local_rotation, ... }
│   ├── texture_2_d.rs
│   ├── mesh.rs
│   └── ...                   # ~350 个文件
└── subclasses/
    ├── mod.rs
    ├── vector3f.rs
    ├── quaternionf.rs
    └── ...                   # ~900 个文件
```

---

## 10. 与当前实现的差异

| 维度 | 当前实现 | 新设计 |
|---|---|---|
| MonoBehaviour | 仍生成 `classes/mono_behaviour.rs`（只有引擎字段） | **完全不生成** |
| `EmitClass.is_mono_behaviour` | 存在 | **删除** |
| `UnityObject` 分派 | 含 `MonoBehaviour` 变体 | **不含** |
| 父类 | 仍有 `ParentSpec` 字段（虽然字段已平铺） | **彻底不生成 parent 字段** |
| 抽象类 | 跳过生成 | 跳过生成（不变） |
| 字段版本控制 | per-field `Option<T>` + 版本分支 | 保持一致；**新增版本枚举处理跨版本类型变化** |
| 公共路径去重 | LCS 锚点分段 | 保持一致 |
| 字段类型跨版本变化 | 取最新版本类型 | **生成 `VersionedEnum`，按版本选变体** |

---

## 11. 保留的已知限制

| 项 | 说明 |
|---|---|
| PPtr 非 generic | 所有 `PPtr<X>` 统一为 `PPtr`，避免 target class 缺失问题 |
| VersionGate 只有 Always/Since | 罕见「消失后重现」字段会落到 `Since`，老版本可能多读字节 |
| subclass 无版本并集 | 取第一次出现的子树定义；跨版本变化未处理 |
| 版本枚举名字冲突 | 合成枚举名 `{Class}{Field}Variant` 若与既有 class/subclass 重名需兜底（加 `_V` 后缀或哈希） |
| 版本枚举变体类型仍无并集 | 若同一版本段内该字段类型还有内部变化（如 subclass 字段变化），仍取该版本段首次出现的定义 |
| 同类型 align 变化已覆盖 | `(类型, align_after)` 任一变化即拆新变体；但同一变体内 align 若再变（极罕见）取首次出现的值 |
| 只读 release | editor build 的字段会丢失 |

## 12. 已踩到、但此前设计未显式覆盖的坑

本节记录当前实现/AGENTS 里已暴露、但本设计稿前几版没专门展开的问题。实施时应一并处理。

### 12.1 顺序重排分支里的 `align_after` 必须按版本取

version-gated parse 分支里读字段时：

- `rust_type` 取 merged `own_map`（与 struct 字段类型一致，避免编译错误）。
- **`align_after` 必须取该版本 layout 里的值**，不能取 merged 值。

反例：`CircleCollider2D.m_used_by_effector` 在 v5.6 无 `align_after`，在 v2023.1 有 `align_after`。若在 v5.6 分支用了 merged 的 `align_after=true`，会多读 4 字节，导致后续字段错位。

> 本设计 6.2 的 version-gated 分支必须遵守此规则。
>
> **版本枚举字段不受此规则影响**：`VersionedEnum` 字段的 `align_after` 完全由枚举的 `Parse` 实现处理（§6.3），宿主 class 的 `set_field_none` 检测到 `VersionedEnum` 类型时直接跳过 align。

### 12.2 `collect_imports` 必须扫版本枚举变体

`emit.rs` 生成 `use` 语句时：

- `VersionedEnum.variants` 中每个变体的实际 `RustType` 都必须扫描，否则类型变化涉及 subclass 时会漏 import。
- ~~必须扫 `version_layouts` 的字段类型~~：不再需要。`VersionedEnum` 已把所有变体类型包含在 `own_fields` 里，`collect_type_imports` 会递归扫描变体。`version_layouts` 中的 per-version 类型不被生成代码直接引用（version-gated 分支用 `own_map` 类型，即 `VersionedEnum`）。

### 12.3 `p02_extract_subclasses` 实际扫描 `node_pool` 而非 `release_root`

设计约束 1 说「只读 release root」，但 subclass 抽取无法只读 release root：

- 某些 subclass（如 `PackingParameters`）只出现在 editor 树的 `SpriteAtlasEditorData` 里，release root 里没有。
- 若只扫 release root，会漏掉这些 subclass，导致引用它们的 class 编译失败。
- 因此 `p02` 必须扫描整个 `node_pool`，但排除 main class 名字以避免同名文件重复。

> 这是「只读 release」约束的已知例外。文档应保留说明，不要改成只扫 release root。

### 12.4 锚点字段的对齐用 merged 值即可

锚点在所有版本里都有且相对顺序一致，其 `align_after` 通常也一致。因此锚点字段可以用 `own_map` 的 `align_after`，无需再查 `version_field_maps`。

### 12.5 版本枚举与字段重排的叠加

若一个类型变化的字段同时落在需要 version-gated 的段里（字段顺序也跨版本变化），应：

1. 按 `version_layouts` 确定该字段在何时读取（段分支内）。
2. 读取时调用 `VersionedEnum::parse(r, ver)?`，由枚举内部按版本选择变体。
3. `align_after` 完全交给枚举的 `Parse` 实现处理，宿主 class 不再额外对齐。

不要重复在宿主 `set_field_none` 里对齐一次、又在枚举 `Parse` 里对齐一次。

### 12.6 版本枚举变体命名的稳定性

同一个字段在不同 TPK 版本里新增/减少类型变化时，枚举变体命名不能依赖「首次/末次出现」这类不稳定概念。建议：

- 变体名按版本号命名（如 `V5_6`、`V2017_4`）。
- build 非 0 时附加 build 号（如 `V2017_4_39`），避免同 major.minor 不同 build 的变体重名。
- `Always` 变体命名为 `Default`。
- 避免用 `Legacy`/`Modern`，因为「现代」是相对概念，TPK 更新后含义会变。

### 12.7 抽象类与 classID 100000-100011 的过滤

TPK 里 classID 100000-100011（`int`/`bool`/`float`/`Vector3f` 等）注册为 class 但从不作为独立对象序列化，且 `release_root=None`。必须在 `p00` 过滤掉，否则：

- 会生成空 struct + 空 parser。
- 引用它们的类（如 `Transform` 引用 `Vector3f`）会读 0 字节，导致后续字段错位。

> 设计约束里只提了「抽象父类跳过」，未强调这 12 个引擎内部类型。实施时保留 `p00_filter_engine_types`。

### 12.8 TPK 版本去重：同 (major, minor, build) 不同 type 后缀

TPK 的 `TpkClassInformation.classes` 列表按 u64 原始值升序排列。Unity 版本的 u64 编码含 `version_type`（Alpha/Beta/Final/...）位，因此同 `(major, minor, build)` 但不同 type 后缀的版本（如 `2021.1.0b1` 和 `2021.1.0f1`）会作为**相邻但独立**的条目存在。

若不去重，`p05` 会把两个条目都纳入 `version_fields` / `version_layouts`，导致：

- `build_versioned_enum` 按 `(类型, align_after)` 分组时，两个同 `(major, minor, build)` 的条目可能分属不同组 → 生成同名变体（`V2021_1` 重复定义）。
- `version_layouts` 里出现两个 `since` 相同的 `VersionLayout` → emit 生成的 `if/else if` 分支条件重复，后者为死代码。

**修复**：`compute_main_self_fields` 和 `compute_main_fields` 在遍历 `info.classes` 构建版本列表时，若当前条目的 `(major, minor, build)` 与上一条相同，则替换（取后者，即更接近 final 的版本）。

---

## 13. 实施步骤（更新）

1. **修改 `p00`**：保留 classID 100000-100011 过滤。
2. **修改 `p04`**：遇到 `class_id == 114` 直接 `continue`；删除 `is_mono_behaviour` 相关代码。
3. **修改 `ir.rs`**：从 `EmitClass` 删除 `is_mono_behaviour` 字段；在 `RustType` 添加 `VersionedEnum` 变体 + `EnumVariant` 结构体。
4. **修改 `p05`**：
   - 遍历 `info.classes` 时去重同 `(major, minor, build)` 的版本条目（§12.8）。
   - 合并字段时检测 `(RustType, align_after)` 跨版本变化——任一变化即生成 `VersionedEnum`。
   - 按 `(类型, align_after)` 分组构建变体；每变体 `align_after` 取该组首次出现版本的 `meta_flag`。
5. **修改 `emit.rs`**：
   - 生成版本枚举定义及其 `impl Parse`（含 per-variant `align_after`）。
   - 生成宿主 class 中对版本枚举字段的读取代码。
   - `emit_field_parse` / `set_field_none` 检测到 `VersionedEnum` 类型时跳过 align。
   - 删除 `emit_impl_parse` 中的 MonoBehaviour 特殊逻辑；`emit_unity_object` 不再为 MonoBehaviour 生成变体。
   - `collect_imports` 扫 `own_fields` + `VersionedEnum.variants`（不再需要扫 `version_layouts`）。
6. **重生成 `crates/generated`**：删除旧产物，重新跑 codegen。
7. **编译验证**：`cargo build --workspace`、`cargo test --workspace`。
8. **文档更新**：本设计稿定稿后，同步更新 `codegen-target-design.md` 与 `codegen-implementation.md` 的相关章节。
