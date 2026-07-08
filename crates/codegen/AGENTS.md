# AGENTS.md

Compact guidance for OpenCode sessions working on `crates/codegen`.

## What this crate is

Offline code generator: `type_tree.tpk` (AssetRipper TypeTree dump) → typed Rust
structs + parsers in `crates/generated/src/`. It is a **build-time tool** — it does
not link `unity-core` or `generated`; it only emits source strings that reference
them.

```
type_tree.tpk ──→ run_pipeline (7 pass) ──→ emit_all ──→ crates/generated/src/*.rs
```

## Commands

```bash
# 重新生成产物（改完 pipeline/emit 后必跑）
rm -rf crates/generated/src && mkdir -p crates/generated/src
cargo run -p unity-asset-codegen -- \
    crates/codegen/tests/fixtures/type_tree.tpk crates/generated/src

cargo build -p unity-asset-codegen          # 只编译 codegen
cargo test  -p unity-asset-codegen           # TPK reader 黄金测试
cargo clippy -p unity-asset-codegen          # 应零警告
cargo run    -p unity-asset-codegen --example explain_inheritance  # 查看 TPK 继承链
```

生成后务必 `cargo build --workspace` 确认产物可编译。Edition `2024` → Rust ≥ 1.85。

## Tests & fixtures

- `tests/tpk_read.rs` 是 TPK reader 的黄金测试，数字（1395 版本 / 412 class /
  首版本 3.4）来自 ADR-0001 §9，与真实 AssetRipper brotli TPK 对齐。
- fixture `tests/fixtures/type_tree.tpk`（brotli 压缩 ~186KB）已提交，勿删改。
- 没有 emit 阶段的单元测试；emit 正确性靠 `crates/generated` 能否编译 + 主 crate
  `src/bin/verify_codegen.rs` 实跑 `example/char_002_amiya.ab` 验证。
- `pipeline.rs` 的 `merged_order_compatible` 有单元测试（覆盖「新字段插在现有字段
  之间」的核心场景）。

## Architecture: pass 管线（`pipeline.rs`）

`run_pipeline` 跑 7 个 pass，产物是 `Pipeline { classes, pptr_targets, all_versions }`：

| pass | 函数 | 职责 |
|---|---|---|
| p00 | `p00_filter_engine_types` | 过滤 classID 100000-100011（引擎内部类型，对齐 AssetRipper `Pass000`） |
| p01 | `p01_normalize_types` | 类型名归一化（`int→SInt32` 等别名） |
| p02 | `p02_extract_subclasses` | 扫 `node_pool` 收集 `NodeType::Type` → subclass（排除非抽象 main class 名字，见下） |
| p03 | `p03_collect_pptr_targets` | 收集 `PPtr<X>` 目标（记录用；PPtr 非 generic） |
| p04 | `p04_resolve_inheritance` | 为每个 main class 建骨架：名字 + `is_abstract`/`is_mono_behaviour` + 重名加 `_ID` |
| p05 | `p05_merge_fields` ★ | **核心**：字段并集 + 版本门控（见下） |
| p06 | `p06_plan_alignment` | `meta_flag & (0x4000\|0x8000)` → `align_after` |

p05 后 `emit.rs` 的 `emit_all` 把每个 `EmitClass` 写成一个 `.rs` 文件。

## 关键设计决策（不要无意中改掉）

### 扁平 struct，无父类

- **没有 `ParentSpec`、没有 `parent` 字段、不生成抽象父类**。TPK 的 `release_root`
  已含整条继承链字段（扁平），每个具体类直接持有全部字段。
- p05 **不扣除祖先字段**——直接取 `release_root` 子节点作为字段列表。
- `is_abstract == true` 的类在 `emit_all` 里被跳过（不生成文件、不进 mod.rs、
  不进 `UnityObject`/`ClassIDType` 分派）。它们在 TPK 里 `release_root` 为空。
- 想加回 parent 链前，先读 `docs/codegen-target-design.md` §2/§6.3——那是已废弃
  的旧方案。

### p00 过滤引擎内部类型 + p02 排除 main class 名字

**p00** 硬编码过滤 classID **100000-100011**（`int`/`bool`/`float`/`MonoObject`/
`Collision`/`Vector3f`/`RootMotionData`/`Collision2D`/`AudioMixerLiveUpdate*`/
`Polygon2D`/`void`）。对齐 AssetRipper `Pass000_ProcessTpk.cs:85` 的
`IsUnacceptable(int typeId) => typeId is >= 100000 and <= 100011`。

这些在 Unity 类型系统里注册了 classID 但**从不作为独立对象序列化**，TPK 里
`abstract=false` 但 `release_root=None` —— 字段定义只作为内嵌 Type 节点出现在其他
类的字段里。若不过滤会生成空 struct + 空 parse，引用它们的类（如 `Transform` 用
`Vector3f`）会读 0 字节导致后续字段错位 → `data truncated`。

