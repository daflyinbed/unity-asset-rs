use unity_asset_codegen::TpkFile;

fn main() {
    let tpk = TpkFile::read_from_path(std::path::Path::new("crates/codegen/tests/fixtures/type_tree.tpk")).unwrap();
    let blob = tpk.into_type_tree().unwrap();

    // 找几个相关 class 的 ID
    let mut name_to_id = std::collections::HashMap::new();
    for (id, info) in &blob.classes {
        if let Some(c) = info.classes.iter().find_map(|e| e.class.as_ref()) {
            let n = &blob.string_pool[c.name_index as usize];
            name_to_id.insert(n.clone(), *id);
        }
    }

    for name in ["Object", "NamedObject", "Texture", "Texture2D"] {
        let id = name_to_id[name];
        let info = &blob.classes[&id];
        let first = info.classes.iter().find_map(|e| e.class.as_ref()).unwrap();
        let base = first.base_index.and_then(|i| blob.string_pool.get(i as usize)).cloned();
        let root_fields: Vec<String> = if let Some(ri) = first.release_root {
            let root = &blob.node_pool[ri as usize];
            root.sub_nodes.iter().map(|&i| {
                let n = &blob.node_pool[i as usize];
                let tn = &blob.string_pool[n.type_name_index as usize];
                let nm = &blob.string_pool[n.name_index as usize];
                format!("{tn} {nm}")
            }).collect()
        } else {
            vec!["<无 release_root>".into()]
        };

        println!("\n=== {name} (classID={id}, base={base:?}) ===");
        println!("  release_root 含 {n} 个字段:", n = root_fields.len());
        for f in &root_fields {
            println!("    {f}");
        }
    }
}
