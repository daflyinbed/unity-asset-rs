# 文件加载与解压层（io 层）

> 范围：从用户输入（路径 / 字节）到 `LoadedFile`（递归再派发后的内部文件树 + 字节）。
> 状态：**本文档描述已落地的实现**（与 `crates/unity-asset/src/bundle.rs` + `crates/unity-asset/src/format.rs` +
> `crates/unity-asset/src/decompress.rs` 一致）。
> 上游：[ADR-0003](adr/0003-file-loading-and-decompression-layer.md)（io 层决策，含修订附录）、
> [`object-layer.md`](object-layer.md)（本层产出的 `LoadedFile::Leaf.bytes` 交给对象解析）、
> [`environment-layer.md`](environment-layer.md)（`Loader` 消费 `LoadedFile` 树跑聚合层）。

---

## 1. 概述

io 层负责 Unity **容器格式解析 + 解压**：读一个 `.bundle` / `.assets` 文件，剥掉外层
UnityFS 归档与压缩块，递归再派发后交付一棵 `LoadedFile` 文件树（Container / Leaf /
Failed）。它**不理解对象语义**——解出的叶子字节是 SerializedFile 还是 `.resS` 由上层
判断（见 `object-layer.md`）。

**输入**：文件路径 / `bytes::Bytes`。
**输出**：`LoadedFile`（Container 含 engine revision + format version + 递归子树；
Leaf 含 name + bytes + sniff 结论；Failed 保留原始字节 + 错误）。

**核心设计取舍**（详见 ADR-0003）：
- 解压 codec = 开集 → `Decompressor` trait + `Registry`（用户可覆盖任意 flag）。
- 字节运输 = `bytes::Bytes`（Arc 共享切片，`slice()` 共享分配，无生命周期污染）。
- 同步、纯内存（无 spill、无 async）。
- 错误隔离：子条目 parse 失败产 `LoadedFile::Failed`（保留原始字节 + 错误），
  不中断整体加载；顶层 parse 失败仍向上传播（`load_recursive` 返回 `Err`）。

> **命名约定**：代码实际类型名是 `Registry` / `Decompressor`。ADR-0003 正文写的
> `DecompressorRegistry` 是早期设计稿用名——**以代码为准**。这是已知的命名漂移。
>
> **导出口**：顶层 `lib.rs` 直转常用入口（`load` / `load_bytes` / `LoadedFile` /
> `FileFormat` / `sniff` / `load_recursive`）；`Bundle` / `RawEntry` / `SerializedFile` /
> `ObjectInfo` / `FileIdentifier` 等中间体归入 [`io`](`crate::io`) 门面模块（高级用户用）。
> 聚合层构造入口是 `Loader`（见 [`environment-layer.md`](environment-layer.md)），它内部
> 自动调 `load_recursive`——普通用户不需要直接调 io 层。

---

## 2. 现状：已实现什么

| 项 | 状态 |
|---|---|
| UnityFS（现代主流容器，format 6/7/8）解包 | ✅ |
| 解压器 registry（`CompressionType` open 枚举 + 可覆盖任意 flag） | ✅ |
| 默认 codec：LZMA / LZ4 / LZ4HC / NONE（flag 0–3） | ✅ |
| 大端 Cursor + 显式边界检查（crate-private，见 §4） | ✅ |
| 用户自定义 codec 接入（如 Arknights LZ4 变体） | ✅（扩展点） |
| 格式闭集 enum + sniff（`FileFormat` + `sniff(&[u8])`） | ✅ |
| 递归再派发（`load_recursive`：Container → 逐子条目重新 sniff + parse） | ✅ |
| UnityWeb / UnityRaw（旧版容器，LZMA / 裸读） | ✅ |
| UnityWebData（WebGL 数据格式，小端） | ✅ |
| gzip / brotli 外层包裹（解压后递归 sniff 内层） | ✅ |
| SerializedFile 弱嗅探（头部尺寸一致性校验） | ✅ |

未实现的容器格式与能力见 §6 路线图。

---

## 3. 公共 API

实现位置：`crates/unity-asset/src/lib.rs`（re-export）+ `crates/unity-asset/src/format.rs` + `crates/unity-asset/src/bundle.rs` + `crates/unity-asset/src/decompress.rs`。

