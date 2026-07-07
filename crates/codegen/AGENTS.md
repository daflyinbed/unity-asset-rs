# AGENTS.md

Compact guidance for OpenCode sessions working on `crates/codegen`.

## What this crate is

Offline code generator: `type_tree.tpk` (AssetRipper TypeTree dump) → typed Rust
structs + parsers in `crates/generated/src/`. It is a **build-time tool** — it does
not link `unity-core` or `generated`; it only emits source strings that reference
them.

```
type_tree.tpk ──→ run_pipeline (6 pass) ──→ emit_all ──→ crates/generated/src/*.rs
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

## Architecture: pass 管线（`pipeline.rs`）

`run_pipeline` 跑 6 个 pass，产物是 `Pipeline { classes, pptr_targets, all_versions }`：

| pass | 函数 | 职责 |
|---|---|---|
| p01 | `p01_normalize_types` | 类型名归一化（`int→SInt32` 等别名） |
| p02 | `p02_extract_subclasses` | 扫所有 `NodeType::Type` 节点 → subclass |
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

### version-gated parse：只在真重排时启用

跨版本字段顺序处理分两档，由 `dedup_layouts`（`pipeline.rs`）决定：

1. **线性 parse + per-field `Option<T>` 门控**（默认）：字段仅增减、公共字段
   相对顺序不变时用。`dedup_layouts` 用 `common_order_consistent` 判定——两个版本
   的公共字段若相对顺序一致即视为「线性可处理」，返回空 layouts。
2. **version-gated 块**（仅 ~40 个真重排类）：字段相对顺序真变了才用。

> **不要去掉 `common_order_consistent`**：否则 ~220 个类会触发 version-gated，
> 生成代码从 ~95K 行暴涨到 ~160K 行（`generated.rlib` 22M→78M）。

### version-gated 的锚点分段优化（`emit.rs`）

`emit_versioned_field_reads` 不是「每版本分支读全部字段」，而是：

1. 求所有版本字段序列的 **LCS 公共子序列**作为「锚点」（`compute_anchors`）。
2. 锚点线性读一次（所有版本都有、相对顺序一致）。
3. 锚点之间的**段**才做 version-gate，且**每段只在自己的变化版本点分支**
   （`emit_slot_segment`），而非把全部版本分支套到所有字段上。

这把 `player_settings.rs` 从 32K 行压到 13K 行。无公共锚点时退化为全量
version-gated（`emit_full_versioned_branches`）。

### 字段类型取最新版本

`merge_fields_across_versions`（`pipeline.rs`）对跨版本改类型的字段取**最新版本**
的类型定义（不是最老的）。因为本库主要面向现代 Unity 版本。

## emit 产物布局

```
crates/generated/src/
├── lib.rs              # re-export unity_core 的 Parse/EndianReader/UnityVersion/PPtr
├── unity_object.rs     # enum UnityObject + parse 分派（不含抽象类、不含 MonoBehaviour）
├── class_id.rs         # enum ClassIDType（含全部 classID，dispatch 只列非抽象）
├── classes/            # ~379 个 main class（不含抽象类）
└── subclasses/         # ~904 个内嵌 struct
```

每个 class 文件 = `pub struct`（扁平字段，version-gated 字段为 `Option<T>`）+
`impl crate::Parse`。emit 用字符串拼接（无 `quote!`），靠 `crates/generated` 能否
编译来间接验证正确性。

## 已知限制

- **TPK 与具体 bundle 可能不匹配**：TPK 是通用 dump，某些类（如 Arknights 的
  Rigidbody2D/SpriteRenderer/CircleCollider2D/AssetBundle）的字段顺序/集合与
  实际序列化数据不一致，typed parse 会 `data truncated`。主 crate 的
  `object_parser.rs` 会自动回退到 bundle 内嵌 TypeTree 解释器——这是设计如此，
  不是 codegen bug。
- PPtr 非 generic：所有 `PPtr<X>` 生成同一 `PPtr` 类型，丢失目标类型信息。
- subclass 无版本并集：取第一次出现的子树，跨版本变化的 subclass 会错。
- `VersionGate` 只有 `Always`/`Since`，无 `Range`（消失后重现的罕见字段会误判）。

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
