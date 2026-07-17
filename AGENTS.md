# AGENTS.md

Compact guidance for sessions working on `unity-asset-rs`.

## What this repo is

A Rust library + CLI for parsing Unity AssetBundles (UnityFS format). Long-term
goal: a typed, high-performance UnityPy/AssetStudio alternative with Python
bindings.

**已实现**（5 层）：
- **io 层**：`src/format.rs`（`FileFormat` enum + `sniff` + `load_recursive` 递归再派发，产出 `LoadedFile` 树）+ `src/bundle.rs`（UnityFS `parse_unityfs`）+ `src/containers/`（`rawweb.rs` UnityWeb/UnityRaw + `webfile.rs` UnityWebData + `compressed.rs` gzip/brotli）+ `src/decompress.rs`（开注册表）。容器 UnityFS/UnityWeb/UnityRaw/UnityWebData/gzip/brotli；解压 LZMA/LZ4/LZ4HC/NONE。
- **对象解析层**：`src/serialized_file.rs` + `src/typetree/` + `src/object_parser.rs`（MonoBehaviour→TypeTree 解释器，其他→codegen typed parse→fallback AB 内嵌 TypeTree）。
- **codegen typed structs**：`crates/codegen` 从 TPK 生成 `crates/generated`（扁平 struct + parser，366 main class + 571 subclass）。
- **聚合层（Environment）**：`src/environment.rs` + `src/names.rs` + `src/detect.rs`。eager parse + PPtr 解引用（`dependencies[file_id]`，对齐 AssetRipper）+ `.resS` streaming_data 零拷贝。跨 bundle：`Environment::builder()`（`path`/`loaded`/`scan_dir`/`registry`）累积后 `build()` 全局解析；`DependencyProvider`/`ResourceProvider` 回调已落地。
- **纹理导出**（`export` feature 默认开）：`src/texture_export.rs`。`TextureFormat` 手写 enum（codegen 不产），ASTC LDR/ETC/BCn/Crunch/未压缩→PNG。暂缓：ASTC HDR、PVRTC/ATC、BC6H SF16（走 UF16）、多输出格式、法线贴图。
- **Sprite 导出**（`export` feature 默认开）：`src/sprite_export.rs`。从 atlas Texture2D 裁 `textureRect` + packing rotation + downscale→PNG，对齐 AssetStudio `SpriteHelper.CutImage` / UnityPy。暂不做：Tight mesh、alpha texture 合并。

**未实现**：CN 加密（`0x100` flag）、Loader 配置入口、on-demand 逐块解压、SpillPolicy::AutoTempFile、bumpalo arena、Python 绑定（ADR-0002）。

实现细节（格式分派 / 数值映射公式 / Y 翻转策略 / Crunch 两段式解码等）见各文件头注释与 `docs/`，此处不重复。

## Workspace 结构

**4 个 crate**（非单 crate）。依赖 `unity-core ← generated ← src`（主 crate）；`codegen` 独立。详见各 crate 自己的 `AGENTS.md`。

| crate | 职责 |
|---|---|
| `unity-asset-rs`（根，`src/`） | io + 对象解析 + 纹理/Sprite 导出（`export` feature）+ bin（`main.rs`：`list`/`object`/`export`，export 含 `--no-sprites`/`--only-sprites`；明日方舟 LZ4 变体 codec `AkLz4Decompressor` 在 CLI 层） |
| `crates/unity-core` | 共享原语（~150 行，零依赖）：`UnityVersion`/`Endian`/`EndianReader`/`Parse`/`PPtr` |
| `crates/codegen` | 离线工具（bin+lib）：TPK→Rust。不 link unity-core/generated，只 emit 引用 |
| `crates/generated` | codegen 产物（~9.4 万行/940 文件）：`UnityObject` enum + 扁平 struct + `impl Parse` + `ClassIDType` |

## Commands

