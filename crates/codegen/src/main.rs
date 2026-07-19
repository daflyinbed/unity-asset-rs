use std::path::{Path, PathBuf};

use unity_asset_codegen::{TpkFile, emit::emit_all, run_pipeline};

fn main() {
    env_logger::init();

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("用法: {} <type_tree.tpk> <out_dir>", args[0]);
        eprintln!(
            "示例: {} crates/codegen/tests/fixtures/type_tree.tpk crates/generated/src",
            args[0]
        );
        std::process::exit(2);
    }

    let tpk_path = PathBuf::from(&args[1]);
    let out_dir = PathBuf::from(&args[2]);

    if let Err(e) = run(&tpk_path, &out_dir) {
        eprintln!("错误: {e}");
        std::process::exit(1);
    }
}

fn run(tpk_path: &Path, out_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("读取 TPK: {}", tpk_path.display());
    let tpk = TpkFile::read_from_path(tpk_path)?;
    let blob = tpk.into_type_tree()?;

    eprintln!(
        "TPK: {} versions, {} classes",
        blob.versions.len(),
        blob.classes.len()
    );

    eprintln!("跑 pass 管线...");
    let state = run_pipeline(blob);

    eprintln!(
        "结果：{} 个 EmitClass（{} main + {} subclass），{} 个 PPtr 目标",
        state.classes.len(),
        state.classes.values().filter(|c| c.is_main()).count(),
        state.classes.values().filter(|c| !c.is_main()).count(),
        state.pptr_targets.len()
    );

    eprintln!("emit 到 {}", out_dir.display());
    emit_all(&state, out_dir)?;

    // 统计生成代码
    let total_lines = count_lines(out_dir)?;
    let total_files = count_rust_files(out_dir)?;
    eprintln!("完成：{total_files} 个 .rs 文件，{total_lines} 行");

    Ok(())
}

fn count_lines(dir: &Path) -> std::io::Result<usize> {
    let mut total = 0;
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            total += count_lines(&path)?;
        } else if path.extension().is_some_and(|e| e == "rs") {
            let content = std::fs::read_to_string(&path)?;
            total += content.lines().count();
        }
    }
    Ok(total)
}

fn count_rust_files(dir: &Path) -> std::io::Result<usize> {
    let mut total = 0;
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            total += count_rust_files(&path)?;
        } else if path.extension().is_some_and(|e| e == "rs") {
            total += 1;
        }
    }
    Ok(total)
}