**p02** 扫 `node_pool` 收集 `NodeType::Type` 节点为 subclass，但排除非抽象 main class
名字（它们的 `release_root` 节点本身也是 `NodeType::Type`，不排除会导致同名文件在
`classes/` 和 `subclasses/` 各一份）。p00 已移除 100000-100011，所以排除逻辑只需
`!is_abstract()`，不再需要 `release_root.is_some()` 检查。

> **p02 仍扫 `node_pool` 发现 subclass 名字**（创建 EmitClass 外壳），但 **p05 的字段
> 提取改为递归走 release_root**（`collect_subclass_from_node`）：遍历所有 main class
> 的所有版本变体的 `release_root` 子树，收集每个 subclass 的 `(version, fields)` 出现，
> 再用 `merge_and_layout`（与 main class 共用）做跨版本合并 + version-gated 判定。
> 仅在无 release_root 出现时（editor-only subclass，如 `PackingParameters`）回退到
> `node_pool` 首次出现。VersionedEnum 变体里的 Subclass 自动包 `Box` 打破递归
> （如 `Clip` → `OffsetPtr` → `OffsetPtrDataVariant` → `Clip`）。

emit 侧 `collect_type_imports` 对 `RustType::Subclass(name)` 按同样规则选路径：
是 main class → `crate::classes::X::X`，否则 → `crate::subclasses::X::X`。

### version-gated parse：线性 vs 重排的判定（`dedup_layouts`）

跨版本字段顺序处理分两档，由 `dedup_layouts`（`pipeline.rs`）决定：

1. **线性 parse + per-field `Option<T>` 门控**（默认）：所有版本字段可以用单一线性
   顺序读取，差异字段用 `Option` 门控。
2. **version-gated 块**（~150 个类，含 main class + subclass）：字段顺序跨版本真变了，必须按版本分支读。

判定线性 parse 安全需**三道检查都通过**：

- **`common_order_consistent`**：两个版本的公共字段相对顺序一致（快检）。
- **`merged_order_compatible`**：`merge_fields_across_versions` 按首次出现位置排序
  得到的合并顺序，对每个版本的字段验证位置序列严格递增。
- **`presence_is_monotonic`**：每个字段的 presence 单调——一旦出现就不再消失。

> **为什么需要第二道检查？** 新字段可能插在现有字段之间（如 v5.3 在 `material` 前
> 插入 `density`）。此时公共字段顺序不变（第一道过），但合并顺序把 `material` 排在
> `density` 前面（因 `material` 首次出现在更老版本），与 v5.3+ 实际字节顺序矛盾 →
> 线性 parse 读错字节。`merged_order_compatible` 检测此矛盾，不兼容时升级为
> version-gated。

> **为什么需要第三道检查？** `merge_fields_across_versions` 对非全版本字段只产出
> `Since(earliest)`，不产出 `Range`。若字段在早期版本存在、后期被移除（非单调），
> `Since(earliest)` 会让线性 parse 在字段已不存在的版本里仍然尝试读取，导致后续
> 字节全部错位。`presence_is_monotonic` 检测此情况，非单调时升级为 version-gated。
> version-gated 分支只读自己 layout 里的字段，字段消失的版本不会多读。

> **不要去掉任一检查**：去掉 `common_order_consistent` 会让 ~220 个类触发
> version-gated，生成代码从 ~80K 行暴涨到 ~160K 行。去掉 `merged_order_compatible`
> 会让新字段插入型重排漏检，typed parse 读错字节。去掉 `presence_is_monotonic`
> 会让字段移除型变化漏检（~247 个字段在 linear-parse 类里会读错字节）。

### version-gated 的锚点分段优化（`emit.rs`）

`emit_versioned_field_reads` 不是「每版本分支读全部字段」，而是：

1. 求所有版本字段序列的 **LCS 公共子序列**作为「锚点」（`compute_anchors`）。
2. 锚点线性读一次（所有版本都有、相对顺序一致）。
3. 锚点之间的**段**才做 version-gate，且**每段只在自己的变化版本点分支**
   （`emit_slot_segment`），而非把全部版本分支套到所有字段上。

这把 `player_settings.rs` 从 32K 行压到 13K 行。无公共锚点时退化为全量
version-gated（`emit_full_versioned_branches`）。

### version-gated emit 的 per-version align_after（`emit.rs`）

`set_field_none` 在 version-gated 分支里读字段时：

- **rust_type 取 `own_map`**（merged，与 struct 字段类型一致，避免版本类型不匹配
  导致编译错误）。
