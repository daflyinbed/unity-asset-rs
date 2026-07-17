# 架构笔记

> AGENTS.md 的详细补充。非显然于文件名的架构约定、坑、设计理由。
> 操作性的命令/fixture 规则仍在 AGENTS.md。

## 5 层结构（已实现）

1. **io 层**：`src/format.rs`（`FileFormat` enum + `sniff` + `load_recursive` 递归再派发，产出 `LoadedFile` 树）+ `src/bundle.rs`（UnityFS `parse_unityfs`）+ `src/containers/`（`rawweb.rs` UnityWeb/UnityRaw + `webfile.rs` UnityWebData + `compressed.rs` gzip/brotli）+ `src/decompress.rs`（开注册表）。容器 UnityFS/UnityWeb/UnityRaw/UnityWebData/gzip/brotli；解压 LZMA/LZ4/LZ4HC/NONE。
2. **对象解析层**：`src/serialized_file.rs` + `src/typetree/` + `src/object_parser.rs`。dispatch：MonoBehaviour（classID=114）→ TypeTree 解释器；其他 → codegen typed parse → 失败 fallback AB 内嵌 TypeTree。两路径各自 `Cursor::new(data, endian)`，zero-cost fallback。详见 [`object-layer.md`](object-layer.md)。
3. **codegen typed structs**：`crates/codegen` 从 TPK 生成 `crates/generated`（扁平 struct + parser，366 main class + 571 subclass）。详见 [`codegen.md`](codegen.md)。
4. **聚合层（Environment）**：`src/environment.rs` + `src/names.rs` + `src/detect.rs`。eager parse + PPtr 解引用（`dependencies[file_id]`，对齐 AssetRipper）+ `.resS` streaming_data 零拷贝。跨 bundle：`Environment::builder()`（`path`/`loaded`/`scan_dir`/`registry`）累积后 `build()` 全局解析；`DependencyProvider`/`ResourceProvider` 回调已落地。详见 [`environment-layer.md`](environment-layer.md)。
5. **导出层**（`export` feature 默认开）：
   - **纹理** `src/texture_export.rs`：`TextureFormat` 手写 enum（codegen 不产），ASTC LDR/ETC/BCn/Crunch/未压缩→PNG。
   - **Sprite** `src/sprite_export.rs`：从 atlas Texture2D 裁 `textureRect` + packing rotation + downscale→PNG，对齐 AssetStudio `SpriteHelper.CutImage` / UnityPy。

**未实现**：CN 加密（`0x100` flag）、Loader 配置入口、on-demand 逐块解压、SpillPolicy::AutoTempFile、bumpalo arena、Python 绑定（ADR-0002）。
**暂缓**：ASTC HDR、PVRTC/ATC、BC6H SF16（走 UF16）、Tight mesh packing、alpha texture 合并、多输出格式、法线贴图。

实现细节（格式分派 / 数值映射公式 / Y 翻转策略 / Crunch 两段式解码等）见各源文件头注释。

## 非显然的架构约定

