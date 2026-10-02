//! `impl ToValue` 发射回归测试（typed struct → `Value`，JSON dump 数据面）。
//!
//! 确保生成的 class 文件：
//! - 含 `impl crate::ToValue`，键名用原始 PascalCase `unity_name`（与 typetree
//!   解释器产出的键一致）；
//! - 版本门控 Option 字段走 `opt_field`（None 跳过）；
//! - TypelessData 字段直取 `Value::Bytes`（绕开 `Vec<u8>` blanket 歧义）。
//!
//! fixture: `tests/fixtures/type_tree.tpk`。

use std::path::Path;

use unity_asset_codegen::{TpkFile, run_pipeline};

const TPK_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/type_tree.tpk");

/// 跑完整 pipeline + emit 到临时目录，读回指定文件内容。
fn generated_src(rel: &str) -> String {
    let tpk = TpkFile::read_from_path(Path::new(TPK_PATH)).unwrap();
    let blob = tpk.into_type_tree().unwrap();
    let state = run_pipeline(blob);
    let tmp_root = std::env::temp_dir().join(format!(
        "unity-asset-codegen-to-value-test-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    unity_asset_codegen::emit::emit_all(&state, &tmp_root).unwrap();
    let src = std::fs::read_to_string(tmp_root.join(rel)).unwrap();
    let _ = std::fs::remove_dir_all(&tmp_root);
    src
}

#[test]
fn class_files_emit_to_value_impls() {
    let src = generated_src("classes/text_asset.rs");
    assert!(
        src.contains("impl crate::ToValue for TextAsset"),
        "TextAsset 应有 ToValue impl"
    );
    // 键名用原始 PascalCase unity_name（非 rust_name 的 m_name）。
    assert!(
        src.contains(r#""m_Name".to_string()"#),
        "键名应为 PascalCase m_Name"
    );
    // 版本门控 Option 字段（m_PathName，ver < 2017.1）走 opt_field。
    assert!(
        src.contains(r#"crate::opt_field("m_PathName", &self.m_path_name)"#),
        "门控字段应走 opt_field"
    );
}

#[test]
fn typeless_data_field_maps_to_value_bytes() {
    // Texture2D 的 image_data（TypelessData）应直取 Value::Bytes。
    let src = generated_src("classes/texture2_d.rs");
    assert!(
        src.contains("crate::Value::Bytes(self.image_data.clone())"),
        "TypelessData 字段应直取 Value::Bytes"
    );
}

#[test]
fn unity_object_enum_dispatches_to_value() {
    let src = generated_src("unity_object.rs");
    assert!(
        src.contains("impl crate::ToValue for UnityObject"),
        "UnityObject 枚举应有 ToValue 分派"
    );
    assert!(
        src.contains("Self::Texture2D(v) => v.to_value(),"),
        "应有 Texture2D 分派臂"
    );
}
