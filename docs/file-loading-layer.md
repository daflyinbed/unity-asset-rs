# 文件加载层设计

> ⛔ **历史归档（2026-07-09）** —— 本文描述的 `World` / `Loader` / `Tree` / 递归再派发 /
> CN 加密 / UnityWeb/UnityRaw/gzip/brotli 等**大部分未实现**；io 层实际只做了 UnityFS +
> 解压 registry。权威现状见 [`io-layer.md`](io-layer.md)。本文保留仅供追溯。

> 范围：从用户输入（路径 / 字节 / 流）到交付 `World`（对象 + 资源），覆盖 io（容器格式 + 解压）与 core（对象解析）两段管线。
> 状态：设计稿（对应 ADR-0003 的实施细化）
> 上游决策：[ADR-0001](adr/0001-codegen-from-typetree-dumps.md)（对象解析）、[ADR-0003](adr/0003-file-loading-and-decompression-layer.md)（io 层架构决策）

---

## 1. 概述

本文档覆盖"从原始文件到 `World`"的整条管线，分 io（容器格式 + 解压，对应 ADR-0003）与 core（对象解析，对应 ADR-0001）两段。两者同处一个 crate（`crates/unity-asset`），各为 `pub(crate)` 模块；中间状态 `Tree` 不暴露给终端用户。

**输入**：文件路径 / `bytes::Bytes` / 流（目录、`.split0..N` 自动合并）。

**输出**（终端用户唯一可见的结果）：`World`，含
- `objects: Vec<UnityObject>` —— parse 出来的对象（owned）
- `by_path: HashMap<(i32, i64), u32>` —— PPtr 解引用索引
- `resources: HashMap<String, ByteSource>` —— `.resS` 流式 blob（纹理/音频/网格数据的 backing store，对象按 `(name, offset, size)` 引用）

**核心设计取舍**（详见 ADR-0003）：
- 文件格式 = 闭集 → `enum` + `if/else` 派发（库维护，不用 trait registry）
- 解压 codec = 开集 → `Decompressor` trait + registry（用户可覆盖任意 flag）
- CN 加密 = N=1 → 硬编码 concrete struct + key 配置（不抽象）
- 字节运输 = `bytes::Bytes`（Arc 共享切片，无生命周期污染）
- 同步、纯内存（无 spill、无 async）

---

## 2. 架构总览

```
输入 (path / Bytes)
  │
  ▼
Loader（驱动器，持有 decompressor registry + cn_key + 配置）
  │  ① io 模块：load_path_internal → Tree        [Tree 是 pub(crate)，不暴露]
  │     • sniff(buf) → FileFormat（if/else 链）
  │     • parse(fmt, src, ctx) → ParsedFile
  │     • Container 的子条目 → 递归 sniff + parse（同一套派发）
  │     • Resource node → 拷成独立 ByteSource（解耦 C）
  │     • SerializedFile node → 借 C 的 Arc 切片（不拷）
  │  ② core 模块：World::from_tree(tree)（by value，pub(crate)）
  │     • 遍历 serialized_files，逐对象 reader().seek(byte_start) → parse → arena owned
  │     • 收集 resources（已独立）→ World.resources
  │     • tree drop → SerializedFile 的 C 切片释放 → C 整块释放
  ▼
World（arena + resources，自洽）—— 唯一对终端用户暴露的结果类型
```

> io 与 core 合并在同一个 crate 内（`crates/unity-asset`），各为 `pub(crate)` 模块。`Tree`/`SerializedFileMeta` 等中间类型**不暴露给终端用户**——它们是 parse 管线的内部状态。终端用户只接触 `World` + 配置类型（`Loader`/`Decompressor`/`CnKey`）。

### 三轴差异化扩展

| 轴 | 开放程度 | 机制 | 谁扩展 |
|---|---|---|---|
| 文件格式 | 闭集 | `enum FileFormat` + `sniff` if/else + `match parse` | 库维护者 |
| 解压 codec | 开集 | `Decompressor` trait + `DecompressorRegistry`（覆盖任意 flag） | **用户** |
| CN 加密 | N=1 | concrete `ArchiveStorageDecryptor` + `CnKey` 配置 | 库内置 |

---

## 3. 核心抽象

### 3.1 字节缓冲：`ByteSource`

```rust
/// Arc 底层、clone 廉价、slice() 共享分配的字节缓冲。
/// 跨递归派发运输字节，无生命周期参数（避免 &'a [u8] 污染签名）。
pub type ByteSource = bytes::Bytes;
```

- 输入文件优先 mmap（`memmap2` → `Bytes::from_owner`）
- bundle 解压产物是新 `ByteSource`（独立 Arc 分配）
- node 按 `(offset, size)` 对解压 buffer `slice` → 共享分配的子条目