```bash
cargo build                       # build lib + bin
cargo build --workspace           # 全部 crate
cargo test                        # 全部测试
cargo test load_from_path         # 单个测试名
cargo test --test parse_bundle    # io 层
cargo test --test environment     # 聚合层
cargo test --test texture_export  # 纹理导出（export feature）
cargo test sprite_export          # Sprite 导出（字节级单测在 src/sprite_export.rs）
cargo run                         # 遍历 example/，列出对象 + 解析状态 + 汇总
cargo run -- path/to/x.bundle     # 单个 bundle
cargo run -- --arknights example/chararts/char_002_amiya.ab export -o /tmp/out
# 重新生成 typed structs（改完 codegen 后必跑）：
rm -rf crates/generated/src && mkdir -p crates/generated/src
cargo run -p unity-asset-codegen -- crates/codegen/tests/fixtures/type_tree.tpk crates/generated/src
cargo clippy --all-targets        # 无 clippy.toml，default lints
```

无 CI / pre-commit / rustfmt.toml / task runner。Edition `2024` → Rust >= 1.85。

## Architecture notes（非显然的）

- **`src/format.rs`**：递归再派发驱动器。公共 API `load`/`load_bytes` 返回 `LoadedFile`（不是 `Bundle`）；`Bundle` 是 UnityFS parse 中间结果。
- **`src/bundle.rs`**：Unity bundle 磁盘上**大端**，用私有大端 `Cursor` + 显式边界检查。**不要换 `byteorder`**——内联 cursor 是有意的。
- **`src/cursor.rs`**：公共可切-endian 游标（io 层 `Endian::Big`，SerializedFile 读 endian 字节后切换，TypeTree 继承）。`impl EndianReader for Cursor` 是 generated 代码读字节的实际入口。
- **`src/decompress.rs`**：开注册表 `CompressionType(pub u16)`，可在任意 flag 槽位注册 codec（含劫持标准 flag，如 Arknights LZ4 变体占 flag 4/5，见 ADR-0003 / UnityPy #308）。**永远不在库里硬编码游戏特定 codec**——这是 `Registry` 扩展点的意义。`Registry::defaults()` 注册 flag 0-3（LZMA/LZ4/LZ4HC/NONE）。Unity LZMA 带 5 字节 properties header 但**无内嵌 unpacked size**，须调用方提供（≠ 标准 `.lzma`）。
- **命名漂移**：代码用 `Registry`/`Decompressor`，ADR-0003 正文写 `DecompressorRegistry`。**代码为准**，不迁就文档改名。
- `bytes::Bytes` 贯穿管线（Arc 共享切片）；公共 API 返回 `Bytes` 非 `Vec<u8>`——保持。
- **对象 dispatch**（`src/object_parser.rs`）：两条路径各自 `Cursor::new(data, endian)`，zero-cost fallback。
- codegen 生成**扁平 struct**（**无 parent 字段**——TPK release_root 已含全继承链），不生成 MonoBehaviour / 抽象父类。跨版本字段用 `VersionedEnum`。
- **`src/environment.rs`**：数据模型全局扁平，多 bundle 灌进同一 Environment 自动 resolve。`.resS` 走 `streaming_data`（空路径→`None`，对齐 AssetRipper，非 UnityPy stem 推导）。命名归一化 `src/names.rs` 对齐 `SpecialFileNames.FixFileIdentifier`。
- **`src/texture_export.rs`**：CLI 层按 `fmt.astc_block()`/`is_bcn()`/`is_crunch()`/`is_etc()`/`is_uncompressed()` 五路分派。Y 翻转在 `flip_vertical_in_place`（Unity bottom-up → PNG top-down，对齐 UnityPy `FLIP_TOP_BOTTOM`）。Crunch sig 都是 `0x4878` 但 bitstream 不兼容，靠版本+format 分派（对齐 AssetRipper `CrunchHandler.IsUseUnityCrunch`）。未压缩数值映射对齐 AssetRipper `NumericConversion`（非 AssetStudio）。
- **`src/sprite_export.rs`**：`SpriteSettings(u32)` bitfield（bit0=packed / bit1=packingMode / bits2-5=packingRotation / bit6=meshType）。Y 翻转共轭（`decode_*` 已内置翻转返回 top-down，crop 在 top-down 空间做，packing rotation 共轭 `flipV ∘ R ∘ flipV`，Rotate90 翻方向）。
- 错误用 `snafu`（`#[snafu(display(...))]` + `#[snafu(visibility(pub))]`）。

