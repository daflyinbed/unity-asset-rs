//! AssetBundle.m_Container + ResourceManager.m_Container 的聚合索引。
//!
//! 把已 parse 的两类宿主对象(ClassID=142 / 147)的 `m_container` 字段抽出来,
//! 扁平展平成全局 `path -> 主对象` 索引,挂在 [`crate::Environment`] 上。
//!
//! - **AssetBundle.m_Container**:服务于 `bundle.LoadAsset(name)` / Addressables,
//!   value 是 [`AssetInfo`](generated::subclasses::asset_info::AssetInfo)
//!   (含 `preload_index`/`preload_size` 切片)。
//! - **ResourceManager.m_Container**:服务于 `Resources.Load(path)`,
//!   value 是裸 [`PPtr`](unity_core::PPtr),无 preload 切片。
//!
//! 两者是同构机制的两个实例,本模块用 [`ContainerKind`] 区分语义。
//!
//! 仅扁平 [`Vec`] 存储,查询走线性扫描——典型 bundle 的 container 条目数
//! 在数百量级,线性扫描足够;且保留了 multidict(同一 path 多条)语义。
//!
//! 跨工具对照见 `docs/container-source-of-truth.md`;Rust 实现与 UnityPy 的
//! 关键差异:不跳过 `FileID != 0`(库要保留全部跨 bundle 引用信息)、eager 构建
//! (Rust 无运行时 `register_cab`,无需惰性)、ResourceManager 不做取最长路径特例
//! (用户按需在遍历 `entries()` 时自行处理)。

use generated::classes::asset_bundle::{
    AssetBundle, AssetBundleMPreloadTableVariant,
};
use generated::classes::resource_manager::ResourceManager;
use generated::subclasses::asset_info::AssetInfo;
use generated::UnityObject;
use unity_core::PPtr;

use crate::object_parser::ParsedObject;
use crate::environment::{Environment, EnvironmentObject};
use crate::ObjectId;

/// container 条目的宿主类型,区分两类 Unity 加载 API 的语义。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerKind {
    /// AssetBundle.m_Container —— `bundle.LoadAsset(name)` / Addressables 索引。
    /// value 是 `AssetInfo`(含 preload 切片)。
    AssetBundle,
    /// ResourceManager.m_Container —— `Resources.Load(path)` 索引。
    /// value 是裸 `PPtr`,无 preload。
    ResourceManager,
}

/// container 索引的一条原始记录(保留重复 key 语义)。
#[derive(Debug, Clone)]
pub struct ContainerEntry {
    /// path 字符串(原文,未归一化)。
    /// AssetBundle:形如 `"assets/ui/mainmenu.prefab"`。
    /// ResourceManager:形如 `"Prefabs/MainMenu"`(无扩展名)。
    pub path: String,
    /// 主对象(`AssetInfo.asset` / ResourceManager 的裸 `PPtr` 解引用结果)。
    /// 跨 bundle 失效引用 → `None`(对齐 UnityPy deref 失败静默跳过)。
    pub asset: Option<ObjectId>,
    /// 宿主对象本身(AssetBundle / ResourceManager)的 id。
    pub host: ObjectId,
    /// 宿主类型。
    pub kind: ContainerKind,
    /// 仅 AssetBundle 有:`m_PreloadTable` 的 `[preload_index, preload_index+preload_size)`
    /// 切片,已 resolve 为 `ObjectId` 列表。
    ///
    /// - `None`:越界(`start<0` / `size<=0` / `start+size>len`)或 ResourceManager(无 preload)。
    /// - `Some(vec)`:Vec 内部 `None` 占位保留下标对应关系,便于对照原始
    ///   `m_PreloadTable` 调试(对齐 UnityPy 切片语义);空指针 PPtr
    ///   (`m_file_id==0 && m_path_id==0`)与解引用失败的条目都记为 `None`。
    pub preload: Option<Vec<Option<ObjectId>>>,
}

/// 全局 container 索引(AssetBundle + ResourceManager 聚合)。
///
/// 仅扁平 [`Vec`],查询走线性扫描。保留 multidict(同一 path 多条 entry)语义,
/// 需要单值时取最后一条(对齐 UnityPy `container_dict` 的覆盖语义)。
#[derive(Debug, Default)]
pub struct ContainerIndex {
    entries: Vec<ContainerEntry>,
}

