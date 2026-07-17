//! 环境聚合层：从 io 层的 `Bundle` 构建 `Environment`——eager parse 全部 SerializedFile
//! 的对象，聚合 `.resS` 流数据，提供 PPtr 解引用与 streaming_data 取数。
//!
//! 设计与命名参照 AssetRipper（`GameBundle` / `AssetCollection` / `ResourceFile` /
//! `SpecialFileNames`）。详见 `docs/environment-layer.md`。

use std::collections::HashSet;
use std::collections::HashMap;
use std::sync::Arc;

use bytes::Bytes;
use generated::classes::audio_clip::AudioClipMAudioData;
use generated::subclasses::streamed_resource::StreamedResource;
use generated::subclasses::streaming_info::{StreamingInfo, StreamingInfoOffsetVariant};
use generated::{ClassIDType, UnityObject};

use crate::decompress::Registry;
use crate::format::{FileFormat, LoadedFile};
use crate::names::fix_file_identifier;
use crate::object_parser::{parse_object, MonoBehaviour, ParsedObject};
use crate::serialized_file::FileIdentifier;
use crate::typetree::PPtrValue;
use crate::{Endian, Error, PPtr, UnityVersion};
use crate::serialized_file::SerializedFile;

/// 稳定的 collection 句柄。包装 `Environment.collections` 的 Vec 下标,
/// 避免把内部存储布局(裸 `usize`)暴露成 API。未来调整存储、做按需解析或
/// Python handle 时,只要保持 `CollectionId` 语义不变,API 不必破坏。
///
/// 调用方不应自行构造;从 [`Environment::collection_id`] / [`EnvironmentObject::collection`]
/// / [`Environment::collections`] 获取。
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct CollectionId(u32);

impl CollectionId {
    /// 内部构造(仅 crate 内)。
    pub(crate) fn new(idx: usize) -> Self {
        Self(idx.try_into().expect("collection 索引超过 u32::MAX"))
    }
    /// 暴露底层 Vec 下标。**仅为与 Vec-backed 访问器互操作的 escape hatch**,
    /// 不要用来跨 `Environment` 实例传递(不同实例的下标不通用)。
    pub fn to_index(self) -> usize {
        self.0 as usize
    }
}

impl std::fmt::Display for CollectionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // 显示底层下标(调试/日志友好;不暴露 newtype 包装)。
        write!(f, "{}", self.0)
    }
}

/// 开集 class id newtype。承载**任何** Unity class id(包括 codegen 跳过的 114
/// MonoBehaviour),不与\"是否可 typed parse\"耦合。
///
/// - [`ClassId::known`] → 可选映射到 codegen 生成的 [`ClassIDType`](generated::ClassIDType)
///   (人类可读枚举;codegen 主动跳过 114,故 MonoBehaviour 只能经此 newtype 表示)。
/// - \"是否可 typed parse\" 由 [`Environment::objects_of_class_id`] / parse dispatch
///   另行判断,不在本类型上硬编码。
///
/// 与裸 `i32` 区分,避免把内部存储/分发细节泄漏成 API。
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct ClassId(pub i32);

impl ClassId {
    /// MonoBehaviour 的 class id。codegen 不生成 typed struct,但 class id 本身有效。
    pub const MONO_BEHAVIOUR: ClassId = ClassId(114);

    pub const fn new(id: i32) -> Self {
        Self(id)
    }
    pub const fn get(self) -> i32 {
        self.0
    }

    /// 映射到 codegen 生成的 [`ClassIDType`](generated::ClassIDType)。
    /// `None` = codegen 未覆盖(如 114 MonoBehaviour,或未知 class id)。
    pub fn known(self) -> Option<ClassIDType> {
        ClassIDType::from_i32(self.0)
    }

    /// codegen 是否为该 class 生成了 typed parse 路径。
    /// (等价于 `self.known().is_some()`,语义独立命名便于读代码。)
    pub fn is_typed_parseable(self) -> bool {
        ClassIDType::from_i32(self.0).is_some()
    }
}

impl From<i32> for ClassId {
    fn from(id: i32) -> Self {
        Self(id)
    }
}
impl From<ClassIDType> for ClassId {
    fn from(c: ClassIDType) -> Self {
        Self(c as i32)
    }
}
impl std::fmt::Display for ClassId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.known() {
            Some(c) => write!(f, "{}({:?})", self.0, c),
            None => write!(f, "{}", self.0),
        }
    }
}

/// 对象的全局身份:`collection` 句柄 + `path_id`。
///
/// 可由 [`EnvironmentObject::id`] 取得,经 [`Environment::resolve_ref`] 解引用。
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct ObjectId {
    pub collection: CollectionId,
    pub path_id: i64,
}

