//! TPK reader 黄金测试：用真实的 AssetRipper type_tree.tpk（brotli 版）验证解析。
//!
//! fixture: `tests/fixtures/type_tree.tpk`（brotli 压缩，186KB）。
//! 数字与 ADR-0001 §9 的实测统计一致。

use unity_asset_codegen::TpkFile;

const TPK_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/type_tree.tpk");

#[test]
fn read_header_and_decompress() {
    let tpk = TpkFile::read_from_path(std::path::Path::new(TPK_PATH)).unwrap();
    let blob = tpk.into_type_tree().unwrap();

    // ADR-0001 §9: ~1395 个 Unity 版本、412 个 TypeID。
    assert_eq!(blob.versions.len(), 1395, "Unity 版本数");
    assert_eq!(blob.classes.len(), 412, "class 总数");

    // 字符串池 / 节点池（与调研报告里 "实际大约 21k" 吻合）。
    assert!(blob.string_pool.len() > 7000, "字符串池大小");
    assert!(blob.node_pool.len() > 20000, "节点池大小");

    // versions 应升序。
    let mut sorted = blob.versions.clone();
    sorted.sort();
    assert_eq!(blob.versions, sorted, "版本应升序");

    // 第一个版本应该是 3.4（ADR-0001 §9: "覆盖 Unity 3.4 → 6000.x"）。
    let first = blob.versions[0];
    assert_eq!(first.major, 3, "首个版本 major");
    assert_eq!(first.minor, 4, "首个版本 minor");
}

#[test]
fn texture2d_class_exists() {
    let tpk = TpkFile::read_from_path(std::path::Path::new(TPK_PATH)).unwrap();
    let blob = tpk.into_type_tree().unwrap();

    let tex = blob.classes.get(&28).expect("Texture2D classID = 28");
    let name_idx = tex
        .classes
        .iter()
        .find_map(|c| c.class.as_ref())
        .map(|c| c.name_index)
        .unwrap();
    assert_eq!(blob.string_pool[name_idx as usize], "Texture2D");
}

#[test]
fn gameobject_node_tree_shape() {
    let tpk = TpkFile::read_from_path(std::path::Path::new(TPK_PATH)).unwrap();
    let blob = tpk.into_type_tree().unwrap();

    // GameObject (classID = 1) 的首个有 release_root 的版本变体
    let go = blob.classes.get(&1).expect("GameObject classID = 1");
    let root_idx = go
        .classes
        .iter()
        .find_map(|c| c.class.as_ref().and_then(|c| c.release_root))
        .expect("至少一个版本有 release root");
    let root = &blob.node_pool[root_idx as usize];

    // 根节点应是 "GameObject" 类型、名字是 "Base"。
    assert_eq!(blob.string_pool[root.type_name_index as usize], "GameObject");
    assert_eq!(blob.string_pool[root.name_index as usize], "Base");

    // 根的 meta_flag 应含 0x8000 (AnyChildUsesAlignBytes)。
    assert_ne!(root.meta_flag & 0x8000, 0, "根节点应有 AnyChildUsesAlignBytes");

    // 应有 m_Component 子节点（vector 类型）。
    let comp = root
        .sub_nodes
        .iter()
        .find_map(|&i| {
            let n = &blob.node_pool[i as usize];
            (blob.string_pool[n.name_index as usize] == "m_Component").then_some(n)
        })
        .expect("GameObject 应有 m_Component 字段");
    assert_eq!(blob.string_pool[comp.type_name_index as usize], "vector");
}
