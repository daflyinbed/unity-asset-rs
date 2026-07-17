//! 环境聚合层：从 io 层的 `Bundle` 构建 `Environment`——eager parse 全部 SerializedFile
//! 的对象，聚合 `.resS` 流数据，提供 PPtr 解引用与 streaming_data 取数。
//!
//! 设计与命名参照 AssetRipper（`GameBundle` / `AssetCollection` / `ResourceFile` /
//! `SpecialFileNames`）。详见 `docs/environment-layer.md`。

use std::collections::HashSet;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
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
use crate::{Endian, Error, PPtr, SerializedFile, UnityVersion};

/// 资产环境：聚合多个 bundle 的全部对象 + `.resS` 资源 + PPtr 依赖解析。
///
/// 等价于 AssetRipper 的 `GameBundle` + 全部 `AssetCollection`。数据模型天然支持跨 bundle：
/// `objects`/`collections`/`resources` 全是全局扁平 Vec，`resolve()` 走全局索引，
/// Phase B 依赖解析全局扫描 `collections`——只要把多个 bundle 灌进同一个 `Environment`，
/// 跨 bundle 引用在 Phase B（全部 collection 建好后）自动 resolve。
///
/// 构造唯一入口 [`Environment::builder`]：累积阶段（`path`/`bundle`/`scan_dir`）逐 bundle 做
/// Phase A，`build()` 收尾时跑一次 Phase B 全局依赖解析。
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
/// 对齐 AssetRipper `IDependencyProvider`。在 [`EnvironmentBuilder::build`] 的 Phase B
/// 中实际回调：当某个 collection 的 external（按 `name_fixed` 匹配）在已加载的全部
/// collection 中找不到时，调 `find_dependency`；返回 `Some(LoadedFile)` 则追加 phase_a
/// 加载该依赖并重试匹配，**新依赖引入的 externals 也会递归处理**（传递依赖，直到
/// 不动点；上限 `MAX_LAZY_LOADS = 4096` 防恶意循环）。默认实现 no-op（返回 `None`，
/// 依赖留空）。
///
/// `Send + Sync` 约束对齐未来 pyo3 / async 场景：provider 以 `Arc<dyn Trait>` 形式
/// 存入 [`Environment`]。
pub trait DependencyProvider: Send + Sync {
    /// 返回 `Some(LoadedFile)` 则追加加载该依赖；`None` 则该引用留空（resolve 返回 `None`）。
    fn find_dependency(&self, _id: &FileIdentifier) -> Option<LoadedFile> {
        None
    }
}

/// 扩展点：`.resS` 在已加载 resources 中未命中时的回调。
///
/// 对齐 AssetRipper `IResourceProvider`。在 [`EnvironmentBuilder::build`] 末尾的
/// `prefetch_resources` 中实际回调：扫描已加载 Texture2D（`m_stream_data.path`）与
/// AudioClip（`m_resource.m_source` / pre-5.0 隐式 `collection.name + ".resS"`）的
/// streaming 引用，对未命中的 name 调 `find_resource`，返回 `Some(ResourceFile)` 则
/// 注册进 `resources`（之后 `streaming_data` 等只读查找命中）。默认实现 no-op。
///
/// `Send + Sync` 约束同 [`DependencyProvider`]。
pub trait ResourceProvider: Send + Sync {
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
    /// 新建 [`EnvironmentBuilder`]——聚合层唯一构造入口。
    ///
    /// 累积阶段（`path`/`bundle`/`scan_dir`）逐 bundle 做 Phase A，[`EnvironmentBuilder::build`]
    /// 收尾时跑一次 Phase B 全局依赖解析（全部 collection 建好后统一匹配 externals，
    /// 跨 bundle 的前向/后向引用都能命中）。
    pub fn builder() -> EnvironmentBuilder {
        EnvironmentBuilder::new()
    }

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
    fn resolve_dependencies(&mut self, pending: &mut Vec<(usize, Vec<FileIdentifier>)>) {
        /// 懒加载次数上限（防恶意/损坏数据导致无限递归）。真实 bundle 依赖图远小于此。
        const MAX_LAZY_LOADS: usize = 4096;

        let has_provider = self.dependency_provider.is_some();
        let mut tried: HashSet<String> = HashSet::new();
        let mut lazy_loads = 0usize;

        let mut i = 0;
        while i < pending.len() {
            // 取出当前 collection 的 externals 快照（owned path_name 列表）+ col_idx，
            // 随即结束对 `pending` 的不可变借用，以便后续 phase_a 可 &mut pending（push
            // 传递依赖）。
            let (col_idx, names) = {
                let (col_idx, externals) = &pending[i];
                let names: Vec<String> =
                    externals.iter().map(|id| id.path_name.clone()).collect();
                (*col_idx, names)
            };

            for name in names {
                let mut target =
                    self.collections.iter().position(|c| c.name_fixed == name);

                // 懒加载：未命中 + 有 provider + 该 name 尚未尝试过 + 未超上限。
                if target.is_none() && has_provider && tried.insert(name.clone()) {
                    if lazy_loads >= MAX_LAZY_LOADS {
                        log::warn!(
                            "resolve_dependencies: 懒加载次数达上限 {MAX_LAZY_LOADS}，停止懒加载"
                        );
                    } else {
                        // 构造查询用 FileIdentifier（provider 只需 path_name 匹配）。
                        let query = FileIdentifier {
                            path_name_origin: String::new(),
                            path_name: name.clone(),
                            guid: [0u8; 16],
                            type_: 0,
                        };
                        let provider = self.dependency_provider.clone(); // Arc clone，廉价
                        if let Some(loaded) =
                            provider.as_ref().and_then(|p| p.find_dependency(&query))
                        {
                            lazy_loads += 1;
                            // phase_a 把新 bundle 灌进 self 的各 Vec；
                            // 新 collection 的 externals 被 push 进 pending（传递依赖入队）。
                            phase_a(
                                loaded,
                                &mut self.objects,
                                &mut self.collections,
                                &mut self.resources,
                                &mut self.failures,
                                pending,
                            );
                            target =
                                self.collections.iter().position(|c| c.name_fixed == name);
                        }
                    }
                }
                self.collections[col_idx].dependencies.push(target);
            }
            i += 1;
        }
    }

