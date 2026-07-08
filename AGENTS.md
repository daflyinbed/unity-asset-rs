# AGENTS.md

Compact guidance for sessions working on `unity-asset-rs`.

## What this repo is

A Rust library + CLI for parsing Unity AssetBundles (UnityFS format). Long-term
goal: a typed, high-performance UnityPy/AssetStudio alternative with Python
bindings.

**Current state** — the file-loading + decompression layer, SerializedFile /
TypeTree parsing, and codegen'd typed object parsing are all **implemented**:

- **io 层**（UnityFS 解包 + 解压 registry）：`src/bundle.rs` + `src/decompress.rs`。
  仅支持 UnityFS 容器 + LZMA/LZ4/LZ4HC/NONE 解压；UnityWeb/UnityRaw/gzip/brotli/
  CN 加密 / 递归再派发未做。
- **对象解析层**（SerializedFile + TypeTree 解释器 + dispatch/fallback）：
  `src/serialized_file.rs` + `src/typetree/` + `src/object_parser.rs`。
- **codegen typed structs**：`crates/codegen`（离线工具）从 TPK 生成 `crates/generated`
  里的扁平 struct + parser（366 main class + 571 subclass）。

**未实现**：`World` 聚合层 + PPtr 解引用 + `.resS` streaming_data（路线图）、
Python 绑定（ADR-0002，未开始）。

## docs/ 已与代码对齐

`docs/` 现已是权威现状文档（中文）。冲突时以它们为准：

- [`docs/io-layer.md`](docs/io-layer.md) — UnityFS + 解压 registry（现状 + 路线图）
- [`docs/object-layer.md`](docs/object-layer.md) — SerializedFile + TypeTree + dispatch/fallback（现状 + 路线图）
- [`docs/codegen.md`](docs/codegen.md) — codegen 单一权威（已落地）
- [`docs/adr/`](docs/adr/) — ADR 正文记原始决策，文末「修订」附录记实现偏差

## Commands

```bash
cargo build                       # build lib + bin
cargo build --workspace           # build 全部 crate（含 codegen / generated / unity-core）
cargo test                        # all tests（io 层：tests/parse_bundle.rs）
cargo test load_from_path         # run a single test by name
cargo test --test parse_bundle    # run just the integration test file
cargo run                         # bin 默认 example/char_002_amiya.ab，dump 全部对象
cargo run -- path/to/x.bundle     # parse a specific bundle
cargo run --bin verify_codegen    # 端到端：对 bundle 全部对象跑 typed/TypeTree，统计成败
# 重新生成 typed structs（改完 codegen 后必跑）：
rm -rf crates/generated/src && mkdir -p crates/generated/src
cargo run -p unity-asset-codegen -- crates/codegen/tests/fixtures/type_tree.tpk crates/generated/src
cargo clippy --all-targets        # no clippy.toml; default lints
```

No CI, no pre-commit hooks, no rustfmt.toml, no task runner — plain cargo only.
Edition is `2024` → requires Rust >= 1.85.

## Workspace 结构

**4 个 crate**（非单 crate）：

| crate | 职责 |
|---|---|
| `unity-asset-rs`（根 crate，`src/`） | io 层（UnityFS 解包）+ 对象解析（SerializedFile/TypeTree/dispatch）+ bin（main / verify_codegen） |
| [`crates/unity-core`](crates/unity-core) | 共享原语（~150 行，零依赖）：`UnityVersion` / `Endian` / `EndianReader` / `Parse` / `PPtr` |
| [`crates/codegen`](crates/codegen) | 离线工具（bin + lib）：TPK → Rust。**不 link** unity-core/generated，只 emit 引用它们的源码 |
| [`crates/generated`](crates/generated) | codegen 产物（独立 crate，~9.4 万行 / 940 文件）：`UnityObject` enum + 各 class 扁平 struct + `impl Parse` + `ClassIDType` |

依赖关系：`unity-core` ← `generated` ← `src/`（主 crate）；`codegen` 独立。
详见各 crate 自己的 `AGENTS.md`。

## Tests & fixtures

- io 层测试在 `tests/parse_bundle.rs`（integration；无单元测试）。
- 它们依赖已提交的 fixture `example/char_002_amiya.ab` —— **不要删除或修改它**。
- 期望字节值（`EXPECTED_FIRST16`、`EXPECTED_LAST16`、sizes、revision）是
  **bit-for-bit 对照过 UnityPy** 的。改它们意味着改了 parser 语义；只有拿到验证过的参考输出才更新。