- **`src/format.rs`** 是递归再派发驱动器。公共 API `load`/`load_bytes` 返回 `LoadedFile`（不是 `Bundle`）；`Bundle` 降为 UnityFS parse 的中间结果。
- **`src/bundle.rs`**：Unity bundle 磁盘上**大端**，用私有大端 `Cursor` + 显式边界检查。**不要换 `byteorder`**——内联 cursor 是有意的。
- **`src/cursor.rs`**：公共可切-endian 游标（io 层 `Endian::Big`，SerializedFile 读 endian 字节后切换，TypeTree 继承）。`impl EndianReader for Cursor` 是 generated 代码读字节的实际入口。
- **`src/decompress.rs`** 暴露一个**开**注册表 `CompressionType(pub u16)`，可在任意 flag 槽位注册 codec（含劫持标准 flag，如 Arknights LZ4 变体占 flag 4/5，见 ADR-0003 / UnityPy #308）。**永远不在库里硬编码游戏特定 codec**——这是 `Registry` 扩展点的意义。`Registry::defaults()` 注册 flag 0-3（LZMA/LZ4/LZ4HC/NONE）；LZMA 用 `lzma-rs` + `UnpackedSize::UseProvided`，Unity LZMA 带 5 字节 properties header 但**无内嵌 unpacked size**，须调用方提供（≠ 标准 `.lzma`）。
- **命名漂移**：代码用 `Registry`/`Decompressor`，ADR-0003 正文写 `DecompressorRegistry`。**代码为准**，不迁就文档改名。
- `bytes::Bytes` 贯穿管线（Arc 共享切片，廉价 `slice`）；公共 API 返回 `Bytes` 非 `Vec<u8>`——保持。
- codegen 生成**扁平 struct**（**无 parent 字段**——TPK release_root 已含全继承链），不生成 MonoBehaviour / 抽象父类。跨版本字段用 `VersionedEnum`。
- **`src/environment.rs`**：数据模型全局扁平，多 bundle 灌进同一 Environment 自动 resolve。`.resS` 走 `streaming_data`（空路径→`None`，对齐 AssetRipper，非 UnityPy stem 推导）。命名归一化 `src/names.rs` 对齐 `SpecialFileNames.FixFileIdentifier`。
- **`src/texture_export.rs`**：`TextureFormat` 是手写 enum（`Texture2D.m_format` 是裸 `i32`，codegen 不产），数值对照 UnityPy `enums/TextureFormat.py`。CLI 层按 `fmt.astc_block()`/`is_bcn()`/`is_crunch()`/`is_etc()`/`is_uncompressed()` 五路分派。ASTC/ETC/BCn 解码委托 path 依赖 `texture-decoder-rs`（零依赖 `#![no_std]`）；未压缩走 `decode_uncompressed`（不经 decoder，数值映射对齐 AssetRipper `NumericConversion`，非 AssetStudio）；Crunch 走 `decode_crunch(data, fmt, ver)` 统一入口 + 版本分水岭分派（对齐 AssetRipper `CrunchHandler.IsUseUnityCrunch`：`ver >= 2017.3` 或 format ∈{64,65} → Unity 变体，否则原始）。Y 翻转在 `flip_vertical_in_place`（Unity bottom-up → PNG top-down，对齐 UnityPy `FLIP_TOP_BOTTOM`）；Crunch sig 都是 `0x4878` 但 bitstream 不兼容，靠版本+format 分派。明日方舟 LZ4 变体 codec `AkLz4Decompressor` 放 CLI 层（`src/main.rs`）——遵守「不在库里硬编码游戏 codec」。
- **`src/sprite_export.rs`**：render data 解析三级兜底——`m_sprite_atlas` PPtr → `m_render_data_map[m_render_data_key]`（`Guid` 未派生 `PartialEq`，按 4 个 u32 字段手动比对）；PPtr 空但 `m_atlas_tags` 非空 → 按名称扫同 Environment 的 SpriteAtlas；否则 standalone `m_rd`。`SpriteSettings(u32)` bitfield：bit0=packed / bit1=packingMode(0=Tight,1=Rectangle) / bits2-5=packingRotation / bit6=meshType。**Y 翻转共轭**：`decode_*` 已内置翻转返回 top-down，故 crop 在 top-down 空间做（`textureRect.y` 换算行号），packing rotation 用共轭 `R' = flipV ∘ R ∘ flipV`（None/FlipH/FlipV/Rotate180 不变，**Rotate90 翻方向**——bottom-up 的 rot90cw = top-down 的 rot90ccw，已 Python 独立验证）；越界 rect clamp 到边界。复用 `texture_export` 全部解码原语 + `texture_pixel_bytes` + `write_png`；`flip_vertical_in_place` 提升为 `pub(crate)` 供 rotation 复用。
- 错误用 `snafu`（`#[snafu(display(...))]` + `#[snafu(visibility(pub))]`）。

## 内存

详见 [`memory-notes.md`](memory-notes.md)。要点：`Bytes` 贯穿管线（Arc 共享，`streaming_data()` slice O(1)）；解析产物自包含（`UnityObject`/`Value` 不持原始字节引用）；SF 逐文件 drop；错误隔离（`ParseFailure` 收集而非中断）。混合 bundle 的 `.resS` 资源用 `Bytes::copy_from_slice` detach 独立分配以释放整个解压 blob（Issue #1 已修）。
