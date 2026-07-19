# 环境聚合层（Environment 层）

> 范围：从 io 层的 `LoadedFile` 树构建 `Environment`——eager parse 全部 SerializedFile 的对象，
> 聚合 `.resS` 流数据，提供 PPtr 解引用与 streaming_data 取数。
> 状态：**本文档描述已落地的实现**（与 `src/environment.rs` + `src/loader.rs` + `src/names.rs` 一致）。
> 上游：[`io-layer.md`](io-layer.md)（本层输入 `LoadedFile`）、
> [`object-layer.md`](object-layer.md)（对象 dispatch `parse_object`）、
> [`codegen.md`](codegen.md)（typed struct `UnityObject`）。
> 参考：AssetRipper `GameBundle` / `AssetCollection` / `ResourceFile` / `SpecialFileNames`，
> 详见 [`../docs/ress-file-handling.md`](../../docs/ress-file-handling.md)。

---

## 增量 source 运行时

`Loader::build` 仍然对它收到的输入 eager parse；这不等于活动 Environment 每次变更都
重读全部文件。服务端先在环境锁外把一个文件或目录构建成独立 Environment，再调用：

- `Environment::insert_source`：只移动新 source 的已解析贡献；
- `Environment::remove_source`：移除该 source 的 collection/object/resource/failure；
- `Environment::replace_source`：新输入构建成功后替换原 source，保留 SourceId；
- `Environment::empty`：为单一活动 Environment 建立聚合容器。

`AssetCollection` 会保留原始 externals。增删 source 后，剩余对象不重新 parse，只压实
Vec、重映射 CollectionId、重建依赖表与 container 索引。CollectionId 因此只在一次
Environment revision 内稳定；桌面端在 mutation 后拉取全量快照。project/path/SQLite
都不进入库层，Rust 的 `SourceId` 只是同一 Environment 生命周期内的来源句柄。

## 1. 定位与术语对齐

Environment 层是对象解析层之上的**聚合层**：io 层交付 `LoadedFile`（递归再派发后的
文件树），对象解析层交付 `parse_object`（单对象 dispatch）；本层把它们组合成一张**可
解引用的对象图**——eager parse 全部对象、构建 PPtr 依赖表、聚合 `.resS` backing
store。

命名与设计参照 **AssetRipper**：

| AssetRipper（C#） | 本 crate（Rust） | 说明 |
|---|---|---|
| `GameBundle` | `Environment` | 顶层聚合（多 bundle） |
| `GameBundle.FromPaths` | `Loader` / `LoaderBuilder` | 声明式累积多 bundle 后 `build()` |
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
多个 bundle 一起加载（[`LoaderBuilder`] 的 `path` / `scan_dir`）时，跨 bundle 引用也
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
    /// 运行时扩展点（build 时从 builder move 入）。`None` 时无懒加载/资源现取。
    dependency_provider: Option<Arc<dyn DependencyProvider>>,
    resource_provider: Option<Arc<dyn ResourceProvider>>,
}

pub struct EnvironmentObject {
    pub collection: CollectionId,   // 所属 AssetCollection 句柄（newtype）
    pub path_id: i64,
    pub class_id: ClassId,           // 开集 class id newtype（含 MonoBehaviour 114）
    pub object: ParsedObject,        // Typed(UnityObject) | Dynamic(Value) | MonoBehaviour(_)
}

pub struct AssetCollection {
    pub name: String,               // 原始 bundle node 名（如 "CAB-xxxx.assets"）
    pub name_fixed: String,         // fix_file_identifier(name)
    pub unity_version: UnityVersion,
    pub endian: Endian,
    /// 依赖列表。[0] = 自身句柄；[1..] = 解析出的外部 collection 句柄。
    /// PPtr.FileID 直接索引此列表（0 = 自身）——与 AssetRipper
    /// `AssetCollection.Dependencies` 约定一致。`None` = 未解析的跨 bundle 引用。
    pub dependencies: Vec<Option<CollectionId>>,
    /// path_id → `Environment.objects` 索引（内部存储细节，非身份）。
    pub assets: HashMap<i64, usize>,
}

pub struct ResourceFile {
    pub name: String,
    pub name_fixed: String,
    pub bytes: Bytes,
}