impl ObjectId {
    pub fn new(collection: CollectionId, path_id: i64) -> Self {
        Self { collection, path_id }
    }
}

/// `Environment` 中对象的借用引用。由 [`Environment::resolve_ref`] / [`Environment::object_ref`]
/// 产出,绑定了 `&Environment` 生命周期,经 [`ObjectRef::get`] 取回 [`EnvironmentObject`]。
#[derive(Clone, Copy)]
pub struct ObjectRef<'env> {
    env: &'env Environment,
    id: ObjectId,
}

impl<'env> ObjectRef<'env> {
    pub fn id(&self) -> ObjectId {
        self.id
    }
    pub fn collection(&self) -> CollectionId {
        self.id.collection
    }
    pub fn path_id(&self) -> i64 {
        self.id.path_id
    }
    /// 取回引用的对象。若底层存储中已不存在(如 lazy 解析未实现时被驱逐)返回 `None`。
    pub fn get(&self) -> Option<&'env EnvironmentObject> {
        self.env.resolve(self.id.collection, 0, self.id.path_id)
    }
}

/// 资产环境：聚合多个 bundle 的全部对象 + `.resS` 资源 + PPtr 依赖解析。
///
/// 等价于 AssetRipper 的 `GameBundle` + 全部 `AssetCollection`。数据模型天然支持跨 bundle：
/// `objects`/`collections`/`resources` 全是全局扁平 Vec，`resolve()` 走全局索引，
/// Phase B 依赖解析全局扫描 `collections`——只要把多个 bundle 灌进同一个 `Environment`，
/// 跨 bundle 引用在 Phase B（全部 collection 建好后）自动 resolve。
///
/// 构造唯一入口 [`crate::Loader::builder`]:累积阶段(`path`/`bytes`/`scan_dir`)逐 bundle 做
/// Phase A,`build()` 收尾时跑一次 Phase B 全局依赖解析。
pub struct Environment {
    /// 跨全部 AssetCollection eager parse 成功的对象。
    pub(crate) objects: Vec<EnvironmentObject>,
    /// 每个 SerializedFile 的逻辑分组（含依赖解析表）。
    pub(crate) collections: Vec<AssetCollection>,
    /// `.resS` / `.resource` 流数据。
    pub(crate) resources: Vec<ResourceFile>,
    /// 解析失败记录（错误隔离，不中断整体）。
    pub(crate) failures: Vec<ParseFailure>,
    /// 运行时扩展点（build 时从 builder move 入）。`None` 时无懒加载/资源现取。
    pub(crate) dependency_provider: Option<Arc<dyn DependencyProvider>>,
    pub(crate) resource_provider: Option<Arc<dyn ResourceProvider>>,
}

/// 一个已解析的对象。
pub struct EnvironmentObject {
    /// 所属 AssetCollection 句柄。
    pub collection: CollectionId,
    pub path_id: i64,
    /// class id(开集 newtype;含 MonoBehaviour 等codegen 跳过的 class)。
    pub class_id: ClassId,
    pub object: ParsedObject,
}

impl EnvironmentObject {
    /// 该对象的全局身份(`collection` + `path_id`)。
    pub fn id(&self) -> ObjectId {
        ObjectId {
            collection: self.collection,
            path_id: self.path_id,
        }
    }
}

/// 一个 SerializedFile 的逻辑分组。对齐 AssetRipper `AssetCollection`。
pub struct AssetCollection {
    /// 原始 bundle node 名（如 `CAB-xxxx.assets`）。
    pub name: String,
    /// `fix_file_identifier(name)`——与 FileIdentifier / resource 的 name_fixed 精确匹配。
    pub name_fixed: String,
    pub unity_version: UnityVersion,
    pub endian: Endian,
    /// 依赖列表。`[0]` = 自身句柄；`[1..]` = 解析出的外部 collection 句柄。
    ///
    /// PPtr.FileID 直接索引此列表（`0` = 自身）——与 AssetRipper
    /// `AssetCollection.Dependencies` 约定一致。`None` = 未解析的跨 bundle 引用。
    pub dependencies: Vec<Option<CollectionId>>,
    /// `path_id` → `Environment.objects` 索引(内部存储细节,非身份)。
    pub assets: HashMap<i64, usize>,
}

/// `.resS` / `.resource` 流数据。对齐 AssetRipper `ResourceFile`。
pub struct ResourceFile {
    pub name: String,
    pub name_fixed: String,
    pub bytes: Bytes,
}

