//! Environment 层端到端测试：从真实 bundle 构建 Environment，验证对象聚合、
//! PPtr 解引用、streaming_data、跨 bundle 聚合。

use std::path::Path;

use unity_asset_rs::{
    Bytes, ClassIDType, ClassId, DependencyProvider, Environment, FileFormat, LoadedFile, ObjectId,
    ParseFailure, ParsedObject, ResourceFile, ResourceProvider, UnityObject,
};

/// 默认 fixture（io 层测试同款）。
const DEFAULT_BUNDLE: &str = "example/char_002_amiya.ab";
/// example/ 目录下全部 fixture（5 个 bundle）。
const EXAMPLE_DIR: &str = "example";
/// 两个 fixture，用于多 bundle 测试。
const TWO_BUNDLES: &[&str] = &[
    "example/char_002_amiya.ab",
    "example/Combine.unity3d",
];

/// 加载 Environment。opt-level=1（见根 Cargo.toml）已让 TypeTree 解释器的递归
/// 返回值临时被优化掉，2MB 默认栈足够，无需额外线程。
fn load_env(path: &str) -> Environment {
    let p = Path::new(path);
    if !p.exists() {
        eprintln!("跳过：{path} 不存在");
        std::process::exit(0);
    }
    Environment::builder()
        .path(path)
        .unwrap_or_else(|e| panic!("加载 {path} 失败: {e}"))
        .build()
}

/// 单独加载一个 bundle 得到的 collection 数（对照组）。
fn single_bundle_collection_count(path: &str) -> usize {
    if !Path::new(path).exists() {
        return 0;
    }
    load_env(path).collections().len()
}

#[test]
fn environment_builds_and_has_objects() {
    let env = load_env(DEFAULT_BUNDLE);
    assert!(!env.objects().is_empty(), "应至少解析出一个对象");
    assert!(!env.collections().is_empty(), "应至少有一个 collection");
}

#[test]
fn dependencies_zero_is_self() {
    // dependencies[0] = self 约定：对每个 collection，resolve_local(自身 path_id) 应命中。
    let env = load_env(DEFAULT_BUNDLE);
    for (col_idx, col) in env.collections().iter().enumerate() {
        let col_id = env.collection_id(col_idx).expect("col_idx 合法");
        assert_eq!(
            col.dependencies.first().copied().flatten(),
            Some(col_id),
            "collection[{col_idx}] dependencies[0] 应为自身"
        );
        for (&path_id, &obj_idx) in &col.assets {
            let resolved = env.resolve_local(col_id, path_id);
            assert_eq!(
                resolved.map(|o| o as *const _),
                Some(&env.objects()[obj_idx] as *const _),
                "resolve_local({col_id}, {path_id}) 应命中 objects[{obj_idx}]"
            );
        }
    }
}

#[test]
fn no_unexpected_failures() {
    let env = load_env(DEFAULT_BUNDLE);
    // char_002_amiya.ab 此前 verify_codegen 全过，不应有解析失败。
    if let Some(f) = env.failures().first() {
        match f {
            ParseFailure::File { name, error } => panic!("file 失败: {name}: {error}"),
            ParseFailure::Object { collection, path_id, class_id, error } => panic!(
                "object 失败: col={collection} pid={path_id} cid={class_id}: {error}"
            ),
            ParseFailure::Container { name, error } => {
                panic!("container 失败: {name}: {error}")
            }
        }
    }
}

#[test]
fn object_id_and_ref_roundtrip() {
    // CollectionId/ObjectId/ObjectRef 三件套:从对象取 id,经 resolve_ref 回到同一对象。
    let env = load_env(DEFAULT_BUNDLE);
    let first = env
        .objects()
        .first()
        .expect("应至少有一个对象");
    let id = first.id();
    assert_eq!(id.path_id, first.path_id);
    assert_eq!(id.collection, first.collection);

    // resolve_ref 命中 → ObjectRef.get() 取回同一对象(指针相等)。
    let r = env.resolve_ref(id).expect("合法 id 应可解析");
    assert_eq!(r.get().map(|o| o as *const _), Some(first as *const _));
    assert_eq!(r.id(), id);
    assert_eq!(r.collection(), first.collection);
    assert_eq!(r.path_id(), first.path_id);

    // object_ref 直接从 &EnvironmentObject 构造。
    let r2 = env.object_ref(first);
    assert_eq!(r2.get().map(|o| o as *const _), Some(first as *const _));

    // 不存在的 path_id → None。
    assert!(env
        .resolve_ref(ObjectId {
            collection: first.collection,
            path_id: i64::MAX
        })
        .is_none());

    // to_index/collection_id 互为逆运算(escape hatch 可用)。
    let idx = first.collection.to_index();
    assert_eq!(env.collection_id(idx), Some(first.collection));
}