/// 解析失败记录（错误隔离）。
pub enum ParseFailure {
    /// SerializedFile 整体解析失败（file-level）。
    File { name: String, error: Error },
    /// 单个对象解析失败（object-level）。
    Object { collection: CollectionId, path_id: i64, class_id: ClassId, error: Error },
    /// io 层子容器/子条目递归 parse 失败（container-level）。
    /// 对应 `LoadedFile::Failed`：保留了原始字节与错误，但 `phase_a` 无法把它当作
    /// SerializedFile/Resource 处理，故记录为失败。`name` 是原始 node 名。
    Container { name: String, error: Error },
}
```

**对象身份 newtype**：`collection`/`class_id` 都从裸整数升级为 newtype，避免把内部
存储/分发细节泄漏成 API：

```rust
/// 稳定的 collection 句柄。包装 `Environment.collections` 的 Vec 下标。
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct CollectionId(u32);

impl CollectionId {
    /// 内部构造（仅 crate 内）。从 `collections()` 迭代时用 `collection_id(idx)`。
    pub(crate) fn new(idx: usize) -> Self;
    /// 暴露底层 Vec 下标。**仅为与 Vec-backed 访问器互操作的 escape hatch**，
    /// 不要用来跨 `Environment` 实例传递（不同实例的下标不通用）。
    pub fn to_index(self) -> usize;
}

/// 开集 class id newtype。承载**任何** Unity class id（包括 codegen 跳过的 114
/// MonoBehaviour），不与“是否可 typed parse”耦合。
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct ClassId(pub i32);

impl ClassId {
    pub const MONO_BEHAVIOUR: ClassId = ClassId(114);
    pub const fn new(id: i32) -> Self;
    pub const fn get(self) -> i32;
    /// 映射到 codegen 生成的 `ClassIDType`。`None` = codegen 未覆盖。
    pub fn known(self) -> Option<ClassIDType>;
    /// codegen 是否为该 class 生成了 typed parse 路径。
    pub fn is_typed_parseable(self) -> bool;
}

/// 对象的全局身份：`collection` 句柄 + `path_id`。
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct ObjectId { pub collection: CollectionId, pub path_id: i64 }

/// `Environment` 中对象的借用引用。绑定了 `&Environment` 生命周期，
/// 经 `resolve_ref` / `object_ref` 产出，`get()` 取回 `EnvironmentObject`。
pub struct ObjectRef<'env> { /* env: &'env Environment, id: ObjectId */ }
```

- `ClassId` 是**开集**：codegen 跳过的 class（最典型是 MonoBehaviour 114）仍能用
  `ClassId::MONO_BEHAVIOUR` 表示，再用 `known() -> Option<ClassIDType>` 可选映射到
  人类可读枚举。“是否可 typed parse”不在 `ClassId` 上硬编码，由 dispatch
  （`objects_of_class_id` / parse）另行判断。
- `CollectionId` 隐藏 `Vec` 存储布局：未来调整存储、做按需解析或 Python handle 时，
  只要保持 newtype 语义不变，API 不必破坏。

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

`Loader::build` 对每个输入源（`path` / `bytes` / `loaded` / `scan_dir`）跑一次
io 层 `load_recursive`（产出 `LoadedFile` 树），再交给共享 helper `phase_a`
（`src/environment.rs::phase_a`）灌进 `Environment` 的各 Vec：

```
LoadedFile 叶子遍历（leaves()）：
  ├─ 后缀 .resS / .ress / .resource（大小写无关）→ ResourceFile { name_fixed, bytes }
  │     bytes 用 Bytes::copy_from_slice detach——断开与 blob Arc 的共享，
  │     使 SerializedFile 解析完毕后整个解压 blob 可释放（见 docs/memory-notes.md）
  ├─ format != SerializedFile → 跳过（日志）
  └─ SerializedFile::parse(bytes)
        ├─ 失败 → ParseFailure::File { name, error }，continue
        └─ 成功 → AssetCollection { name_fixed, dependencies: [Some(self)], unity_version, endian }
              └─ eager parse 每个 object（ObjectInfo::parse）：
                    ├─ Ok → EnvironmentObject，assets[path_id] = idx
                    └─ Err → ParseFailure::Object { collection, path_id, class_id, error }，continue

