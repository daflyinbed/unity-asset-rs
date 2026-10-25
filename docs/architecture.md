# 架构笔记

> AGENTS.md 的详细补充。非显然于文件名的架构约定、坑、设计理由。
> 操作性的命令/fixture 规则仍在 AGENTS.md。

## 5 层结构（已实现）

1. **io 层**：`crates/unity-asset/src/format.rs`（`FileFormat` enum + `sniff` + `load_recursive` 递归再派发，产出 `LoadedFile` 树）+ `crates/unity-asset/src/bundle.rs`（UnityFS `parse_unityfs`）+ `crates/unity-asset/src/containers/`（`rawweb.rs` UnityWeb/UnityRaw + `webfile.rs` UnityWebData + `compressed.rs` gzip/brotli）+ `crates/unity-asset/src/decompress.rs`（开注册表）。容器 UnityFS/UnityWeb/UnityRaw/UnityWebData/gzip/brotli；解压 LZMA/LZ4/LZ4HC/NONE。
2. **对象解析层**：`crates/unity-asset/src/serialized_file.rs` + `crates/unity-asset/src/typetree/` + `crates/unity-asset/src/object_parser.rs`。dispatch：MonoBehaviour（classID=114）→ TypeTree 解释器；其他 → codegen typed parse → 失败 fallback AB 内嵌 TypeTree。两路径各自 `Cursor::new(data, endian)`，zero-cost fallback。详见 [`object-layer.md`](object-layer.md)。
3. **codegen typed structs**：`crates/codegen` 从 TPK 生成 `crates/generated`（扁平 struct + parser，361 main class struct + 574 subclass struct）。详见 [`codegen.md`](codegen.md)。
4. **聚合层（Environment）**：`crates/unity-asset/src/environment.rs` + `crates/unity-asset/src/names.rs` + `crates/unity-asset/src/detect.rs` + `crates/unity-asset/src/loader.rs`。eager parse + PPtr 解引用（`dependencies[file_id]`，对齐 AssetRipper）+ `.resS` streaming_data 零拷贝。跨 bundle：**`Loader::builder()`**（`path`/`bytes`/`loaded`/`scan_dir`/`registry` 声明式累积）→ `build() -> Result<LoadReport>` 全局解析（顶层 IO 错误 → `Err`，局部失败 → `LoadReport::diagnostics`）。`DependencyProvider`/`ResourceProvider` 回调只负责「打开字节」，Loader 内部做 sniff/解析/归一化。对象身份用 `CollectionId`/`ObjectId`/`ObjectRef` newtype（非裸 `usize`）；class id 用 `ClassId` 开集（含 MonoBehaviour 114）。详见 [`environment-layer.md`](environment-layer.md)。
5. **导出层**（`export` feature 默认开）：
   - **门面** `crates/unity-asset/src/export.rs`：`DecodedImage` 中间表示 + `decode_texture` 统一 5 路分派 + `Environment::decode_image` 高层入口（自动识别 Texture2D/Sprite）。底层原语在 `texture_export` / `sprite_export`。
   - **纹理** `crates/unity-asset/src/texture_export.rs`：`TextureFormat` 手写 enum（codegen 不产），ASTC LDR/ETC/BCn/Crunch/未压缩→PNG。
   - **Sprite** `crates/unity-asset/src/sprite_export.rs`：从 atlas Texture2D 裁 `textureRect` + packing rotation + downscale→PNG，对齐 AssetStudio `SpriteHelper.CutImage` / UnityPy。
   - **音频** `crates/unity-asset/src/audio_export.rs`：复用 `Environment::audio_data` 取内联/`.resS` 字节；FSB5 经同 workspace crate `crates/fsb-rs`（Fmod5Sharp Rust port）逐 sample 重建为 WAV/Ogg，旧版原始格式与 tracker module 按 AssetRipper 规则保留。

**Mesh 几何解码已落地**：`Environment::decode_mesh` 返回 `DecodedMesh`（位置、法线、切线、颜色、保留维数的 UV、四权重 skin、bind poses、SubMesh）。`mesh/` 按 AssetRipper 移植版本化 VertexData / CompressedMesh；支持外部顶点资源、16/32 位索引、baseVertex。关闭 export feature 仍可用。详见 [`mesh-sprite-plan.md`](mesh-sprite-plan.md)。

**Tight Sprite 已落地**：共用 Mesh 顶点/索引解码；CPU 三角形遮罩，standalone 有有效 UV 时采样 atlas。SpriteAtlas 使用解旋裁图，避免采用打包前的旧 UV；独立 alpha texture 红通道合并；atlas 内 PPtr 以 atlas 所属 collection 解引用。