#[test]
#[allow(clippy::collapsible_if)]
fn objects_of_class_filter_works() {
    let env = load_env(DEFAULT_BUNDLE);
    let total = env.objects().len();
    let mut sum = 0usize;
    let mut seen = std::collections::HashSet::new();
    for o in env.objects() {
        let is_new = seen.insert(o.class_id);
        if is_new {
            if let Some(cls) = o.class_id.known() {
                sum += env.objects_of_class(cls).count();
            }
        }
    }
    assert!(sum <= total, "objects_of_class 求和 {sum} 不应超过总数 {total}");
}

#[test]
fn class_id_open_set_supports_mono_behaviour() {
    // ClassId 是开集:114 MonoBehaviour 虽然 codegen 跳过(ClassIDType 无此变体),
    // 但 ClassId 能表示;objects_of_class_id / mono_behaviours 都能查到。
    let env = load_env(DEFAULT_BUNDLE);
    // 114 不在 ClassIDType 中。
    assert!(ClassIDType::from_i32(114).is_none(), "codegen 应跳过 114");
    // ClassId::known() 对 114 返回 None。
    assert!(ClassId::MONO_BEHAVIOUR.known().is_none());
    // 但 objects_of_class_id(ClassId::MONO_BEHAVIOUR) 能查到 MonoBehaviour 对象。
    let via_open = env.objects_of_class_id(ClassId::MONO_BEHAVIOUR).count();
    let via_method = env.mono_behaviours().count();
    assert_eq!(via_open, via_method, "mono_behaviours() 应等价于 objects_of_class_id(MONO_BEHAVIOUR)");
    assert!(via_open > 0, "char_002_amiya.ab 应含 MonoBehaviour");

    // known() 对 Texture2D 应命中。
    let tex = ClassId::from(ClassIDType::Texture2D);
    assert!(tex.known().is_some());
    assert!(tex.is_typed_parseable());
    // From<i32> / get() 互逆。
    assert_eq!(ClassId::from(28).get(), 28);
    // Display 含已知变体名。
    assert!(format!("{tex}").contains("Texture2D"));
    // Display 未知 id 只显示数字。
    assert_eq!(format!("{}", ClassId::from(99999)), "99999");
}

#[test]
fn streaming_data_on_texture2d() {
    let env = load_env(DEFAULT_BUNDLE);
    // 找 Texture2D，若有 m_stream_data 且 path 非空，尝试取 streaming_data。
    let mut checked = 0;
    for o in env.objects() {
        let Some(ParsedObject::Typed(UnityObject::Texture2D(tex))) = Some(&o.object) else {
            continue;
        };
        let Some(sd) = tex.m_stream_data.as_ref() else {
            continue;
        };
        if sd.path.is_empty() {
            continue;
        }
        match env.streaming_data(sd) {
            Some(bytes) => {
                assert_eq!(bytes.len(), sd.size as usize, "streaming_data 长度应匹配 size");
            }
            None => {
                // 资源未命中（可能跨 bundle 或无 .resS）——记录但不 panic
                eprintln!(
                    "  信息：Texture2D {:?} streaming_data 未命中 path={:?}",
                    tex.m_name, sd.path
                );
            }
        }
        checked += 1;
    }
    eprintln!("streaming_data 检查了 {checked} 个 Texture2D（带 m_stream_data.path）");
}

// ── 跨 bundle 聚合测试 ──