/// 解析失败记录（错误隔离）。
#[derive(Debug)]
pub enum ParseFailure {
    /// SerializedFile 整体解析失败（file-level）。
    File { name: String, error: Error },
    /// 单个对象解析失败（object-level）。
    Object {
        collection: CollectionId,
        path_id: i64,
        class_id: ClassId,
        error: Error,
    },
    /// io 层子容器/子条目递归 parse 失败（container-level）。
    ///
    /// 对应 [`crate::format::LoadedFile::Failed`]:保留了原始字节与错误,
    /// 但 `phase_a` 无法把它当作 SerializedFile/Resource 处理,故记录为失败。
    /// `name` 是原始 node 名(如 `CAB-xxxx.assets`)。
    Container { name: String, error: Error },
}

/// PPtr 引用的抽象：typed `PPtr`（unity-core）与 dynamic `PPtrValue`（typetree）统一。
pub trait PPtrLike {
    fn file_id(&self) -> i32;
    fn path_id(&self) -> i64;
}

impl PPtrLike for PPtr {
    fn file_id(&self) -> i32 {
        self.m_file_id
    }
    fn path_id(&self) -> i64 {
        self.m_path_id
    }
}

impl PPtrLike for PPtrValue {
    fn file_id(&self) -> i32 {
        self.file_id
    }
    fn path_id(&self) -> i64 {
        self.path_id
    }
}

/// 扩展点:外部引用(`FileIdentifier`)在已加载 collection 中未命中时的回调。
///
/// 对齐 AssetRipper `IDependencyProvider`,但**只负责\"打开字节\"**——Loader/
/// `resolve_dependencies` 接到 [`NamedBytes`] 后自己跑 `load_recursive` + `phase_a`
/// (sniff、递归解析、`name_fixed` 归一化都在内部,不劳 Provider 构造)。旧实现要求
/// Provider 返回已解析的 `LoadedFile` 并自备 `Registry`/命名规则,职责过重。
///
/// 在 `build` 的 Phase B 中实际回调:某个 collection 的 external(按 `name_fixed`
/// 匹配)在已加载的全部 collection 中找不到时,调 `open`;返回 `Ok(Some(_))` 则
/// 追加加载该依赖并重试匹配,**新依赖引入的 externals 也会递归处理**(传递依赖,
/// 直到不动点;上限 `MAX_LAZY_LOADS = 4096` 防恶意循环)。默认 no-op。
///
/// `Send + Sync` 约束对齐未来 pyo3 / async 场景:provider 以 `Arc<dyn Trait>` 形式
/// 存入 [`Environment`]。
pub trait DependencyProvider: Send + Sync {
    /// 返回 `Ok(Some(NamedBytes))` 则 Loader 内部加载该依赖;`Ok(None)` 则该引用
    /// 留空(`resolve` 返回 `None`);`Err` 记录为 `ParseFailure::Container`。
    fn open(&self, _req: &DependencyRequest<'_>) -> Result<Option<NamedBytes>, ProviderError> {
        Ok(None)
    }
}

/// 扩展点:`.resS` 在已加载 resources 中未命中时的回调。
///
/// 对齐 AssetRipper `IResourceProvider`,**只负责\"打开字节\"**——Loader/
/// `prefetch_resources` 接到 `Bytes` 后自己算 `name_fixed` 并注册进 `resources`
/// (旧实现要求 Provider 构造带 `name_fixed` 的 `ResourceFile`,职责过重)。
///
/// 在 `build` 末尾 `prefetch_resources` 中实际回调:扫描已加载 Texture2D
/// (`m_stream_data.path`)与 AudioClip(`m_resource.m_source` / pre-5.0 隐式
/// `collection.name + ".resS"`)的 streaming 引用,对未命中的 name 调 `open`,
/// 返回 `Ok(Some(_))` 则注册进 `resources`。默认 no-op。
///
/// `Send + Sync` 约束同 [`DependencyProvider`]。
pub trait ResourceProvider: Send + Sync {
    /// 返回 `Ok(Some(Bytes))` 则注册进全局 resources;`Ok(None)` 则 streaming 取数
    /// 返回 `None`;`Err` 记录为 `ParseFailure::Container`。
    fn open(&self, _req: &ResourceRequest<'_>) -> Result<Option<Bytes>, ProviderError> {
        Ok(None)
    }
}

/// Provider 打开依赖时返回的原始字节 + 名称。Loader 内部据此跑 `load_recursive`。
#[derive(Debug, Clone)]
pub struct NamedBytes {
    /// 建议的来源名(日志/诊断用)。Loader 不依赖此值做匹配——匹配走
    /// `fix_file_identifier` 归一化后的 name(由 Loader 自己算)。
    pub name: String,
    pub bytes: Bytes,
}