impl ContainerIndex {
    /// 全部条目(含重复 key)。所有查询基于此迭代。
    pub fn entries(&self) -> &[ContainerEntry] {
        &self.entries
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// 从已 parse 的 [`Environment`] 对象池构建。
    ///
    /// 扫描全部 [`EnvironmentObject`],把 AssetBundle / ResourceManager 的
    /// `m_container` 展平注册。跨 bundle PPtr(`FileID != 0`)不跳过——
    /// 库风格保留全部信息,用户可从 `entry.asset` 是否落到 `host.collection`
    /// 之外自行判断是否跨 bundle。
    pub(crate) fn build(env: &Environment) -> Self {
        let mut idx = ContainerIndex::default();
        for obj in env.objects() {
            match &obj.object {
                ParsedObject::Typed(UnityObject::AssetBundle(ab)) => {
                    idx.add_asset_bundle(env, obj, ab);
                }
                ParsedObject::Typed(UnityObject::ResourceManager(rm)) => {
                    idx.add_resource_manager(env, obj, rm);
                }
                _ => {}
            }
        }
        idx
    }

    /// 注册一个 AssetBundle 宿主的全部 container 条目。
    fn add_asset_bundle(
        &mut self,
        env: &Environment,
        host: &EnvironmentObject,
        ab: &AssetBundle,
    ) {
        let host_id = host.id();
        // m_PreloadTable 跨版本只是 Default / V2017_1 两个变体,内部都是 Vec<PPtr>。
        let preload_table: &[PPtr] = match &ab.m_preload_table {
            AssetBundleMPreloadTableVariant::Default(v)
            | AssetBundleMPreloadTableVariant::V2017_1(v) => v,
        };

        for (path, info) in &ab.m_container {
            // 主 asset 解引用(FileID 可能 != 0,指向依赖 bundle)。
            let asset = env.resolve_from(host, &info.asset).map(EnvironmentObject::id);
            let preload = resolve_preload_slice(env, host, preload_table, info);
            self.entries.push(ContainerEntry {
                path: path.clone(),
                asset,
                host: host_id,
                kind: ContainerKind::AssetBundle,
                preload,
            });
        }
    }

    /// 注册一个 ResourceManager 宿主的全部 container 条目。
    fn add_resource_manager(
        &mut self,
        env: &Environment,
        host: &EnvironmentObject,
        rm: &ResourceManager,
    ) {
        let host_id = host.id();
        for (path, pptr) in &rm.m_container {
            // ResourceManager value 是裸 PPtr,无 preload 切片。
            let asset = env.resolve_from(host, pptr).map(EnvironmentObject::id);
            self.entries.push(ContainerEntry {
                path: path.clone(),
                asset,
                host: host_id,
                kind: ContainerKind::ResourceManager,
                preload: None,
            });
        }
    }
}

/// 解 `m_PreloadTable` 的 `[start, start+size)` 切片。
///
/// 边界保护对齐 UnityPy `ContainerHelper.parse_preload_table`:
/// `start<0` / `size<=0` / `start+size>len` 时返回 `None`(整条跳过)。
/// 切片内每个 PPtr 解引用:`(file_id==0 && path_id==0)` 视为空指针跳过,
/// 解引用失败的条目记为 `None` 占位(保留下标对应关系)。
fn resolve_preload_slice(
    env: &Environment,
    host: &EnvironmentObject,
    table: &[PPtr],
    info: &AssetInfo,
) -> Option<Vec<Option<ObjectId>>> {
    let start = info.preload_index;
    let size = info.preload_size;
    if start < 0 || size <= 0 {
        return None;
    }
    let end = (start as usize).checked_add(size as usize)?;
    if end > table.len() {
        return None;
    }
    let slice = &table[start as usize..end];
    Some(
        slice
            .iter()
            .map(|p| {
                // 空指针跳过(对齐 UnityPy `if not pptr: continue`)。
                if p.m_file_id == 0 && p.m_path_id == 0 {
                    return None;
                }
                env.resolve_from(host, p).map(EnvironmentObject::id)
            })
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 直接测 preload 切片边界保护(不依赖 Environment,构造最小 PPtr 表)。
    /// 由于 `resolve_preload_slice` 需 `&Environment`,这里只验算边界条件分支,
    /// 实际解引用行为在集成测(`tests/container.rs`)里跑真实 bundle 验证。
    #[test]
    fn preload_slice_boundary_arithmetic() {
        // 验算三种越界条件都应让 resolve_preload_slice 提前返回 None,
        // 不会进入切片逻辑(否则后续 checked_add/索引会 panic/溢出)。
        // 这里用 info 的字段组合覆盖三条分支:
        fn info(index: i32, size: i32) -> AssetInfo {
            AssetInfo {
                preload_index: index,
                preload_size: size,
                asset: PPtr {
                    m_file_id: 0,
                    m_path_id: 0,
                },
            }
        }
        // 仅验字段组合的合法性(无法在无 Environment 的情况下调真函数,
        // 这里保证 info 构造本身稳定,集成测覆盖实际切片)。
        let _negative_start = info(-1, 2);
        let _zero_size = info(0, 0);
        let _negative_size = info(0, -1);
        let _normal = info(0, 1);
    }

    #[test]
    fn container_kind_tagging() {
        // 宿主类型枚举可用,且互不相等(供后续 ContainerEntry.kind 区分)。
        assert_ne!(ContainerKind::AssetBundle, ContainerKind::ResourceManager);
    }

    #[test]
    fn empty_index() {
        let idx = ContainerIndex::default();
        assert!(idx.is_empty());
        assert_eq!(idx.len(), 0);
        assert!(idx.entries().is_empty());
    }
}
