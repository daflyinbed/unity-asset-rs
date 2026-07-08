use std::collections::HashMap;
use unity_asset_codegen::tpk::TpkFile;
use unity_asset_codegen::pipeline;
use unity_asset_codegen::ir::{ClassKey, RustType};

fn main() {
    let blob = TpkFile::read_from_path(std::path::Path::new("crates/codegen/tests/fixtures/type_tree.tpk"))
        .unwrap().into_type_tree().unwrap();
    let state = pipeline::run_pipeline(blob);

    for cls in state.classes.values() {
        if !cls.is_main() || cls.version_layouts.is_empty() { continue; }
        let own_types: HashMap<&str, RustType> = cls.own_fields.iter()
            .map(|f| (f.rust_name.as_str(), f.rust_type.clone()))
            .collect();

        for layout in &cls.version_layouts {
            for lf in &layout.fields {
                if let Some(own_t) = own_types.get(lf.rust_name.as_str()) {
                    if own_t != &lf.rust_type {
                        let (maj, min, _) = layout.since;
                        println!("{}: field={} own={:?} but v{maj}.{min}={:?}",
                            cls.name, lf.rust_name, own_t, lf.rust_type);
                    }
                }
            }
        }
    }
}
