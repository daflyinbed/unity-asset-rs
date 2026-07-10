# AGENTS.md

Compact guidance for sessions working on `unity-asset-rs`.

## What this repo is

A Rust library + CLI for parsing Unity AssetBundles (UnityFS format). Long-term
goal: a typed, high-performance UnityPy/AssetStudio alternative with Python
bindings.

**Current state** — the file-loading + decompression layer, SerializedFile /
TypeTree parsing, codegen'd typed object parsing, and the Environment aggregation
layer (PPtr deref + `.resS` streaming_data) are all **implemented**:

- **io 层**（递归再派发 + 多容器格式 + 解压 registry）：`src/format.rs`（`FileFormat`
  enum + `sniff` + `load_recursive` 驱动器，产出 `LoadedFile` 树）+ `src/bundle.rs`
  （UnityFS `parse_unityfs`）+ `src/containers/`（`rawweb.rs` UnityWeb/UnityRaw +
  `webfile.rs` UnityWebData + `compressed.rs` gzip/brotli）+ `src/decompress.rs`。
  支持 UnityFS/UnityWeb/UnityRaw/UnityWebData/gzip/brotli 容器 + LZMA/LZ4/LZ4HC/NONE
  解压；CN 加密 / Loader / on-demand 解压 / SpillPolicy 未做。
- **对象解析层**（SerializedFile + TypeTree 解释器 + dispatch/fallback）：
  `src/serialized_file.rs` + `src/typetree/` + `src/object_parser.rs`。
- **codegen typed structs**：`crates/codegen`（离线工具）从 TPK 生成 `crates/generated`
  里的扁平 struct + parser（366 main class + 571 subclass）。
- **聚合层**（Environment）：`src/environment.rs` + `src/names.rs`。eager parse 全部
  对象 + `.resS` 资源 + PPtr 解引用（`dependencies[file_id]` 约定对齐 AssetRipper）+
  streaming_data 零拷贝取数。**跨 bundle 已落地**：`EnvironmentBuilder` 累积多 bundle 后
  `build()` 一次性全局依赖解析；聚合层唯一构造入口 `Environment::builder()`（`path`/`loaded`/
  `scan_dir`/`registry`）。`src/detect.rs` 做 UnityFS + gzip 签名嗅探。

**未实现**：CN 加密（`0x100` flag）、Loader/LoaderBuilder 配置入口、
on-demand 逐块解压、SpillPolicy::AutoTempFile、
`DependencyProvider`/`ResourceProvider` trait 的实际回调（API 面已留，eager 模型暂不调）、
bumpalo arena（路线图）、Python 绑定（ADR-0002，未开始）。

## 父目录 workspace（参考实现 + 跨项目文档）

本仓库的父目录 `/home/xwbx/Documents/unity/` 是一个**多项目 workspace**（本身非 git
repo），里面除了本仓库外还有一整套 Unity 资产解析的参考实现和跨项目分析文档。遇到拿不准
的格式/语义时，**优先去父目录 grep 参考实现**，而不是自己发明逻辑。

- **[`../AGENTS.md`](../AGENTS.md)** — workspace 级指引，列清了各子项目角色、构建命令、
  以及 PPtr/`.resS` 等跨项目语义差异。**做非平凡的跨项目对照前先读它。**
- **[`../docs/`](../docs/)** — workspace 级跨项目分析文档（中文），与本仓库 `docs/` 互补：
  - `ress-file-handling.md` — `.resS` streaming：UnityPy vs AssetRipper 对照。
  - `tpk-class-parser-generation.md` — AssetRipper 完整 TPK → class + parser 生成管线。
  - `assetripper-monobehaviour-handling.md` — AssetRipper MonoBehaviour 处理细节。