#[test]
fn builder_multi_bundle_accumulates_collections() {
    // Builder 喂入两个 bundle，collections / objects 数应不少于各自单独加载之和
    // （跨 bundle 依赖解析不减少已有对象，只可能让 resolve 命中更多）。
    let env = Environment::builder()
        .path(TWO_BUNDLES[0])
        .unwrap()
        .path(TWO_BUNDLES[1])
        .unwrap()
        .build();

    let a = single_bundle_collection_count(TWO_BUNDLES[0]);
    let b = single_bundle_collection_count(TWO_BUNDLES[1]);
    assert!(
        a > 0 && b > 0,
        "两个 fixture 都应能单独加载出 collection（a={a} b={b}）"
    );
    assert_eq!(
        env.collections().len(),
        a + b,
        "两 bundle 聚合后 collection 数应等于各自之和"
    );
}

#[test]
fn scan_dir_scans_all_bundles() {
    if !Path::new(EXAMPLE_DIR).exists() {
        eprintln!("跳过：{EXAMPLE_DIR} 不存在");
        std::process::exit(0);
    }
    let env = Environment::builder()
        .scan_dir(EXAMPLE_DIR)
        .expect("目录加载失败")
        .build();
    // example/ 下 5 个 bundle，每个至少 1 个 collection。
    assert!(
        env.collections().len() >= 5,
        "example/ 下至少应有 5 个 collection（实得 {}）",
        env.collections().len()
    );
    assert!(
        !env.objects().is_empty(),
        "目录加载应解析出对象"
    );
    // 依赖约定不变：每个 collection 的 dependencies[0] = self。
    for (col_idx, col) in env.collections().iter().enumerate() {
        assert_eq!(
            col.dependencies.first().copied().flatten(),
            env.collection_id(col_idx),
            "目录加载后 collection[{col_idx}] dependencies[0] 应仍为自身"
        );
    }
}

#[test]
fn cross_bundle_pptr_resolution_resolved_count_increases() {
    // 把两个 bundle 一起加载（同一 Environment）时，跨 bundle 依赖能命中；
    // 对照：单独加载时那些外部引用为 None。聚合后「已解析依赖槽」数应 >= 各自单独之和。
    let together = Environment::builder()
        .path(TWO_BUNDLES[0])
        .expect("加载 bundle a 失败")
        .path(TWO_BUNDLES[1])
        .expect("加载 bundle b 失败")
        .build();
    let a_alone = load_env(TWO_BUNDLES[0]);
    let b_alone = load_env(TWO_BUNDLES[1]);

    let count_resolved = |env: &Environment| {
        env.collections()
            .iter()
            .map(|c| c.dependencies.iter().filter(|d| d.is_some()).count())
            .sum::<usize>()
    };
    let together_resolved = count_resolved(&together);
    let apart_resolved = count_resolved(&a_alone) + count_resolved(&b_alone);
    // 聚合后 resolved 槽数 >= 分开加载之和（跨 bundle 引用被解析时严格 >）。
    assert!(
        together_resolved >= apart_resolved,
        "聚合后已解析依赖槽数 ({together_resolved}) 应 >= 分开加载之和 ({apart_resolved})"
    );
    if together_resolved > apart_resolved {
        eprintln!(
            "跨 bundle 依赖解析：聚合比分开多解析了 {} 个依赖槽",
            together_resolved - apart_resolved
        );
    } else {
        eprintln!("信息：这两 bundle 间无跨 bundle 引用（resolved 槽数相等）");
    }

    // 全目录扫描：example/ 下 5 个 bundle 一起加载，确认存在真实跨 bundle 解析。
    if Path::new(EXAMPLE_DIR).exists() {
        let dir_env = Environment::builder()
            .scan_dir(EXAMPLE_DIR)
            .expect("目录加载失败")
            .build();
        let dir_together = count_resolved(&dir_env);
        let mut dir_apart = 0usize;
        for name in &[
            "char_002_amiya.ab",
            "Combine.unity3d",
            "DownloadClassIcons.unity3d",
            "DownloadCommonUI.unity3d",
            "Prefabs.unity3d",
        ] {
            let p = format!("{EXAMPLE_DIR}/{name}");
            if Path::new(&p).exists() {
                dir_apart += count_resolved(&load_env(&p));
            }
        }
        let cross = dir_together.saturating_sub(dir_apart);
        eprintln!(
            "全目录：聚合 {dir_together} vs 单独之和 {dir_apart}（跨 bundle 解析 {cross} 个）"
        );
        assert!(
            dir_together >= dir_apart,
            "目录聚合 resolved ({dir_together}) 应 >= 单独之和 ({dir_apart})"
        );
    }
}