```rust
/// 从文件路径加载 Unity 资产文件（递归再派发）。
pub fn load(path: impl AsRef<Path>) -> Result<LoadedFile>;

/// 用自定义解压器注册表加载（递归再派发）。
pub fn load_with(path: impl AsRef<Path>, registry: &Registry) -> Result<LoadedFile>;

/// 从内存字节加载（递归再派发）。
pub fn load_bytes(src: Bytes) -> Result<LoadedFile>;

/// 用自定义解压器注册表从内存字节加载（递归再派发）。
pub fn load_bytes_with(src: Bytes, registry: &Registry) -> Result<LoadedFile>;
```

`load` / `load_bytes` 内部用 `Registry::defaults()`；`*_with` 接用户 registry。最终都
走 `format::load_recursive(src, name, registry, None)`。

### 结果类型

```rust
/// io 层递归再派发的产物：一棵可能嵌套的文件树。
pub enum LoadedFile {
    Container {
        unity_revision: Option<String>,   // bundle 有；webfile/compressed 无
        format_version: Option<u32>,      // bundle 的 format（6/7/8）；非 bundle 为 None
        format: FileFormat,               // 本层的格式（调试/日志用）
        children: Vec<LoadedFile>,        // 递归后的子树
    },
    /// 叶子：SerializedFile 或裸资源。
    Leaf {
        name: String,
        bytes: Bytes,
        unity_revision: Option<String>,   // 从父 Container 继承
        format: FileFormat,               // sniff 结论（SerializedFile / Resource 等）
    },
    /// 子条目递归 parse 失败。携带**原始字节**（可供诊断/重试）与错误。
    ///
    /// 旧实现把失败伪装成 `Leaf { format: Resource, bytes: Bytes::new() }`，
    /// 丢弃原始字节且只写日志——下游既看不到失败也拿不回字节。本变体让失败成为
    /// 显式 API 结果（对齐 ADR-0003「错误隔离 Failure 叶节点」）。
    Failed {
        name: String,
        bytes: Bytes,
        error: Error,
        unity_revision: Option<String>,   // 从父 Container 继承
    },
}

/// 失败节点引用（view），由 `LoadedFile::failures` 产出。
pub struct LoadedFailure<'a> {
    pub name: &'a str,
    pub bytes: &'a Bytes,
    pub error: &'a Error,
}

impl LoadedFile {
    /// 深度优先展平到叶子列表（供下游消费）。**不含** `Failed` 节点
    ///（用 `failures()` 单独查）。每个叶子返回
    /// `(name, bytes, unity_revision, format)` 四元组。
    pub fn leaves(&self) -> Vec<(&str, &Bytes, Option<&str>, FileFormat)>;

    /// 深度优先收集所有 `Failed` 节点的引用 view。
    pub fn failures(&self) -> Vec<LoadedFailure<'_>>;

    /// 消费整棵树，把所有 `Failed` 节点的 `name`/`error` 按值取出。
    /// `Error` 未实现 `Clone`（含 `io::Error`），借用版本 `failures()` 只能返回引用；
    /// 调用方若需要拥有（如塞进聚合层 `ParseFailure::Container`）用此版本。
    pub fn into_failures(self) -> Vec<(String, Error)>;
}

/// 闭集格式 enum（无 trait、无 Vec）。
pub enum FileFormat {
    SerializedFile, FileStreamBundle, WebBundle, RawBundle,
    WebFile, GZip, Brotli, ArchiveBundle, Resource,
}

/// 嗅探字节切片的格式（if/else 链）。
pub fn sniff(bytes: &[u8]) -> FileFormat;

/// 递归再派发驱动器。
pub fn load_recursive(src: Bytes, name: &str, registry: &Registry,
                      parent_revision: Option<&str>) -> Result<LoadedFile>;
```

`Bundle` 仍存在但语义为 **UnityFS parse 的中间结果**（`parse_unityfs` 产出，递归前）。
经 [`io`](`crate::io`) 门面模块导出（顶层不再直转）：

```rust
pub struct Bundle {
    pub unity_revision: String,
    pub format_version: u32,
    pub entries: Vec<RawEntry>,  // 原始子字节，尚未 sniff
}
pub struct RawEntry { pub name: String, pub bytes: Bytes, pub flags: u32 }
```

