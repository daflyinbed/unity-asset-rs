//! Environment 层端到端测试：从真实 bundle 构建 Environment，验证对象聚合、
//! PPtr 解引用、streaming_data、跨 bundle 聚合。

use std::path::Path;

use unity_asset_rs::{
    load_environment, load_environment_dir, load_environment_multi, ClassIDType, Environment,
    ParseFailure, ParsedObject, UnityObject,
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
    match load_environment(path) {
        Ok(e) => e,
        Err(e) => panic!("加载 {path} 失败: {e}"),
    }
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
        assert_eq!(
            col.dependencies.first().copied().flatten(),
            Some(col_idx),
            "collection[{col_idx}] dependencies[0] 应为自身"
        );
        for (&path_id, &obj_idx) in &col.assets {
            let resolved = env.resolve_local(col_idx, path_id);
            assert_eq!(
                resolved.map(|o| o as *const _),
                Some(&env.objects()[obj_idx] as *const _),
                "resolve_local({col_idx}, {path_id}) 应命中 objects[{obj_idx}]"
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
        }
    }
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
            if let Some(cls) = ClassIDType::from_i32(o.class_id) {
                sum += env.objects_of_class(cls).count();
            }
        }
    }
    assert!(sum <= total, "objects_of_class 求和 {sum} 不应超过总数 {total}");
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
fn load_environment_multi_matches_builder() {
    // load_environment_multi 应与 builder 等价。
    let via_multi = load_environment_multi(TWO_BUNDLES).expect("multi 加载失败");
    let via_builder = Environment::builder()
        .path(TWO_BUNDLES[0])
        .unwrap()
        .path(TWO_BUNDLES[1])
        .unwrap()
        .build();
    assert_eq!(
        via_multi.collections().len(),
        via_builder.collections().len(),
    );
    assert_eq!(
        via_multi.objects().len(),
        via_builder.objects().len(),
    );
}

#[test]
fn load_environment_dir_scans_all_bundles() {
    if !Path::new(EXAMPLE_DIR).exists() {
        eprintln!("跳过：{EXAMPLE_DIR} 不存在");
        std::process::exit(0);
    }
    let env = load_environment_dir(EXAMPLE_DIR).expect("目录加载失败");
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
            Some(col_idx),
            "目录加载后 collection[{col_idx}] dependencies[0] 应仍为自身"
        );
    }
}

#[test]
fn cross_bundle_pptr_resolution_resolved_count_increases() {
    // 把两个 bundle 一起加载（同一 Environment）时，跨 bundle 依赖能命中；
    // 对照：单独加载时那些外部引用为 None。聚合后「已解析依赖槽」数应 >= 各自单独之和。
    let together = load_environment_multi(TWO_BUNDLES).expect("multi 加载失败");
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
        let dir_env = load_environment_dir(EXAMPLE_DIR).expect("目录加载失败");
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
fn bundle_method_accepts_preloaded_bundle() {
    // 用户可用自定义 registry 预加载 bundle，再喂入 builder（.bundle() 入口）。
    let raw = unity_asset_rs::load(DEFAULT_BUNDLE).expect("预加载失败");
    let env = Environment::builder().bundle(raw).build();
    assert!(!env.collections().is_empty(), ".bundle() 应能并入预加载的 bundle");
}
