# ADR-0003: 文件加载与解压层 —— 递归再派发 + 三轴差异化扩展

> Status: Accepted（核心解压 registry + 递归再派发 + 多容器格式 + gzip/brotli + Loader + 错误隔离已落地；CN 加密未实现，见文末「修订」附录）
> Date: 2026-06-20
> Deciders: unity-asset-rs 项目维护者

## 背景 (Context)

ADR-0001 解决了"给定 `SerializedFile` 字节如何解析对象"。本决策解决它**之前**的一段：从用户输入（路径/字节/流）穿过 Unity 多种容器格式与压缩，交付一个或多个 `SerializedFile`（对象元数据，对象体留待 ADR-0001 惰性解析）。

现实复杂度（实测 AssetRipper / AssetStudio / UnityPy 三家）：

- **容器格式多样且可任意嵌套**：松散 `.assets`、`UnityFS`/`UnityWeb`/`UnityRaw` bundle、`UnityWebData`（WebGL）、外层 `gzip`/`brotli` 再包住上述任意一种；嵌套深度不固定（如 `brotli(UnityFS(.assets))`）。
- **容器只应拆包，不应理解内部语义**：bundle 解出原始字节 + 名字，不关心是 `.assets` 还是又一个 bundle——识别内容是派发器的事（AssetRipper `SchemeReader`/`IScheme` 的精髓）。
- **三个扩展轴开放程度不同**：
  - *文件格式* = 闭集（Unity 格式稳定，罕见新增）。
  - *解压 codec* = 开集。驱动证据：UnityPy issue [#308](https://github.com/K0lb3/UnityPy/issues/308) + PR [#309](https://github.com/K0lb3/UnityPy/pull/309)——明日方舟（Arknights）CN 服用一种 **LZ4 变体**（压缩数据 literal/match nibble 对调，修正后喂标准 LZ4）占了 flag 4/5（死槽位 LZHAM）；维护者 K0lb3 明确指出**自定义压缩可能劫持标准 flag**，故"未知 flag fallback"不够，必须能覆盖任意 flag 的 handler。
  - *CN 资源包加密*（`0x100` flag，Unity CN）= **目前仅此一例**，算法固定、密钥表从 bundle header 内嵌 key material 派生。
- **Rust 所有权约束**：AssetRipper 用 `SmartStream.CreatePartial` 零拷贝切片；Rust 下若 `&'a [u8]` + 生命周期贯穿递归派发，签名会爆炸。下游对象解析三家参考实现全部用 positioned reader（AssetStudio `ObjectReader.Reset()`、UnityPy `reader.position = byte_start`），交接应给 `Read + Seek`。

## 决策 (Decision)

采用 **AssetRipper 式递归再派发**构建文件加载层，并按三个扩展轴的开放程度**差异化**选用机制：

1. **递归再派发**：每种格式只"识别 + 解析自身结构 + 列出原始子字节"；驱动器看到 `Container` 就对每个子条目重新走 `sniff + parse`。同一套派发服务顶层与子条目，任意嵌套免费获得，无需每层手写递归。
2. **格式 = 闭集 → `enum FileFormat` + `sniff` if/else + `match parse`**（无 trait、无 `Vec`）。加格式 = 加一个枚举变体 + 一组 if/else（库维护者扩展）。
3. **解压 = 开集 → `Decompressor` trait + `DecompressorRegistry`**，按 `CompressionType(u16)` 键控，**用户可注册覆盖任意 flag**（含标准 LZ4=2），应对 Arknights 等私改；默认实现 LZMA/LZ4/LZ4HC（`lzma-rs`/`lz4_flex`）。
4. **CN 加密 = N=1 → 不抽象**，仅一个 concrete struct `ArchiveStorageDecryptor` + `CnKey` 配置项。UnityFS 解析器内 `if flags & 0x100 { ArchiveStorageDecryptor::new(reader, key)? }`，按块 `decrypt_block(data, index)` 后再调解压器。
5. **字节运输**：层内用 `ByteSource = bytes::Bytes`（Arc 共享、`slice()` 共享分配、跨递归无生命周期参数）；交接处 `SerializedFileMeta.reader() -> Cursor<&[u8]>`（`Read + Seek`），匹配下游 positioned-reader 范式，endian 由 meta 的 `big_endian` 标志驱动。
6. **同步、纯内存**（不做 spill、不做 async）。

## 备选方案 (Alternatives Considered)

| 方案 | 优点 | 缺点 | 主要影响维度 |
| --- | --- | --- | --- |
| A. 三轴统一用 trait registry（`Vec<Box<dyn FileScheme>>` + `Decompressor` + `BundleDecryptorFactory`） | 单一机制、对称、运行时可扩展 | 格式为闭集、加密仅 1 例，对二者用 trait registry 属过度抽象；`Box<dyn>` 有微开销 | 可维护性:-, 成本:- |
| B. AssetStudio 式单 `switch(FileType)` + 每层手写递归 | 实现直观、无 trait | 加格式/嵌套要改分发器并手写递归；扩展性差（三家实现的痛点） | 可维护性:-, 可扩展性:- |
| C. `&'a [u8]` + 生命周期贯穿递归 | 零运行时开销、真零拷贝 | `'a` 污染所有 trait/struct 签名；嵌套异构字节来源（mmap vs 解压产物）难统一生命周期 | 可维护性:-, 成本:- |
| D. 解压用固定 enum switch（不设 registry） | 无 trait 对象、编译期穷尽 | 违背 issue 308 结论——私改可劫持标准 flag，必须可覆盖任意槽位；Arknights 类场景无法接入 | 可扩展性:-, 可靠性:- |
| E. CN 加密用 `BundleDecryptorFactory` trait | 与解压对称、可扩展第二种算法 | N=1 且唯一变量是 key；预留 trait 属 YAGNI；真出现第二种时改动仅限 `parse_filestream_bundle` 一个函数 | 成本:-, 可维护性:- |

**否决理由**：

- **A**：闭集与单例轴不需要开放抽象；"机制数 = 轴数"反而更贴合实际，避免对稳定/单一事物过度设计。
- **B**：AssetRipper 专门用 `SchemeReader` 解决的就是"每层手写递归"问题；首版即采用正确抽象避免日后重构。
- **C**：`bytes::Bytes` 已用近乎零额外成本（共享分配、廉价 slice）换来无生命周期污染的 API；`&'a [u8]` 的签名爆炸不划算。
- **D**：直接违背 issue 308 的核心诉求与用户明确的"需支持自定义解压"要求。
- **E**：CN 加密目前唯一、算法固定、仅 key 可变；硬编码 + key 配置足够，出现第二例时局部重构成本低。

## 后果 (Consequences)

### 正面
- **扩展机制与开放程度对齐**：闭集（格式）用 enum、开集（解压）用 registry、单例（加密）用硬编码——无过设计也无欠设计。
- **任意嵌套免费获得**：递归再派发集中在驱动器一处，新格式无需重写递归代码。
- **解压可覆盖任意 flag**：registry 满足 Arknights 等"劫持标准 flag"的私改场景，用户接入只需 `with_decompressor(CompressionType(n), Box::new(MyCodec))`。
- **所有权干净**：`bytes::Bytes` 共享分配，公共 API 无生命周期参数。
- **交接清晰**：`SerializedFileMeta` 经 `Read + Seek` 精确对接 ADR-0001，与三家参考实现的对象解析范式一致。
- **错误隔离**：单文件失败捕获为 `Failure` 叶节点，不中断整体加载。

### 负面
- 三套不同机制（enum / registry / 硬编码）服务于三个轴，初次理解成本高于"统一 trait registry"。
- 格式集闭锁，使用方无法运行时新增自定义**文件格式**（仅解压/加密可扩展）。
- 纯内存、无 spill：超大 bundle（>2GB）有 OOM 风险。

### 中性 / 待观察
- 同步实现；pyo3 绑定（ADR-0002）在边界阻塞调用，暂不引入 `async`。
- CN 加密硬编码；若未来出现复用 `0x100` flag 的第二种加密算法，改动局限在 `parse_filestream_bundle` 一个函数（concrete struct → trait），成本低。
- scheme 派发用 `Vec` 顺序 vs 优先级数值：已定为 enum + if/else，无此问题。

## 质量权衡 (Quality Trade-offs)

| 维度 | 影响 | 说明 |
| --- | --- | --- |
| 可维护性 | ↑ | 扩展机制按轴开放程度对齐，新增格式/codec 各自独立、不改旧代码；递归派发集中一处。代价：三套机制增加初次理解成本 |
| 可靠性 | ↑ | 单文件错误隔离不传播；默认 codec 用成熟库（`lzma-rs`/`lz4_flex`）；magic 嗅探 + header 合法性双重校验；registry 让私改 codec 可被正确接入而非报错中断 |
| 性能 | ↑ | `bytes::Bytes` 共享切片近乎零拷贝；enum 分发无 vtable；registry 查表 O(1)；解压按需。代价：`Box<dyn Decompressor>` 微开销（调用频次低，可忽略） |
| 安全性 | → | 内存安全由 Rust 类型系统保证；无特权/网络/密码学特权操作；CN key 处理在调用方，库不内嵌密钥 |
| 成本 | ↑（一次性）/ →（长期） | enum + registry + concrete decryptor 为一次性投入；复用成熟解压库压低实现量；长期扩展成本低 |

## 合规与风险 (Compliance & Risks)

- **游戏私改格式**：Arknights 等私改 codec 由使用方自行实现并注册（`Decompressor` trait），库默认**不**内置任何游戏特定代码，仅提供扩展点；符合"通用工具 + 可插拔"定位。
- **CN 加密密钥**：`CnKey` 由调用方提供，库不内嵌、不持久化；解密仅用于本地已合法获取的资源包读取。
- **依赖风险**：`bytes` / `lzma-rs` / `lz4_flex` 均为成熟 MIT/Apache-2.0 crate；任一可经 registry 替换，无锁定。
- **超大 bundle OOM**：首版无 spill，>2GB bundle 有风险；缓解：文档标注限制，`SpillPolicy::AutoTempFile` 列入路线图。

## 实施备注 (Implementation Notes)

1. **crate**：`crates/io`，依赖 `bytes`/`lzma-rs`/`lz4_flex`；对上暴露 `Loader`/`Tree`/`SerializedFileMeta`，供 `crates/core`（ADR-0001）构建 `World`。
2. **格式派发**：`enum FileFormat { SerializedFile, FileStreamBundle, WebBundle, RawBundle, WebFile, GZip, Brotli, ArchiveBundle, Resource }` + `fn sniff(&[u8]) -> FileFormat`（if/else 链）+ `fn parse(fmt, src, name, ctx) -> Result<ParsedFile>`（match）。顺序同 AssetRipper 优先级（SerializedFile 先于各 bundle）。每个格式一个 `parse_xxx` 自由函数。
3. **递归再派发**：`Loader::load_recursive` 见 `sniff`→`parse`，若 `ParsedFile::Container` 则对 `children` 逐个递归。`Container` 只携带原始子字节 + 名字 + 可选 `engine_version`（被剥离版本的 SerializedFile 继承）。
4. **解压 registry**：`Decompressor` trait（`decompress(input, expected) -> Vec<u8>`，`expected` 为提示，LZ4 必需、gzip 可忽略）+ `DecompressorRegistry`（`HashMap<CompressionType, Box<dyn Decompressor>>`，`defaults()` 注册 LZMA/LZ4/LZ4HC，`register()` 覆盖任意槽位）。顶层 gz/brotli 也走同一 registry。bundle 逐块按 `block.compression` 查表。
5. **Arknights 接入示例**：用户实现 `AkLz4: Decompressor`（nibble 修正后调用标准 LZ4），`reg.register(CompressionType(4), Box::new(AkLz4))`。
6. **CN 加密**：concrete `struct ArchiveStorageDecryptor { index, substitute }` + `impl { fn new(reader, key) -> Self; fn decrypt_block(&mut self, block, index) }`。Loader 配置项 `cn_key: Option<CnKey>`；UnityFS 解析器内 `if flags & UsesAssetBundleEncryption { Some(ArchiveStorageDecryptor::new(reader, key)?) }`，逐块 `decrypt_block` 后调解压器。
7. **字节运输**：`pub type ByteSource = bytes::Bytes`；`SerializedFileMeta.bytes: ByteSource` + `fn reader(&self) -> Cursor<&[u8]>`。
8. **交接**：`Tree::serialized_files()` → `World::from_serialized_files(...)`（ADR-0001）接管对象解析与 `PPtr` 解引用。
9. **错误隔离**：`load_recursive` 捕获 `parse` 错误包成 `Failure` 叶节点（name/path/原因），不向上传播。
10. **验收基线**：用 UnityPy/AssetRipper 同批测试 bundle（UnityFS/UnityWeb/`brotli(.assets)`/CN 加密/Arknights 私改）做对照——`Tree::serialized_files()` 数量与对象元数据偏移表一致；自定义 `Decompressor` 注入能替换默认 codec 并成功解压。

## 与现有 ADR 的关系

- **ADR-0001**：本层是其上游。`SerializedFileMeta.objects`（`ObjectInfo` 偏移表）+ `types`（类型表/内嵌 typetree）即 ADR-0001 parser 输入；`World::from_serialized_files` 消费本层 `Tree`。
- **ADR-0002**：pyo3 绑定暴露的 `Environment.load()`（Python 侧入口）委托本层 `Loader`；版本断言用 `SerializedFileMeta.unity_version` 完成。

---

## 修订（2026-07-09，对齐实现）

本决策的核心——**解压 codec 开集 + registry 覆盖任意 flag**——已落地（`crates/unity-asset/src/decompress.rs` 的 `Registry` + `Decompressor` trait + 默认 LZMA/LZ4/LZ4HC/NONE）。但正文描述的完整「递归再派发 + 三轴差异化扩展」架构大部分**尚未实现**。权威现状见 [`docs/io-layer.md`](../io-layer.md)。

### 已落地

| 正文项 | 实际实现 |
|---|---|
| 解压开集 → `Decompressor` trait + registry | ✅ `Decompressor` trait + `Registry`（**命名以代码为准**，非正文的 `DecompressorRegistry`——已知命名漂移） |
| `CompressionType(pub u16)` open 枚举，可覆盖任意 flag | ✅ `CompressionType(pub u16)`，`register()` 覆盖任意槽位 |
| 默认 LZMA/LZ4/LZ4HC（`lzma-rs`/`lz4_flex`） | ✅ defaults() 注册 LZMA+LZ4+LZ4HC+NONE（flag 0-3） |
| 字节运输 `bytes::Bytes` | ✅ `Bytes` 共享切片、`slice()` 共享分配 |
| UnityFS 解析 | ✅ `crates/unity-asset/src/bundle.rs::parse`，format 6/7/8，大端 Cursor |
| Unity LZMA ≠ 标准 .lzma（UseProvided unpacked size） | ✅ 与 `crates/codegen/src/tpk/compression.rs` 同源 |
| Arknights LZ4 变体接入扩展点 | ✅ 用户自行实现 + `register(CompressionType(4/5), ...)`，库不内置游戏代码 |
| 递归再派发（Container → 逐子条目重新 sniff + parse） | ✅ `crates/unity-asset/src/format.rs::load_recursive`，`LoadedFile` 树（Container/Leaf） |
| `enum FileFormat` 闭集 + `sniff` if/else + `match parse` | ✅ `crates/unity-asset/src/format.rs`（`FileFormat` enum + `sniff` + `load_recursive`） |
| UnityWeb / UnityRaw / UnityWebData / gzip / brotli | ✅ `crates/unity-asset/src/containers/`（`rawweb.rs` + `webfile.rs` + `compressed.rs`） |
| `load` / `load_bytes` 返回 `LoadedFile`（树） | ✅ 原 `Bundle`（扁平 files）改为 `LoadedFile`（递归后的树），`Bundle` 降为 UnityFS 中间结果 |

### 未实现（范围收缩 / 推迟）

| 正文项 | 状态 |
|---|---|
| CN 加密（concrete `ArchiveStorageDecryptor` + `CnKey`） | ⬜ |
| `World` / `Tree` 聚合 + `.resS` 独立拷贝分离策略 | ⬜ io 层无聚合层；`.resS` 分离已由聚合层 `Environment` 落地（见 `docs/environment-layer.md`） |
| `SerializedFileMeta.reader() -> Cursor<&[u8]>` 交接 ADR-0001 | 实际交接方式不同：主 crate 直接对 `LoadedFile::Leaf.bytes` 调 `SerializedFile::parse`，见 `docs/object-layer.md` |

> 注：正文「实施备注」里的 `Loader`/`LoaderBuilder` 与「错误隔离 `Failure` 叶节点」两项**已落地**
> （分别在聚合层 `crates/unity-asset/src/loader.rs` 与 io 层 `LoadedFile::Failed`，见
> [`docs/environment-layer.md`](../environment-layer.md) 与 [`docs/io-layer.md`](../io-layer.md) §6），
> 此前本表误标 ⬜，已订正。

正文关于「解压开集用 registry、覆盖任意 flag」的核心论证（Arknights 劫持 flag 4/5）与实现一致，仍然成立。「递归再派发 + 多容器格式 + gzip/brotli + Loader + 错误隔离」已落地；CN 加密是后续路线图。
