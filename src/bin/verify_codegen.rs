//! 验证：用 codegen 生成的代码解析 bundle 里的全部对象。
//! MonoBehaviour (classID=114) 走 TypeTree 解释器，其余走 generated typed parse。
//!
//! 默认遍历 `example/` 目录下的全部 bundle；可传一个路径（文件或目录）作为第 1
//! 参数，只处理它。`.resS`（streaming-resource 伴生文件）不是 SerializedFile，跳过。

use std::path::Path;

use unity_asset_rs::{load, parse_object, ClassIDType, LoadedFile, SerializedFile};

/// 全局累计统计。
#[derive(Default)]
struct Stats {
    bundles: usize,
    total: usize,
    typed_ok: usize,
    /// MonoBehaviour（classID=114）：typed shell + 动态 structure。
    mono_ok: usize,
    /// 其他类型的 TypeTree 解释器回退（typed parse 失败后走 dynamic）。
    dynamic_ok: usize,
    typed_fail: usize,
    skipped_files: usize,
    failures: Vec<(String, i32, String, String)>, // (bundle, classID, type_name, error)
}

fn main() {
    env_logger::init();
    let target = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "example".into());
    let path = Path::new(&target);

    // 收集要处理的 bundle 文件列表。
    let files: Vec<String> = if path.is_dir() {
        let mut entries: Vec<String> = std::fs::read_dir(path)
            .unwrap_or_else(|e| {
                eprintln!("读取目录 {target} 失败: {e}");
                std::process::exit(1);
            })
            .filter_map(|e| e.ok())
            .filter_map(|e| {
                let p = e.path();
                p.is_file().then(|| p.to_string_lossy().into_owned())
            })
            .collect();
        entries.sort();
        entries
    } else if path.is_file() {
        vec![target.clone()]
    } else {
        eprintln!("路径不存在: {target}");
        std::process::exit(1);
    };

    if files.is_empty() {
        eprintln!("{target} 下没有文件");
        std::process::exit(1);
    }

    let mut stats = Stats::default();
    for file in &files {
        eprintln!("\n#################### {file} ####################");
        process_bundle(file, &mut stats);
    }

    print_global_summary(&stats);
    if !stats.failures.is_empty() {
        std::process::exit(1);
    }
}

/// 处理单个资产文件，把统计累加进 `stats`。
fn process_bundle(path: &str, stats: &mut Stats) {
    let loaded: LoadedFile = match load(path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("✗ 加载资产文件失败: {e}");
            stats.failures.push((path.to_string(), -1, String::new(), format!("load: {e}")));
            return;
        }
    };
    stats.bundles += 1;
    let leaves = loaded.leaves();
    eprintln!("bundle: {} 个内部文件", leaves.len());

    for (name, bytes, _ur) in &leaves {
        // .resS 是 streaming-resource 伴生文件（纹理/音频流数据），不是 SerializedFile。
        if name.ends_with(".resS") {
            stats.skipped_files += 1;
            eprintln!("\n=== 跳过（streaming resource）: {} ({} bytes) ===", name, bytes.len());
            continue;
        }

        eprintln!("\n=== 内部文件: {} ({} bytes) ===", name, bytes.len());
        let sf = match SerializedFile::parse((*bytes).clone()) {
            Ok(sf) => sf,
            Err(e) => {
                eprintln!("  ✗ 解析 SerializedFile 失败: {e}");
                stats.failures.push((
                    path.to_string(),
                    -1,
                    name.to_string(),
                    format!("SerializedFile::parse: {e}"),
                ));
                continue;
            }
        };
        eprintln!(
            "Unity {}, format v{}, endian {:?}, {} 个对象",
            sf.unity_version, sf.version, sf.endian, sf.objects.len()
        );

        let refs = sf.ref_type_table();
        for (i, obj) in sf.objects.iter().enumerate() {
            stats.total += 1;
            let class_id = obj.class_id(&sf);
            let type_name = ClassIDType::from_i32(class_id)
                .map(|c| format!("{c:?}"))
                .unwrap_or_else(|| format!("ClassID({class_id})"));
            match parse_object(&sf, obj, &refs) {
                Ok(unity_asset_rs::ParsedObject::Typed(_)) => {
                    stats.typed_ok += 1;
                    eprintln!("  [{i}] classID={class_id:>4} {type_name:<28} ✓ typed");
                }
                Ok(unity_asset_rs::ParsedObject::MonoBehaviour(_)) => {
                    stats.mono_ok += 1;
                    eprintln!(
                        "  [{i}] classID={class_id:>4} {type_name:<28} ✓ monobehaviour (shell+structure)"
                    );
                }
                Ok(unity_asset_rs::ParsedObject::Dynamic(_)) => {
                    stats.dynamic_ok += 1;
                    eprintln!("  [{i}] classID={class_id:>4} {type_name:<28} ✓ dynamic (TypeTree)");
                }
                Err(e) => {
                    stats.typed_fail += 1;
                    eprintln!("  [{i}] classID={class_id:>4} {type_name:<28} ✗ {e}");
                    stats.failures.push((
                        path.to_string(),
                        class_id,
                        type_name,
                        format!("{e}"),
                    ));
                }
            }
        }
    }
}

/// 打印全部 bundle 跑完后的全局汇总。
fn print_global_summary(stats: &Stats) {
    eprintln!("\n========== 全局汇总 ==========");
    eprintln!("bundle 数:    {}", stats.bundles);
    eprintln!("总对象数:     {}", stats.total);
    eprintln!("typed 成功:   {}", stats.typed_ok);
    eprintln!("monobehaviour: {}", stats.mono_ok);
    eprintln!("dynamic:      {}", stats.dynamic_ok);
    eprintln!("失败:         {}", stats.typed_fail);
    eprintln!("跳过文件:     {}", stats.skipped_files);
    if !stats.failures.is_empty() {
        eprintln!("\n失败明细:");
        for (bundle, cid, name, err) in &stats.failures {
            eprintln!("  [{bundle}] classID={cid} {name}: {err}");
        }
    }
}