#[test]
fn loaded_method_accepts_preloaded_file() {
    // 用户可用自定义 registry 预加载资产文件，再喂入 builder（.loaded() 入口）。
    let raw = unity_asset_rs::load(DEFAULT_BUNDLE).expect("预加载失败");
    let env = Environment::builder().loaded(raw).build();
    assert!(!env.collections().is_empty(), ".loaded() 应能并入预加载的资产文件");
}


// ── DependencyProvider / ResourceProvider 扩展点测试 ──
//
// 数据背景（已用 io 层 SerializedFile.externals 探查确认）：
//   Prefabs.unity3d (collection cab-3f1af86a...) 有 4 个 external：
//     - cab-da81ad817a790a545a5e2c603fe62ad5  ← DownloadClassIcons.unity3d 的 collection
//     - cab-6ec58657aa6ee704cff1feb25c6ab42c  ← 不在任何 example bundle 中（真缺失）
//     - unity default resources                ← 内建，不在 example bundle 中（真缺失）
//     - cab-2872263b14181ced2800d40fa34406c6  ← DownloadCommonUI.unity3d 的 collection
// 故单独加载 Prefabs 时，这 4 个 external 全为 None；用 provider 对其中 2 个按需加载
// 目标 bundle 后，对应的依赖槽应被 resolve 成 Some。

/// Prefabs 引用的、可在 example 中找到的 external（2 个）。
const PREFABS_EXT_CLASSICONS: &str = "cab-da81ad817a790a545a5e2c603fe62ad5";
const PREFABS_EXT_COMMONUI: &str = "cab-2872263b14181ced2800d40fa34406c6";

/// 统计 env 中 resolved（Some）依赖槽总数（跨全部 collection，含 [0]=self）。
fn count_resolved_deps(env: &Environment) -> usize {
    env.collections()
        .iter()
        .map(|c| c.dependencies.iter().filter(|d| d.is_some()).count())
        .sum()
}

/// stub DependencyProvider：按 path_name 映射到磁盘 bundle 路径，命中则重新 load。
///
/// 调用计数走 `Arc<Mutex<HashMap>>`（外部 clone 共享），这样 provider 被 builder
/// 消费后测试仍能查询 `find_dependency` 的实际调用情况。
struct StubDependencyProvider {
    calls: std::sync::Arc<std::sync::Mutex<std::collections::HashMap<String, usize>>>,
}

impl StubDependencyProvider {
    fn new() -> Self {
        Self {
            calls: std::sync::Arc::new(std::sync::Mutex::new(
                std::collections::HashMap::new(),
            )),
        }
    }
    /// 返回共享计数句柄（provider 注册后用它查询）。
    fn counter(&self) -> std::sync::Arc<std::sync::Mutex<std::collections::HashMap<String, usize>>> {
        self.calls.clone()
    }
    /// 根据 query 的 path_name 决定加载哪个 example bundle（模拟「磁盘现取」）。
    fn bundle_for(name: &str) -> Option<&'static str> {
        match name {
            PREFABS_EXT_CLASSICONS => Some("example/DownloadClassIcons.unity3d"),
            PREFABS_EXT_COMMONUI => Some("example/DownloadCommonUI.unity3d"),
            _ => None,
        }
    }
}

impl DependencyProvider for StubDependencyProvider {
    fn find_dependency(
        &self,
        id: &unity_asset_rs::FileIdentifier,
    ) -> Option<unity_asset_rs::LoadedFile> {
        *self.calls.lock().unwrap().entry(id.path_name.clone()).or_insert(0) += 1;
        Self::bundle_for(&id.path_name)
            .map(unity_asset_rs::load)
            .and_then(|r| r.ok())
    }
}