整棵 LoadedFile 树的 Failed 节点（io 层子条目递归 parse 失败）→
  ParseFailure::Container { name, error }（见 LoadedFile::into_failures）
```

`phase_a` 是 crate-private 共享实现，`Loader::build` 与 `DependencyProvider`
懒加载回灌都用它。`Environment::resolve_dependencies`（Phase B）+ `prefetch_resources`
（`ResourceProvider`）在所有 `phase_a` 跑完后调一次。

### Phase B：初始化依赖列表

```
对每个 AssetCollection i（col_id: CollectionId）：
  sf = 对应 SerializedFile（externals 已在 Phase A 读出）
  for identifier in sf.externals:
    target = resolve_collection(identifier.path_name)
           = collections 里 name_fixed == identifier.path_name 的 CollectionId
    collection_i.dependencies.push(Some(target))   // 未找到 → None（跨 bundle）
```

`resolve_collection`：全局范围（全部 `collections`）按 `name_fixed` 精确匹配（对齐
`Bundle.ResolveCollection` 的 `TryResolveFromCollections`）——跨 bundle 引用因此能命中。

---

## 6. PPtr 解引用

对齐 `AssetCollection.TryGetAsset(fileIndex, pathID)` → `Dependencies[fileIndex]` →
`TryGetAsset(pathID)`。

```rust
impl Environment {
    pub fn resolve(
        &self,
        source_collection: CollectionId,
        file_id: i32,
        path_id: i64,
    ) -> Option<&EnvironmentObject> {
        let col = self.collections.get(source_collection.to_index())?;
        let target_idx = *col.dependencies.get(file_id as usize)?.as_ref()?;
        let target = self.collections.get(target_idx.to_index())?;
        let &obj_idx = target.assets.get(&path_id)?;
        self.objects.get(obj_idx)
    }

    /// 便捷：从源对象解析（自动取 source.collection）。
    pub fn resolve_from(
        &self,
        source: &EnvironmentObject,
        pptr: &impl PPtrLike,
    ) -> Option<&EnvironmentObject> {
        self.resolve(source.collection, pptr.file_id(), pptr.path_id())
    }

    /// `file_id == 0` 的便捷场景：同 collection 内 `path_id` 查找。
    pub fn resolve_local(
        &self,
        collection: CollectionId,
        path_id: i64,
    ) -> Option<&EnvironmentObject>;

    /// 用 `ObjectId` 解引用，返回绑定了 `&Environment` 生命周期的 `ObjectRef`。
    /// 仅校验该 id 在环境内可解析（collection 合法 + path_id 命中 assets），
    /// 实际对象取回在 `ObjectRef::get` 时做。
    pub fn resolve_ref(&self, id: ObjectId) -> Option<ObjectRef<'_>>;

    /// 由 `&EnvironmentObject` 构造 `ObjectRef`。
    pub fn object_ref(&self, obj: &EnvironmentObject) -> ObjectRef<'_>;

    /// 由下标构造 `CollectionId`（仅供调用方从 `collections()` 迭代时取句柄）。
    pub fn collection_id(&self, index: usize) -> Option<CollectionId>;
}

