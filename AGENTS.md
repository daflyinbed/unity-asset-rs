# AGENTS.md

Compact guidance for sessions working on `unity-asset-rs`.

## What this repo is

A Rust library + CLI for parsing Unity AssetBundles (UnityFS format). Long-term
goal: a typed, high-performance UnityPy/AssetStudio alternative with Python
bindings.

**5 层已实现**：io（UnityFS 解包）→ 对象解析（SerializedFile/TypeTree/dispatch）→ codegen typed structs → 聚合层（Environment，PPtr 解引用 + `.resS` streaming + 跨 bundle）→ 导出（纹理 + Sprite→PNG，`export` feature 默认开）。

**未实现**：CN 加密、Loader 配置、on-demand 解压、SpillPolicy::AutoTempFile、bumpalo arena、Python 绑定。

架构细节、设计理由、非显然约定见 [`docs/architecture.md`](docs/architecture.md)；各层现状见 `docs/*-layer.md`。

## Workspace 结构（4 crate）

依赖 `unity-core ← generated ← src`（主 crate）；`codegen` 独立。各 crate 有自己的 `AGENTS.md`。

| crate | 职责 |
|---|---|
| `unity-asset-rs`（根，`src/`） | io + 对象解析 + 纹理/Sprite 导出 + bin（`main.rs`：`list`/`object`/`export`，含 `--no-sprites`/`--only-sprites`/`--arknights`） |
| `crates/unity-core` | 共享原语（零依赖）：`UnityVersion`/`Endian`/`EndianReader`/`Parse`/`PPtr` |
| `crates/codegen` | 离线工具：TPK→Rust，只 emit 引用，不 link |
| `crates/generated` | codegen 产物：`UnityObject` enum + 扁平 struct + `ClassIDType` |

## Commands

```bash
cargo build --workspace           # 全部 crate
cargo test                        # 全部测试
cargo test --test parse_bundle    # io 层
cargo test --test environment     # 聚合层
cargo test --test texture_export  # 纹理导出（export feature）
cargo test sprite_export          # Sprite（字节级单测在 src/sprite_export.rs）
cargo run                         # 遍历 example/，对象 + 解析状态 + 汇总（失败退出码 1）
cargo run -- --arknights example/chararts/char_002_amiya.ab export -o /tmp/out
# 重新生成 typed structs（改完 codegen 后必跑）：
rm -rf crates/generated/src && mkdir -p crates/generated/src
cargo run -p unity-asset-codegen -- crates/codegen/tests/fixtures/type_tree.tpk crates/generated/src
cargo clippy --all-targets
```

无 CI / pre-commit / rustfmt.toml / task runner。Edition `2024` → Rust >= 1.85。

## ⚠️ 硬性规则（改代码前必读）

- **不在库里硬编码游戏特定 codec**——走 `Registry` 扩展点（`src/decompress.rs`），如 Arknights LZ4 变体放 CLI 层 `AkLz4Decompressor`。
- **大端 cursor 不要换 `byteorder`**——`src/bundle.rs` 内联 cursor 是有意的。
- **勿降 `opt-level=1`**（根 `Cargo.toml` `[profile.dev]`/`[profile.test]`）——降回 0 会在深嵌套 typetree 下溢出 2MB 测试线程栈。
- **不要删/改 fixture** `example/char_002_amiya.ab` / `example/DownloadCommonUI.unity3d`。期望字节值（`EXPECTED_FIRST16` 等）是 **bit-for-bit 对照 UnityPy** 的，改=改 parser 语义。
- **代码为准**——`Registry`/`Decompressor` vs ADR-0003 的 `DecompressorRegistry` 命名漂移，不迁就文档改名。
- **拿不准格式/语义**：先看 `docs/`，再 grep 父目录参考实现（尤其 AssetRipper），不要自创逻辑。
- 公共 API 刻意精简（`src/lib.rs`），export 遵循既有范围，勿随意扩 `pub`。

## 参考实现 & 跨项目文档

父目录 `/home/xwbx/Documents/unity/` 是多项目 workspace（非 git repo），含参考实现 + 跨项目文档。拿不准时优先 grep。

- **[`../AGENTS.md`](../AGENTS.md)** — workspace 级指引（PPtr/`.resS` 跨项目语义差异）。**做跨项目对照前先读。**
- **[`../docs/`](../docs/)** — 跨项目分析（中文）：`ress-file-handling.md` / `tpk-class-parser-generation.md` / `assetripper-monobehaviour-handling.md` / `astc-hdr-alpha-handling.md`。

参考实现（多数只读 upstream 镜像，**改动前先与用户确认**；git 操作定向到具体子目录）：

| 目录 | 角色 |
|---|---|
| `../AssetRipper/` (C# .NET 10) | 全功能 importer/exporter；本仓库 PPtr/`.resS`/命名**刻意对齐**它 |
| `../UnityPy/` (Python) | byte-level parity 默认对照目标 |
| `../AssetStudio/` (C#) | UnityPy 原型，读取逻辑可参考 |
| `../Tpk/` + `../TypeTreeDumps/` | TPK 库 + 全 Unity 版本 struct dump |

**本仓库 `docs/`（中文，权威现状，冲突时以它们为准）**：
[`architecture.md`](docs/architecture.md)（架构+约定） /
[`io-layer.md`](docs/io-layer.md) /
[`object-layer.md`](docs/object-layer.md) /
[`environment-layer.md`](docs/environment-layer.md) /
[`codegen.md`](docs/codegen.md) /
[`memory-notes.md`](docs/memory-notes.md) /
[`adr/`](docs/adr/)（0001 codegen / 0002 pyo3 / 0003 io 层）。