### 3.2 文件格式派发：`FileFormat` enum + `sniff` + `parse`

```rust
pub enum FileFormat {
    SerializedFile,
    FileStreamBundle,   // UnityFS（现代主流）
    WebBundle,          // UnityWeb
    RawBundle,          // UnityRaw
    WebFile,            // UnityWebData（WebGL）
    GZip,
    Brotli,
    ArchiveBundle,      // UnityArchive（占位）
    Resource,           // 兜底（.resS、原始 blob）
}

/// 顺序嗅探 magic / header 合法性。SerializedFile 先于各 bundle（避免误判）。
pub(crate) fn sniff(buf: &[u8]) -> FileFormat { /* if/else 链 */ }

pub(crate) fn parse(
    fmt: FileFormat,
    src: ByteSource,
    name: String,
    ctx: &mut ParseCtx,
) -> Result<ParsedFile> {
    match fmt {
        FileFormat::SerializedFile   => parse_serialized_file(src, name, ctx),
        FileFormat::FileStreamBundle => parse_filestream_bundle(src, name, ctx),
        FileFormat::GZip             => parse_wrapped(src, name, ctx, CompressionType::GZip),
        // ...
        FileFormat::Resource         => Ok(ParsedFile::Resource(RawResource { bytes: src, name })),
    }
}
```

每个格式一个 `parse_xxx` 自由函数，独立模块（`scheme/serialized_file.rs`、`scheme/filestream_bundle.rs`、…）。`sniff` 只读头部 magic，`parse` 解析自身结构。

### 3.3 递归再派发（驱动器统一执行）

scheme 只"识别 + 解析自身结构 + 列出原始子字节"；驱动器看到 `Container` 就对每个子条目**重新走 `sniff + parse`**：

```rust
fn load_recursive(&self, src: ByteSource, name: String) -> Result<TreeNode> {
    let fmt = sniff(src.as_ref());
    match parse(fmt, src, name, &mut ctx)? {
        ParsedFile::SerializedFile(sf) => Ok(TreeNode::Serialized(sf)),
        ParsedFile::Container(c) => {
            let mut kids = Vec::with_capacity(c.children.len());
            for child in c.children {
                kids.push(self.load_recursive(child.bytes, child.name)?);  // 重新 sniff+parse
            }
            Ok(TreeNode::Container(LoadedContainer { kind: c.kind, children: kids }))
        }
        ParsedFile::Resource(r) => {
            // ★ 持久数据：拷成独立分配，和 C 解耦（见 §4）
            let independent = ByteSource::from(r.bytes.to_vec());
            Ok(TreeNode::Resource(RawResource { bytes: independent, name: r.name }))
        }
        ParsedFile::Failed(f) => Ok(TreeNode::Failed(f)),
    }
}
```

同一套派发服务顶层与子条目，任意嵌套（如 `brotli(UnityFS(.assets))`）免费获得，无需每层手写递归。

### 3.4 `ParsedFile`：scheme 的产物

```rust
pub enum ParsedFile {
    SerializedFile(SerializedFileMeta),   // .assets —— 本层目标产物
    Container(ContainerMeta),             // bundle/web/gz/brotli —— 含原始子条目
    Resource(RawResource),                // 未识别兜底（.resS、纹理、音频 blob）
    Failed(Failure),                      // 解析错误，捕获后继续
}

pub struct ContainerMeta {
    pub kind: ContainerKind,                  // UnityFS / UnityWeb / UnityRaw / WebData / GZip / Brotli
    pub engine_version: Option<UnityVersion>, // 被剥离版本的 SerializedFile 继承
    pub children: Vec<ChildEntry>,            // 原始字节 + 名字，待驱动器再 dispatch
}
```

### 3.5 可插拔解压器：`Decompressor` + `DecompressorRegistry`

```rust
#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub struct CompressionType(pub u16);   // 开放：用户可注册任意槽位
impl CompressionType {
    pub const NONE:  Self = Self(0);
    pub const LZMA:  Self = Self(1);
    pub const LZ4:   Self = Self(2);
    pub const LZ4HC: Self = Self(3);
    // 4/5: 历史 LZHAM 死槽位，现被游戏用于自定义 codec（如 Arknights LZ4 变体）
}

/// 单个 codec，无状态。bundle 逐块调用；顶层 gz/brotli 也走它。
pub trait Decompressor: Send + Sync {
    /// `input`   一个压缩 blob（一个 StorageBlock，或一个外层包裹文件）
    /// `expected` 解压后大小提示：LZ4 必需，gzip/brotli 可忽略（传 0）
    fn decompress(&self, input: &[u8], expected: usize) -> Result<Vec<u8>>;
}

pub struct DecompressorRegistry { /* HashMap<CompressionType, Box<dyn Decompressor>> */ }
impl DecompressorRegistry {
    pub fn new() -> Self;                    // 空
    pub fn defaults() -> Self;               // LZMA + LZ4 + LZ4HC（lzma-rs / lz4_flex）
    /// 注册并返回旧值（若有）。可覆盖任意槽位，含标准 LZ4。
    pub fn register(&mut self, ct: CompressionType, d: Box<dyn Decompressor>)
        -> Option<Box<dyn Decompressor>>;
    pub fn get(&self, ct: CompressionType) -> Result<&dyn Decompressor>;
}
```

