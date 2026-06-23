# Codegen 当前实现状态

> 范围：记录 `crates/codegen` 当前**实际实现**了什么（D1-D5 已完成），与 [`codegen-design.md`](codegen-design.md) 描述的设计目标的偏差，以及已知限制。
> 状态：实施记录（描述当前代码，非规划稿）
> 上游：[ADR-0001](adr/0001-codegen-from-typetree-dumps.md)、[`codegen-design.md`](codegen-design.md)

---

## 1. 与设计文档的关系

| 文档 | 角色 |
|---|---|
| [`codegen-design.md`](codegen-design.md) | 设计目标（"应该长成什么样"）|
| **本文档** | 实施记录（"当前代码实际长成什么样"）|

凡本文档与 `codegen-design.md` 冲突，以**本文档为准**（描述代码现状）。差异点详见 §7。

---

## 2. 当前实现规模

| 项 | 数值 |
|---|---|
| codegen 手写源码 | 2,433 行（10 个 .rs 文件） |
| 生成代码（`crates/generated/`）| **79,706 行** / **1,320 个 .rs 文件** |
| EmitClass 总数 | 1,315（412 main + 903 subclass）|
| PPtr 目标类型 | 97 |
| Unity 版本覆盖 | 1,395 个（5.x → 6000.x，含 3.4）|
| Class 覆盖 | 412 个 classID |
| 工作区测试 | 19 个全通过 |
| Clippy | codegen 干净；generated crate 588 个无害 warning |

`crates/generated/` 是 codegen 跑一次后提交进仓库的产物，标 `linguist-generated=true`（`.gitattributes`），GitHub diff 默认折叠。

---

## 3. Workspace 拓扑

```
unity-asset-rs/
├── Cargo.toml                  # workspace 根（含主 crate unity-asset-rs）
├── crates/
│   ├── codegen/                # 离线 codegen 工具（bin + lib），2433 行手写
│   │   ├── Cargo.toml
│   │   ├── tests/fixtures/type_tree.tpk   # 测试用 TPK（brotli 186KB）
│   │   └── src/
│   │       ├── lib.rs          # 模块导出
│   │       ├── main.rs         # CLI
│   │       ├── tpk/            # TPK 二进制读取（小端）
│   │       ├── ir.rs           # IR 数据结构
│   │       ├── names.rs        # 命名转换
│   │       ├── pipeline.rs     # pass 系统（p01-p06）
│   │       └── emit.rs         # 代码生成（p07）
│   └── generated/              # ★ 生成产物，独立 crate，79706 行
│       ├── Cargo.toml
│       └── src/                # codegen 跑一次后提交进仓库
│           ├── lib.rs          # Parse trait + EndianReader trait + PPtr + UnityVersion
│           ├── unity_object.rs # enum UnityObject 分派
│           ├── class_id.rs     # enum ClassIDType
│           ├── classes/        # 412 个主 class 文件
│           └── subclasses/     # 903 个 subclass 文件
```

**主 crate（`unity-asset-rs`）当前**尚未接入 generated crate（待后续 D6）。

---

## 4. 数据流

```
type_tree.tpk
     │
     │  TpkFile::read_from_path
     ▼
TpkTypeTree                          (ir 的源头：节点池 + 字符串池 + class 列表)
     │
     │  pipeline::run_pipeline
     ▼
Pipeline                             (ir 的产物：BTreeMap<ClassKey, EmitClass>)
     │
     │  emit::emit_all
     ▼
crates/generated/src/*.rs            (最终 Rust 源码，提交进仓库)
```

跑一次的命令：

```bash
cargo run -p unity-asset-codegen -- \
    crates/codegen/tests/fixtures/type_tree.tpk \
    crates/generated/src
```

---

## 5. 命名规则（实际实现）

实现位置：`crates/codegen/src/names.rs`。

### 5.1 字段名（`to_rust_field_name`）

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
1. `heck::ToSnakeCase` 转 snake_case（保留 `m_`/`k_` 前缀，处理空格/点等非标识符字符）
2. 首字符是数字 → 加 `f_` 前缀（Rust 标识符不能以数字开头）
3. 命中 Rust 关键字 → 加 `r#` 转义

### 5.2 类型名（`to_rust_type_name`）

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
1. `PPtr<X>` 原样返回（PPtr 是 generic）
2. `collapse_acronyms` 处理连续大写字母（AABB→Aabb）
3. 命中 Rust 原始类型/关键字 → 加 `_` 后缀（struct 名不能用 `r#`）

---

## 6. Pass 系统（实际实现）

### 6.1 总览

实现位置：`crates/codegen/src/pipeline.rs`。**纯顺序管线，无依赖图**（与 AssetRipper 一致）。

