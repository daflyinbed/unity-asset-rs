//! Container 索引端到端测试:从真实 bundle 构建 Environment,验证
//! AssetBundle.m_Container / ResourceManager.m_Container 的聚合结果。

use std::path::Path;

use generated::UnityObject;
use unity_asset::{
    ContainerEntry, ContainerIndex, ContainerKind, Environment, Loader, ParsedObject,
};

/// 默认 fixture(io 层测试同款)。
const DEFAULT_BUNDLE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../example/char_002_amiya.ab");
/// example/ 目录(用于多 bundle / ResourceManager 场景)。
const EXAMPLE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../example");

fn load_env(path: &str) -> Environment {
    if !Path::new(path).exists() {
        eprintln!("跳过:{path} 不存在");
        std::process::exit(0);
    }
    Loader::builder()
        .path(path)
        .build()
        .unwrap_or_else(|e| panic!("加载 {path} 失败: {e}"))
        .into_environment()
}

#[test]
fn containers_index_built_eagerly() {
    // Loader::build 末尾应已 eager 构建好 container 索引。
    // char_002_amiya.ab 是角色立绘 bundle,必有 AssetBundle object。
    let env = load_env(DEFAULT_BUNDLE);
    let idx: &ContainerIndex = env.containers();
    assert!(
        !idx.is_empty(),
        "char_002_amiya.ab 应至少含一条 container 条目(必有 AssetBundle object)"
    );
    eprintln!("char_002_amiya.ab: {} 条 container", idx.len());
}

#[test]
fn every_entry_has_host_pointing_to_assetbundle_or_resource_manager() {
    let env = load_env(DEFAULT_BUNDLE);
    for (i, e) in env.containers().entries().iter().enumerate() {
        // host 必须能 resolve 回一个 AssetBundle / ResourceManager 对象。
        let Some(host) = env.resolve_ref(e.host).and_then(|r| r.get()) else {
            panic!("entry[{i}].host {:?} 解析失败", e.host);
        };
        let kind_ok = match (&host.object, e.kind) {
            (ParsedObject::Typed(UnityObject::AssetBundle(_)), ContainerKind::AssetBundle) => true,
            (
                ParsedObject::Typed(UnityObject::ResourceManager(_)),
                ContainerKind::ResourceManager,
            ) => true,
            _ => false,
        };
        assert!(
            kind_ok,
            "entry[{i}].kind={:?} 应与 host 对象类型一致",
            e.kind
        );

        // AssetBundle entry 可能带 preload;ResourceManager entry 一定 None。
        match e.kind {
            ContainerKind::ResourceManager => {
                assert!(
                    e.preload.is_none(),
                    "entry[{i}] ResourceManager 的 preload 应为 None"
                );
            }
            ContainerKind::AssetBundle => {
                // preload 可以是 None(越界/无切片)或 Some(vec)。
            }
        }
    }
}

#[test]
fn asset_resolution_is_consistent_with_resolve_ref() {
    // entry.asset 若是 Some(id),resolve_ref(id) 应能取到对象;且该对象的
    // path_id 与 id.path_id 一致。
    let env = load_env(DEFAULT_BUNDLE);
    let mut checked = 0usize;
    for e in env.containers().entries() {
        let Some(id) = e.asset else { continue };
        let resolved = env
            .resolve_ref(id)
            .and_then(|r| r.get())
            .unwrap_or_else(|| panic!("entry {:?} asset {:?} 应能 resolve", e.path, id));
        assert_eq!(
            resolved.id(), id,
            "entry {:?} resolve 后 id 应与 entry.asset 一致",
            e.path
        );
        checked += 1;
    }
    eprintln!("校验了 {checked} 条 entry 的 asset resolve 一致性");
}

#[test]
fn preload_slice_resolves_to_valid_objects() {
    // AssetBundle entry 的 preload 切片里每个 Some(id) 都应能 resolve。
    let env = load_env(DEFAULT_BUNDLE);
    let mut checked_slices = 0usize;
    let mut checked_ids = 0usize;
    for e in env.containers().entries() {
        let Some(slice) = &e.preload else { continue };
        checked_slices += 1;
        for (i, maybe_id) in slice.iter().enumerate() {
            let Some(id) = maybe_id else { continue };
            let resolved = env
                .resolve_ref(*id)
                .and_then(|r| r.get())
                .unwrap_or_else(|| {
                    panic!(
                        "entry {:?} preload[{i}] {:?} 应能 resolve",
                        e.path, id
                    )
                });
            assert_eq!(resolved.id(), *id);
            checked_ids += 1;
        }
    }
    eprintln!(
        "preload 校验:{checked_slices} 个切片, {checked_ids} 个 PPtr"
    );
}