**设计依据**：UnityPy issue [#308](https://github.com/K0lb3/UnityPy/issues/308) + PR [#309](https://github.com/K0lb3/UnityPy/pull/309)。明日方舟（Arknights）CN 服用一种 LZ4 变体（literal/match nibble 对调），占 flag 4/5。维护者 K0lb3 指出**自定义压缩可能劫持标准 flag**，故"未知 flag fallback"不够，必须能覆盖任意 flag 的 handler。

**Arknights 接入示例**：
```rust
struct AkLz4;
impl Decompressor for AkLz4 {
    fn decompress(&self, input: &[u8], expected: usize) -> Result<Vec<u8>> {
        let fixed = swap_ak_nibbles(input);            // 修正 nibble
        DEFAULT_LZ4.decompress(&fixed, expected)       // 复用注册表内 LZ4
    }
}
// 注册到 flag 4 和 5
reg.register(CompressionType(4), Box::new(AkLz4));
reg.register(CompressionType(5), Box::new(AkLz4));
```

### 3.6 CN 加密：硬编码 concrete struct（N=1，不抽象）

CN 加密发生在 **UnityFS 解析器内部**，和解压交织：从 bundle header 读 key material 构造解密器，逐块解密后再解压。**不是独立一层**。

```rust
/// Unity CN 资源包加密 key（用户唯一需提供的输入）。
pub struct CnKey(Arc<[u8]>);
impl CnKey {
    pub fn from_bytes(key: impl Into<Box<[u8]>>) -> Self;
}

// 内部（非 public）：Unity CN 的 ArchiveStorageDecryptor 算法
struct ArchiveStorageDecryptor { index: [u8; 16], substitute: [u8; 16] }
impl ArchiveStorageDecryptor {
    fn new(reader: &mut (impl Read + Seek), key: &CnKey) -> Result<Self>;  // 从 header 读 key material
    fn decrypt_block(&mut self, block: &mut Vec<u8>, index: usize);         // 按块序号解密
}
```

UnityFS 解析流水线：
```rust
let mut dec = if flags & UsesAssetBundleEncryption {
    let key = ctx.cn_key.as_ref().ok_or_else(|| err!("bundle 加密但未设 cn_key"))?;
    Some(ArchiveStorageDecryptor::new(&mut reader, key)?)
} else { None };

// BlocksInfo 元数据块
let bi = reader.read(compressed_size);
let bi = if let Some(d) = dec.as_mut() { d.decrypt_block(bi, 0) } else { bi };
let bi = ctx.decompressors.get(meta_compression).decompress(&bi, uncompressed_size)?;

// 每个数据 StorageBlock
for (i, blk) in blocks_info.iter().enumerate() {
    let raw = reader.read(blk.compressed_size);
    let raw = if let Some(d) = dec.as_mut() { d.decrypt_block(raw, i) } else { raw };
    let raw = ctx.decompressors.get(blk.compression).decompress(&raw, blk.uncompressed_size)?;
    out.extend(raw);
}
```

**为什么不用 trait**：目前唯一一例、算法固定、仅 key 可变。硬编码 + key 配置足够。出现第二种加密时改动仅限 `parse_filestream_bundle` 一个函数（concrete struct → trait），成本低。

---

## 4. 内存生命周期（关键设计）

### 4.1 三阶段流转

```
阶段0 输入        ByteSource A = mmap(file)              [Arc ref=1]
  │  brotli parse 拿到 A，读压缩字节(Arc切片)，解压 ──┐
  ▼                                                  │ 解压分配
阶段1 解压#1      ByteSource B = decompress(...)      [Arc ref=1]
                  A 的临时切片丢弃 → A ref 归零 → A 释放
  │  bundle parse 拿到 B 切片，逐块读压缩字节，解压 ──┐
  ▼                                                  │ 解压分配
阶段2 解压#2      ByteSource C = concat(各块解压)     [Arc ref=1]
                  B 切片丢弃 → B 释放
  │  按 node slice C → 子条目 Vec<ByteSource>        [各为 C 的 Arc 切片]
  ▼  递归派发
阶段3 元数据      SerializedFileMeta.bytes = C 的 Arc 切片    [C ref 保持>0]
                  RawResource.bytes = ByteSource::from(slice.to_vec())  [★ 独立分配，和 C 解耦]
  │  reader() -> Cursor<&[u8]> 瞬时借给 ADR-0001
  ▼
阶段4 object→arena 见 §4.3
```

**关键点**：
- 解压**前**共享最开始的内存——但是 **Arc 切片**（`bytes::Bytes::slice`），不是 `&'a [u8]`，避免签名爆炸。
- 每次**解压分配新 Arc buffer**（B、C 各一块）。解压完旧切片丢弃 → 旧 buffer refcount 归零 → 自动释放。稳态下**只有最末级解压 buffer（C）存活**。
- `SerializedFileMeta.bytes` 是 C 的 Arc 切片，让 C 续命。

### 4.2 SerializedFile vs .resS 分离策略

bundle 解压后所有 block 拼成一块 C，每个 node（`CAB-xxx.assets`、`CAB-xxx.resS`）是 C 的 Arc 切片——**共享同一块分配**。问题：即使 SerializedFile parse 完、它的切片逻辑上"用完了"，**C 也释放不了**，因为 `.resS` 切片还引用着它。

**解法**：两种 node 生命周期不同，分别处理——

| node 类型 | 生命周期 | 处理 | 是否拷贝 |
|---|---|---|---|
| SerializedFile | parse 完即弃（瞬时） | 借 C 的 Arc 切片，直接 parse | **不拷** |
| .resS Resource | World 常驻（持久） | `ByteSource::from(slice.to_vec())` 拷成独立分配 | **拷一次** |

```rust
// 驱动器 dispatch 之后
match parse(fmt, node_slice, name, ctx)? {
    ParsedFile::SerializedFile(sf) => {
        // sf.bytes 是 C 的 Arc 切片 —— 不拷，直接 parse 用
        TreeNode::Serialized(sf)
    }
    ParsedFile::Resource(r) => {
        // 持久数据：拷成独立分配，和 C 解耦
        let independent = ByteSource::from(r.bytes.to_vec());
        TreeNode::Resource(RawResource { bytes: independent, name: r.name })
    }
    ...
}
```

**结果**：`World::from_tree` parse 完所有 SerializedFile 后，`tree` drop → SerializedFile 的 C 切片全丢弃 → C 的 Arc refcount 归零 → **C 整块释放**。World 只持有 arena（对象 owned）+ `.resS` 独立 ByteSource。

```
parse 前:   C [ SF字节 | .resS字节 ]                ← 一块
parse 中:   C [ SF字节 | .resS字节 ] + .resS独立拷贝  ← 短暂并存
parse 后:   C 释放（SF 切片丢弃、.resS 已独立）       ← 只剩 .resS 独立分配 + arena
```

**拷贝量**：只拷 `.resS` 那部分（持久的），SerializedFile 直接借 C parse 不拷。

**边界情况**：`.resS` 也可能是外部独立文件（不在 bundle 里），那种情况它本来就不在 C 里（自己一份 mmap/Vec），天然独立、不用拷。拷贝解耦只对 bundle 内嵌的 `.resS` node 生效。

### 4.3 object parse → arena（交接 ADR-0001）

`World::from_tree(tree: Tree)` **按值接收**——编译器强制 tree 在调用后不可访问，SerializedFile 的 ByteSource 在内部 eager parse 完就随 tree 析构释放。用户根本没法"还拿着 ByteSource"。

```rust
impl World {
    /// pub(crate)：仅 crate 内 io→core 交接，不暴露给终端用户。
    /// 用户入口是 Loader::load()，内部调这个。
    pub(crate) fn from_tree(tree: Tree) -> Result<World> {
        let mut objects = Vec::new();
        let mut by_path = HashMap::new();
        for sf in tree.serialized_files() {
            let mut r = sf.reader();                    // Cursor<&[u8]>，瞬时借 C 切片
            for (i, obj) in sf.objects.iter().enumerate() {
                r.seek(SeekFrom::Start(obj.byte_start))?;
                let parsed = UnityObject::parse(&mut r, &sf.unity_version, &resources)?;
                by_path.insert((file_id, obj.path_id), objects.len() as u32);
                objects.push(parsed);                   // owned，拷进 Vec
            }
        }
        let resources = tree.collect_resources();       // .resS 已独立，挪进 World
        drop(tree);                                     // SerializedFile C 切片释放 → C 释放
        Ok(World { objects, by_path, resources })
    }
}
```

**arena 形态**：首版用 `Vec<UnityObject>`（每对象 owned 堆分配），不用 bumpalo。理由：bumpalo 的 `'arena` 生命周期 + "World 同时拥有 arena 和指向 arena 的引用"是经典自引用难题；首版避免 `'a` 污染 API、避免 unsafe。Unity 对象量级（千~万）下 Vec 完全够，bumpalo 的优势要在百万级才明显。等 profile 数据再上 bumpalo。

**对象字段归属**：
- 小标量/字符串（int, float, name, Vector3…）：从 SerializedFile 字节拷出 → owned（`i32`/`String`/…）
- 大 blob（texture `image_data`、mesh 顶点、audio）：通过 `StreamingInfo { offset, size, path }` 指向 `World.resources[path]` 的 ByteSource，**零拷贝引用**（不拷进对象，共享 .resS backing）

### 4.4 峰值内存

单 bundle 解压瞬间：压缩输入切片 + 解压输出 C **同时存在**（`compressed + uncompressed`），不可避开（解压算法需要）。AssetRipper 的 fast-path（单块未压缩走 partial stream 零拷贝）能避开无压缩场景，列为后续优化。与"纯 in-memory、不 spill"一致——首版接受这个峰值。

---

## 5. 公共 API

终端用户可见的类型**只有**：`World`、`UnityObject`、`Loader`/`LoaderBuilder`、`Decompressor`/`DecompressorRegistry`/`CompressionType`、`CnKey`、`LoadError`。**没有任何持有 `ByteSource` 的类型泄露**——`Tree`/`SerializedFileMeta`/`ObjectInfo`/`RawResource` 等全部 `pub(crate)`，是 parse 管线的内部状态。

```rust
// crates/unity-asset/src/lib.rs
pub use bytes::Bytes as ByteSource;   // 仅用于 Decompressor 实现内部，不在公共结果类型里出现
pub use error::{LoadError, Result};

// ═══════════ 入口（唯一面向终端用户） ═══════════

/// 便捷：默认配置加载，直接得到 World。
/// 内部走 io→Tree→core→World，Tree 是局部变量、parse 完随返回释放。
pub fn load(path: impl AsRef<Path>) -> Result<World> {
    Loader::new().load(path)
}

impl Loader {
    /// io + parse 全流程，返回 World。Tree 是局部中间体，不暴露。
    pub fn load(&self, path: impl AsRef<Path>) -> Result<World> {
        let tree = self.load_path_internal(path)?;   // pub(crate)
        World::from_tree(tree)                        // pub(crate)，消耗 Tree
    }
    pub fn load_bytes(&self, src: ByteSource, name: impl Into<String>) -> Result<World>;

    pub fn new() -> Self;            // 默认 codec（LZMA / LZ4 / LZ4HC），无加密
    pub fn builder() -> LoaderBuilder;
}
impl Default for Loader { fn default() -> Self { Self::new() } }

// ═══════════ 配置 ═══════════

pub struct LoaderBuilder { /* .. */ }
impl LoaderBuilder {
    /// 覆盖/新增单个 codec 槽位（可覆盖标准 LZ4=2，或注册 4/5 等私改槽位）。
    pub fn decompressor(mut self, ct: CompressionType, d: Box<dyn Decompressor>) -> Self;
    /// 整套替换 registry。
    pub fn decompressors(mut self, reg: DecompressorRegistry) -> Self;
    /// Unity CN 资源包加密 key（仅此一例加密，不抽象）。
    pub fn cn_key(mut self, key: CnKey) -> Self;
    /// SerializedFile 版本被剥离（"0.0.0"）时的兜底版本。
    pub fn fallback_version(mut self, v: UnityVersion) -> Self;
    pub fn build(self) -> Loader;
    /// 便捷：build + load 一步。
    pub fn load(self, path: impl AsRef<Path>) -> Result<World>;
}

// ═══════════ 解压扩展（用户侧，开集） ═══════════

pub trait Decompressor: Send + Sync {
    fn decompress(&self, input: &[u8], expected: usize) -> Result<Vec<u8>>;
}

pub struct DecompressorRegistry { /* HashMap<CompressionType, Box<dyn Decompressor>> */ }
impl DecompressorRegistry {
    pub fn new() -> Self;
    pub fn defaults() -> Self;        // LZMA + LZ4 + LZ4HC
    pub fn register(&mut self, ct: CompressionType, d: Box<dyn Decompressor>)
        -> Option<Box<dyn Decompressor>>;
    pub fn get(&self, ct: CompressionType) -> Result<&dyn Decompressor>;
}

// ═══════════ CN 加密（N=1，concrete） ═══════════

pub struct CnKey(Arc<[u8]>);
impl CnKey {
    pub fn from_bytes(key: impl Into<Box<[u8]>>) -> Self;
}

// ═══════════ World（唯一的结果类型） ═══════════

pub struct World {
    pub objects: Vec<UnityObject>,               // parse 出来的对象（owned，含字段 + 引用描述符）
    pub by_path: HashMap<(i32, i64), u32>,       // PPtr 解引用 → objects 下标
    pub resources: HashMap<String, ByteSource>,  // .resS 流式 blob backing（对象按 StreamingInfo 引用）
}

impl World {
    /// 取 StreamingInfo 指向的 blob（零拷贝：返回 Arc 切片，共享 resources backing）。
    /// 对象不自持 blob 字节 —— Texture2D/AudioClip/Mesh 的流式数据走这里取。
    pub fn streaming_data(&self, info: &StreamingInfo) -> Result<ByteSource> {
        let src = self.resources.get(&info.path)
            .ok_or(LoadError::ResourceNotFound(info.path.clone()))?;
        Ok(src.slice(info.offset..info.offset + info.size))
    }

    /// PPtr 解引用 → 被引用对象。返回 &UnityObject，用户 match 到具体变体。
    /// （PPtr 无法 parse 当场解析：对象间循环引用 + Vec 下标未定，只能运行时查 by_path）
    pub fn resolve(&self, pptr: &PPtr) -> Option<&UnityObject> {
        let idx = *self.by_path.get(&(pptr.file_id, pptr.path_id))?;
        self.objects.get(idx as usize)
    }

    /// 按 (file_id, path_id) 直接取对象。
    pub fn get(&self, file_id: i32, path_id: i64) -> Option<&UnityObject> {
        let idx = *self.by_path.get(&(file_id, path_id))?;
        self.objects.get(idx as usize)
    }

    /// typed 便捷方法（如 resolve_texture2d / resolve_material）由 codegen 按 class 生成，
    /// 内部调 resolve + match 变体。手写 match 的语法糖，非必需。
}

// ═══════════ 错误 ═══════════

#[derive(Debug, thiserror::Error)]
pub enum LoadError {
    #[error("io: {0}")]                              Io(#[from] std::io::Error),
    #[error("unsupported compression {0:?} (未注册；可用 LoaderBuilder::decompressor 注册)")]
    UnsupportedCompression(CompressionType),
    #[error("bundle 加密（flag 0x100）但未设 cn_key；用 LoaderBuilder::cn_key 提供")]
    MissingCnKey,
    #[error("格式解析失败 ({kind}): {reason}")]       Parse { kind: ContainerKind, reason: String },
    #[error("资源文件未加载: {0}（StreamingInfo 指向的 .resS 缺失）")]
    ResourceNotFound(String),
    #[error("路径不存在: {0}")]                       NotFound(PathBuf),
}
```

### 内部类型（`pub(crate)`，不暴露给终端用户）

下列类型仅 crate 内 io→core 交接用，`pub(crate)` 可见：

- `Tree` / `TreeNode` / `LoadedContainer` / `ContainerKind` —— io 产出的中间树
- `SerializedFileMeta` / `ObjectInfo` / `SerializedType` / `FileIdentifier` —— SerializedFile 元数据（交接给 core parser）
- `RawResource` / `Failure` —— 资源 blob 与错误叶节点
- `Loader::load_path_internal` / `World::from_tree` —— 内部交接函数

这些类型的字段对 crate 内全公开（便于 io/core 模块互相读），但对 crate 外不可见。**终端用户永远拿不到 `ByteSource` 作为 parse 产物**——`World.resources` 的 `ByteSource` 是对象按 `StreamingInfo` 引用的 backing，用户通过 `world.streaming_data(info)` 取一个 Arc 切片（零拷贝共享），不直接持有 resources 句柄。`StreamingInfo` 与 `PPtr` 本身是 public（它们是生成对象 struct 的字段类型，用户需要把它们传给 `world.streaming_data` / `world.resolve`）。

---

## 6. 用法示例

### 6.1 最简

```rust
let world = unity_asset::load("game/level1.bundle")?;
for obj in &world.objects {
    if let UnityObject::Texture2D(t) = obj {
        println!("{}: {}x{}", t.m_Name, t.m_Width, t.m_Height);
    }
}
```

### 6.2 自定义解压（Arknights LZ4 变体，占 flag 4/5）

```rust
struct AkLz4;
impl Decompressor for AkLz4 {
    fn decompress(&self, input: &[u8], expected: usize) -> Result<Vec<u8>> {
        let fixed = swap_ak_nibbles(input);
        DEFAULT_LZ4.decompress(&fixed, expected)
    }
}

let world = Loader::builder()
    .decompressor(CompressionType(4), Box::new(AkLz4))
    .decompressor(CompressionType(5), Box::new(AkLz4))
    .load("arknights/ab.bundle")?;   // builder.load = build + load，直接得 World
```

### 6.3 CN 加密

```rust
let world = Loader::builder()
    .cn_key(CnKey::from_bytes(b"...\0"))
    .load("cn/game.bundle")?;
```

### 6.4 取流式 blob + 解引用 PPtr

对象不自持 blob 字节、也不自持被引用对象 —— 都经 `World` 取：

```rust
let world = load("file")?;
for obj in &world.objects {
    if let UnityObject::Texture2D(tex) = obj {
        // blob：StreamingInfo 描述符 → world.streaming_data 取 Arc 切片（零拷贝）
        let pixels = if !tex.image_data.is_empty() {
            &tex.image_data          // 内联（非流式），对象自带
        } else if let Some(info) = &tex.m_StreamData {
            &world.streaming_data(info)?   // 流式：从 .resS 取，Arc 共享 backing
        } else { continue };
        decode_png(pixels, tex.m_Width, tex.m_Height, tex.m_TextureFormat)?;
    }

    if let UnityObject::MeshRenderer(r) = obj {
        // PPtr 解引用：Material 在别的对象里
        if let Some(UnityObject::Material(mat)) = r.m_Materials.first().and_then(|p| world.resolve(p)) {
            println!("mesh uses material {}", mat.m_Name);
        }
    }
}
```

> 注：`tex.image_data` vs `m_StreamData` 的内联/流式分支判断较繁琐，codegen 可按 class 生成便捷方法（如 `world.texture_data(&tex)`）统一两条路径。属 core/codegen 层便利，非本层必需。

---

## 7. 与现有 ADR 的关系

- **ADR-0001**（对象解析）：io 模块产出的 `SerializedFileMeta.objects`（`ObjectInfo` 偏移表）+ `types`（类型表/内嵌 typetree）是 core 模块 parser 的输入；core 内部 `World::from_tree`（`pub(crate)`）消费 io 的 `Tree`，eager parse 进 `Vec<UnityObject>`。两者同 crate，交接类型 `pub(crate)`。
- **ADR-0002**（pyo3 绑定）：Python 侧 `Environment.load()` 委托 `unity_asset::load()`；版本断言用 `SerializedFileMeta.unity_version`（内部可见）。
- **ADR-0003**（本层架构决策）：本文档是其细化实施设计。

---

## 8. crate 拆分

| crate | 职责 | 模块 / 关键类型 | 依赖 |
|---|---|---|---|
| `crates/unity-asset` | **io + core 合一**：文件加载 + 对象解析 + `World` | `io` 模块（`Loader`/`Tree`/`SerializedFileMeta`/`Decompressor`，`pub(crate)`）<br>`core` 模块（`World`/`UnityObject`/parser，对外 `pub`） | `bytes`、`memmap2`、`lzma-rs`、`lz4_flex`、`crates/generated` |
| `crates/generated` | **只装 codegen 产出的 Rust 代码**，独立编译缓存 | `UnityObject` enum、各 class 的 `struct` + `parse`（~2 万行，提交进仓库） | 无（纯生成代码） |
| `crates/codegen` | **库维护者 CLI**：TPK → Rust（写 `crates/generated/src/`）。只读 TPK，生成 struct + `parse` impl，不接触任何 .ab | 独立 bin | TPK |
| `crates/pyo3` | Python 绑定（ADR-0002）；含 **下游开发者 `gen_pyi` 子模块**：TPK + .ab → `.pyi`（为具体项目的 MonoBehaviour 脚本生成存根） | pyclass + 继承投影；`gen_pyi` 加载 .ab 抽取 MonoScript | `crates/unity-asset`、`pyo3`；`gen_pyi` 可能额外需要 il2cpp/dotnet 元数据 |

**为什么 io 和 core 合一**：`Tree` 是 parse 管线的内部中间状态，不暴露给终端用户；分两个 crate 会强制把 `Tree`/`SerializedFileMeta` 设成跨 crate `pub`，反而泄露内部类型。合并后它们是 `pub(crate)`，io→core 在同一 crate 内交接，零泄露。终端用户只看到 `crates/unity-asset` 的公共 API（§5）。

**为什么 `generated` 与 codegen CLI 分离**：
- `crates/generated` 只含生成代码、无逻辑，**独立编译一次后 cargo 缓存其 artifact**。改 `crates/unity-asset` 的手写代码时不必重编这 ~2 万行生成代码——这是拆分的核心收益。
- `crates/codegen` 是离线工具（读 TPK、跑生成），**不参与主 crate 的依赖图**——TPK 升级或生成逻辑改动时手动/CI 跑一次，把产物提交进 `crates/generated`。它失败或改动不阻塞 `crates/unity-asset` 的构建。
- `crates/unity-asset` 通过 `crates/generated` 依赖拿到 `UnityObject` 等类型，无运行时对 codegen CLI 的依赖。

**为什么 Rust codegen（`crates/codegen`）与 pyi codegen（`crates/pyo3::gen_pyi`）不在同一个 crate**：两者都"读 TPK"，但复用率低（真正语言无关的核心——TPK 加载 + 节点遍历 + 拓扑排序——约 150 行，其余各异），且关键维度的分歧足以抵消合并收益：

| 维度 | `crates/codegen`（Rust） | `crates/pyo3::gen_pyi`（.pyi） |
|---|---|---|
| 输入 | **仅 TPK** | **TPK + .ab**（.ab 提供 MonoBehaviour 脚本信息，TPK 不含） |
| 输出语义 | 可执行解析代码（struct + `impl parse`，字节级读取） | 纯声明式类型存根（仅字段名 + Python 类型，不执行） |
| IR 颗粒度 | 每版本字节级布局（byte_size / align / 版本条件分支） | 逻辑字段（名 + 类型 + optional/required） |
| 依赖 | 轻（仅需 TPK 解析） | 重（需 `crates/unity-asset` 加载 .ab；MonoBehaviour 字段还要 il2cpp/dotnet 元数据） |
| 触发频率 | TPK 升级时（季度级，库维护者） | 每个项目一次（下游开发者） |
| 产物去向 | 提交进 `crates/generated/` | 跟随 Python 包，或用户现场生成 |

合并会让 Rust 生成器被迫拉进整套 `unity-asset` io/parse 依赖（它完全不需要 .ab），还会形成 `codegen → unity-asset → generated` 的循环嫌疑；两个 IR 颗粒度差太大，强行共用要么对 pyi 过度规格化、要么对 Rust 信息不足。TPK 加载逻辑本身 `unity-asset` 运行时也要用（TPK 兜底解析），各生成器直接复用即可，不必再抽独立的共享 crate。

生成流程：
- **Rust**：`crates/codegen`（CLI）读 TPK → 写 `crates/generated/src/` → 提交 → `crates/unity-asset` 消费。
- **.pyi**：`crates/pyo3::gen_pyi` 读 TPK + 用户提供的 .ab → 写 `.pyi`（随 Python 包发布或用户现场生成）→ 下游类型检查器消费。

---

## 9. 默认 FileFormat 嗅探顺序

同 AssetRipper 优先级（前缀嗅探，越易误判的越靠前）：

| # | Scheme | 嗅探 |
|---|---|---|
| 1 | SerializedFile | header 结构合法性（metadataSize/fileSize/generation） |
| 2 | GZip | magic `1f 8b` |
| 3 | Brotli | 0x20 处 `brotli` |
| 4 | WebFile | `UnityWebData1.0` / `TuanjieWebData` |
| 5 | ArchiveBundle | `UnityArchive`（占位，抛 NotImplemented） |
| 6 | WebBundle | `UnityWeb` |
| 7 | RawBundle | `UnityRaw` |
| 8 | FileStreamBundle | `UnityFS`（现代主流） |

全不命中 → `Resource` 兜底（保留字节）。

---

## 10. 待定 / 后续

| 项 | 状态 | 说明 |
|---|---|---|
| `SpillPolicy::AutoTempFile` | 路线图 | 首版纯内存，>2GB bundle 有 OOM 风险；大 bundle 解压溢出临时文件 |
| on-demand 逐块解压（AssetRipper `BundleFileBlockReader` 式） | 路线图 | 不 concat 整块 C，按需解压+缓存上一块，峰值压到 ~1× + 几块；但要换掉 `Cursor<&[u8]>` 为跨块 reader |
| `bumpalo` arena | 待 profile | 首版 `Vec<UnityObject>`；若 profile 显示分配是瓶颈再上 bumpalo（需解决自引用：调用方持有 arena + `ouroboros`，或 `World<'a>` 暴露生命周期） |
| `async` 版本 | 暂不做 | 首版同步；pyo3 边界阻塞调用 |
| Zstd 解压 | 待定 | AssetRipper 按 magic `28 B5 2F FD` 嗅探（复用 LZ4 槽位）；若遇到再注册 |

---

## 11. 验收基线

用 UnityPy/AssetRipper 同批测试 bundle 做对照：

- **格式覆盖**：`UnityFS` / `UnityWeb` / `UnityRaw` / `UnityWebData` / `brotli(.assets)` / `gzip(bundle)` 均能正确 `load()` 得到 `World`
- **对象元数据**：内部 `Tree::serialized_files()` 数量、每个 `SerializedFileMeta.objects` 偏移表与 UnityPy 一致（用 crate 内单元测试验证，不对外暴露）
- **自定义解压**：注入 `AkLz4` Decompressor 能替换默认 codec 并成功解压 Arknights 私改 bundle
- **CN 加密**：设 `cn_key` 后能加载 Unity CN 加密 bundle
- **内存**：`World::from_tree` 后，原 bundle 的解压 buffer C 已释放（用 `valgrind`/`dhat` 验证峰值回落）；用户拿到的 `World` 不间接 pin 住 SerializedFile 的字节
- **错误隔离**：单个损坏文件不中断整体加载，记录为 `Failure` 叶节点（内部）