- **参考实现仓库**（多数只读，用作 ground truth）：
  | 目录 | 语言 | 用途 |
  |---|---|---|
  | `../AssetRipper/` | C# (.NET 10) | 全功能 importer/exporter；TPK 管线 + 运行时消费的权威实现。本仓库的 PPtr/`.resS`/命名约定**刻意对齐 AssetRipper**。 |
  | `../AssetStudio/` | C# | UnityPy 的原型；AssetBundle/SerializedFile/TypeTree 读取逻辑可参考。 |
  | `../UnityPy/` | Python | 成熟 extractor/patcher；本仓库 byte-level parity 的默认对照目标。 |
  | `../Tpk/` | C# | TPK（Type tree PacKage）库 + console app，打包 TypeTreeDumps 成 `.tpk`。 |
  | `../TypeTreeDumps/` | bash + data | 全 Unity 版本（≥3.4）struct dump，Tpk 的原始输入。 |

> 注意：参考仓库多为 upstream 镜像，**改动前先与用户确认**。git 操作须定向到具体子目录
> （workspace 根非 git repo）。

## docs/ 已与代码对齐

`docs/` 现已是权威现状文档（中文）。冲突时以它们为准：

- [`docs/io-layer.md`](docs/io-layer.md) — UnityFS + 解压 registry（现状 + 路线图）
- [`docs/object-layer.md`](docs/object-layer.md) — SerializedFile + TypeTree + dispatch/fallback（现状 + 路线图）
- [`docs/environment-layer.md`](docs/environment-layer.md) — Environment 聚合层（PPtr 解引用 + streaming_data + 跨 bundle 聚合，已落地）
- [`docs/codegen.md`](docs/codegen.md) — codegen 单一权威（已落地）
- [`docs/adr/`](docs/adr/) — ADR 正文记原始决策，文末「修订」附录记实现偏差

## Commands

```bash
cargo build                       # build lib + bin
cargo build --workspace           # build 全部 crate（含 codegen / generated / unity-core）
cargo test                        # all tests（io 层：tests/parse_bundle.rs；聚合层：tests/environment.rs）
cargo test load_from_path         # run a single test by name
cargo test --test parse_bundle    # run just the integration test file
cargo test --test environment     # run just the environment 聚合层 tests
cargo run                         # bin 默认 example/char_002_amiya.ab，dump 全部对象
cargo run -- path/to/x.bundle     # parse a specific bundle
cargo run --bin verify_codegen    # 端到端：默认遍历 example/ 下全部 bundle，跑 typed/TypeTree，统计成败
cargo run --bin verify_codegen -- example/Prefabs.unity3d  # 只跑单个 bundle（或单个目录）
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
- Environment 聚合层测试在 `tests/environment.rs`（integration）。历史问题：debug
  默认 opt-level=0 不优化 TypeTree 解释器 `interpret` 的递归 `Value` 返回值临时，
  深嵌套 typetree 下每帧 ~60B 栈消耗累计到 5–6MB，溢出测试线程 2MB 默认栈。现已在
  根 `Cargo.toml` 设 `[profile.dev]`/`[profile.test] opt-level=1`，触发栈槽复用后 2MB
  默认栈足够，无需 workaround。备选方案（box+arena，类似 oxc）见路线图，未实现。
- 它们依赖已提交的 fixture `example/char_002_amiya.ab` —— **不要删除或修改它**。
- 期望字节值（`EXPECTED_FIRST16`、`EXPECTED_LAST16`、sizes、revision）是
  **bit-for-bit 对照过 UnityPy** 的。改它们意味着改了 parser 语义；只有拿到验证过的参考输出才更新。
- `src/bin/verify_codegen.rs` 是对象解析的端到端验证：默认遍历 `example/` 下全部
  bundle，对每个 bundle 的全部对象跑 typed/TypeTree，统计 typed 成功 / dynamic / 失败。
  可传一个路径（文件或目录）只处理它。非测试，手动跑。
- codegen 的 TPK reader 黄金测试在 `crates/codegen/tests/tpk_read.rs`，数字（1395 版本
  / 412 class / 首版本 3.4）来自真实 AssetRipper brotli TPK。
- 测试 init `env_logger` at `debug`；`RUST_LOG=trace` 看详细 block / typetree 解码。

## Architecture notes that aren't obvious from filenames

- **`src/format.rs`** 是递归再派发驱动器：`FileFormat` enum（闭集）+ `sniff(&[u8])`
  if/else 链 + `load_recursive`（Container → 逐子条目重新 sniff+parse，产出 `LoadedFile`
  树）。公共 API `load`/`load_bytes` 返回 `LoadedFile`，不是 `Bundle`。`Bundle` 降为
  UnityFS parse 的中间结果（`parse_unityfs` 产出）。
- **`src/containers/`** 是多容器格式解析：`rawweb.rs`（UnityWeb/UnityRaw，共用 header
  + directory 逻辑，UnityWeb 额外做 LZMA 解压 `scenes[last]`）、`webfile.rs`
  （UnityWebData，**小端**）、`compressed.rs`（gzip/brotli 外层包裹，解压后递归 sniff）。
- **`src/bundle.rs`** 是 UnityFS 解析器（`parse_unityfs`）。用私有大端 `Cursor` + 显式边界
  检查——Unity bundle 在磁盘上是**大端**。不要换 `byteorder`，内联 cursor 是有意的。
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
- **`src/environment.rs`** 是聚合层：`EnvironmentBuilder`（`Environment::builder()`，累积后
  `build()`）eager parse 全部对象，构建 `AssetCollection.dependencies`
  （`[0]=self` 约定对齐 AssetRipper `AssetCollection.Dependencies`），PPtr 解引用经
  `dependencies[file_id]` → target collection → `assets[path_id]`。**跨 bundle 已落地**：
  数据模型本就全局扁平，Phase B 全局扫描 `collections`，多 bundle 灌进同一 Environment 即
  自动 resolve。入口 `path`/`loaded`/`scan_dir` 接收 `LoadedFile`（递归再派发后的树），
  `phase_a` 用 `leaves()` 展平消费。`.resS` 流数据走 `streaming_data`（空路径→`None`，对齐 AssetRipper，非
  UnityPy stem 推导）。命名归一化 `src/names.rs::fix_file_identifier` 对齐
  `SpecialFileNames.FixFileIdentifier`。`src/detect.rs` 做 UnityFS + gzip 签名嗅探（目录扫描用）。
- 错误用 `snafu`。在 `src/error.rs` 加变体时遵循既有 `#[snafu(display(...))]` +
  `#[snafu(visibility(pub))]` 模式。

