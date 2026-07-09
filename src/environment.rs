//! 环境聚合层：从 io 层的 `Bundle` 构建 `Environment`——eager parse 全部 SerializedFile
//! 的对象，聚合 `.resS` 流数据，提供 PPtr 解引用与 streaming_data 取数。
//!
//! 设计与命名参照 AssetRipper（`GameBundle` / `AssetCollection` / `ResourceFile` /
//! `SpecialFileNames`）。详见 `docs/environment-layer.md`。

use std::collections::HashMap;

use bytes::Bytes;
use generated::subclasses::streamed_resource::StreamedResource;
use generated::subclasses::streaming_info::{StreamingInfo, StreamingInfoOffsetVariant};
use generated::ClassIDType;

use crate::decompress::Registry;
use crate::names::fix_file_identifier;
use crate::object_parser::{parse_object, MonoBehaviour, ParsedObject};
use crate::serialized_file::FileIdentifier;
use crate::typetree::PPtrValue;
use crate::{Bundle, Endian, Error, PPtr, SerializedFile, UnityVersion};

/// 资产环境：聚合多个 bundle 的全部对象 + `.resS` 资源 + PPtr 依赖解析。
///
/// 等价于 AssetRipper 的 `GameBundle` + 全部 `AssetCollection`。数据模型天然支持跨 bundle：
/// `objects`/`collections`/`resources` 全是全局扁平 Vec，`resolve()` 走全局索引，
/// Phase B 依赖解析全局扫描 `collections`——只要把多个 bundle 灌进同一个 `Environment`，
/// 跨 bundle 引用在 Phase B（全部 collection 建好后）自动 resolve。
///
/// 构造见 [`Environment::builder`]（累积多 bundle）或 [`Environment::from_bundle`]（单 bundle）。
/// 便捷入口见 `load_environment_multi` / `load_environment_dir`。
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

/// 一个已解析的对象。
pub struct EnvironmentObject {
    /// 所属 AssetCollection 索引。
    pub collection: usize,
    pub path_id: i64,
    pub class_id: i32,
    pub object: ParsedObject,
}

/// 一个 SerializedFile 的逻辑分组。对齐 AssetRipper `AssetCollection`。
pub struct AssetCollection {
    /// 原始 bundle node 名（如 `CAB-xxxx.assets`）。
    pub name: String,
    /// `fix_file_identifier(name)`——与 FileIdentifier / resource 的 name_fixed 精确匹配。
    pub name_fixed: String,
    pub unity_version: UnityVersion,
    pub endian: Endian,
    /// 依赖列表。`[0]` = 自身索引；`[1..]` = 解析出的外部 collection 索引。
    ///
    /// PPtr.FileID 直接索引此列表（`0` = 自身）——与 AssetRipper
    /// `AssetCollection.Dependencies` 约定一致。`None` = 未解析的跨 bundle 引用。
    pub dependencies: Vec<Option<usize>>,
    /// `path_id` → `Environment.objects` 索引。
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
        collection: usize,
        path_id: i64,
        class_id: i32,
        error: Error,
    },
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

/// 扩展点：外部引用（`FileIdentifier`）在已加载 collection 中未命中时的回调。
///
/// 对齐 AssetRipper `IDependencyProvider`。默认实现 no-op（返回 `None`，依赖留空）。
/// 当前 eager 模型在 [`EnvironmentBuilder::build`] 中尚未实际回调此 trait；
/// 后续接入懒加载（按需加载缺失 bundle）时实现即可，无需改公共 API。
pub trait DependencyProvider {
    /// 返回 `Some(Bundle)` 则追加加载该依赖；`None` 则该引用留空（resolve 返回 `None`）。
    fn find_dependency(&self, _id: &FileIdentifier) -> Option<Bundle> {
        None
    }
}

