use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use unity_asset_codegen::ir::{EmitClass, RustType, VersionGate};
use unity_asset_codegen::{run_pipeline, TpkFile};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let args: Vec<String> = std::env::args().collect();
    let tpk_path = args
        .get(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("crates/codegen/tests/fixtures/type_tree.tpk"));

    eprintln!("读取 TPK: {}", tpk_path.display());
    let tpk = TpkFile::read_from_path(&tpk_path)?;
    let blob = tpk.into_type_tree()?;
    eprintln!("TPK: {} versions, {} classes", blob.versions.len(), blob.classes.len());

    let state = run_pipeline(blob);

    let classes: Vec<&EmitClass> = state.classes.values().collect();
    let main_classes: Vec<&EmitClass> = classes.iter().copied().filter(|c| c.is_main()).collect();
    let subclasses: Vec<&EmitClass> = classes.iter().copied().filter(|c| !c.is_main()).collect();

    println!("\n========== 基础统计 ==========");
    println!("总 class 数:        {}", classes.len());
    println!("  main classes:     {}", main_classes.len());
    println!("  subclasses:       {}", subclasses.len());

    // 版本数统计
    let version_counts: Vec<usize> = classes
        .iter()
        .map(|c| c.version_layouts.len().max(1)) // 空 layouts 表示只有一个合并顺序
        .collect();
    let total_versions: usize = version_counts.iter().sum();
    let min_v = version_counts.iter().min().copied().unwrap_or(0);
    let max_v = version_counts.iter().max().copied().unwrap_or(0);
    let avg_v = if !version_counts.is_empty() {
        total_versions as f64 / version_counts.len() as f64
    } else {
        0.0
    };
    println!("\n每个 class 覆盖的版本 layout 数:");
    println!("  min: {}, max: {}, avg: {:.2}", min_v, max_v, avg_v);

    // 字段增减统计
    println!("\n========== 字段增减 (VersionGate) ==========");
    let mut always = 0usize;
    let mut since_added = 0usize;
    let mut range_count = 0usize;
    for c in &classes {
        for f in &c.own_fields {
            match f.version_gate {
                Some(VersionGate::Always) | None => always += 1,
                Some(VersionGate::Since(_, _, _)) => since_added += 1,
                Some(VersionGate::Range { .. }) => range_count += 1,
            }
        }
    }
    let total_fields: usize = classes.iter().map(|c| c.own_fields.len()).sum();
    println!("总字段数 (去重后):  {}", total_fields);
    println!("  Always (全版本):  {} ({:.2}%)", always, pct(always, total_fields));
    println!("  Since (新增):     {} ({:.2}%)", since_added, pct(since_added, total_fields));
    println!("  Range (曾消失):   {} ({:.2}%)", range_count, pct(range_count, total_fields));

    // 字段类型跨版本变化
    println!("\n========== 同名字段跨版本类型变化 ==========");
    let mut type_change_fields = Vec::new();
    for c in &classes {
        let mut types_by_name: BTreeMap<&str, Vec<&RustType>> = BTreeMap::new();
        for layout in &c.version_layouts {
            for f in &layout.fields {
                let v = types_by_name
                    .entry(f.rust_name.as_str())
                    .or_default();
                if !v.contains(&&f.rust_type) {
                    v.push(&f.rust_type);
                }
            }
        }
        for (name, types) in types_by_name {
            if types.len() > 1 {
                type_change_fields.push((c.name.clone(), name.to_string(), types.len()));
            }
        }
    }
    println!("有类型变化的字段数: {} (涉及 {} 个 class)", type_change_fields.len(), {
        let mut cs = BTreeSet::new();
        for (c, _, _) in &type_change_fields {
            cs.insert(c.clone());
        }
        cs.len()
    });
    println!("\n前 20 个示例 (class, field, 不同类型数):");
    for (c, f, n) in type_change_fields.iter().take(20) {
        println!("  {}.{}  ({} 种类型)", c, f, n);
    }

    // 字段顺序变化
    println!("\n========== 字段顺序重排 ==========");
    let mut reorder_classes = 0usize;
    let mut insertion_only_classes = 0usize;
    let mut true_reorder_classes = 0usize;
    let mut reorder_examples = Vec::new();

    for c in &classes {
        if c.version_layouts.is_empty() {
            continue;
        }
        reorder_classes += 1;

        let orders: Vec<Vec<&str>> = c
            .version_layouts
            .iter()
            .map(|l| l.fields.iter().map(|f| f.rust_name.as_str()).collect())
            .collect();

        let mut any_reorder = false;
        'outer: for i in 0..orders.len() {
            for j in (i + 1)..orders.len() {
                if !common_order_consistent(&orders[i], &orders[j]) {
                    any_reorder = true;
                    break 'outer;
                }
            }
        }

        if any_reorder {
            true_reorder_classes += 1;
        } else {
            insertion_only_classes += 1;
        }

        reorder_examples.push((c.name.clone(), c.version_layouts.len(), any_reorder));
    }

    println!("需要 version-gated parse 的 class 数: {} / {} ({:.2}%)",
        reorder_classes, classes.len(), pct(reorder_classes, classes.len()));
    println!("  真重排 (公共字段顺序变了): {} 个", true_reorder_classes);
    println!("  仅插入 (新字段插到现有字段之间): {} 个", insertion_only_classes);
    println!("\nversion-gated class 示例 (class, 版本分支数, 是否真重排):");
    reorder_examples.sort_by_key(|b| std::cmp::Reverse(b.1));
    for (c, n, true_reorder) in reorder_examples.iter().take(20) {
        println!("  {}  ({} 分支, {})", c, n, if *true_reorder { "真重排" } else { "仅插入" });
    }

    // 综合：每 class 的字段变化种类
    println!("\n========== 变化 class 概览 ==========");
    let mut changed_classes = BTreeSet::new();
    for c in &classes {
        if c.version_layouts.is_empty() {
            continue;
        }
        changed_classes.insert(&c.name);
    }
    for (c, _, _) in &type_change_fields {
        changed_classes.insert(c);
    }
    for c in &classes {
        if c.own_fields.iter().any(|f| !matches!(f.version_gate, Some(VersionGate::Always) | None)) {
            changed_classes.insert(&c.name);
        }
    }
    println!("至少有一种跨版本字段变化的 class 数: {} / {}", changed_classes.len(), classes.len());

    Ok(())
}

fn pct(part: usize, total: usize) -> f64 {
    if total == 0 {
        0.0
    } else {
        part as f64 * 100.0 / total as f64
    }
}

fn common_order_consistent(a: &[&str], b: &[&str]) -> bool {
    let a_common: Vec<&str> = a.iter().copied().filter(|x| b.contains(x)).collect();
    let b_common: Vec<&str> = b.iter().copied().filter(|x| a.contains(x)).collect();
    a_common == b_common
}
