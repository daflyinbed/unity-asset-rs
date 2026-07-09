# 环境聚合层（Environment 层）

> 范围：从 io 层的 `Bundle` 构建 `Environment`——eager parse 全部 SerializedFile 的对象，
> 聚合 `.resS` 流数据，提供 PPtr 解引用与 streaming_data 取数。
> 状态：**本文档描述已落地的实现**（与 `src/environment.rs` + `src/names.rs` 一致）。
> 上游：[`io-layer.md`](io-layer.md)（本层输入 `Bundle`）、
> [`object-layer.md`](object-layer.md)（对象 dispatch `parse_object`）、
> [`codegen.md`](codegen.md)（typed struct `UnityObject`）。
> 参考：AssetRipper `GameBundle` / `AssetCollection` / `ResourceFile` / `SpecialFileNames`，
> 详见 [`../docs/ress-file-handling.md`](../../docs/ress-file-handling.md)。

---

## 1. 定位与术语对齐

Environment 层是对象解析层之上的**聚合层**：io 层交付 `Bundle`（解压后的内部文件 +
字节），对象解析层交付 `parse_object`（单对象 dispatch）；本层把它们组合成一张**可
解引用的对象图**——eager parse 全部对象、构建 PPtr 依赖表、聚合 `.resS` backing
store。

命名与设计参照 **AssetRipper**：

| AssetRipper（C#） | 本 crate（Rust） | 说明 |
|---|---|---|
| `GameBundle` | `Environment` | 顶层聚合（多 bundle） |
| `GameBundle.FromPaths` | `EnvironmentBuilder` | 累积多 bundle 后 `build()` |
| `IDependencyProvider` | `DependencyProvider` | 外部引用懒加载扩展点（no-op 默认） |
| `IResourceProvider` | `ResourceProvider` | `.resS` 磁盘现取扩展点（no-op 默认） |
| `AssetCollection` | `AssetCollection` | 一个 SerializedFile 解析后的逻辑分组 |
| `AssetCollection.Dependencies` | `AssetCollection.dependencies` | `[0] = self` 约定，PPtr.FileID 直接索引 |
| `SerializedAssetCollection.InitializeDependencyList` | 两阶段构建的 Phase B | 按 name 解析 externals |
| `ResourceFile` | `ResourceFile` | `.resS` / `.resource` 流数据 |
| `SpecialFileNames.FixFileIdentifier` | `fix_file_identifier` | 统一名称归一化 |
| `FileBase.NameFixed` | `name_fixed` | 归一化后的名字，匹配双方都用它 |
| `StreamingInfoExtensions.GetContent` | `streaming_data` | 空路径→`None`（与 AssetRipper 一致） |