> `load` 返回的 `LoadedFile` 是递归后的**文件树**——Container 内嵌 Container/Leaf/Failed，
> 任意嵌套（如 `brotli(UnityFS(.assets))`）免费获得。下游用 `leaves()` 展平消费、
> `failures()` / `into_failures()` 单独查失败节点。
>
> **重新导出口**：顶层 `lib.rs` 只直转面向普通用户的入口（`load`/`load_bytes`/`LoadedFile`/
> `FileFormat`/`sniff`/`load_recursive` 等）；`Bundle`/`RawEntry`/`SerializedFile`/
> `ObjectInfo`/`FileIdentifier` 等中间体归入 [`io`](`crate::io`) 门面模块（高级用户用），
> 不再在顶层暴露。

---

## 4. UnityFS 解析流程

实现位置：`crates/unity-asset/src/bundle.rs::parse`。UnityFS 在磁盘上是**大端**，用 **crate-private** 大端
`Cursor`（`crates/unity-asset/src/cursor.rs`，`mod cursor`——不再 `pub`）。`Cursor` 的 `seek` / `align` /
`remaining` 全做边界检查：`seek`/`align` 越界返回 `Truncated` 错误（不 panic），
`remaining()` 用 `saturating_sub`（下溢归零）。**不要**换 `byteorder`，内联 cursor 是有意的，
新增功能也不要重新 `pub` Cursor——用户面只通过 `EndianReader` trait 与高级解析入口
（`Loader` / `Environment` / `SerializedFile`）交互。

`Cursor` 经 `impl unity_core::EndianReader for Cursor` 桥接 generated crate 的 `Parse`
trait，是 generated 代码读字节的实际入口（见 [`object-layer.md`](object-layer.md) §6.1）。

### 4.1 Header

```
cstr  signature            // "UnityFS"
u32   format_version       // 必须 >= 6
cstr  unity_version        // player 版本（MVP 不用，消费即可）
cstr  unity_revision       // 引擎版本
i64   size                 // 声明大小
u32   compressed_blocks_info_size
u32   uncompressed_blocks_info_size
u32   archive_flags
```

`format_version < 6` → `UnsupportedCompression`（复用错误槽）。`format_version >= 7`：
header 后对齐到 16。

### 4.2 archive flags

| 位 | 名 | 含义 |
|---|---|---|
| `0x3F`（低 6 位） | compression | blocks-info 压缩类型 |
| `0x80` | blocks-info-at-end | blocks-info 压缩块位于文件末尾 |
| `0x200` | need-padding | 读 block 数据前对齐到 16 |

```
const ARCHIVE_COMPRESSION_MASK: u32 = 0x3F;
const ARCHIVE_BLOCKS_INFO_AT_THE_END: u32 = 0x80;
const ARCHIVE_BLOCK_INFO_NEED_PADDING_AT_START: u32 = 0x200;
```

### 4.3 blocks-info 定位与解压

- 末尾 flag：`total - compressed_blocks_info_size`。
- 否则：`blocks_info_pos`（header 后，v7+ 对齐 16）。
- 用 `archive_flags` 低 6 位查 registry 解压。

解出的 blocks-info：

```
16 字节 uncompressed data hash（忽略）
u32  block_count
  [StorageBlock × block_count]:  u32 uncompressed_size ; u32 compressed_size ; u16 flags
u32  node_count
  [Node × node_count]:  i64 offset ; i64 size ; u32 flags ; cstr path
```

`StorageBlock.compression()` = `flags & 0x3F` → `CompressionType`。

### 4.4 数据块逐块解压、拼接

定位 block 数据起始：`blocks-info` 在末尾时从 `blocks_info_pos`；否则紧随其后。
`0x200` flag 时对齐到 16。

逐块按 `block.compression()` 查 registry 解压，`extend_from_slice` 拼成 `Vec<u8>`
（预分配 `total_uncompressed`）。单块解压失败 → `Decompress` 错误；越界 → `Truncated`。

### 4.5 按 node 切片

拼接出的 blob 按 node 的 `(offset, size)` `slice`（共享分配的 Arc 切片），产出
`Bundle { unity_revision, format_version, entries: Vec<RawEntry> }`。越界 → `Truncated`。
`RawEntry` 是原始子字节（尚未 sniff）；递归驱动器 `load_recursive` 会对每个 entry 重新
sniff + parse，最终产出 `LoadedFile` 树。

---

## 5. 可插拔解压器：`Decompressor` + `Registry`

实现位置：`crates/unity-asset/src/decompress.rs`。这是本层的核心扩展点。