/// typed PPtr（unity-core）与 dynamic PPtrValue（typetree）统一。
pub trait PPtrLike {
    fn file_id(&self) -> i32;
    fn path_id(&self) -> i64;
}
```

`file_id == 0` 天然落到 `dependencies[0] = Some(self)`，无需特判。`resolve_from`
签名里 `pptr` 现在按引用传入（`&impl PPtrLike`），避免消费调用方手里的 PPtr。

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

### 7.1 AudioClip 音频字节（pre-5.0 流式回填）

`Environment::audio_data(obj)` 对齐 AssetRipper `AudioClipExtensions.GetAudioData` 的三路
分发，覆盖 pre-5.0 与 post-5.0 两种流式布局：

| 来源 | 触发条件 | 取数路径 |
|---|---|---|
| **内联**（pre-5.0，`m_Stream != 2`） | `m_audio_data == Inline(v)` | 直接返回 `v`（parse 时已读） |
| **流式指针**（pre-5.0，`m_Stream == 2`） | `m_audio_data == Streamed { size, offset }` | 从 `<collection.name>.resS` 按 `{offset, size}` slice |
| **StreamedResource**（post-5.0） | `m_resource` 有、`m_audio_data == None` | `streamed_resource_data`（上文） |

pre-5.0 流式路径对齐 AssetRipper `AudioClipHelper.ReadStreamedByteArray`：
`resourceFileName = Collection.Name + ".resS"`，经 `resolve_resource`（`fix_file_identifier`
精确匹配）找到 bundle 内同名 `.resS`，再 `offset..offset+size` 零拷贝 slice。

pre-5.0 的 `m_audio_data` 字段类型是 codegen 生成的 `AudioClipMAudioData` 枚举
（`Inline` / `Streamed`），把值依赖读取封装在枚举变体里——详见
[codegen §11.1](codegen.md#111-值依赖读特判pre-50-audioclipmaudiodatapass505-rust-移植)。

---

## 8. 公共 API

`src/lib.rs` 顶层导出（部分）：

```rust
// src/lib.rs
pub mod environment;
pub mod loader;
pub mod names;
// …io / types / export 门面模块见 src/lib.rs