**跨 bundle 聚合已落地**。bundle 内 SerializedFile 间互引 + `.resS` 流数据在本层解析；
多个 bundle 一起加载（[`EnvironmentBuilder`] 的 `path` / `scan_dir`）时，跨 bundle 引用也
能解析——Phase B 全局扫描 `collections`，
跨 bundle 的前向/后向 PPtr 引用都能命中。详见 [§13 跨 bundle Environment](#13-跨-bundle-environment)。

---

## 2. 核心数据结构

```rust
pub struct Environment {
    /// 跨全部 AssetCollection eager parse 成功的对象。
    objects: Vec<EnvironmentObject>,
    /// 每个 SerializedFile 的逻辑分组（含依赖解析表）。
    collections: Vec<AssetCollection>,
    /// `.resS` / `.resource` 流数据。
    resources: Vec<ResourceFile>,
    /// 解析失败记录（错误隔离，不中断整体）。
    failures: Vec<ParseFailure>,
}

pub struct EnvironmentObject {
    pub collection: usize,          // 所属 AssetCollection 索引
    pub path_id: i64,
    pub class_id: i32,
    pub object: ParsedObject,       // Typed(UnityObject) | Dynamic(Value)
}

pub struct AssetCollection {
    pub name: String,               // 原始 bundle node 名（如 "CAB-xxxx.assets"）
    pub name_fixed: String,         // fix_file_identifier(name)
    pub unity_version: UnityVersion,
    pub endian: Endian,
    /// 依赖列表。[0] = 自身索引；[1..] = 解析出的外部 collection 索引。
    /// PPtr.FileID 直接索引此列表（0 = 自身）——与 AssetRipper
    /// `AssetCollection.Dependencies` 约定一致。`None` = 未解析的跨 bundle 引用。
    pub dependencies: Vec<Option<usize>>,
    /// path_id → `Environment.objects` 索引。
    pub assets: HashMap<i64, usize>,
}

pub struct ResourceFile {
    pub name: String,
    pub name_fixed: String,
    pub bytes: Bytes,
}

pub struct ParseFailure {
    pub collection: usize,
    pub path_id: i64,
    pub class_id: i32,
    pub error: Error,
}
```

**设计取舍**：
- **对象扁平 `Vec` + 每 collection 的 `assets` 索引**：AssetRipper 把 assets 存在
  collection 内（`Dict<path_id, asset>`）；本层用扁平 `objects` 便于全局迭代（等价于
  `FetchAssetsInHierarchy`），同时每 collection 持有 `assets` 映射服务于 PPtr 解引用
  （`collection → assets[path_id]`）。
- **不用 bumpalo arena**：`UnityObject` 内部全是 `Vec`/`String`（已堆分配），arena 仅省
  enum 外壳分配（微乎其微）；且 `Environment { arena, &arena[..] }` 自引用需 unsafe/Pin，
  得不偿失。性能瓶颈在解析而非分配。列路线图，profiling 驱动后再说。

---

## 3. 名称归一化：`fix_file_identifier`

对齐 AssetRipper `SpecialFileNames.FixFileIdentifier`。collection、resource、
FileIdentifier.path **三方**的 `name_fixed` 都用同一个函数，精确相等比较——不做
UnityPy 的 4 候选名猜测。

```rust
/// 对齐 SpecialFileNames.FixFileIdentifier：
/// lowercase → strip "library/" / "resources/" → strip "archive:/" → basename
fn fix_file_identifier(name: &str) -> String {
    let s = name.to_ascii_lowercase();
    let s = s.strip_prefix("library/").or_else(|| s.strip_prefix("resources/")).unwrap_or(s);
    if let Some(rest) = s.strip_prefix("archive:/") {
        basename(rest)     // 取最后一段路径分量
    } else {
        s
    }
}
```

AssetRipper `FileBase.Name` setter 在赋值时自动算 `NameFixed`；本层在构建 collection /
resource / 解析 FileIdentifier 时统一调 `fix_file_identifier`。

---

## 4. 前置改动：捕获 externals

PPtr 跨文件解析的前提是 `SerializedFile` 不再丢弃 externals。当前
`src/serialized_file.rs:182-193` 把 externals 全部 `let _ =` 丢弃——必须先捕获。

```rust
/// 对齐 AssetRipper FileIdentifier。
pub struct FileIdentifier {
    pub path_name_origin: String,   // 原始路径（磁盘相对路径）
    pub path_name: String,          // fix_file_identifier(path_name_origin)
    pub guid: [u8; 16],
    pub type_: i32,                 // AssetType: 0 = Normal(按 path_name 匹配)，1 = Meta(按 Guid)
}
```

`SerializedFile` 新增字段 `pub externals: Vec<FileIdentifier>`；read_externals 块改为
实际填充（不再 `let _ =`）。向后兼容（新增字段）。

PPtr 的 `file_id` 含义（对齐 `AssetCollection.Dependencies`）：
- `file_id == 0` → `dependencies[0]` = 自身 collection。
- `file_id > 0` → `dependencies[file_id]` = 第 `file_id` 个 external 解析出的 collection。
  externals 列表的第 `i` 项对应 `dependencies[i + 1]`（AssetRipper
  `InitializeDependencyList` 用 `SetDependency(i + 1, dependency)`）。

---

## 5. 两阶段构建

对齐 AssetRipper：先读全部文件（`FromSerializedFile` + `ReadData`），再统一初始化依赖
列表（`InitializeAllDependencyLists`）——依赖解析要按名字查已存在的 collections，必须
等 Phase A 全部完成。

### Phase A：读全部文件 + 对象 + 资源

```
Bundle.files 遍历：
  ├─ 后缀 .resS / .ress / .resource（大小写无关）→ ResourceFile { name_fixed, bytes }
  │     bytes 用 Bytes::copy_from_slice detach——断开与 blob Arc 的共享，
  │     使 SerializedFile 解析完毕后整个解压 blob 可释放（见 docs/memory-notes.md）
  └─ 其它 → SerializedFile::parse(bytes)
        ├─ 失败 → ParseFailure（file-level），continue
        └─ 成功 → AssetCollection { name_fixed, dependencies: [Some(self)], unity_version, endian }
              └─ eager parse 每个 object（parse_object）：
                    ├─ Ok → EnvironmentObject，assets[path_id] = idx
                    └─ Err → ParseFailure（object-level），continue
```

### Phase B：初始化依赖列表

```
对每个 AssetCollection i：
  sf = 对应 SerializedFile（externals 已在 Phase A 读出）
  for (j, identifier) in sf.externals.enumerate():
    target = resolve_collection(identifier.path_name)
           = collections 里 name_fixed == identifier.path_name 的索引
    collection_i.dependencies.push(Some(target))   // 未找到 → None（跨 bundle）
```

`resolve_collection`：单 bundle 范围内按 `name_fixed` 精确匹配（对齐
`Bundle.ResolveCollection` 的 `TryResolveFromCollections`）。

---

## 6. PPtr 解引用

对齐 `AssetCollection.TryGetAsset(fileIndex, pathID)` → `Dependencies[fileIndex]` →
`TryGetAsset(pathID)`。

```rust
impl Environment {
    pub fn resolve(
        &self,
        source_collection: usize,
        file_id: i32,
        path_id: i64,
    ) -> Option<&EnvironmentObject> {
        let col = self.collections.get(source_collection)?;
        let target_idx = *col.dependencies.get(file_id as usize)?.as_ref()?;
        let target = self.collections.get(target_idx)?;
        let &obj_idx = target.assets.get(&path_id)?;
        self.objects.get(obj_idx)
    }

    /// 便捷：从源对象解析（自动取 source.collection）。
    pub fn resolve_from(
        &self,
        source: &EnvironmentObject,
        pptr: impl PPtrLike,
    ) -> Option<&EnvironmentObject> {
        self.resolve(source.collection, pptr.file_id(), pptr.path_id())
    }
}

/// typed PPtr（unity-core）与 dynamic PPtrValue（typetree）统一。
pub trait PPtrLike {
    fn file_id(&self) -> i32;
    fn path_id(&self) -> i64;
}
```

`file_id == 0` 天然落到 `dependencies[0] = Some(self)`，无需特判。

---

## 7. streaming_data

对齐 `StreamingInfoExtensions.GetContent` / `StreamedResourceExtensions.GetContent` +
`Bundle.ResolveResource`。**空路径 → `None`**（AssetRipper 行为，不做 UnityPy 的 stem
推导）；resolved 后做 bounds 校验（`len >= offset + size`），再零拷贝 `Bytes::slice`。

```rust
impl Environment {
    /// 取 `StreamingInfo` 指向的流数据（零拷贝 Bytes slice）。
    pub fn streaming_data(&self, info: &StreamingInfo) -> Option<Bytes> {
        let path = info.path.as_str();
        if path.is_empty() || info.size == 0 { return None; }
        let offset = streaming_offset(&info.offset);   // u32 / u64 variant 提取
        let res = self.resolve_resource(path)?;
        let end = offset.checked_add(info.size as u64)?;
        if res.bytes.len() as u64 < end { return None; }   // 完整性校验
        Some(res.bytes.slice(offset as usize..end as usize))
    }

    /// 取 `StreamedResource`（AudioClip）指向的流数据。
    pub fn streamed_resource_data(&self, res: &StreamedResource) -> Option<Bytes> {
        if res.m_source.is_empty() || res.m_size == 0 { return None; }
        let end = res.m_offset.checked_add(res.m_size)?;
        let file = self.resolve_resource(&res.m_source)?;
        if file.bytes.len() as u64 < end { return None; }
        Some(file.bytes.slice(res.m_offset as usize..end as usize))
    }

    /// 对齐 Bundle.ResolveResource：空名→None，否则 name_fixed 精确匹配 resources。
    fn resolve_resource(&self, name: &str) -> Option<&ResourceFile> {
        if name.is_empty() { return None; }
        let fixed = fix_file_identifier(name);
        self.resources.iter().find(|r| r.name_fixed == fixed)
    }
}
```

`StreamingInfo.offset` 是版本枚举（`Default(u32)` / `V2020_1(u64)`），由内部 helper
`streaming_offset` 统一提取为 `u64`——对齐 `StreamingInfoExtensions.GetOffset`。

---

## 8. 公共 API

`src/lib.rs` 新增：

```rust
mod environment;
pub use environment::{
    AssetCollection, DependencyProvider, Environment, EnvironmentBuilder, EnvironmentObject,
    ParseFailure, PPtrLike, ResourceFile, ResourceProvider,
};
```

聚合层构造唯一入口是 [`EnvironmentBuilder`]（`Environment::builder()`），无自由函数入口。
io 层 `load*`（`load`/`load_with`/`load_bytes`/`load_bytes_with`）产出 `Bundle`，是不同层级。

典型用法：

```rust
// 单 bundle
let env = Environment::builder().path("char_002_amiya.ab")?.build();

// 多 bundle（跨 bundle PPtr 自动 resolve）
let env = Environment::builder()
    .path("main.ab")?
    .path("shared.ab")?
    .build();

// 整个目录（递归 + UnityFS 签名嗅探）
let env = Environment::builder().scan_dir("game/StreamingAssets")?.build();

// 自定义 codec（如 Arknights LZ4 变体）
let env = Environment::builder().registry(&my_reg).path("a.ab")?.build();

for obj in env.objects() {
    if let ParsedObject::Typed(UnityObject::GameObject(go)) = &obj.object {
        // 解引用 m_Component 里的 PPtr<Transform> / PPtr<MonoBehaviour> ...
        // （具体字段形态由 codegen 决定，按 VersionedEnum 处理）
    }
}
```

---

## 9. 错误隔离

对齐 ADR-0003 路线图「错误隔离 Failure 叶节点」。`EnvironmentBuilder::build` 是
**infallible**（返回 `Environment`，不返回 `Result`）：

- io 层错误（`load`）发生在 build 之前（`builder().path(..)?` 的 `?`）。
- SerializedFile::parse 失败 → file-level `ParseFailure`，continue。
- 单对象 `parse_object` 失败 → object-level `ParseFailure`，continue。
- 用户通过 `env.failures()` 检查；`env.objects()` 只含成功者。

---

## 10. 实施阶段（已完成）

| 阶段 | 内容 | 状态 |
|---|---|---|
| **Phase 1** | 捕获 externals：`src/serialized_file.rs` 新增 `FileIdentifier` + `SerializedFile.externals`；`src/lib.rs` 导出 | ✅ |
| **Phase 2** | 新建 `src/environment.rs`：全部类型 + `fix_file_identifier`（`src/names.rs`）+ 两阶段构建 + resolve + streaming_data；`src/lib.rs` 导出 | ✅ |
| **Phase 3** | 验证：`tests/environment.rs`（5 个集成测试）+ 全部 `example/` bundle 烟测（0 失败） | ✅ |

---

## 11. 不在本次范围

- **bumpalo arena**（profiling 驱动后再说）。
- **懒加载 `DependencyProvider`**：trait 已落地，但当前 eager 模型不实际回调它（所有
  bundle 在 build 前就已加载）。后续在 Phase B `target.is_none()` 分支接入即可按需加载
  缺失 bundle，无需改公共 API。
- **磁盘 `.resS` `ResourceProvider`**：trait 已落地，但当前不回调。后续接入跨 bundle
  共享的 loose resource（磁盘现取）时实现即可。
- **gzip / brotli bundle 嗅探**：`EnvironmentBuilder::scan_dir` 当前只认 `UnityFS` 签名，
  gzip(`1f 8b`)/brotli/`UnityWeb`/`UnityRaw` bundle 静默跳过。路线图见 [§13.4](#134-gzip--brotli-路线图)。
- **`AssetType.Meta`（GUID 匹配）**：外部引用匹配仅按 `name_fixed`（Normal 类）。
  Meta 类（`type_==1`，按 GUID 匹配）罕见，留作后续。
- **旧版 AudioClip pre-5.0 流式**（codegen §11 已记的值依赖特殊 case，需单独处理）。

---

## 13. 跨 bundle Environment

多 bundle 聚合已落地。**核心洞察**：数据模型本就天然支持跨 bundle——`objects`/
`collections`/`resources` 全是全局扁平 `Vec`，`resolve()` 走全局索引，Phase B 依赖解析
全局扫描 `collections`。把多个 bundle 灌进**同一个** `Environment`，并在全部 Phase A 完
成后做**一次** Phase B，跨 bundle 引用**自动 resolve**。无需改 `AssetCollection` /
`resolve` / `resolve_resource` 结构。这恰好对齐 AssetRipper 两阶段加载
（`InitializeFromPaths` → `InitializeAllDependencyLists`）。

### 13.1 构造入口

聚合层唯一构造入口是 [`EnvironmentBuilder`]（`Environment::builder()`）：

```rust
// Builder：累积多 bundle，最后一次性依赖解析
let env = Environment::builder()
    .path("a.ab")?          // fs::read + 驻留 registry + Phase A
    .path("b.ab")?
    .bundle(preloaded)?     // 或喂入已解析的 Bundle（自定义 registry 预加载）
    .scan_dir("more/")?     // 或递归扫描目录（UnityFS 签名嗅探）
    .build();               // Phase B 全局依赖解析

// 自定义 codec（如 Arknights LZ4 变体）：registry 设一次，后续 path/scan_dir 均复用
let env = Environment::builder()
    .registry(&my_reg)
    .path("a.ab")?
    .build();
```

`registry()` 默认 `Registry::defaults()`（LZMA + LZ4 + LZ4HC + NONE）。

### 13.2 Builder 结构

```rust
pub struct EnvironmentBuilder {
    // 累积阶段暂存——build() 时 move 进 Environment
    objects: Vec<EnvironmentObject>,
    collections: Vec<AssetCollection>,
    resources: Vec<ResourceFile>,
    failures: Vec<ParseFailure>,
    pending_deps: Vec<(usize, Vec<FileIdentifier>)>,
    // 解压器注册表（默认 Registry::defaults()，registry() 覆盖；path/scan_dir 均复用）
    registry: Registry,
    // 扩展点（见 §13.3）
    dependency_provider: Option<Box<dyn DependencyProvider>>,
    resource_provider: Option<Box<dyn ResourceProvider>>,
}
```

- `path()` / `scan_dir()` / `bundle()` 各做一次 Phase A（复用 `phase_a` helper）。
- `build()` 把四个 Vec move 进新 `Environment`，调一次 `resolve_dependencies`（Phase B）。

### 13.3 扩展点 trait

对齐 AssetRipper `IDependencyProvider` / `IResourceProvider`，为懒加载 / 磁盘 resS 预留
API 面。**默认 no-op；当前 eager 模型不实际回调它们**（trait + builder setter 已落地，
实现在 Phase B 接入）：

```rust
pub trait DependencyProvider {
    /// 外部引用未命中时回调。Some(Bundle) → 追加加载；None → 留空。
    fn find_dependency(&self, _id: &FileIdentifier) -> Option<Bundle> { None }
}
pub trait ResourceProvider {
    /// `.resS` 未命中时回调。Some(ResourceFile) → 注册进 resources。
    fn find_resource(&self, _name: &str) -> Option<ResourceFile> { None }
}
```

后续接入懒加载：在 `resolve_dependencies` 的 `target.is_none()` 分支调
`find_dependency` → 返回 `Some(Bundle)` 则追加 `phase_a` + 重试查找。无需改公共 API。

### 13.4 gzip / brotli 路线图

`EnvironmentBuilder::scan_dir` 当前只嗅探 `UnityFS` 签名（`src/detect.rs`）。gzip(`1f 8b`) /
brotli（`0x20` 处 magic）/ `UnityWeb` / `UnityRaw` / `UnityArchive` bundle 被静默跳过
（`log::warn`）。后续若要支持，对齐 AssetStudio `FileReader.CheckFileType` +
`DecompressGZip`：

1. 加 `flate2` 依赖（gzip）/ 复用 codec crate 的 `brotli`（brotli）。
2. `detect.rs` 扩 `looks_like_gzip` + `pregunpack_gzip(Bytes) -> Bytes`。
3. `scan_dir` 嗅探顺序：先 gzip magic → 解压 → 再嗅探 `UnityFS`。

AssetStudio 的关键行为：gzip bundle 解压后内层仍是 UnityFS（解压后再嗅探），故嗅探是
两级的（`1f 8b` → 解压 → `UnityFS`）。

---

| 文档 | 角色 |
|---|---|
| **本文档** | Environment 层**现状权威**（Environment + Builder + PPtr 解引用 + streaming_data + 跨 bundle 聚合，已落地） |
| [`io-layer.md`](io-layer.md) | 本层输入（`Bundle` / `BundleFile`） |
| [`object-layer.md`](object-layer.md) | 对象 dispatch（`parse_object` / `ParsedObject`） |
| [`codegen.md`](codegen.md) | typed struct（`UnityObject` / `StreamingInfo` / `StreamedResource`） |
| [`../docs/ress-file-handling.md`](../../docs/ress-file-handling.md) | `.resS` 处理对比（AssetRipper / UnityPy），本层设计依据 |