```rust
/// 压缩类型。低 6 位 flag 的编码。
///
/// 开放枚举：用户可通过 Registry::register 注册自定义槽位
/// （如明日方舟私改 LZ4 占 flag 4/5）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CompressionType(pub u16);

impl CompressionType {
    pub const NONE: Self  = Self(0);
    pub const LZMA:  Self = Self(1);
    pub const LZ4:   Self = Self(2);
    pub const LZ4HC: Self = Self(3);
}

/// 单个 codec，无状态。`expected` 为解压后大小提示：LZ4 必需；NONE 忽略。
pub trait Decompressor: Send + Sync {
    fn decompress(&self, input: &[u8], expected: usize) -> Result<Vec<u8>>;
}

pub struct Registry { /* HashMap<CompressionType, Arc<dyn Decompressor>> */ }
impl Registry {
    pub fn new() -> Self;            // 空
    pub fn defaults() -> Self;       // LZMA + LZ4 + LZ4HC + NONE（flag 0-3）
    pub fn register(&mut self, ct: CompressionType, d: Arc<dyn Decompressor>)
        -> Option<Arc<dyn Decompressor>>;   // 注册（覆盖）任意槽位
    pub fn get(&self, ct: CompressionType) -> Result<&dyn Decompressor>;
}
```

### 5.1 默认 codec

`Registry::defaults()` 注册：
- `LZMA`（flag 1）：`lzma-rs` + `UnpackedSize::UseProvided(expected)`。
- `LZ4` / `LZ4HC`（flag 2/3，共用 `Lz4Decompressor`，LZ4HC 仅编码侧不同）：`lz4_flex`。
- `NONE`（flag 0）：`IdentityDecompressor`（原样返回）。

**Unity LZMA ≠ 标准 `.lzma`**：Unity 的 LZMA 流带 5 字节 properties header（1 byte
lc/lp/pb + 4 byte dict size，LE），但**流内不含 8 字节 unpacked-size 字段**，必须由
调用方提供（故用 `UseProvided`）。与 `crates/codegen/src/tpk/compression.rs` 的
`decompress_lzma` 同源（参考 AssetRipper `LzmaHandler.cs` + SharpCompress `LzmaStream`）。

### 5.2 自定义 codec 接入（Arknights LZ4 变体）