| # | Pass | 职责 |
|---|---|---|
| 00 | `TpkFile::read_from_path` | 读 TPK + 解压 + 二进制反序列化 |
| 01 | `p01_normalize_types` | 类型名归一化（`int → SInt32` 等）|
| 02 | `p02_extract_subclasses` | 抽取所有 `NodeType::Type` 节点为 subclass |
| 03 | `p03_collect_pptr_targets` | 收集所有 `PPtr<X>` 的目标 class |
| 04 | `p04_resolve_inheritance` | 填 `EmitClass.parent`；重名 class 加 `_ID` 后缀 |
| 05 | `p05_merge_fields` ★ | 字段并集 + 版本门控推断（核心）|
| 06 | `p06_plan_alignment` | 计算 `align_after`（meta_flag & 0x4000/0x8000）|
| 07 | `emit::emit_all` | 生成 Rust 源码到目录 |

### 6.2 各 pass 细节

#### p01 normalize_types
对 `string_pool` 里每个字符串应用别名表（`SInt32 → SInt32`、`short → SInt16` 等 ~10 对），照搬 AssetRipper `UniversalNode.cs:131-143`。

#### p02 extract_subclasses
遍历所有节点，凡 `NodeType::Type`（非 primitive / 非 PPtr / 非容器的有子节点 struct）的 `type_name` 都注册为 subclass。subclass 在 `crates/generated/src/subclasses/<name>.rs` 里生成独立 struct。

#### p03 collect_pptr_targets
扫所有节点，收集 `PPtr<X>` 的 target（去重）。MVP 阶段 PPtr 是**非 generic**，所以这些信息未实际用于代码生成（仅记录）。

#### p04 resolve_inheritance ★
对每个 main class：
1. 收集所有版本变体的 `base_name`（过滤空串）→ 决定 `ParentSpec`：
   - 0 个 → `Root`
   - 1 个 → `Stable(name)`
   - ≥2 个 → `Variable { enum_name, branches }`
2. **重名处理**：扫描所有 main class，对重名（不同 classID 同名，Unity 历史 ID 复用，如 `VideoClip=327` 和 `VideoClip=329`）的从第二个开始加 `_<classID>` 后缀

#### p05 merge_fields ★★（核心算法）
对每个 main class：

1. `self_fields = merge_fields_across_versions(所有版本变体的 release_root)`
2. **不扣祖先字段**（详见 §7.1），直接 `own_fields = self_fields`

对每个 subclass（无继承）：取第一次出现的节点子树作为定义。

`merge_fields_across_versions` 算法：
1. 按 `rust_name` 去重收集所有字段（不用 unity_name 去重，因为不同 unity_name 可能映射到同一 rust_name）
2. 对每个字段计算 `VersionGate`：
   - 在所有版本变体里都出现 → `Always`
   - 否则取最早出现版本 → `Since(major, minor, build)`
   - **MVP 不支持 Range / Disjoint**
3. gated 字段的 RustType 包 `Option<T>`
4. 按首次出现位置排序（保持字节流顺序）

#### p06 plan_alignment
对每个字段：`meta_flag & (0x4000 | 0x8000) != 0` → `align_after = true`。
- `0x4000` = `AlignBytes`
- `0x8000` = `AnyChildUsesAlignBytes`

照搬 AssetRipper `SerializablePrimitive.cs:66`。

### 6.3 emit（p07）

实现位置：`crates/codegen/src/emit.rs`。**字符串拼接**（不用 `quote!` / `syn` / `prettyplease`，避免依赖）。

每个 `EmitClass` 输出到 `classes/<snake>.rs` 或 `subclasses/<snake>.rs`，结构：
1. 文件头注释 + imports
2. `enum XxxParent`（仅 Variable parent）
3. `pub struct Xxx { pub parent: ..., pub <fields>... }`
4. `impl crate::Parse for Xxx { fn parse... }`

外加 4 个特殊文件：
- `lib.rs`：定义 `Parse` trait / `EndianReader` trait / `PPtr` / `UnityVersion`
- `unity_object.rs`：`enum UnityObject` + 按 `ClassIDType` 分派
- `class_id.rs`：`enum ClassIDType`
- `classes/mod.rs` / `subclasses/mod.rs`：模块声明

---

## 7. 与设计文档的关键偏差

### 7.1 嵌套 parent 实际未生效（最重大偏差）

**设计**（`codegen-design.md` §4.1）：
```rust
pub struct Texture2D {
    pub parent: Texture,           // 装祖先字段
    pub m_width: i32,              // 只装本类字段
}
// 用户访问祖父字段：tex2d.parent.parent.m_name
```

**实际 TPK 数据**：
- abstract class（Object / NamedObject / Texture）的 `release_root` **完全是空的**
- 所有字段都被推到具体子类（如 Texture2D）的 release_root 里平铺