#[test]
fn dependency_provider_lazy_loads_missing_bundles() {
    // 前置：Prefabs + 两个 target bundle 都存在。
    for b in &[
        "example/Prefabs.unity3d",
        "example/DownloadClassIcons.unity3d",
        "example/DownloadCommonUI.unity3d",
    ] {
        if !Path::new(b).exists() {
            eprintln!("跳过：缺少 fixture {b}");
            return;
        }
    }

    // 对照组：无 provider，单独加载 Prefabs —— 4 个 external 全 None。
    let baseline = Environment::builder().path("example/Prefabs.unity3d").unwrap().build();
    let baseline_resolved = count_resolved_deps(&baseline);

    // 实验组：注册 provider，对 2 个 external 按需加载目标 bundle。
    let stub = StubDependencyProvider::new();
    let counter = stub.counter();
    let env = Environment::builder()
        .path("example/Prefabs.unity3d")
        .unwrap()
        .dependency_provider(stub)
        .build();

    let with_provider_resolved = count_resolved_deps(&env);
    eprintln!("baseline resolved = {baseline_resolved}, with provider = {with_provider_resolved}");

    // provider 应对 2 个可加载的 external 各被调用（tried 去重后各 1 次）。
    let classic_calls = *counter.lock().unwrap().get(PREFABS_EXT_CLASSICONS).unwrap_or(&0);
    let ui_calls = *counter.lock().unwrap().get(PREFABS_EXT_COMMONUI).unwrap_or(&0);
    assert!(classic_calls >= 1, "provider 应对 DownloadClassIcons 的 external 被调用");
    assert!(ui_calls >= 1, "provider 应对 DownloadCommonUI 的 external 被调用");

    // resolved 槽应比 baseline 至少多 2（2 个 external 被 resolve）。
    // 注意：懒加载并入的 bundle 自己也有 [0]=self 槽，故增量 >= 2 + 并入 collection 数。
    assert!(
        with_provider_resolved >= baseline_resolved + 2,
        "provider 懒加载后 resolved 槽 ({with_provider_resolved}) 应 >= baseline ({baseline_resolved}) + 2"
    );
    // 并入的 2 个 bundle 应作为新 collection 出现。
    let names: Vec<String> = env
        .collections()
        .iter()
        .map(|c| c.name_fixed.clone())
        .collect();
    assert!(
        names.iter().any(|n| n == PREFABS_EXT_CLASSICONS),
        "DownloadClassIcons 的 collection 应被并入"
    );
    assert!(
        names.iter().any(|n| n == PREFABS_EXT_COMMONUI),
        "DownloadCommonUI 的 collection 应被并入"
    );
}

#[test]
fn no_dependency_provider_leaves_externals_unresolved() {
    // 回归保护：未注册 provider 时，行为退化为纯 eager —— 真缺失的 external 留 None。
    if !Path::new("example/Prefabs.unity3d").exists() {
        eprintln!("跳过：缺少 fixture example/Prefabs.unity3d");
        return;
    }
    let env = Environment::builder().path("example/Prefabs.unity3d").unwrap().build();
    let any_none = env
        .collections()
        .iter()
        .flat_map(|c| c.dependencies.iter())
        .any(|d| d.is_none());
    assert!(any_none, "无 provider 时真缺失的 external 应留 None");
}

// ── ResourceProvider 测试 ──
//
// 场景：Combine.unity3d 含 1 个 SerializedFile + 1 个 .resS（Texture2D 的 streaming
// 数据）。正常加载时 resS 在 bundle 内，resolve_resource 命中、provider 不触发。
// 为制造「resS 缺失」场景，测试用 io 层 load Combine 后**过滤掉 resS leaf**，只把
// SerializedFile leaf 重新喂入 builder——这样 prefetch_resources 扫到 Texture2D 的
// streaming 引用却找不到 resS，触发 ResourceProvider 回调。

/// Combine 的 resS 经 fix_file_identifier 后的归一化名（小写 basename）。
const COMBINE_RESS_NAME_FIXED: &str = "cab-2f6c851076a27266ce7d23d6e02ddafe.ress";

/// 从 `load(path)` 结果中挑出第一个 SerializedFile leaf，丢弃 resS / 其它叶子，
/// 重新包装成单叶 `LoadedFile`（制造「bundle 内无 resS」场景）。
fn load_serialized_only(path: &str) -> Option<LoadedFile> {
    let loaded = unity_asset_rs::load(path).ok()?;
    for (name, bytes, rev, fmt) in loaded.leaves() {
        if fmt == FileFormat::SerializedFile {
            return Some(LoadedFile::Leaf {
                name: name.to_string(),
                bytes: bytes.clone(),
                unity_revision: rev.map(|s| s.to_string()),
                format: FileFormat::SerializedFile,
            });
        }
    }
    None
}