/// Provider 打开依赖时传入的请求:来源 collection + 原始 FileIdentifier + 归一化名。
#[derive(Debug, Clone)]
pub struct DependencyRequest<'a> {
    /// 发起该依赖的 collection。
    pub source_collection: CollectionId,
    /// 原始 FileIdentifier(含 `path_name_origin` / `guid` / `type_`)。
    pub identifier: &'a FileIdentifier,
    /// `fix_file_identifier(identifier.path_name)`——已归一化的 name,
    /// Provider 可直接按此查表。
    pub normalized_name: &'a str,
}

/// Provider 打开 `.resS` 时传入的请求:来源 collection + 原始路径 + 归一化名。
#[derive(Debug, Clone)]
pub struct ResourceRequest<'a> {
    /// 发起该 streaming 引用的对象所在 collection。
    pub source_collection: CollectionId,
    /// 原始路径(如 `archive:/CAB-xxx/CAB-xxx.resS` 或裸 `xxx.resS`)。
    pub original_path: &'a str,
    /// `fix_file_identifier(original_path)`——已归一化的 name。
    pub normalized_name: &'a str,
}

/// Provider 错误。记录为 `ParseFailure::Container` 不中断整体 build。
#[derive(Debug, Clone)]
pub struct ProviderError(pub String);

impl std::fmt::Display for ProviderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
impl std::error::Error for ProviderError {}

impl EnvironmentObject {
    /// 若该对象是 MonoBehaviour（classID=114），返回 typed shell + 动态 structure 引用。
    ///
    /// shell 字段 `m_script`/`m_game_object` 均为 `PPtrValue`（已实现 `PPtrLike`），
    /// 可直接用 `env.resolve_from(self, &mb.m_script)` 解引用到目标资产。
    pub fn as_mono_behaviour(&self) -> Option<&MonoBehaviour> {
        match &self.object {
            ParsedObject::MonoBehaviour(mb) => Some(mb),
            _ => None,
        }
    }
}

impl Environment {
    /// Phase B：初始化依赖列表（全局 name_fixed 精确匹配 + [`DependencyProvider`] 懒加载）。
    ///
    /// 遍历 `pending` 中每个 collection 的 externals，按 `name_fixed` 在**全部**
    /// `self.collections` 中查找目标——跨 bundle 引用因此能命中。未命中的外部引用：
    /// - 无 `dependency_provider` → 留 `None`（resolve 返回 `None`）。
    /// - 有 provider → 调 `find_dependency`，返回 `Some(LoadedFile)` 则追加 `phase_a` 加载
    ///   并重试匹配；**新加载 bundle 引入的 externals 也加入 pending 继续处理**（传递
    ///   依赖，递归直到不动点）。
    ///
    /// 防恶意循环：`tried` 去重（同一 `path_name` 只触发一次懒加载）+ `MAX_LAZY_LOADS`
    /// 上限（4096，远超任何真实场景）。对齐 AssetRipper `InitializeAllDependencyLists`。
    pub(crate) fn resolve_dependencies(
        &mut self,
        pending: &mut Vec<(CollectionId, Vec<FileIdentifier>)>,
    ) {
        /// 懒加载次数上限（防恶意/损坏数据导致无限递归）。真实 bundle 依赖图远小于此。
        const MAX_LAZY_LOADS: usize = 4096;

        let has_provider = self.dependency_provider.is_some();
        let mut tried: HashSet<String> = HashSet::new();
        let mut lazy_loads = 0usize;

        let mut i = 0;
        while i < pending.len() {
            // 取出当前 collection 的 externals 快照(owned clone,含 path_name + guid 等)
            // + col_id,随即结束对 `pending` 的不可变借用,以便后续 phase_a 可 &mut pending。
            let (col_id, externals_snap): (CollectionId, Vec<FileIdentifier>) = {
                let (col_id, externals) = &pending[i];
                (*col_id, externals.clone())
            };

            for id in &externals_snap {
                let name = id.path_name.clone();
                let mut target = self
                    .collections
                    .iter()
                    .position(|c| c.name_fixed == name)
                    .map(CollectionId::new);

                // 懒加载：未命中 + 有 provider + 该 name 尚未尝试过 + 未超上限。
                if target.is_none() && has_provider && tried.insert(name.clone()) {
                    if lazy_loads >= MAX_LAZY_LOADS {
                        log::warn!(
                            "resolve_dependencies: 懒加载次数达上限 {MAX_LAZY_LOADS}，停止懒加载"
                        );
                    } else {
                        // Provider 只负责"打开字节"。Loader 自己跑 load_recursive。
                        let req = DependencyRequest {
                            source_collection: col_id,
                            identifier: id,
                            normalized_name: &name,
                        };
                        let provider = self.dependency_provider.clone(); // Arc clone，廉价
                        // provider 一定存在(has_provider 已校验);直接调 open。
                        let opened = provider
                            .as_ref()
                            .expect("has_provider 已校验")
                            .open(&req);
                        match opened {
                            Ok(Some(named)) => {
                                lazy_loads += 1;
                                // 用 Provider 返回的原始字节跑 load_recursive。
                                let registry = Registry::defaults();
                                match crate::format::load_recursive(
                                    named.bytes,
                                    &named.name,
                                    &registry,
                                    None,
                                ) {
                                    Ok(loaded) => {
                                        // phase_a 把新 bundle 灌进 self 的各 Vec;
                                        // 新 collection 的 externals 被 push 进 pending(传递依赖入队)。
                                        phase_a(
                                            loaded,
                                            &mut self.objects,
                                            &mut self.collections,
                                            &mut self.resources,
                                            &mut self.failures,
                                            pending,
                                        );
                                        target = self
                                            .collections
                                            .iter()
                                            .position(|c| c.name_fixed == name)
                                            .map(CollectionId::new);
                                    }
                                    Err(e) => {
                                        self.failures.push(ParseFailure::Container {
                                            name: named.name.clone(),
                                            error: e,
                                        });
                                    }
                                }
                            }
                            Ok(None) => {} // provider 无此依赖,留 None
                            Err(pe) => {
                                self.failures.push(ParseFailure::Container {
                                    name: name.clone(),
                                    error: Error::Decompress {
                                        reason: format!("dependency provider: {pe}"),
                                    },
                                });
                            }
                        }
                    }
                }
                self.collections[col_id.to_index()].dependencies.push(target);
            }
            i += 1;
        }
    }