/// 扩展点：`.resS` 在已加载 resources 中未命中时的回调。
///
/// 对齐 AssetRipper `IResourceProvider`。默认实现 no-op。
/// 后续接入磁盘 `.resS` 现取（跨 bundle 共享的 loose resource）时实现即可。
pub trait ResourceProvider {
    /// 返回 `Some(ResourceFile)` 则注册进全局 resources；`None` 则 streaming 取数返回 `None`。
    fn find_resource(&self, _name: &str) -> Option<ResourceFile> {
        None
    }
}

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
    /// 空构造。
    fn new() -> Environment {
        Environment {
            objects: Vec::new(),
            collections: Vec::new(),
            resources: Vec::new(),
            failures: Vec::new(),
        }
    }

    /// 从 io 层的 `Bundle` 构建 `Environment`（infallible——错误隔离进 `failures`）。
    ///
    /// 单 bundle 便捷入口。多 bundle 场景用 [`Environment::builder`]。
    pub fn from_bundle(bundle: Bundle) -> Environment {
        let mut env = Environment::new();
        let mut pending = Vec::new();
        env.add_bundle_inner(bundle, &mut pending);
        env.resolve_dependencies(&mut pending);
        env
    }

    /// 新建 [`EnvironmentBuilder`]——累积多个 bundle 后 `build()` 一次性做全局依赖解析。
    pub fn builder() -> EnvironmentBuilder {
        EnvironmentBuilder::new()
    }

    /// Phase A：读 bundle 的全部文件 + eager parse 对象 + 收集资源。
    ///
    /// 把每个 SerializedFile 灌成一个 `AssetCollection`（`dependencies[0]=Some(self)`），
    /// `.resS` 灌成 `ResourceFile`（`copy_from_slice` detach）。externals 暂存到 `pending`，
    /// 等 [`resolve_dependencies`] 在全部 collection 建好后统一解析（这样跨 bundle 的
    /// 前向/后向引用都能命中——对齐 AssetRipper 两阶段加载）。
    fn add_bundle_inner(
        &mut self,
        bundle: Bundle,
        pending: &mut Vec<(usize, Vec<FileIdentifier>)>,
    ) {
        phase_a(
            bundle,
            &mut self.objects,
            &mut self.collections,
            &mut self.resources,
            &mut self.failures,
            pending,
        );
    }

    /// Phase B：初始化依赖列表（全局 name_fixed 精确匹配）。
    ///
    /// 遍历 `pending` 中每个 collection 的 externals，按 `name_fixed` 在**全部**
    /// `self.collections` 中查找目标——跨 bundle 引用因此能命中。未命中的外部引用留 `None`
    /// （resolve 时返回 `None`）。对齐 AssetRipper `InitializeAllDependencyLists`。
    ///
    /// 当前 eager 模型不回调 [`DependencyProvider`]（TODO：在 `target.is_none()` 分支按需
    /// 加载缺失 bundle），故此方法只接收 `pending`。
    fn resolve_dependencies(&mut self, pending: &mut [(usize, Vec<FileIdentifier>)]) {
        for (col_idx, externals) in pending.iter() {
            for identifier in externals {
                let target = self
                    .collections
                    .iter()
                    .position(|c| c.name_fixed == identifier.path_name);
                // TODO(扩展点): target.is_none() 时调 dependency_provider.find_dependency(identifier)
                //   → 返回 Some(Bundle) 则 add_bundle_inner + 重试查找（懒加载跨 bundle 引用）。
                self.collections[*col_idx].dependencies.push(target);
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

    /// 便捷：从源对象解析（自动取 `source.collection`）。
    pub fn resolve_from(
        &self,
        source: &EnvironmentObject,
        pptr: &impl PPtrLike,
    ) -> Option<&EnvironmentObject> {
        self.resolve(source.collection, pptr.file_id(), pptr.path_id())
    }

    /// `file_id == 0` 的便捷场景：同 collection 内 `path_id` 查找。
    pub fn resolve_local(&self, collection: usize, path_id: i64) -> Option<&EnvironmentObject> {
        self.resolve(collection, 0, path_id)
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

    /// 按 classID 过滤对象（typed 与 dynamic 均含）。
    pub fn objects_of_class(&self, cls: ClassIDType) -> impl Iterator<Item = &EnvironmentObject> {
        let target = cls as i32;
        self.objects.iter().filter(move |o| o.class_id == target)
    }

    /// 全部 MonoBehaviour 对象（classID=114）。
    ///
    /// 注：codegen 未生成 `ClassIDType::MonoBehaviour`（跳过 114），故不能走
    /// `objects_of_class`；用专用方法按 `class_id == 114` 过滤。结果可通过
    /// `EnvironmentObject::as_mono_behaviour()` 取 typed shell。
    pub fn mono_behaviours(&self) -> impl Iterator<Item = &EnvironmentObject> {
        self.objects.iter().filter(|o| o.class_id == 114)
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

/// Phase A：读 bundle 的全部文件 + eager parse 对象 + 收集资源，灌入给定目标。
///
/// 这是 `Environment::add_bundle_inner` 与 `EnvironmentBuilder::add_bundle` 的共享实现
/// （两者字段同构）。每个 SerializedFile → 一个 `AssetCollection`（`dependencies[0]=Some(self)`），
/// `.resS` → `ResourceFile`（`copy_from_slice` detach）。externals 暂存到 `pending`，
/// 等 [`Environment::resolve_dependencies`] 在全部 collection 建好后统一解析（跨 bundle
/// 前向/后向引用都能命中——对齐 AssetRipper 两阶段加载）。
fn phase_a(
    bundle: Bundle,
    objects: &mut Vec<EnvironmentObject>,
    collections: &mut Vec<AssetCollection>,
    resources: &mut Vec<ResourceFile>,
    failures: &mut Vec<ParseFailure>,
    pending: &mut Vec<(usize, Vec<FileIdentifier>)>,
) {
    for bf in bundle.files {
        if is_resource_file(&bf.name) {
            // detach：copy_from_slice 把资源拷到独立分配，断开与 blob Arc 的共享。
            // 这样 SerializedFile 解析完毕后整个解压 blob 可被释放，避免 .assets
            // 区域因 .resS 切片的 Arc 共享而驻留（详见 docs/memory-notes.md）。
            resources.push(ResourceFile {
                name_fixed: fix_file_identifier(&bf.name),
                name: bf.name,
                bytes: Bytes::copy_from_slice(&bf.bytes),
            });
            continue;
        }
        let sf = match SerializedFile::parse(bf.bytes) {
            Ok(sf) => sf,
            Err(e) => {
                failures.push(ParseFailure::File { name: bf.name, error: e });
                continue;
            }
        };
        let col_idx = collections.len();
        let unity_version = sf.unity_version_parsed;
        let endian = sf.endian;
        let name = bf.name.clone();
        let name_fixed = fix_file_identifier(&name);
        let refs = sf.ref_type_table();
        let mut assets = HashMap::new();
        for obj in &sf.objects {
            let class_id = obj.class_id(&sf);
            match parse_object(&sf, obj, &refs) {
                Ok(parsed) => {
                    let obj_idx = objects.len();
                    objects.push(EnvironmentObject {
                        collection: col_idx,
                        path_id: obj.path_id,
                        class_id,
                        object: parsed,
                    });
                    assets.insert(obj.path_id, obj_idx);
                }
                Err(e) => {
                    failures.push(ParseFailure::Object {
                        collection: col_idx,
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
            dependencies: vec![Some(col_idx)], // [0] = self
            assets,
        });
        pending.push((col_idx, externals));
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

/// [`Environment`] 的 builder——累积多个 bundle 后 `build()` 一次性做全局依赖解析。
///
/// 对齐 AssetRipper 两阶段加载：累积阶段（`path`/`bundle`）= Phase A，逐 bundle 读文件 +
/// eager parse 对象；[`build`] = Phase B，在全部 collection 建好后统一解析 externals，
/// 因此跨 bundle 的前向/后向 PPtr 引用都能命中。
///
/// # 示例
/// ```no_run
/// # use unity_asset_rs::Environment;
/// let env = Environment::builder()
///     .path("a.ab")?
///     .path("b.ab")?
///     .build();
/// # Ok::<(), unity_asset_rs::Error>(())
/// ```
pub struct EnvironmentBuilder {
    objects: Vec<EnvironmentObject>,
    collections: Vec<AssetCollection>,
    resources: Vec<ResourceFile>,
    failures: Vec<ParseFailure>,
    pending_deps: Vec<(usize, Vec<FileIdentifier>)>,
    dependency_provider: Option<Box<dyn DependencyProvider>>,
    resource_provider: Option<Box<dyn ResourceProvider>>,
}

impl EnvironmentBuilder {
    fn new() -> EnvironmentBuilder {
        EnvironmentBuilder {
            objects: Vec::new(),
            collections: Vec::new(),
            resources: Vec::new(),
            failures: Vec::new(),
            pending_deps: Vec::new(),
            dependency_provider: None,
            resource_provider: None,
        }
    }

    /// 加载一个 bundle 文件路径（`fs::read` + `Registry::defaults`），并入累积区。
    pub fn path(mut self, path: impl AsRef<std::path::Path>) -> Result<Self, crate::Error> {
        let path = path.as_ref();
        let bytes = std::fs::read(path).map_err(|e| crate::Error::Io { source: e })?;
        let bundle = crate::bundle::parse(Bytes::from(bytes), &Registry::defaults())?;
        self.add_bundle(bundle);
        Ok(self)
    }

    /// 并入一个已解析的 `Bundle`（用户可用自定义 registry 预加载后喂入）。
    pub fn bundle(mut self, bundle: Bundle) -> Self {
        self.add_bundle(bundle);
        self
    }

    /// 注册扩展点：外部引用未命中时回调（对齐 AssetRipper `IDependencyProvider`）。
    /// 当前 eager 模型在 build 中尚未实际回调；为后续懒加载预留 API 面。
    pub fn dependency_provider(mut self, provider: impl DependencyProvider + 'static) -> Self {
        self.dependency_provider = Some(Box::new(provider));
        self
    }

    /// 注册扩展点：`.resS` 未命中时回调（对齐 AssetRipper `IResourceProvider`）。
    /// 当前 eager 模型在 build 中尚未实际回调；为后续磁盘 resS 现取预留 API 面。
    pub fn resource_provider(mut self, provider: impl ResourceProvider + 'static) -> Self {
        self.resource_provider = Some(Box::new(provider));
        self
    }

    /// 收尾：把累积的四个 Vec move 进新 `Environment`，跑一次全局依赖解析。
    ///
    /// `dependency_provider` / `resource_provider` 当前不被回调（eager 模型）；
    /// 保留在 builder 上供未来懒加载实现使用（见 [`DependencyProvider`] TODO）。
    pub fn build(mut self) -> Environment {
        let mut env = Environment {
            objects: self.objects,
            collections: self.collections,
            resources: self.resources,
            failures: self.failures,
        };
        env.resolve_dependencies(&mut self.pending_deps);
        env
    }

    /// Phase A helper：把 bundle 灌入累积的 Vec + 追加 pending_deps。
    /// 复用 [`phase_a`]（与 `Environment::add_bundle_inner` 同源）。
    fn add_bundle(&mut self, bundle: Bundle) {
        phase_a(
            bundle,
            &mut self.objects,
            &mut self.collections,
            &mut self.resources,
            &mut self.failures,
            &mut self.pending_deps,
        );
    }
}

impl Default for EnvironmentBuilder {
    fn default() -> Self {
        Self::new()
    }
}