**当前实现**：
```rust
pub struct Texture {
    pub parent: NamedObject,       // Texture 自己 own_fields = 空
}

pub struct Texture2D {
    pub parent: Texture,           // 仅作类型标记，Texture 自身是空壳
    pub m_name: String,            // 本该属于 NamedObject，但留在 Texture2D 里
    pub m_width: i32,
    // ... 所有字段都直接在 Texture2D 里
}
// 用户访问字段：tex2d.m_name（直接拿，不用 chain）
```

字节流解析**是正确的**（Unity 字节流确实是平铺的），但 `parent` 字段成了纯类型标记。

**为什么没做严格嵌套**：需要实现 AssetRipper `Pass005_SplitAbstractClasses` 的"派生类共有字段提升"算法（找 abstract class 的所有派生类，把 ≥70% 派生类共有的字段反向推回到 abstract class）。工程量大，MVP 跳过。

### 7.2 PPtr 改成非 generic

**设计**：`PPtr<T>` generic + PhantomData。

**实际**：`PPtr` 非 generic。
```rust
pub struct PPtr {
    pub m_file_id: i32,
    pub m_path_id: i64,
}
```

**原因**：TPK 里有些 PPtr target（如 `PrefabInstance`）在所有版本里都不存在（被改名/移除），如果 PPtr generic 就需要为这些类型生成占位 struct。非 generic 简化掉。

**代价**：用户拿不到类型区分（`PPtr<GameObject>` 和 `PPtr<Texture2D>` 都是 `PPtr`）。未来要做可改回 `PPtr<T>` + `PhantomData<fn() -> T>` + 为 missing target 生成 forward declaration。

### 7.3 enum 全 i32（已是设计，无偏差）

整数字段（如 `m_texture_format`）一律生成 `i32`，**不生成 enum**。codegen 输入只有 TPK，无 consolidated.json。

### 7.4 MonoBehaviour 未特殊处理

**设计**：MonoBehaviour 最小骨架 + `token: Vec<u8>` 兜底（供运行时 TypeTree 解释器）。

**实际**：codegen 仅标记 `is_mono_behaviour = true`，但 emit 时**没有特殊分支**——照普通 class emit（含继承 + 所有字段）。`token` 兜底字段未生成。

**原因**：MonoBehaviour 的用户脚本字段需要运行时 TypeTree 解释器（不在 codegen 范围）。MVP 阶段只 emit 引擎字段部分。

---

## 8. 已知限制

| 项 | 现状 | 影响 |
|---|---|---|
| 嵌套 parent 未生效 | 所有字段含祖先，平铺在叶子类里 | 见 §7.1 |
| PPtr 无 generic | 所有 PPtr 都同类型 | 见 §7.2 |
| VersionGate 简化 | 只支持 `Always` / `Since`；不支持 `Range` / `Disjoint` | 罕见字段（消失后又重新出现）会落到 `Since`，导致字段在消失版本里仍尝试 parse（运行时可能错误）|
| 子类未做版本并集 | subclass 取第一次出现的子树 | 同 subclass 跨版本字段变化时会 emit 错（罕见）|
| MonoBehaviour 无 token 兜底 | 用户脚本字段无法解析 | 见 §7.4 |
| emit 字符串拼接 | 无语法校验 | 边界 bug 容易引入（但当前 fixture 测试覆盖了主要形态）|
| generated crate 588 个 warning | 大多是 unused `_ver` 参数 | 无害，未来 EndianReader 信号完善后可消 |
| `Range` / `Disjoint` 未实现 | 见上 | 待升级 |

---

## 9. 已跑通的边界 case

实现过程中遇到并修复的 12 个 corner case（顺序即发现顺序）：

1. **字段名带空格**（`image data`）→ heck snake_case 处理
2. **`Option<Subclass>` 没 import** → collect_type_imports 递归
3. **align 时机错**（gated 字段在 if 外 align）→ 重构 emit_field_parse 让 align 在 if 内
4. **closure 内 `?` 错** → field_read_expr 改为返回 `Result<T, E>` 表达式，调用方加 `?`
5. **`PPtr<PrefabInstance>` 引用不存在 class** → PPtr 改非 generic
6. **VideoClip class ID 复用同名**（`VideoClip=327` 和 `329`）→ 重名加 `_<classID>` 后缀
7. **字段名 `4_3` 以数字开头** → 加 `f_` 前缀
8. **scrollposition_x 重复**（unity_name 不同但 rust_name 相同）→ 按 rust_name 去重
9. **class 名 `float` / `void`**（Rust 关键字）→ 加 `_` 后缀（`float_` / `void_`）
10. **Variable parent enum 重复 variant**（同父类跨非相邻版本段）→ 去重
11. **unity_object.rs 没 import Parse trait** → trait method 调用失败 → 加 `use crate::Parse;`
12. **Object class 的 base 是空串 `""`**（不是 None）→ 过滤空串