    /// 预拉取缺失的 `.resS` 资源（[`ResourceProvider`] 回调）。
    ///
    /// 在 [`resolve_dependencies`] 之后、`build` 返回前调用。扫描已加载对象的
    /// streaming 引用,对未在 `resources` 命中的 name 调 `open`,命中则注册进
    /// `resources`——之后 `streaming_data` / `streamed_resource_data` /
    /// `read_old_streamed_audio` 等只读查找即能命中。
    ///
    /// 覆盖的 streaming 引用来源（对齐聚合层取数路径）：
    /// - **Texture2D** `m_stream_data.path`（`streaming_data` 入口）。
    /// - **AudioClip** `m_resource.m_source`（post-5.0，`streamed_resource_data` 入口）。
    /// - **AudioClip** pre-5.0 `m_audio_data == Streamed`（`read_old_streamed_audio`
    ///   用 `collection.name + ".resS"` 隐式名）。
    ///
    /// 无 provider 时 no-op。其他对象类型若有 streaming 引用会漏（`resolve_resource`
    /// 仍返回 `None`，与现状一致，不回归）。
    pub(crate) fn prefetch_resources(&mut self) {
        let Some(provider) = &self.resource_provider else {
            return;
        };
        // 先收集全部 wanted (原始 path, 来源 collection)——不可变借 objects,再释放借用后
        // mutate resources。
        let mut wanted: Vec<(String, CollectionId)> = Vec::new();
        for o in &self.objects {
            match &o.object {
                ParsedObject::Typed(UnityObject::Texture2D(t)) => {
                    if let Some(sd) = &t.m_stream_data
                        && !sd.path.is_empty()
                    {
                        wanted.push((sd.path.clone(), o.collection));
                    }
                }
                ParsedObject::Typed(UnityObject::AudioClip(a)) => {
                    if let Some(r) = &a.m_resource
                        && !r.m_source.is_empty()
                    {
                        wanted.push((r.m_source.clone(), o.collection));
                    }
                    // pre-5.0 隐式 .resS 名 = collection.name + ".resS"
                    if matches!(a.m_audio_data, Some(AudioClipMAudioData::Streamed { .. }))
                        && let Some(col) = self.collections.get(o.collection.to_index())
                    {
                        wanted.push((format!("{}.resS", col.name), o.collection));
                    }
                }
                _ => {}
            }
        }
        // provider 调用:对每个未命中的 name 查一次(去重 + name_fixed 归一化匹配)。
        // 克隆 Arc 以释放对 self 的借用,便于后续 self.resources.push。
        let provider = provider.clone();
        for (path, source_collection) in wanted {
            let fixed = fix_file_identifier(&path);
            let already = self.resources.iter().any(|r| r.name_fixed == fixed);
            if already {
                continue;
            }
            // Provider 只返回 Bytes;Loader 自己算 name_fixed 并构造 ResourceFile。
            let req = ResourceRequest {
                source_collection,
                original_path: &path,
                normalized_name: &fixed,
            };
            match provider.open(&req) {
                Ok(Some(bytes)) => {
                    // 防御:若 name_fixed 与已存在重复,跳过(不覆盖)。
                    let dup = self.resources.iter().any(|r| r.name_fixed == fixed);
                    if !dup {
                        self.resources.push(ResourceFile {
                            name: path.clone(),
                            name_fixed: fixed,
                            bytes,
                        });
                    }
                }
                Ok(None) => {} // provider 无此资源,留空
                Err(pe) => {
                    self.failures.push(ParseFailure::Container {
                        name: path.clone(),
                        error: Error::Decompress {
                            reason: format!("resource provider: {pe}"),
                        },
                    });
                }
            }
        }
    }

