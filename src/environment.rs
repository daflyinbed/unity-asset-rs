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

use crate::names::fix_file_identifier;
use crate::object_parser::{parse_object, MonoBehaviour, ParsedObject};
use crate::serialized_file::FileIdentifier;
use crate::typetree::PPtrValue;
use crate::{Bundle, Endian, Error, PPtr, SerializedFile, UnityVersion};

/// 资产环境：聚合一个 bundle 的全部对象 + `.resS` 资源 + PPtr 依赖解析。
///
/// 等价于 AssetRipper 的 `GameBundle` + 全部 `AssetCollection`。单 bundle 范围——
/// 跨 bundle 引用（PPtr 指向本 bundle 外的 collection）resolve 时返回 `None`。
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
    /// 从 io 层的 `Bundle` 构建 `Environment`（infallible——错误隔离进 `failures`）。
    ///
    /// 两阶段构建（对齐 AssetRipper）：Phase A 读全部文件 + eager parse 对象 + 收集
    /// 资源；Phase B 按 name 解析 externals 填充依赖列表。
    pub fn from_bundle(bundle: Bundle) -> Environment {
        let mut env = Environment {
            objects: Vec::new(),
            collections: Vec::new(),
            resources: Vec::new(),
            failures: Vec::new(),
        };
        // (collection_idx, externals)——Phase B 用。
        let mut pending_deps: Vec<(usize, Vec<FileIdentifier>)> = Vec::new();

        // ── Phase A：读全部文件 + 对象 + 资源 ──
        for bf in bundle.files {
            if is_resource_file(&bf.name) {
                // detach：copy_from_slice 把资源拷到独立分配，断开与 blob Arc 的共享。
                // 这样 SerializedFile 解析完毕后整个解压 blob 可被释放，避免 .assets
                // 区域因 .resS 切片的 Arc 共享而驻留（详见 docs/memory-notes.md）。
                env.resources.push(ResourceFile {
                    name_fixed: fix_file_identifier(&bf.name),
                    name: bf.name,
                    bytes: Bytes::copy_from_slice(&bf.bytes),
                });
                continue;
            }
            let sf = match SerializedFile::parse(bf.bytes) {
                Ok(sf) => sf,
                Err(e) => {
                    env.failures.push(ParseFailure::File { name: bf.name, error: e });
                    continue;
                }
            };
            let col_idx = env.collections.len();
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
                        let obj_idx = env.objects.len();
                        env.objects.push(EnvironmentObject {
                            collection: col_idx,
                            path_id: obj.path_id,
                            class_id,
                            object: parsed,
                        });
                        assets.insert(obj.path_id, obj_idx);
                    }
                    Err(e) => {
                        env.failures.push(ParseFailure::Object {
                            collection: col_idx,
                            path_id: obj.path_id,
                            class_id,
                            error: e,
                        });
                    }
                }
            }
            // sf 不再被借用，提取 externals（Phase B 用）。
            let externals = sf.externals;
            env.collections.push(AssetCollection {
                name,
                name_fixed,
                unity_version,
                endian,
                dependencies: vec![Some(col_idx)], // [0] = self
                assets,
            });
            pending_deps.push((col_idx, externals));
        }

        // ── Phase B：初始化依赖列表 ──
        for (col_idx, externals) in pending_deps {
            for identifier in &externals {
                let target = env
                    .collections
                    .iter()
                    .position(|c| c.name_fixed == identifier.path_name);
                env.collections[col_idx].dependencies.push(target);
            }
        }

        env
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

/// 提取 `StreamingInfoOffsetVariant` 的 offset（统一为 `u64`）。对齐
/// `StreamingInfoExtensions.GetOffset`。
fn streaming_offset(offset: &StreamingInfoOffsetVariant) -> u64 {
    match offset {
        StreamingInfoOffsetVariant::Default(o) => *o as u64,
        StreamingInfoOffsetVariant::V2020_1(o) => *o,
    }
}