---

## 10. 关键算法备忘

### 10.1 字段并集 + 版本门控（p05）

```
input:  Vec<(UnityVersion, Vec<FlatField>)>   // 一个 class 的所有版本变体

1. 收集所有 rust_name 去重 → all_names_ordered
2. 对每个 rust_name:
   - 计算 present_versions（该字段在哪些版本变体里出现）
   - gate = if 全版本都有 { Always } else { Since(earliest) }
   - rust_type = if Always { T } else { Option<T> }
3. 按首次出现位置排序（保持字节流顺序）
```

### 10.2 Variable parent 的版本分支

```
input:  info.classes (Vec<(UnityVersion, Option<TpkUnityClass>)>)

1. 抽出 (version, base_name) pairs（base 为 None 跳过）
2. 按版本升序
3. 合并相邻同 base_name
4. emit 为 if/else chain：
   let parent = if ver >= V1 { Enum::Base1(Base1::parse(..)?) }
                else if ver >= V2 { Enum::Base2(Base2::parse(..)?) }
                else { panic!("无匹配分支") };
5. enum 定义去重（同 base 跨非相邻段时 variant 不重复）
```

### 10.3 emit 单个字段的 read 表达式

`field_read_expr` 返回**不含外层 `?`** 的 `Result<T, R::Error>` 表达式，调用方按需加 `?`：

```rust
// 简单类型：直接返回 read 的 Result
r.read_i32()

// Bytes (TypelessData)：
( { let n = r.read_i32()? as usize; r.read_bytes(n) } )

// List<T>：
( { let n = r.read_i32()?; (0..n).map(|_| { <elem_expr> }).collect::<Result<Vec<_>, _>>() } )

// Subclass：
SubclassName::parse(r, ver)
```

调用方 emit：

```rust
// 非 gated：
let m_width = r.read_i32()?;
r.align(4)?;   // 如果 align_after

// gated（Option）：
let m_x = if *ver >= V {
    let v = r.read_i32()?;
    r.align(4)?;
    Some(v)
} else {
    None
};
```

---

## 11. 如何运行

### 11.1 重新生成 `crates/generated/`

TPK 升级或 codegen 逻辑改动后：

```bash
# 1. 拿 TPK（已有 fixture 在 crates/codegen/tests/fixtures/type_tree.tpk）
# 2. 重生成
rm -rf crates/generated/src
mkdir -p crates/generated/src
cargo run -p unity-asset-codegen -- \
    crates/codegen/tests/fixtures/type_tree.tpk \
    crates/generated/src

# 3. 编译验证
cargo build --workspace
```

### 11.2 跑测试

```bash
cargo test --workspace         # 19 个测试全过
cargo clippy -p unity-asset-codegen --all-targets -- -D warnings
```

### 11.3 用不同压缩格式的 TPK

AssetRipper 提供 4 种压缩（None / LZ4 / LZMA / Brotli），都支持。下载：

```
https://nightly.link/AssetRipper/Tpk/workflows/type_tree_tpk/master/<compression>_file.zip
```

其中 `<compression>` ∈ {`uncompressed`, `lz4`, `lzma`, `brotli`}。MVP 测试用 brotli（最小 186KB）。

---

## 12. 后续工作

按优先级排序：

| 优先级 | 项 | 工作量 | 说明 |
|---|---|---|---|
| 高 | 接入 `crates/unity-asset` | 中 | 为现有 bundle Cursor impl `EndianReader`，从 SerializedFile 调 `UnityObject::parse`；用真实 .bundle 字节流验证 |
| 高 | 实现严格嵌套 parent（Pass005 字段提升） | 大 | 见 §7.1，约 200-300 行 |
| 中 | VersionGate 支持 Range | 小 | 处理"V1..V2" 范围门控 |
| 中 | subclass 跨版本并集 | 中 | 当前只取第一次出现 |
| 中 | MonoBehaviour token 兜底 | 小 | 见 §7.4 |
| 低 | PPtr 改回 generic + forward decl | 小 | 见 §7.2 |
| 低 | 用 `quote!` 替换字符串拼接 | 中 | 提高可维护性 |
| 低 | 消 generated crate 的 warning | 小 | 让 EndianReader 接受未用参数的 trait method |

---

## 13. 参考

- 设计稿：[`codegen-design.md`](codegen-design.md)
- ADR：[ADR-0001](adr/0001-codegen-from-typetree-dumps.md)
- AssetRipper TPK 仓库：`https://github.com/AssetRipper/Tpk`
- AssetRipper AssemblyDumper：`https://github.com/AssetRipper/AssetRipper/tree/master/Source/AssetRipper.AssemblyDumper`