    // ── PPtr 解引用 ──

    /// 解引用 PPtr。对齐 `AssetCollection.TryGetAsset(fileIndex, pathID)`：
    /// `dependencies[file_id]` → target collection → `assets[path_id]`。
    ///
    /// `file_id == 0` 天然落到 `dependencies[0] = Some(self)`。
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

    /// 便捷：从源对象解析（自动取 `source.collection`）。
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
    ) -> Option<&EnvironmentObject> {
        self.resolve(collection, 0, path_id)
    }

    /// 用 [`ObjectId`] 解引用,返回绑定了 `&Environment` 生命周期的 [`ObjectRef`]。
    ///
    /// 仅校验该 id 在环境内可解析(`collection` 合法 + `path_id` 命中其 assets),
    /// 实际对象取回在 [`ObjectRef::get`] 时做。
    pub fn resolve_ref(&self, id: ObjectId) -> Option<ObjectRef<'_>> {
        let exists = self
            .collections
            .get(id.collection.to_index())
            .and_then(|c| c.assets.get(&id.path_id))
            .is_some();
        if exists {
            Some(ObjectRef { env: self, id })
        } else {
            None
        }
    }

    /// 由 `&EnvironmentObject` 构造 [`ObjectRef`]。
    pub fn object_ref(&self, obj: &EnvironmentObject) -> ObjectRef<'_> {
        ObjectRef {
            env: self,
            id: obj.id(),
        }
    }

    /// 由下标构造 [`CollectionId`](仅供调用方从 `collections()` 迭代时取句柄)。
    pub fn collection_id(&self, index: usize) -> Option<CollectionId> {
        (index < self.collections.len()).then_some(CollectionId::new(index))
    }

    // ── streaming_data ──

    /// 取 `StreamingInfo` 指向的流数据（零拷贝 `Bytes` slice）。
    ///
    /// 对齐 `StreamingInfoExtensions.GetContent`：空路径 → `None`；`size == 0` → `None`；
    /// offset+size 溢出或越界 → `None`。命中后 `Bytes::slice`（共享 Arc，零拷贝）。
    pub fn streaming_data(&self, info: &StreamingInfo) -> Option<Bytes> {
        if info.path.is_empty() || info.size == 0 {
            return None;
        }
        let offset = streaming_offset(&info.offset);
        let end = offset.checked_add(info.size as u64)?;
        let res = self.resolve_resource(&info.path)?;
        if (res.bytes.len() as u64) < end {
            return None;
        }
        Some(res.bytes.slice(offset as usize..end as usize))
    }

    /// 取 `StreamedResource`（AudioClip）指向的流数据。
    pub fn streamed_resource_data(&self, res: &StreamedResource) -> Option<Bytes> {
        if res.m_source.is_empty() || res.m_size == 0 {
            return None;
        }
        let end = res.m_offset.checked_add(res.m_size)?;
        let file = self.resolve_resource(&res.m_source)?;
        if (file.bytes.len() as u64) < end {
            return None;
        }
        Some(file.bytes.slice(res.m_offset as usize..end as usize))
    }

    /// 取 AudioClip 的音频字节（零拷贝 `Bytes` slice）。
    ///
    /// 对齐 AssetRipper `AudioClipExtensions.GetAudioData` 的三路分发：
    /// - pre-5.0 `m_audio_data` 为 `Inline` → 返回内联字节（parse 时已读）。
    /// - pre-5.0 `m_audio_data` 为 `Streamed`（`m_Stream==2`）→ 按 `{size, offset}`
    ///   从 `<collection.name>.resS` 取。resource 名 = `collection.name + ".resS"`
    ///   （对齐 AssetRipper `AudioClipHelper.ReadStreamedByteArray`）。
    /// - post-5.0 无 `m_audio_data` 但有 `m_resource` → 走 [`streamed_resource_data`]。
    ///
    /// 非 AudioClip 或取不到 → `None`。
    pub fn audio_data(&self, obj: &EnvironmentObject) -> Option<Bytes> {
        let ParsedObject::Typed(UnityObject::AudioClip(ac)) = &obj.object else {
            return None;
        };
        if let Some(data) = &ac.m_audio_data {
            return match data {
                AudioClipMAudioData::Inline(v) => Some(Bytes::from(v.clone())),
                AudioClipMAudioData::Streamed { size, offset } => {
                    self.read_old_streamed_audio(obj.collection, *offset, *size)
                }
            };
        }
        // post-5.0：走 StreamedResource（m_resource）。
        ac.m_resource
            .as_ref()
            .and_then(|res| self.streamed_resource_data(res))
    }

    /// pre-5.0 AudioClip streaming：按 `{offset, size}` 从 `<collection.name>.resS` 取。
    ///
    /// 对齐 AssetRipper `AudioClipHelper.ReadStreamedByteArray`：
    /// `resourceFileName = audioClip.Collection.Name + ".resS"`，在 bundle 内
    /// `ResolveResource`（经 `fix_file_identifier` 精确匹配）。越界 → `None`。
    fn read_old_streamed_audio(
        &self,
        collection: CollectionId,
        offset: u32,
        size: u32,
    ) -> Option<Bytes> {
        if size == 0 {
            return None;
        }
        let col = self.collections.get(collection.to_index())?;
        let resource_name = format!("{}.resS", col.name);
        let file = self.resolve_resource(&resource_name)?;
        let end = (offset as u64).checked_add(size as u64)?;
        if (file.bytes.len() as u64) < end {
            return None;
        }
        Some(file.bytes.slice(offset as usize..end as usize))
    }

    /// 对齐 `Bundle.ResolveResource`：空名 → `None`，否则 `name_fixed` 精确匹配。
    fn resolve_resource(&self, name: &str) -> Option<&ResourceFile> {
        if name.is_empty() {
            return None;
        }
        let fixed = fix_file_identifier(name);
        self.resources.iter().find(|r| r.name_fixed == fixed)
    }

    // ── 访问器 ──

    pub fn objects(&self) -> &[EnvironmentObject] {
        &self.objects
    }

    /// 按 [`ClassIDType`] 过滤对象(typed 与 dynamic 均含)。
    ///
    /// 常见场景入口。**无法**查 MonoBehaviour(114)——codegen 跳过 114,
    /// `ClassIDType` 无此变体;用 [`Environment::objects_of_class_id`] 配
    /// [`ClassId::MONO_BEHAVIOUR`] 查,或用便捷的 [`Environment::mono_behaviours`]。
    pub fn objects_of_class(
        &self,
        cls: ClassIDType,
    ) -> impl Iterator<Item = &EnvironmentObject> {
        let target = ClassId::from(cls);
        self.objects.iter().filter(move |o| o.class_id == target)
    }

    /// 按开集 [`ClassId`] 过滤对象(可表示 codegen 跳过的 class,如 MonoBehaviour)。
    pub fn objects_of_class_id(
        &self,
        cls: ClassId,
    ) -> impl Iterator<Item = &EnvironmentObject> {
        self.objects.iter().filter(move |o| o.class_id == cls)
    }

    /// 全部 MonoBehaviour 对象(`class_id == 114`)。
    ///
    /// 结果可通过 [`EnvironmentObject::as_mono_behaviour`] 取 typed shell。
    pub fn mono_behaviours(&self) -> impl Iterator<Item = &EnvironmentObject> {
        self.objects_of_class_id(ClassId::MONO_BEHAVIOUR)
    }

    pub fn collections(&self) -> &[AssetCollection] {
        &self.collections
    }

    /// 按 `name_fixed`（`fix_file_identifier` 归一化后的名）查 collection。
    pub fn collection_by_name(&self, fixed_name: &str) -> Option<&AssetCollection> {
        self.collections.iter().find(|c| c.name_fixed == fixed_name)
    }

    pub fn resources(&self) -> &[ResourceFile] {
        &self.resources
    }

    pub fn failures(&self) -> &[ParseFailure] {
        &self.failures
    }
}