- `src/bin/verify_codegen.rs` 是对象解析的端到端验证（对 bundle 全部对象跑 typed/
  TypeTree，统计 typed 成功 / dynamic / 失败）。非测试，手动跑。
- codegen 的 TPK reader 黄金测试在 `crates/codegen/tests/tpk_read.rs`，数字（1395 版本
  / 412 class / 首版本 3.4）来自真实 AssetRipper brotli TPK。
- 测试 init `env_logger` at `debug`；`RUST_LOG=trace` 看详细 block / typetree 解码。

## Architecture notes that aren't obvious from filenames

- **`src/bundle.rs`** 是 UnityFS 解析器。用私有大端 `Cursor` + 显式边界检查——Unity bundle
  在磁盘上是**大端**。不要换 `byteorder`，内联 cursor 是有意的。
- **`src/cursor.rs`** 是公共可切-endian 游标（io 层用 `Endian::Big`，SerializedFile 读到
  endian 字节后切换，TypeTree 继承）。它 `impl unity_core::EndianReader for Cursor`，
  这是 generated 代码能读字节的实际入口。
- **`src/decompress.rs`** 暴露一个**开**注册表：`CompressionType(pub u16)` 让用户在任意
  flag 槽位注册 codec，包括劫持标准 flag（如 Arknights 用 LZ4 变体占 flag 4/5——见
  ADR-0003 / UnityPy issue #308）。**永远不要在库里硬编码游戏特定 codec**——这正是
  `Registry` 扩展点存在的意义。
- `Registry::defaults()` 注册 **LZMA + LZ4 + LZ4HC + NONE**（UnityFS flag 0-3）。LZMA
  （flag 1）用 `lzma-rs` + `UnpackedSize::UseProvided`，与
  `crates/codegen/src/tpk/compression.rs` 同源——Unity 的 LZMA 流带 5 字节 properties header
  但**无内嵌 unpacked size**，必须由调用方提供。Unity LZMA ≠ 标准 `.lzma`。
- **命名漂移**：代码用 `Registry` / `Decompressor`；ADR-0003 正文写的是
  `DecompressorRegistry`。**代码为准**——不要为了迁就文档而改名。
- `bytes::Bytes` 贯穿管线（Arc 共享切片，廉价 `slice`）。公共 API 返回 `Bytes`，不是
  `Vec<u8>`——保持。
- **对象 dispatch** 在 `src/object_parser.rs`：MonoBehaviour(classID=114) → TypeTree 解释器；
  其他 → codegen'd typed parse（`UnityObject::parse(class, &mut Cursor, ver)`）→ 失败
  fallback 到 AB 内嵌 TypeTree。两条路径各自 `Cursor::new(data, endian)`，zero-cost fallback。
- codegen 生成**扁平 struct**（**无 parent 字段**——TPK release_root 已含全部继承链字段），
  不生成 MonoBehaviour / 抽象父类。字段类型跨版本变化用 `VersionedEnum`。
- 错误用 `snafu`。在 `src/error.rs` 加变体时遵循既有 `#[snafu(display(...))]` +
  `#[snafu(visibility(pub))]` 模式。

## Conventions

- **Docstrings 和内联注释用中文。** 编辑既有文件时保持一致；新代码与邻居风格一致。
- 公共 API 刻意保持精简（`src/lib.rs`）：io 入口（`load`/`load_with`/`load_bytes`/
  `load_bytes_with` + `Bundle`/`BundleFile`/`Registry`/`Decompressor*`）+ 对象解析
  （`parse_object`/`ParsedObject`/`SerializedFile`/`SerializedType`/`ObjectInfo`/`UnityObject`/
  `ClassIDType` + typetree 类型 + `unity_core` 的 `Endian`/`PPtr`/`UnityVersion`）。

## Reference docs (read before non-trivial work)

- [`docs/io-layer.md`](docs/io-layer.md) — io 层现状（UnityFS + registry）。
- [`docs/object-layer.md`](docs/object-layer.md) — 对象解析现状（SerializedFile + TypeTree + dispatch）。
- [`docs/codegen.md`](docs/codegen.md) — codegen 现状（扁平 struct + VersionedEnum）。
- [`docs/adr/`](docs/adr/) — 架构决策记录（0001 codegen / 0002 pyo3 / 0003 io 层）。