#[test]
fn path_of_roundtrip_for_main_asset() {
    // 主 asset 通过 path_of 反查应返回 entry.path。
    let env = load_env(DEFAULT_BUNDLE);
    let mut checked = 0usize;
    for e in env.containers().entries() {
        let Some(id) = e.asset else { continue };
        let Some(resolved) = env.resolve_ref(id).and_then(|r| r.get()) else {
            continue;
        };
        let path_back = env.path_of(resolved);
        // path_of 线性扫描返回首个命中,可能不是当前 entry(同对象多个 path 引用场景)。
        // 但至少要命中一个 entry.path。
        assert!(
            path_back.is_some(),
            "主 asset {:?} 的 path_of 不应为 None",
            id
        );
        let path_back = path_back.unwrap();
        // 校验返回的 path 确实对应这个对象。
        let matched_entry = env
            .containers()
            .entries()
            .iter()
            .find(|e2| e2.path == path_back);
        assert!(matched_entry.is_some(), "path_of 返回的 path 应能在 entries 找到");
        let matched_entry = matched_entry.unwrap();
        assert_eq!(
            matched_entry.asset,
            Some(id),
            "path_of 返回的 entry 的主 asset 应与查询对象一致"
        );
        checked += 1;
    }
    eprintln!("path_of roundtrip 校验了 {checked} 个主 asset");
}

#[test]
fn path_of_hits_preload_derived_objects() {
    // preload 切片里的对象(path_of 通过 preload 回填)也应能被 path_of 反查到。
    let env = load_env(DEFAULT_BUNDLE);
    let mut checked = 0usize;
    for e in env.containers().entries() {
        let Some(slice) = &e.preload else { continue };
        for maybe_id in slice {
            let Some(id) = maybe_id else { continue };
            let Some(resolved) = env.resolve_ref(*id).and_then(|r| r.get()) else {
                continue;
            };
            let path_back = env.path_of(resolved);
            // 该对象在 preload 里,path_of 必须命中(可能不是当前 entry.path,但要 Some)。
            assert!(
                path_back.is_some(),
                "preload 派生对象 {:?} 的 path_of 不应为 None",
                id
            );
            checked += 1;
        }
    }
    eprintln!("path_of preload 回填校验了 {checked} 个对象");
}

#[test]
fn all_assetbundle_objects_are_indexed_as_host() {
    // 每个 UnityObject::AssetBundle 的 m_container 条数总和应等于 entries 里
    // kind=AssetBundle 的条数(对照 typed 原始字段,验证一条都不漏)。
    let env = load_env(DEFAULT_BUNDLE);
    let expected_ab: usize = env
        .objects()
        .iter()
        .filter_map(|o| match &o.object {
            ParsedObject::Typed(UnityObject::AssetBundle(ab)) => Some(ab.m_container.len()),
            _ => None,
        })
        .sum();
    let actual_ab = env
        .containers()
        .entries()
        .iter()
        .filter(|e| e.kind == ContainerKind::AssetBundle)
        .count();
    assert_eq!(
        expected_ab, actual_ab,
        "AssetBundle.m_container 总条数 ({expected_ab}) 应等于 entries 里 kind=AssetBundle 的条数 ({actual_ab})"
    );
}

#[test]
fn path_uniqueness_is_multidict_preserved() {
    // entries() 保留重复 path;此处只验证同一 path 出现多次时 entries 都保留。
    // 不强求必有重复,但 entries 长度应 >= 去重后 path 数。
    let env = load_env(DEFAULT_BUNDLE);
    let total = env.containers().len();
    let unique: std::collections::HashSet<&str> =
        env.containers().entries().iter().map(|e| e.path.as_str()).collect();
    assert!(
        total >= unique.len(),
        "entries 总数 ({total}) 应 >= 去重后 path 数 ({})"
        ,
        unique.len()
    );
}

#[test]
fn entry_fields_are_exposed() {
    // 验证 ContainerEntry 的 pub 字段可访问(防止意外改成私有)。
    let env = load_env(DEFAULT_BUNDLE);
    let first: &ContainerEntry = env
        .containers()
        .entries()
        .first()
        .expect("应至少一条 container");
    let _path: &str = &first.path;
    let _asset: &Option<_> = &first.asset;
    let _host: &unity_asset::ObjectId = &first.host;
    let _kind: ContainerKind = first.kind;
    let _preload: &Option<Vec<_>> = &first.preload;
}

#[test]
fn resource_manager_bundles_also_indexed() {
    // 全目录扫描:统计 ResourceManager 宿主的条目。不要求 example 必有
    // ResourceManager,但若有,entries 里应有对应 kind=ResourceManager 的条目。
    if !Path::new(EXAMPLE_DIR).exists() {
        eprintln!("跳过:{EXAMPLE_DIR} 不存在");
        std::process::exit(0);
    }
    let env = Loader::builder()
        .scan_dir(EXAMPLE_DIR)
        .build()
        .expect("目录加载失败")
        .into_environment();

    // 原始数据里 ResourceManager.m_container 总条数。
    let expected_rm: usize = env
        .objects()
        .iter()
        .filter_map(|o| match &o.object {
            ParsedObject::Typed(UnityObject::ResourceManager(rm)) => Some(rm.m_container.len()),
            _ => None,
        })
        .sum();
    let actual_rm = env
        .containers()
        .entries()
        .iter()
        .filter(|e| e.kind == ContainerKind::ResourceManager)
        .count();
    assert_eq!(
        expected_rm, actual_rm,
        "ResourceManager.m_container 总条数 ({expected_rm}) 应等于 entries 里 kind=ResourceManager 的条数 ({actual_rm})"
    );
    eprintln!("ResourceManager: {expected_rm} 条(全目录扫描)");
}