/// `.resS` / `.ress` / `.resource` 后缀判定（大小写无关）。对齐 AssetRipper
/// `ResourceFile.IsDefaultResourceFile`。
fn is_resource_file(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    lower.ends_with(".ress") || lower.ends_with(".resource")
}

/// Phase A：读 `LoadedFile` 的全部叶子 + eager parse 对象 + 收集资源，灌入给定目标。
///
/// 这是 [`crate::Loader`] 的共享实现。每个 SerializedFile → 一个
/// `AssetCollection`（`dependencies[0]=Some(self)`），`.resS` → `ResourceFile`
/// （`copy_from_slice` detach）。externals 暂存到 `pending`，等
/// [`Environment::resolve_dependencies`] 在全部 collection 建好后统一解析（跨 bundle
/// 前向/后向引用都能命中——对齐 AssetRipper 两阶段加载）。
pub(crate) fn phase_a(
    loaded: LoadedFile,
    objects: &mut Vec<EnvironmentObject>,
    collections: &mut Vec<AssetCollection>,
    resources: &mut Vec<ResourceFile>,
    failures: &mut Vec<ParseFailure>,
    pending: &mut Vec<(CollectionId, Vec<FileIdentifier>)>,
) {
    // Error 未实现 Clone(含 io::Error),所以先借 leaves 处理 SerializedFile/Resource,
    // 再 consume 整棵树取 Failed 节点的 owning Error。两次遍历都不修改树,顺序无依赖。
    process_leaves(&loaded, objects, collections, resources, failures, pending);
    for (name, error) in loaded.into_failures() {
        failures.push(ParseFailure::Container { name, error });
    }
}