## Conventions

- **Docstrings 和内联注释用中文。** 编辑既有文件时保持一致；新代码与邻居风格一致。
- 公共 API 刻意保持精简（`src/lib.rs`）：io 入口（`load`/`load_with`/`load_bytes`/
  `load_bytes_with` + `LoadedFile`/`FileFormat`/`Bundle`/`RawEntry`/`Registry`/`Decompressor*`）
  + 对象解析（`parse_object`/`ParsedObject`/`SerializedFile`/`SerializedType`/
  `ObjectInfo`/`FileIdentifier`/`UnityObject`/`ClassIDType` + typetree 类型 + `unity_core`
  的 `Endian`/`PPtr`/`UnityVersion`）+ 聚合层（`Environment`/`EnvironmentBuilder`/`EnvironmentObject`/
  `AssetCollection`/`ResourceFile`/`ParseFailure`/`PPtrLike`/`DependencyProvider`/
  `ResourceProvider`）。

## Reference docs (read before non-trivial work)

- [`docs/io-layer.md`](docs/io-layer.md) — io 层现状（UnityFS + registry）。
- [`docs/object-layer.md`](docs/object-layer.md) — 对象解析现状（SerializedFile + TypeTree + dispatch）。
- [`docs/environment-layer.md`](docs/environment-layer.md) — 聚合层现状（Environment + Builder + PPtr 解引用 + streaming_data + 跨 bundle 聚合）。
- [`docs/codegen.md`](docs/codegen.md) — codegen 现状（扁平 struct + VersionedEnum）。
- [`docs/adr/`](docs/adr/) — 架构决策记录（0001 codegen / 0002 pyo3 / 0003 io 层）。