pub use loader::{Diagnostic, DiagnosticKind, LoadReport, Loader, LoaderBuilder, Severity};
pub use environment::{
    AssetCollection, ClassId, CollectionId, DependencyProvider, DependencyRequest,
    Environment, EnvironmentObject, NamedBytes, ObjectId, ObjectRef, ParseFailure,
    PPtrLike, ProviderError, ResourceFile, ResourceProvider, ResourceRequest,
};
```

聚合层构造唯一入口是 [`LoaderBuilder`]（`Loader::builder()`），无自由函数入口。
io 层 `load*`（`load`/`load_with`/`load_bytes`/`load_bytes_with`）产出 `LoadedFile`
（递归再派发后的文件树），是不同层级——`Loader` 内部会自动调 `load_recursive`。

### 8.1 声明式 Builder

`LoaderBuilder` 是**声明式**：`path` / `bytes` / `loaded` / `scan_dir` / `registry` /
`dependency_provider` / `resource_provider` 都只记录输入（返回 `Self`，不返回 `Result`），
真正工作（`fs::read` + `load_recursive` + Phase A）统一放到 `build()`：

```rust
impl Loader {
    pub fn builder() -> LoaderBuilder;
}
impl LoaderBuilder {
    // 声明式：只记录，不立即执行，返回 Self。
    pub fn registry(self, reg: &Registry) -> Self;
    pub fn path(self, path: impl AsRef<Path>) -> Self;            // build() 时才 fs::read
    pub fn bytes(self, name: impl Into<String>, bytes: Bytes) -> Self;
    pub fn loaded(self, loaded: LoadedFile) -> Self;              // 高级：预解析树直接灌入
    pub fn scan_dir(self, path: impl AsRef<Path>) -> Self;
    pub fn dependency_provider(self, provider: impl DependencyProvider + 'static) -> Self;
    pub fn resource_provider(self, provider: impl ResourceProvider + 'static) -> Self;

    /// 执行加载。顶层 IO/配置错误 → `Err`；局部错误进入 `diagnostics`。
    pub fn build(self) -> Result<LoadReport>;
    /// 严格模式：任何 Error 级 diagnostic 都让 build 失败（Warning 级仍进入 report）。
    pub fn build_strict(self) -> Result<LoadReport>;
}
```

> 与旧 `EnvironmentBuilder` 的关键差异：旧的 `.path(X)?` 链式（每步返回 `Result`，
> 当场 `fs::read` + Phase A）**已删除**。现在所有 `?` 都集中在 `build()` 一次——
> 顶层 IO 错误在这里返回；单文件/单对象失败进 `LoadReport::diagnostics`。

### 8.2 LoadReport + Diagnostic

`build()` 成功返回 `LoadReport`，含已构建的 `Environment` + 构建期诊断：

```rust
pub struct LoadReport {
    environment: Environment,
    diagnostics: Vec<Diagnostic>,
}
impl LoadReport {
    pub fn environment(&self) -> &Environment;
    pub fn into_environment(self) -> Environment;
    pub fn diagnostics(&self) -> &[Diagnostic];
    pub fn has_errors(&self) -> bool;
}

pub enum Severity { Error, Warning }
pub enum DiagnosticKind { File, Object, Container }  // 对应 ParseFailure 三变体

pub struct Diagnostic {
    pub severity: Severity,
    pub message: String,
    pub kind: Option<DiagnosticKind>,
}
```

`Diagnostic` 是对 `Environment::failures`（结构化 `ParseFailure`）的扁平化 + scan_dir
等非结构化信息（跳过非可加载文件、空目录）的统一载体。`Severity::Error` = 单文件/单
对象/provider 错误（可恢复，记录但不中断 build）；`Severity::Warning` = 不影响结果的
信息（如 scan_dir 跳过文件）。`DiagnosticKind` 标注结构化来源（File/Object/Container）。

### 8.3 典型用法

```rust
// 单 bundle —— 注意所有 ? 都在 build()，不再在 path()
let report = Loader::builder().path("char_002_amiya.ab").build()?;
let env = report.environment();

// 多 bundle（跨 bundle PPtr 自动 resolve）
let report = Loader::builder()
    .path("main.ab")
    .path("shared.ab")
    .build()?;
let env = report.environment();

// 整个目录（递归 + UnityFS/gzip/brotli 签名嗅探）
let report = Loader::builder().scan_dir("game/StreamingAssets").build()?;

// 自定义 codec（如 Arknights LZ4 变体）：registry 设一次，后续 path/scan_dir 均复用
let report = Loader::builder()
    .registry(&my_reg)
    .path("a.ab")
    .build()?;

// 严格模式：任何 Error 级 diagnostic 直接 build 失败（含失败原因的 message）
let env = Loader::builder().path("a.ab").build_strict()?.into_environment();

// 诊断检查（非严格模式下用）
let report = Loader::builder().path("a.ab").build()?;
if report.has_errors() {
    for d in report.diagnostics() {
        eprintln!("{d}");   // Display 形如 "[ERROR] object 失败 col=0 pid=42 cid=114: …"
    }
}
let env = report.into_environment();

for obj in env.objects() {
    // obj.class_id 是 ClassId（开集 newtype）
    if let Some(cls) = obj.class_id.known() {
        if cls == ClassIDType::GameObject {
            // typed 路径；解引用 m_Component 里的 PPtr 用 env.resolve_from
        }
    }
    if obj.class_id == ClassId::MONO_BEHAVIOUR {
        if let Some(mb) = obj.as_mono_behaviour() {
            let script = env.resolve_from(obj, &mb.m_script);   // PPtr 解引用
        }
    }
}
```

> `Loader` 内部复用 `Registry::defaults()`（LZMA + LZ4 + LZ4HC + NONE）。自定义 codec
> 时用 `.registry(&my_reg)` 一次，后续 `path`/`scan_dir`/`bytes` 都复用。

---

## 9. 错误隔离

对齐 ADR-0003 路线图「错误隔离 Failure 叶节点」。`LoaderBuilder::build` 区分两类错误：

- **顶层错误 → `Result::Err`**：`fs::read` 失败、`load_recursive` 顶层 parse 失败、
  IO/配置错误等调用方必须知道的硬错误。`build_strict` 模式下，任何 Error 级 diagnostic
  也归入此类（拼成带失败原因的 message）。
- **局部错误 → `LoadReport::diagnostics`（`Severity::Error`）+ `Environment::failures`
  （结构化 `ParseFailure`）**：
  - SerializedFile::parse 失败 → `ParseFailure::File { name, error }`。
  - 单对象 `ObjectInfo::parse` 失败 → `ParseFailure::Object { collection, path_id, class_id, error }`。
  - io 层子条目递归 parse 失败（`LoadedFile::Failed`）→ `ParseFailure::Container { name, error }`。
  - Provider `open` 返回 `Err` → `ParseFailure::Container`（不中断 build）。
- **非结构化 Warning**：scan_dir 跳过非可加载文件、空目录等不影响结果的信息 →
  `Severity::Warning`（不进 `failures`，只在 `diagnostics`）。

用户通过 `report.diagnostics()` / `report.has_errors()` 检查诊断，或
`report.environment().failures()` 取结构化失败；`env.objects()` 只含成功者。
`Diagnostic::from_failure` 把 `ParseFailure` 三变体投影成对应 `DiagnosticKind`
（File/Object/Container）。

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
- **`AssetType.Meta`（GUID 匹配）**：外部引用匹配仅按 `name_fixed`（Normal 类）。
  Meta 类（`type_==1`，按 GUID 匹配）罕见，留作后续。
- **`ResourceProvider` 对非 Texture2D/AudioClip 的 streaming 引用**：`prefetch_resources`
  当前只扫 Texture2D（`m_stream_data.path`）+ AudioClip（`m_resource.m_source` / pre-5.0
  隐式 `.resS` 名）。其他对象类型若有 streaming 引用会漏，`resolve_resource` 仍返回 `None`
  （与无 provider 时一致，不回归）。

> **`DependencyProvider` / `ResourceProvider` 已实际回调**（原在此处列为待办）：
> build 时 Phase B 在 `target.is_none()` 分支调 `open`，返回 `Ok(Some(NamedBytes))`
> 则 Loader 内部跑 `load_recursive` + `phase_a` 并重试匹配，**传递依赖递归直到不动点**
> （`tried` 去重 + `MAX_LAZY_LOADS=4096` 上限防恶意循环）；build 末尾 `prefetch_resources`
> 扫 Texture2D/AudioClip streaming 引用，对未命中的 name 调 `open`，Loader 自己算
> `name_fixed` 并注册进 `resources`。两 provider 以 `Arc<dyn Trait + Send + Sync>` 从
> builder move 入 `Environment`（`Send + Sync` 对齐未来 pyo3/async）。详见
> [§13.3 扩展点 trait](#133-扩展点-trait) + [§7 streaming_data](#7-streaming_data)。
>
> **gzip / brotli bundle 嗅探已支持**（原在此处列为待办）：`LoaderBuilder::scan_dir`
> 经 `detect::looks_like_loadable_path` 现嗅探 UnityFS + gzip + brotli 三种外层格式
> （io 层 `load_recursive` 本就透明解压，只是目录扫描入口的预过滤此前漏了 brotli）。
> 详见 [`docs/io-layer.md`](io-layer.md) §6。
>
> 旧版 AudioClip pre-5.0 流式**已处理**（原在此处列为待办）：codegen 生成
> `AudioClipMAudioData` 枚举 + 值依赖读（见 [codegen §11.1](codegen.md#111-值依赖读特判pre-50-audioclipmaudiodatapass505-rust-移植)），
> 聚合层用 `Environment::audio_data` 回填 `.resS` 字节（见下文 streaming_data 小节）。

---

## 13. 跨 bundle Environment

多 bundle 聚合已落地。**核心洞察**：数据模型本就天然支持跨 bundle——`objects`/
`collections`/`resources` 全是全局扁平 `Vec`，`resolve()` 走全局索引，Phase B 依赖解析
全局扫描 `collections`。把多个 bundle 灌进**同一个** `Environment`，并在全部 Phase A 完
成后做**一次** Phase B，跨 bundle 引用**自动 resolve**。无需改 `AssetCollection` /
`resolve` / `resolve_resource` 结构。这恰好对齐 AssetRipper 两阶段加载
（`InitializeFromPaths` → `InitializeAllDependencyLists`）。

### 13.1 构造入口

聚合层唯一构造入口是 [`LoaderBuilder`]（`Loader::builder()`）：

```rust
// Builder：声明式累积多 bundle，build() 时统一跑 IO + Phase A + Phase B
let report = Loader::builder()
    .path("a.ab")          // 只记录路径，不立即执行
    .path("b.ab")
    .loaded(preloaded)     // 高级：喂入已解析的 LoadedFile（自定义 registry 预加载）
    .scan_dir("more/")     // 或递归扫描目录（UnityFS/gzip/brotli 签名嗅探）
    .build()?;             // Phase A + Phase B 全局依赖解析
let env = report.environment();

// 自定义 codec（如 Arknights LZ4 变体）：registry 设一次，后续 path/scan_dir 均复用
let report = Loader::builder()
    .registry(&my_reg)
    .path("a.ab")
    .build()?;
```

`registry()` 默认 `Registry::defaults()`（LZMA + LZ4 + LZ4HC + NONE）。所有 `?` 集中在
`build()` 一次——声明式 builder 没有 `.path(X)?` 链式 Result。

### 13.2 Loader / LoaderBuilder 结构

```rust
pub struct Loader {
    sources: Vec<Source>,    // Path / Bytes / Loaded / ScanDir
    registry: Registry,
    dependency_provider: Option<Arc<dyn DependencyProvider>>,
    resource_provider: Option<Arc<dyn ResourceProvider>>,
}

#[derive(Default)]
pub struct LoaderBuilder {
    sources: Vec<Source>,
    registry: Registry,
    dependency_provider: Option<Arc<dyn DependencyProvider>>,
    resource_provider: Option<Arc<dyn ResourceProvider>>,
}

enum Source {
    Path(PathBuf),                                  // build() 时才 fs::read
    Bytes { name: String, bytes: Bytes },           // 已读入内存的字节
    Loaded(LoadedFile),                             // 预解析树直接灌入（高级）
    ScanDir(PathBuf),                               // build() 时遍历 + 嗅探加载
}
```

- `path` / `bytes` / `loaded` / `scan_dir` 只 push 一个 `Source`（声明式，返回 `Self`）。
- `build()` 对每个 `Source` 跑 `load_recursive`（`Path`/`Bytes`/`ScanDir`）或直接用
  `loaded`，再交给共享 helper `phase_a` 灌进 `objects`/`collections`/`resources`/
  `failures` + `pending`。全部 source 处理完后构造 `Environment`，调一次
  `resolve_dependencies`（Phase B，含 `DependencyProvider` 懒加载）+ `prefetch_resources`
  （`ResourceProvider` 预拉取缺失 `.resS`），最后把 `Environment` + `diagnostics` 打包成
  `LoadReport`。

### 13.3 扩展点 trait

对齐 AssetRipper `IDependencyProvider` / `IResourceProvider`。**已实际回调**（非预留 API 面），
两 trait 均 `Send + Sync`，以 `Arc<dyn Trait>` 从 builder move 入 `Environment`。**接口已
收敛为只负责“打开字节”**——Loader/`Environment` 接到字节后自己跑 `load_recursive` +
`phase_a` + `name_fixed` 归一化（旧实现要求 Provider 自备 `Registry`/命名规则并返回已解析
的 `LoadedFile`/`ResourceFile`，职责过重）：

```rust
pub trait DependencyProvider: Send + Sync {
    /// 外部引用未命中时回调。`Ok(Some(NamedBytes))` → Loader 内部加载该依赖；
    /// `Ok(None)` → 该引用留空（resolve 返回 None）；`Err` → 记录为 ParseFailure::Container。
    fn open(&self, _req: &DependencyRequest<'_>) -> Result<Option<NamedBytes>, ProviderError> {
        Ok(None)
    }
}
pub trait ResourceProvider: Send + Sync {
    /// `.resS` 未命中时回调。`Ok(Some(Bytes))` → Loader 自己算 name_fixed 并注册进
    /// resources；`Ok(None)` → streaming 取数返回 None；`Err` → ParseFailure::Container。
    fn open(&self, _req: &ResourceRequest<'_>) -> Result<Option<Bytes>, ProviderError> {
        Ok(None)
    }
}

/// Provider 打开依赖时返回的原始字节 + 建议名。Loader 内部据此跑 load_recursive。
/// `name` 仅用于诊断/日志——Loader **不依赖此值做匹配**，匹配走自己算的 name_fixed。
pub struct NamedBytes { pub name: String, pub bytes: Bytes }

/// Provider 打开依赖时传入的请求：来源 collection + 原始 FileIdentifier + 归一化名。
pub struct DependencyRequest<'a> {
    pub source_collection: CollectionId,
    pub identifier: &'a FileIdentifier,           // 含 path_name_origin / guid / type_
    pub normalized_name: &'a str,                 // fix_file_identifier(identifier.path_name)
}

/// Provider 打开 `.resS` 时传入的请求：来源 collection + 原始路径 + 归一化名。
pub struct ResourceRequest<'a> {
    pub source_collection: CollectionId,
    pub original_path: &'a str,                   // 如 archive:/CAB-xxx/CAB-xxx.resS
    pub normalized_name: &'a str,                 // fix_file_identifier(original_path)
}