/// `phase_a` 的 leaf 处理:对每个 `SerializedFile` 叶子 eager parse 全部对象,
/// 对 `.resS`/`.resource` 叶子 detach 成 `ResourceFile`。
fn process_leaves(
    loaded: &LoadedFile,
    objects: &mut Vec<EnvironmentObject>,
    collections: &mut Vec<AssetCollection>,
    resources: &mut Vec<ResourceFile>,
    failures: &mut Vec<ParseFailure>,
    pending: &mut Vec<(CollectionId, Vec<FileIdentifier>)>,
) {
    for (name, bytes, _unity_revision, format) in loaded.leaves() {
        if is_resource_file(name) {
            // detach：copy_from_slice 把资源拷到独立分配，断开与 blob Arc 的共享。
            // 这样 SerializedFile 解析完毕后整个解压 blob 可被释放，避免 .assets
            // 区域因 .resS 切片的 Arc 共享而驻留（详见 docs/memory-notes.md）。
            resources.push(ResourceFile {
                name_fixed: fix_file_identifier(name),
                name: name.to_string(),
                bytes: Bytes::copy_from_slice(bytes),
            });
            continue;
        }
        if format != FileFormat::SerializedFile {
            log::debug!("phase_a: 跳过非 SerializedFile 叶子 {name:?}（sniff={format:?}）");
            continue;
        }
        let sf = match SerializedFile::parse(bytes.clone()) {
            Ok(sf) => sf,
            Err(e) => {
                failures.push(ParseFailure::File { name: name.to_string(), error: e });
                continue;
            }
        };
        let col_id = CollectionId::new(collections.len());
        let unity_version = sf.unity_version_parsed;
        let endian = sf.endian;
        let name = name.to_string();
        let name_fixed = fix_file_identifier(&name);
        let refs = sf.ref_type_table();
        let mut assets = HashMap::new();
        for obj in &sf.objects {
            let class_id = ClassId::from(obj.class_id(&sf));
            match parse_object(&sf, obj, &refs) {
                Ok(parsed) => {
                    let obj_idx = objects.len();
                    objects.push(EnvironmentObject {
                        collection: col_id,
                        path_id: obj.path_id,
                        class_id,
                        object: parsed,
                    });
                    assets.insert(obj.path_id, obj_idx);
                }
                Err(e) => {
                    failures.push(ParseFailure::Object {
                        collection: col_id,
                        path_id: obj.path_id,
                        class_id,
                        error: e,
                    });
                }
            }
        }
        // sf 不再被借用，提取 externals（resolve_dependencies 用）。
        let externals = sf.externals;
        collections.push(AssetCollection {
            name,
            name_fixed,
            unity_version,
            endian,
            dependencies: vec![Some(col_id)], // [0] = self
            assets,
        });
        pending.push((col_id, externals));
    }
}

/// 提取 `StreamingInfoOffsetVariant` 的 offset（统一为 `u64`）。对齐
/// `StreamingInfoExtensions.GetOffset`。
fn streaming_offset(offset: &StreamingInfoOffsetVariant) -> u64 {
    match offset {
        StreamingInfoOffsetVariant::Default(o) => *o as u64,
        StreamingInfoOffsetVariant::V2020_1(o) => *o,
    }
}