**设计依据**：UnityPy issue [#308](https://github.com/K0lb3/UnityPy/issues/308) + PR
[#309](https://github.com/K0lb3/UnityPy/pull/309)。明日方舟（Arknights）CN 服用一种
LZ4 变体（literal/match nibble 对调），占 flag 4/5（死槽位 LZHAM）。维护者 K0lb3 指出
**自定义压缩可能劫持标准 flag**，故「未知 flag fallback」不够，必须能覆盖任意 flag。

**库默认不内置任何游戏特定代码**——只提供扩展点。用户自行实现并注册：

```rust
struct AkLz4;
impl Decompressor for AkLz4 {
    fn decompress(&self, input: &[u8], expected: usize) -> Result<Vec<u8>> {
        let fixed = swap_ak_nibbles(input);     // 修正 nibble
        let mut reg = Registry::defaults();
        let std_lz4 = reg.get(CompressionType::LZ4)?;
        std_lz4.decompress(&fixed, expected)    // 复用注册表内 LZ4
    }
}

let mut reg = Registry::defaults();
reg.register(CompressionType(4), Arc::new(AkLz4));
reg.register(CompressionType(5), Arc::new(AkLz4));

let bundle = load_with("arknights/ab.bundle", &reg)?;
```

---

## 6. 路线图（未实现）

ADR-0003 描述了一个更完整的「递归再派发 + 三轴差异化扩展」架构。递归再派发 + 格式
enum/sniff + 多容器格式（UnityWeb/UnityRaw/UnityWebData）+ gzip/brotli 外层包裹
**已落地**，下列项尚未实现。

| 项 | 设计要点 | 状态 |
|---|---|---|
| **递归再派发** | 每种格式只「识别 + 解析自身结构 + 列出原始子字节」；驱动器看到 Container 就对每个子条目重新 sniff + parse。同一套派发服务顶层与子条目，任意嵌套（如 `brotli(UnityFS(.assets))`）免费获得 | ✅ |
| **格式闭集 enum + sniff** | `enum FileFormat { ... }` + `sniff(&[u8])` if/else 链 + `match parse`（`crates/unity-asset/src/format.rs`） | ✅ |
| **UnityWeb / UnityRaw / UnityWebData** | 旧版容器格式（`crates/unity-asset/src/containers/rawweb.rs` + `crates/unity-asset/src/containers/webfile.rs`） | ✅ |
| **gzip / brotli 外层包裹** | 外层压缩包住上述任意一种容器，解压后递归 sniff（`crates/unity-asset/src/containers/compressed.rs`） | ✅ |
| **CN 加密** | `0x100` flag，Unity CN。concrete `ArchiveStorageDecryptor { index, substitute }` + `CnKey` 配置项，在 UnityFS 解析器内按块 `decrypt_block` 后再解压（N=1，不抽象为 trait） | ⬜ |
| **Loader / LoaderBuilder** | 高级门面：声明式 `.path()`/`.bytes()`/`.loaded()`/`.scan_dir()`/`.registry()`/`.dependency_provider()`/`.resource_provider()` 累积，`build() -> Result<LoadReport>` 统一跑 io + 对象 + 聚合层 Phase A/B；顶层 IO 错误 → `Err`，局部失败 → `LoadReport::diagnostics`。已落地，详见 [`environment-layer.md`](environment-layer.md) §8/§13 | ✅ |
| **.resS 分离与 World** | `.resS` node 由聚合层（`Environment`）捕获为 `ResourceFile`，`StreamingInfo` 引用经 `streaming_data` 零拷贝取数。已落地，详见 [`environment-layer.md`](environment-layer.md) | ✅ |
| **on-demand 逐块解压** | AssetRipper `BundleFileBlockReader` 式：不 concat 整块，按需解压+缓存上一块，峰值压到 ~1× + 几块；但要换 `Cursor<&[u8]>` 为跨块 reader | ⬜ |
| **SpillPolicy::AutoTempFile** | >2GB bundle 解压溢出临时文件，避免 OOM | ⬜ |
| **错误隔离 Failure 叶节点** | 单文件失败捕获成 `Failure`，不中断整体加载。`load_recursive` 子条目失败产 `LoadedFile::Failed { name, bytes, error }`（**保留原始字节**）；聚合层把它投影成 `ParseFailure::Container` + `Diagnostic`（`DiagnosticKind::Container`）。顶层 parse 失败仍向上传播（`load_recursive` 返回 `Err`） | ✅ |

### 内存模型（当前）

当前 io 层把所有 block 解压并 `extend_from_slice` 拼成一个 `Vec<u8>`，再按 node `slice`
成 `Bytes`（共享 Arc）。SerializedFile / objects 默认由聚合层 `Environment`（经 `Loader`）
eager parse；`.resS` 独立拷贝分离策略由聚合层完成（`Bytes::copy_from_slice` detach，详见
[`environment-layer.md`](environment-layer.md)）。直接调 io 层 `load`/`load_bytes` 的高级
用户拿到 `LoadedFile` 树后，可按需自己 parse（`SerializedFile::parse` + `ObjectInfo::parse`）。

---

## 7. 错误类型

io 层错误见 `crates/unity-asset/src/error.rs`，遵循 `#[snafu(display(...))]` + `#[snafu(visibility(pub))]`
模式。io 相关变体：

- `Io`（io::Error）
- `Truncated { reason }`（数据截断 / 越界）
- `BadSignature { sig }`（非 "UnityFS"）
- `UnsupportedVersion { version }`（format < 6）
- `UnsupportedFormat { format }`（识别但不解析的格式，如 ArchiveBundle）
- `UnsupportedCompression { compression }`（flag 未注册）
- `Decompress { reason }`
- `Lz4SizeMismatch { expected, actual }`

---

## 8. 与其他文档的关系

| 文档 | 角色 |
|---|---|
| **本文档** | io 层**现状权威**（UnityFS + Registry + LoadedFile 树，已落地） |
| [ADR-0003](adr/0003-file-loading-and-decompression-layer.md) | io 层决策（含修订附录：已落地 vs 未实现） |
| [`object-layer.md`](object-layer.md) | 本层产出的 `LoadedFile::Leaf.bytes` 交给 SerializedFile / 对象解析 |
| [`environment-layer.md`](environment-layer.md) | 聚合层（`Loader` 消费 `LoadedFile` 树，跑 Phase A/B + Provider） |

冲突时以本文档为准。