/// Provider 错误。记录为 ParseFailure::Container 不中断整体 build。
pub struct ProviderError(pub String);
```

**`DependencyProvider` 懒加载**（Phase B `resolve_dependencies`）：某 collection 的 external
按 `name_fixed` 在全部 collection 中查不到（`target.is_none()`）时，调 `open`：
- 返回 `Ok(Some(NamedBytes))` → Loader 用其 `bytes` 跑 `load_recursive`，再 `phase_a`
  灌入 `Environment` 的各 Vec + `pending`（新 collection 的 externals 入队 → 传递依赖）
  + 重试匹配。
- **传递依赖递归**：新加载 bundle 引入的 externals 也加入 pending 队列，继续尝试解析，
  直到不动点。
- 防恶意循环：`tried: HashSet<String>` 去重（同一 `path_name` 只触发一次懒加载）+
  `MAX_LAZY_LOADS = 4096` 上限（超限 `log::warn` 停止）。
- `load_recursive` 本身失败 → `ParseFailure::Container`；provider `Err` → 同样记为
  `ParseFailure::Container`（不中断 build）。

**`ResourceProvider` 预拉取**（`build` 末尾 `prefetch_resources`，`resolve_dependencies` 之后）：
扫描已加载 Texture2D（`m_stream_data.path`）+ AudioClip（`m_resource.m_source` / pre-5.0
隐式 `collection.name + ".resS"`）的 streaming 引用，对未在 `resources` 命中的 name 调
`open`，Loader 自己算 `name_fixed` + 构造 `ResourceFile` push 进 `resources`——之后
`streaming_data` 等只读查找即能命中。覆盖范围见 [§11](#11-不在本次范围)。

### 13.4 gzip / brotli 嗅探

`LoaderBuilder::scan_dir` 经 `src/detect.rs::looks_like_loadable_path` 嗅探 **UnityFS +
gzip + brotli** 三种外层格式。io 层 `format::load_recursive` 本就透明解压 gzip/brotli（解压后
递归 sniff 内层），故 `scan_dir` 下游无需改——只需预过滤阶段识别这三种 magic：

- UnityFS：前 7 字节 `b"UnityFS"`。
- gzip：前 2 字节 `0x1F 0x8B`。
- brotli：`compressed::looks_like_brotli` 解析帧头 + 35 字节签名
  `b"UnityWeb Compressed Content (brotli)"`（需 ~40 字节头部，`detect.rs` 读 64 字节缓冲）。

`UnityWeb` / `UnityRaw` / `UnityArchive` bundle 仍跳过——这些需全量读后 `format::sniff`，
目录扫描为避免全量读大文件暂不支持，跳过会作为 `Severity::Warning` diagnostic 呈现
（不再只走 `log::warn`）。`path()` 入口（非目录扫描）则透明支持全部 io 层格式。

AssetStudio 的关键行为：gzip bundle 解压后内层仍是 UnityFS（解压后再嗅探），故嗅探是
两级的（`1f 8b` → 解压 → `UnityFS`）。

---

| 文档 | 角色 |
|---|---|
| **本文档** | Environment 层**现状权威**（Environment + Loader + PPtr 解引用 + streaming_data + 跨 bundle 聚合，已落地） |
| [`io-layer.md`](io-layer.md) | 本层输入（`LoadedFile`） |
| [`object-layer.md`](object-layer.md) | 对象 dispatch（`parse_object` / `ObjectInfo::parse` / `ParsedObject`） |
| [`codegen.md`](codegen.md) | typed struct（`UnityObject` / `StreamingInfo` / `StreamedResource`） |
| [`../docs/ress-file-handling.md`](../../docs/ress-file-handling.md) | `.resS` 处理对比（AssetRipper / UnityPy），本层设计依据 |