/// stub ResourceProvider：对**任意** name 返回一个确定性合成 ResourceFile
/// （内容是 `[0xAB; N]`），通过共享 `Arc<AtomicBool>` 记录是否被调用。
struct StubResourceProvider {
    called: std::sync::Arc<std::sync::atomic::AtomicBool>,
}

impl StubResourceProvider {
    fn new() -> Self {
        Self {
            called: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
        }
    }
    fn flag(&self) -> std::sync::Arc<std::sync::atomic::AtomicBool> {
        self.called.clone()
    }
}

impl ResourceProvider for StubResourceProvider {
    fn find_resource(&self, _name: &str) -> Option<ResourceFile> {
        self.called.store(true, std::sync::atomic::Ordering::Relaxed);
        // 合成足够大的资源覆盖 Combine 的 Texture2D streaming size（1MB），
        // 使 streaming_data 的 bounds 校验通过。首字节用 0xAB 标记便于断言。
        let mut buf = vec![0xABu8; 1 << 20];
        if !buf.is_empty() {
            buf[0] = 0xAB;
        }
        Some(ResourceFile {
            // name 固定为归一化后的值（绕过 fix_file_identifier，仅供测试）。
            name: String::new(),
            name_fixed: COMBINE_RESS_NAME_FIXED.to_string(),
            bytes: Bytes::from(buf),
        })
    }
}

#[test]
fn resource_provider_prefetches_missing_ress() {
    if !Path::new("example/Combine.unity3d").exists() {
        eprintln!("跳过：缺少 fixture example/Combine.unity3d");
        return;
    }
    let standalone = match load_serialized_only("example/Combine.unity3d") {
        Some(f) => f,
        None => {
            eprintln!("跳过：Combine.unity3d 未解出 SerializedFile leaf");
            return;
        }
    };

    let stub = StubResourceProvider::new();
    let flag = stub.flag();
    let env = Environment::builder()
        .loaded(standalone)
        .resource_provider(stub)
        .build();

    // provider 应被 prefetch_resources 调用过（Combine 的 Texture2D 引用了缺失 resS）。
    assert!(
        flag.load(std::sync::atomic::Ordering::Relaxed),
        "ResourceProvider 应被 prefetch_resources 回调"
    );
    // 合成 resS 应被注册进 resources。
    assert!(
        env.resources()
            .iter()
            .any(|r| r.name_fixed == COMBINE_RESS_NAME_FIXED),
        "合成 resS 应被注册进 resources"
    );
    // streaming_data 应能命中（走 provider 注册的资源），且返回合成的确定性字节。
    let tex_stream = env.objects().iter().find_map(|o| {
        if let ParsedObject::Typed(UnityObject::Texture2D(t)) = &o.object {
            return t.m_stream_data.clone();
        }
        None
    });
    let Some(sd) = tex_stream else {
        eprintln!("跳过：Combine 中无带 streaming 的 Texture2D");
        return;
    };
    let data = env.streaming_data(&sd);
    assert!(data.is_some(), "streaming_data 应命中（经 provider 注册的 resS）");
    let data = data.unwrap();
    assert!(
        !data.is_empty(),
        "streaming_data 取到的字节非空"
    );
    // 合成资源是 [0xAB; 32]，offset=0 处应为 0xAB（只要 size>0）。
    assert_eq!(data[0], 0xAB, "streaming_data 首字节应为合成资源标记 0xAB");
}

#[test]
fn no_resource_provider_keeps_ress_unresolved_when_missing() {
    // 回归保护：无 provider 且 bundle 内无 resS 时，streaming_data 返回 None（不 panic）。
    if !Path::new("example/Combine.unity3d").exists() {
        eprintln!("跳过：缺少 fixture example/Combine.unity3d");
        return;
    }
    let standalone = match load_serialized_only("example/Combine.unity3d") {
        Some(f) => f,
        None => return,
    };
    let env = Environment::builder().loaded(standalone).build();
    let tex_stream = env.objects().iter().find_map(|o| {
        if let ParsedObject::Typed(UnityObject::Texture2D(t)) = &o.object {
            return t.m_stream_data.clone();
        }
        None
    });
    let Some(sd) = tex_stream else { return; };
    assert!(
        env.streaming_data(&sd).is_none(),
        "无 provider 且 bundle 内无 resS 时 streaming_data 应返回 None"
    );
}
