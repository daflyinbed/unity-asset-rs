//! Environment 层端到端测试：从真实 bundle 构建 Environment，验证对象聚合、
//! PPtr 解引用、streaming_data。
//!
//! 注意：debug 模式下 `interpret` 递归帧较大（5–6MB），测试线程默认 2MB 栈会溢出。
//! 故 `load_env` 在 8MB 栈线程里跑（release 模式无此问题）。

use std::path::Path;

use unity_asset_rs::{
    load_environment, ClassIDType, Environment, ParseFailure, ParsedObject, UnityObject,
};

/// 默认 fixture（io 层测试同款）。
const DEFAULT_BUNDLE: &str = "example/char_002_amiya.ab";

/// 在大栈线程里加载 Environment（debug 模式 interpret 递归需要 >5MB 栈）。
fn load_env(path: &str) -> Environment {
    let p = Path::new(path);
    if !p.exists() {
        eprintln!("跳过：{path} 不存在");
        std::process::exit(0);
    }
    let path = path.to_owned();
    std::thread::Builder::new()
        .stack_size(8 * 1024 * 1024)
        .spawn(move || match load_environment(&path) {
            Ok(e) => e,
            Err(e) => panic!("加载 {path} 失败: {e}"),
        })
        .unwrap()
        .join()
        .unwrap()
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