    /// 预拉取缺失的 `.resS` 资源（[`ResourceProvider`] 回调）。
    ///
    /// 在 [`resolve_dependencies`] 之后、`build` 返回前调用。扫描已加载对象的
    /// streaming 引用，对未在 `resources` 命中的 name 调 `find_resource`，命中则
    /// 注册进 `resources`——之后 `streaming_data` / `streamed_resource_data` /
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
    fn prefetch_resources(&mut self) {
        let Some(provider) = &self.resource_provider else {
            return;
        };
        // 先收集全部 wanted name（不可变借 objects），再释放借用后 mutate resources。
        let mut wanted: Vec<String> = Vec::new();
        for o in &self.objects {
            match &o.object {
                ParsedObject::Typed(UnityObject::Texture2D(t)) => {
                    if let Some(sd) = &t.m_stream_data
                        && !sd.path.is_empty()
                    {
                        wanted.push(sd.path.clone());
                    }
                }
                ParsedObject::Typed(UnityObject::AudioClip(a)) => {
                    if let Some(r) = &a.m_resource
                        && !r.m_source.is_empty()
                    {
                        wanted.push(r.m_source.clone());
                    }
                    // pre-5.0 隐式 .resS 名 = collection.name + ".resS"
                    if matches!(a.m_audio_data, Some(AudioClipMAudioData::Streamed { .. }))
                        && let Some(col) = self.collections.get(o.collection)
                    {
                        wanted.push(format!("{}.resS", col.name));
                    }
                }
                _ => {}
            }
        }
        // provider 调用：对每个未命中的 name 查一次（去重 + name_fixed 归一化匹配）。
        // 克隆 Arc 以释放对 self 的借用，便于后续 self.resources.push。
        let provider = provider.clone();
        for name in wanted {
            let fixed = fix_file_identifier(&name);
            let already = self.resources.iter().any(|r| r.name_fixed == fixed);
            if already {
                continue;
            }
            if let Some(rf) = provider.find_resource(&name) {
                // 防御：若 provider 返回的 name_fixed 与已存在重复，跳过（不覆盖）。
                let dup = self
                    .resources
                    .iter()
                    .any(|r| r.name_fixed == rf.name_fixed);
                if !dup {
                    self.resources.push(rf);
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
        collection: usize,
        offset: u32,
        size: u32,
    ) -> Option<Bytes> {
        if size == 0 {
            return None;
        }
        let col = self.collections.get(collection)?;
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

/// Phase A：读 `LoadedFile` 的全部叶子 + eager parse 对象 + 收集资源，灌入给定目标。
///
/// 这是 `EnvironmentBuilder::add_loaded` 的共享实现。每个 SerializedFile → 一个
/// `AssetCollection`（`dependencies[0]=Some(self)`），`.resS` → `ResourceFile`
/// （`copy_from_slice` detach）。externals 暂存到 `pending`，等
/// [`Environment::resolve_dependencies`] 在全部 collection 建好后统一解析（跨 bundle
/// 前向/后向引用都能命中——对齐 AssetRipper 两阶段加载）。
fn phase_a(
    loaded: LoadedFile,
    objects: &mut Vec<EnvironmentObject>,
    collections: &mut Vec<AssetCollection>,
    resources: &mut Vec<ResourceFile>,
    failures: &mut Vec<ParseFailure>,
    pending: &mut Vec<(usize, Vec<FileIdentifier>)>,
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
        let col_idx = collections.len();
        let unity_version = sf.unity_version_parsed;
        let endian = sf.endian;
        let name = name.to_string();
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

/// [`Environment`] 的 builder——聚合层唯一构造入口。累积多个 bundle 后 `build()` 一次性
/// 做全局依赖解析。
///
/// 对齐 AssetRipper 两阶段加载：累积阶段（`path`/`bundle`/`scan_dir`）= Phase A，逐 bundle
/// 读文件 + eager parse 对象；[`build`] = Phase B，在全部 collection 建好后统一解析
/// externals，因此跨 bundle 的前向/后向 PPtr 引用都能命中。
///
/// # 示例
///
/// 单 bundle / 多 bundle：
/// ```no_run
/// # use unity_asset_rs::Environment;
/// let env = Environment::builder()
///     .path("a.ab")?
///     .path("b.ab")?
///     .build();
/// # Ok::<(), unity_asset_rs::Error>(())
/// ```
///
/// 递归扫描目录（UnityFS 签名嗅探，跳过非 bundle 文件）：
/// ```no_run
/// # use unity_asset_rs::Environment;
/// let env = Environment::builder()
///     .scan_dir("example/")?
///     .build();
/// # Ok::<(), unity_asset_rs::Error>(())
/// ```
///
/// 自定义解压器（如 Arknights LZ4 变体）：`registry` 设一次，后续 `path`/`scan_dir` 均复用。
/// ```no_run
/// # use unity_asset_rs::{Environment, Registry};
/// let env = Environment::builder()
///     .registry(&Registry::defaults())
///     .path("a.ab")?
///     .build();
/// # Ok::<(), unity_asset_rs::Error>(())
/// ```
pub struct EnvironmentBuilder {
    objects: Vec<EnvironmentObject>,
    collections: Vec<AssetCollection>,
    resources: Vec<ResourceFile>,
    failures: Vec<ParseFailure>,
    pending_deps: Vec<(usize, Vec<FileIdentifier>)>,
    registry: Registry,
    dependency_provider: Option<Arc<dyn DependencyProvider>>,
    resource_provider: Option<Arc<dyn ResourceProvider>>,
}

impl EnvironmentBuilder {
    fn new() -> EnvironmentBuilder {
        EnvironmentBuilder {
            objects: Vec::new(),
            collections: Vec::new(),
            resources: Vec::new(),
            failures: Vec::new(),
            pending_deps: Vec::new(),
            registry: Registry::defaults(),
            dependency_provider: None,
            resource_provider: None,
        }
    }

    /// 设定解压器注册表（clone 入驻）。后续 [`path`](Self::path) /
    /// [`scan_dir`](Self::scan_dir) 均用它解包，而非默认 registry。
    ///
    /// 默认为 [`Registry::defaults`]（LZMA + LZ4 + LZ4HC + NONE）。需要自定义 codec（如
    /// Arknights LZ4 变体占 flag 4/5——见 ADR-0003）时在此注册。
    pub fn registry(mut self, reg: &Registry) -> Self {
        self.registry = reg.clone();
        self
    }

    /// 加载一个资产文件路径（`fs::read` + 递归再派发 + 驻留 registry），并入累积区。
    pub fn path(mut self, path: impl AsRef<Path>) -> Result<Self, crate::Error> {
        let path = path.as_ref();
        let bytes = std::fs::read(path).map_err(|e| crate::Error::Io { source: e })?;
        let loaded = crate::format::load_recursive(
            Bytes::from(bytes),
            &path.to_string_lossy(),
            &self.registry,
            None,
        )?;
        self.add_loaded(loaded);
        Ok(self)
    }

    /// 递归扫描目录，对每个可加载文件做递归再派发，跳过不可识别的文件。
    ///
    /// 对齐 AssetStudio `LoadFolder`：递归遍历目录树，读头部嗅探是否为可加载格式
    /// （UnityFS / gzip 等），命中才加载，否则 `log::warn` 静默跳过。
    pub fn scan_dir(mut self, path: impl AsRef<Path>) -> Result<Self, crate::Error> {
        let path = path.as_ref();
        let mut any = false;
        for entry in walk_files(path)? {
            if crate::detect::looks_like_loadable_path(&entry)? {
                let bytes =
                    std::fs::read(&entry).map_err(|e| crate::Error::Io { source: e })?;
                let loaded = crate::format::load_recursive(
                    Bytes::from(bytes),
                    &entry.to_string_lossy(),
                    &self.registry,
                    None,
                )?;
                self.add_loaded(loaded);
                any = true;
            } else {
                log::warn!("scan_dir: 跳过非可加载文件: {}", entry.display());
            }
        }
        if !any {
            log::warn!("scan_dir: {} 下未发现任何可加载文件", path.display());
        }
        Ok(self)
    }

    /// 并入一个已解析的 `LoadedFile`（用户可用自定义 registry 预加载后喂入）。
    pub fn loaded(mut self, loaded: LoadedFile) -> Self {
        self.add_loaded(loaded);
        self
    }

    /// 注册扩展点：外部引用未命中时回调（对齐 AssetRipper `IDependencyProvider`）。
    /// build 时 Phase B 在 `target.is_none()` 分支实际回调 `find_dependency`，
    /// 返回 `Some(LoadedFile)` 则追加加载并重试匹配（传递依赖递归直到不动点）。
    pub fn dependency_provider(
        mut self,
        provider: impl DependencyProvider + 'static,
    ) -> Self {
        self.dependency_provider = Some(Arc::new(provider));
        self
    }

    /// 注册扩展点：`.resS` 未命中时回调（对齐 AssetRipper `IResourceProvider`）。
    /// build 末尾 `prefetch_resources` 扫描 Texture2D / AudioClip 的 streaming 引用，
    /// 对未命中的 name 实际回调 `find_resource`，命中则注册进 `resources`。
    pub fn resource_provider(
        mut self,
        provider: impl ResourceProvider + 'static,
    ) -> Self {
        self.resource_provider = Some(Arc::new(provider));
        self
    }

    /// 收尾：把累积的四个 Vec + 两个 provider move 进新 `Environment`，跑一次全局
    /// 依赖解析（Phase B），再预拉取缺失的 `.resS` 资源。
    ///
    /// Phase B 顺序：先 `resolve_dependencies`（含 [`DependencyProvider`] 懒加载 +
    /// 传递依赖递归），再 `prefetch_resources`（[`ResourceProvider`] 预拉取 streaming
    /// 引用）。两者均在 provider 为 `None` 时 no-op，退化为纯 eager 模型。
    pub fn build(mut self) -> Environment {
        let mut env = Environment {
            objects: self.objects,
            collections: self.collections,
            resources: self.resources,
            failures: self.failures,
            dependency_provider: self.dependency_provider,
            resource_provider: self.resource_provider,
        };
        env.resolve_dependencies(&mut self.pending_deps);
        env.prefetch_resources();
        env
    }

    /// Phase A helper：把 `LoadedFile` 灌入累积的 Vec + 追加 pending_deps。复用 [`phase_a`]。
    fn add_loaded(&mut self, loaded: LoadedFile) {
        phase_a(
            loaded,
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

/// 递归收集目录下全部文件路径（先序）。符号链接子目录跳过（避免循环）。
fn walk_files(root: &Path) -> Result<Vec<PathBuf>, crate::Error> {
    let mut out = Vec::new();
    walk_inner(root, &mut out)?;
    Ok(out)
}

fn walk_inner(dir: &Path, out: &mut Vec<PathBuf>) -> Result<(), crate::Error> {
    for entry in std::fs::read_dir(dir).map_err(|e| crate::Error::Io { source: e })? {
        let entry = entry.map_err(|e| crate::Error::Io { source: e })?;
        let path = entry.path();
        let ft = entry.file_type().map_err(|e| crate::Error::Io { source: e })?;
        if ft.is_dir() {
            // 不跟符号链接目录，防循环。
            if !path.is_symlink() {
                walk_inner(&path, out)?;
            }
        } else {
            out.push(path);
        }
    }
    Ok(())
}