**未实现**：CN 加密（`0x100` flag）、on-demand 逐块解压、SpillPolicy::AutoTempFile、bumpalo arena、Python 绑定（ADR-0002）。
**暂缓**：ASTC HDR、PVRTC/ATC、BC6H SF16（走 UF16）、多输出格式、法线贴图。

实现细节（格式分派 / 数值映射公式 / Y 翻转策略 / Crunch 两段式解码等）见各源文件头注释。

## 非显然的架构约定

- **根 `crates/unity-asset/src/lib.rs`** 分三层:`pub mod io`(io 层门面:LoadedFile/sniff/load_recursive/SerializedFile)+ `pub mod typetree`(TypeTree 解释器)+ `pub mod types`(generated ClassIDType/UnityObject 出口)+ `pub mod export`(纹理/Sprite 导出门面)。顶层直转推荐入口(`Loader`/`Environment`/`CollectionId`/`ClassId`/`DecodedImage` 等);中间体(Bundle/RawEntry/parse_object)归入对应 mod,不再顶层暴露。
- **`crates/unity-asset/src/format.rs`** 是递归再派发驱动器。公共 API `load`/`load_bytes` 返回 `LoadedFile`(不是 `Bundle`);`Bundle` 降为 UnityFS parse 的中间结果。失败子条目产 `LoadedFile::Failed { name, bytes, error }`(保留原始字节 + 错误,不伪装成空 Resource)。
- **`crates/unity-asset/src/bundle.rs`**：Unity bundle 磁盘上**大端**,用 crate-private 大端 `Cursor` + 显式边界检查。**不要换 `byteorder`**——内联 cursor 是有意的。
- **`crates/unity-asset/src/cursor.rs`**:**crate-private**(不再 `pub`)。可切-endian 游标,`seek`/`align`/`remaining` 全做边界检查(越界返回 `Truncated` 错误而非 panic)。`impl EndianReader for Cursor` 是 generated 代码读字节的实际入口。新增功能不要重新 pub Cursor——用户面只通过 `EndianReader` trait 与高级解析入口交互。
- **`crates/unity-asset/src/decompress.rs`** 暴露一个**开**注册表 `CompressionType(pub u16)`，可在任意 flag 槽位注册 codec（含劫持标准 flag，如 Arknights LZ4 变体占 flag 4/5，见 ADR-0003 / UnityPy #308）。**永远不在库里硬编码游戏特定 codec**——这是 `Registry` 扩展点的意义。`Registry::defaults()` 注册 flag 0-3（LZMA/LZ4/LZ4HC/NONE）；LZMA 用 `lzma-rs` + `UnpackedSize::UseProvided`，Unity LZMA 带 5 字节 properties header 但**无内嵌 unpacked size**，须调用方提供（≠ 标准 `.lzma`）。
- **命名漂移**：代码用 `Registry`/`Decompressor`，ADR-0003 正文写 `DecompressorRegistry`。**代码为准**，不迁就文档改名。
- `bytes::Bytes` 贯穿管线（Arc 共享切片，廉价 `slice`）；公共 API 返回 `Bytes` 非 `Vec<u8>`——保持。
- codegen 生成**扁平 struct**（**无 parent 字段**——TPK release_root 已含全继承链），不生成 MonoBehaviour / 抽象父类。跨版本字段用 `VersionedEnum`。
- **`crates/unity-asset/src/environment.rs`**：数据模型全局扁平,多 bundle 灌进同一 Environment 自动 resolve。`.resS` 走 `streaming_data`(空路径→`None`,对齐 AssetRipper,非 UnityPy stem 推导)。命名归一化 `crates/unity-asset/src/names.rs` 对齐 `SpecialFileNames.FixFileIdentifier`。对象身份:`CollectionId(u32)`(collection 句柄,`pub(crate) new` + `pub to_index` escape hatch)+ `ObjectId { collection, path_id }` + `ObjectRef<'env>`(经 `resolve_ref`/`object_ref` 构造)。class id:`ClassId(i32)` 开集(含 MonoBehaviour 114,codegen 跳过),`known()` → `Option<ClassIDType>` 可选映射,`is_typed_parseable()` 判断 typed dispatch 是否覆盖。
- **`crates/unity-asset/src/loader.rs`**:声明式 `Loader`/`LoaderBuilder`/`LoadReport`/`Diagnostic` 取代旧 `EnvironmentBuilder`。`path`/`bytes`/`scan_dir` 只记录输入,`build()` 才统一跑 IO + Phase A + Phase B。顶层 IO/配置错误 → `Result::Err`;局部失败 → `LoadReport::diagnostics`(`Severity::Error`/`Warning`,`DiagnosticKind::{File,Object,Container}`)。`build_strict` 让任何 Error 级 diagnostic 失败。scan_dir 跳过非可加载文件等不再走 `log::warn`,而是 Warning 级 diagnostic。
- **Provider 接口**:`DependencyProvider::open(&DependencyRequest) -> Result<Option<NamedBytes>, ProviderError>` + `ResourceProvider::open(&ResourceRequest) -> Result<Option<Bytes>, ProviderError>`。Provider **只负责打开字节**,Loader 内部跑 `load_recursive` + `phase_a` + `name_fixed` 归一化(旧接口要求 Provider 自备 Registry/命名规则并返回已解析 LoadedFile,职责过重)。`DependencyRequest`/`ResourceRequest` 含来源 `CollectionId` + 原始标识 + 归一化名。Provider 返回 `Err` 记录为 `ParseFailure::Container` 不中断 build。
- **`crates/unity-asset/src/object_parser.rs`**:对象 dispatch——MonoBehaviour(`class_id == ClassId::MONO_BEHAVIOUR.get()`)→ TypeTree 解释器;其他 → codegen typed parse → 失败 fallback AB 内嵌 TypeTree。两路径各自 `Cursor::new(data, endian)`,zero-cost fallback。常规用户入口 `ObjectInfo::parse(&self, sf) -> Result<ParsedObject>`(自动取 refs/版本/endian);自由函数 `parse_object(sf, obj, refs)` 保留供已知 refs 复用。
- **`crates/unity-asset/src/texture_export.rs`**：`TextureFormat` 是手写 enum（`Texture2D.m_format` 是裸 `i32`，codegen 不产），数值对照 UnityPy `enums/TextureFormat.py`。5 路分派**统一**在 `export::decode_texture`(astc/bcn/crunch/etc/uncompressed),CLI 与 sprite_export 都调它,不再各自重复 if-else 链。ASTC/ETC/BCn 解码委托同 workspace 的 `crates/texture-decoder-rs`（零依赖 `#![no_std]`）；未压缩走 `decode_uncompressed`（不经 decoder，数值映射对齐 AssetRipper `NumericConversion`，非 AssetStudio）；Crunch 走 `decode_crunch(data, fmt, ver)` 统一入口 + 版本分水岭分派（对齐 AssetRipper `CrunchHandler.IsUseUnityCrunch`：`ver >= 2017.3` 或 format ∈{64,65} → Unity 变体，否则原始）。Y 翻转在 `flip_vertical_in_place`（Unity bottom-up → PNG top-down，对齐 UnityPy `FLIP_TOP_BOTTOM`）；Crunch sig 都是 `0x4878` 但 bitstream 不兼容，靠版本+format 分派。明日方舟 LZ4 变体 codec `AkLz4Decompressor` 放 CLI 层（`src/main.rs`）——遵守「不在库里硬编码游戏 codec」。
- **`crates/unity-asset/src/export.rs`**:导出门面。`DecodedImage { rgba, width, height }` 中间表示(`write_png`/`write_png_path`)+ `decode_texture` 统一 5 路分派 + `Environment::decode_image(obj, &DecodeOptions)` 高层入口(自动识别 Texture2D/Sprite、取版本、解 atlas)。`DecodeImageError` 统一两路错误。
- **`crates/unity-asset/src/sprite_export.rs`**：render data 解析三级兜底——`m_sprite_atlas` PPtr → `m_render_data_map[m_render_data_key]`（`Guid` 未派生 `PartialEq`，按 4 个 u32 字段手动比对）；PPtr 空但 `m_atlas_tags` 非空 → 按名称扫同 Environment 的 SpriteAtlas；否则 standalone `m_rd`。`SpriteSettings(u32)` bitfield：bit0=packed / bit1=packingMode(0=Tight,1=Rectangle) / bits2-5=packingRotation / bit6=meshType。**Y 翻转共轭**：`decode_*` 已内置翻转返回 top-down，故 crop 在 top-down 空间做（`textureRect.y` 换算行号），packing rotation 用共轭 `R' = flipV ∘ R ∘ flipV`（None/FlipH/FlipV/Rotate180 不变，**Rotate90 翻方向**——bottom-up 的 rot90cw = top-down 的 rot90ccw，已 Python 独立验证）；越界 rect clamp 到边界。复用 `texture_export` 全部解码原语 + `texture_pixel_bytes` + `write_png`；`flip_vertical_in_place` 提升为 `pub(crate)` 供 rotation 复用。
- 错误用 `snafu`（`#[snafu(display(...))]` + `#[snafu(visibility(pub))]`）。

## 内存

详见 [`memory-notes.md`](memory-notes.md)。要点：`Bytes` 贯穿管线（Arc 共享，`streaming_data()` slice O(1)）；解析产物自包含（`UnityObject`/`Value` 不持原始字节引用）；SF 逐文件 drop；错误隔离（`ParseFailure` 收集而非中断）。混合 bundle 的 `.resS` 资源用 `Bytes::copy_from_slice` detach 独立分配以释放整个解压 blob（Issue #1 已修）。