- **align_after 优先取 per-version `version_field_maps`**：同一个字段在不同版本的
  对齐可能不同（如 `CircleCollider2D.m_used_by_effector` 在 v5.6 无 align、v2023.1
  有 align）。用 merged align 会在老版本分支多读 align 字节 → 后续字段错位。

锚点字段用 `own_map` 的 align（锚点在所有版本都有且顺序一致，align 也应一致）。

### collect_imports 必须扫 version_layouts

`collect_imports`（`emit.rs`）除了扫 `own_fields`，还要扫 `version_layouts` 的字段
类型。version-gated 块可能引用老版本的字段类型（如 `Mesh.m_shapes` 老版本是
`Vec<MeshBlendShape>`，新版本是 `Vec<MeshBlendShapeVertex>`），`own_fields` 只取最新
版本类型，不扫 `version_layouts` 会漏 import → 编译错误。

### 字段类型取最新版本

`merge_fields_across_versions`（`pipeline.rs`）对跨版本改类型的字段取**最新版本**
的类型定义（不是最老的）。因为本库主要面向现代 Unity 版本。

## emit 产物布局

```
crates/generated/src/
├── lib.rs              # re-export unity_core 的 Parse/EndianReader/UnityVersion/PPtr
├── unity_object.rs     # enum UnityObject + parse 分派（不含抽象类、不含 MonoBehaviour）
├── class_id.rs         # enum ClassIDType（含全部 classID，dispatch 只列非抽象）
├── classes/            # ~400 个 main class（不含抽象类、不含 100000-100011）
└── subclasses/         # ~535 个内嵌 struct
```

每个 class 文件 = `pub struct`（扁平字段，version-gated 字段为 `Option<T>`）+
`impl crate::Parse`。emit 用字符串拼接（无 `quote!`），靠 `crates/generated` 能否
编译来间接验证正确性。

## 已知限制

- **TPK 与具体 bundle 仍可能不匹配**：typed parse 失败时，主 crate 的
  `object_parser.rs` 会自动回退到 bundle 内嵌 TypeTree 解释器（设计如此）。
  曾确认 `CircleCollider2D`/`Rigidbody2D`/`SpriteRenderer`/`AssetBundle` 在
  `example/char_002_amiya.ab`（Unity 2017.4）上 typed parse 成功（修复了
  `dedup_layouts` 漏检 + version-gated `align_after` 两个 bug 后）。其他 bundle
  如仍回退到 dynamic，说明有未覆盖的版本差异。
- PPtr 非 generic：所有 `PPtr<X>` 生成同一 `PPtr` 类型，丢失目标类型信息。
- subclass 跨版本合并走 release_root 递归收集（`collect_subclass_from_node`），
  与 main class 共用 `merge_and_layout`。仅在无 release_root 出现时（editor-only
  subclass）回退到 node_pool 首次出现。VersionedEnum 变体里的 Subclass 自动包
  `Box` 以打破递归类型循环（如 Clip ↔ OffsetPtr）。
- `VersionGate::Range` 变体存在且 emit 能处理，但 `merge_fields_across_versions`
  只产出 `Always`/`Since`，不产出 `Range`。非单调 presence（字段消失后重现，或
  字段被移除不再回来）由 `dedup_layouts` 的 `presence_is_monotonic` 检查兜底：
  非单调时强制 version-gated 块，每版本分支只读自己 layout 里的字段，不会在
  字段不存在的版本里多读。`Range` 变体本身仍未被产出（保留以备将来需要）。
- version-gated 块的 **rust_type** 取 `own_map`（merged）。跨版本类型变化的字段
  会生成 VersionedEnum，其 `parse` 按版本选变体，因此 type 层面正确。同名
  Subclass 内部字段变化也由 subclass 自身的 version-gated parse 处理。剩余
  限制：`set_field_none` 的 align_after 取 per-version `version_field_maps`，
  但 VersionedEnum 变体内部的 align 由 `build_versioned_enum` 的 per-version
  meta_flag 计算——两者口径一致，暂无已知问题。

## Conventions

- **注释和 docstring 用中文**。与 crate 其余部分保持一致。
- emit 生成的代码里 `crate::Parse` / `crate::EndianReader` / `crate::UnityVersion`
  等路径解析到 `crates/generated/src/lib.rs` 的 re-export——改 emit 时注意别引入
  lib.rs 未 re-export 的符号。

## Reference docs（改 pipeline/emit 前先读）

- `docs/codegen-target-design.md` — **codegen 目标设计**（扁平 struct + parser），
  冲突时以此为准。
- `docs/codegen-implementation.md` — 实现记录 + §7「关键偏差」。
- `docs/codegen-design.md` — 原始设计稿（**已过时**，parent 模型已移除，仅作历史参考）。
- `docs/object-parsing-design.md` §6/§10 — 主 crate 的 dispatch + TypeTree 回退架构。