## Tests & fixtures

- io 层 `tests/parse_bundle.rs`；聚合层 `tests/environment.rs`；纹理导出 `tests/texture_export.rs`；codegen TPK 黄金测试 `crates/codegen/tests/tpk_read.rs`（1395 版本/412 class/首版本 3.4，来自真实 AssetRipper brotli TPK）。
- **栈溢出 workaround**：debug 默认 opt-level=0 下深嵌套 typetree 的 `interpret` 递归临时量会吃满 2MB 线程栈。已在根 `Cargo.toml` 设 `[profile.dev]`/`[profile.test] opt-level=1` 触发栈槽复用，勿降。
- 纹理导出用 `example/DownloadCommonUI.unity3d`（2 个 ASTC_RGB_6x6，走 `.resS` streaming）端到端；未压缩走字节级单测（公式对齐 AssetRipper `ChangeSign`/`ConvertSingle<byte>`）。明日方舟 `chararts` bundle 的 LZ4 变体在 CLI 层，不在测试覆盖内。
- Sprite 导出端到端测试暂缺（`example/` 无 Sprite 资产，按约定不新增 fixture，TODO）；Rotate90 共轭方向已用 Python 独立验证。
- **不要删除或修改 fixture** `example/char_002_amiya.ab` / `example/DownloadCommonUI.unity3d`。期望字节值（`EXPECTED_FIRST16`/`EXPECTED_LAST16`/sizes/revision）是 **bit-for-bit 对照 UnityPy** 的；改它们=改 parser 语义，仅拿到验证过的参考输出才更新。
- CLI `list`（`src/main.rs`）兼做对象解析端到端验证（遍历 `example/`，标注 typed/dynamic/monobehaviour/fail + 汇总，失败退出码 1）。非测试，手动跑。
- 测试 init `env_logger` at `debug`；`RUST_LOG=trace` 看详细解码。

## 参考实现 & 跨项目文档

父目录 `/home/xwbx/Documents/unity/` 是多项目 workspace（本身非 git repo），含一整套参考实现 + 跨项目文档。**拿不准格式/语义时优先去父目录 grep 参考实现，不要自己发明逻辑。**

- **[`../AGENTS.md`](../AGENTS.md)** — workspace 级指引（子项目角色 + PPtr/`.resS` 跨项目语义差异）。**做非平凡跨项目对照前先读。**
- **[`../docs/`](../docs/)** — 跨项目分析（中文）：`ress-file-handling.md`、`tpk-class-parser-generation.md`、`assetripper-monobehaviour-handling.md`、`astc-hdr-alpha-handling.md`。

参考实现（多数只读 upstream 镜像，**改动前先与用户确认**；git 操作定向到具体子目录）：

| 目录 | 语言 | 角色 |
|---|---|---|
| `../AssetRipper/` | C# (.NET 10) | 全功能 importer/exporter；本仓库 PPtr/`.resS`/命名**刻意对齐**它 |
| `../UnityPy/` | Python | byte-level parity 的默认对照目标 |
| `../AssetStudio/` | C# | UnityPy 原型，读取逻辑可参考 |
| `../Tpk/` | C# | TPK 库 + console app |
| `../TypeTreeDumps/` | bash+data | 全 Unity 版本 struct dump，Tpk 原始输入 |

**本仓库 `docs/`（中文，权威现状文档，冲突时以它们为准）**：
[`io-layer.md`](docs/io-layer.md) / [`object-layer.md`](docs/object-layer.md) / [`environment-layer.md`](docs/environment-layer.md) / [`codegen.md`](docs/codegen.md) / [`adr/`](docs/adr/)（0001 codegen / 0002 pyo3 / 0003 io 层）。

## Conventions

- **Docstrings 和内联注释用中文。** 编辑既有文件保持一致；新代码与邻居风格一致。
- 公共 API 刻意精简（`src/lib.rs`），export 时遵循既有范围，勿随意扩 `pub`。
- 拿不准的格式/语义：先看本仓库 `docs/`，再 grep 父目录参考实现（尤其 AssetRipper），不要自创逻辑。
