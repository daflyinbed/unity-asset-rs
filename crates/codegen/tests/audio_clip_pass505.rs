//! pre-5.0 AudioClip m_AudioData 值依赖读（Pass505 Rust 移植）回归测试。
//!
//! 确保生成的 `audio_clip.rs`：
//! - 含 `AudioClipMAudioData` 枚举（`Inline` / `Streamed` 两变体）；
//! - pre-5.0 分支按 `m_stream == Some(2)` 分流读 `u32 size; u32 offset;` 或内联数组；
//! - `m_audio_data` 字段类型为 `Option<AudioClipMAudioData>`。
//!
//! 对齐 AssetRipper `Pass505_FixOldAudioClip` + `AudioClipHelper.ReadOldByteArray`。
//! fixture: `tests/fixtures/type_tree.tpk`。

use std::path::Path;

use unity_asset_codegen::{TpkFile, run_pipeline};

const TPK_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/type_tree.tpk");

/// 跑完整 pipeline + emit 到临时目录，读回 `classes/audio_clip.rs` 内容。
fn generated_audio_clip_src() -> String {
    let tpk = TpkFile::read_from_path(Path::new(TPK_PATH)).unwrap();
    let blob = tpk.into_type_tree().unwrap();
    let state = run_pipeline(blob);
    // 用进程唯一目录避免并发测试串扰；结束时删除。
    let tmp_root = std::env::temp_dir().join(format!(
        "unity-asset-codegen-test-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    unity_asset_codegen::emit::emit_all(&state, &tmp_root).unwrap();
    let src = std::fs::read_to_string(tmp_root.join("classes").join("audio_clip.rs")).unwrap();
    let _ = std::fs::remove_dir_all(&tmp_root);
    src
}

#[test]
fn audio_clip_m_audio_data_enum_exists() {
    let src = generated_audio_clip_src();

    // 枚举定义。
    assert!(
        src.contains("pub enum AudioClipMAudioData"),
        "应有 AudioClipMAudioData 枚举定义"
    );
    assert!(src.contains("Inline(Vec<u8>)"), "Inline 变体应为 Vec<u8>");
    assert!(
        src.contains("Streamed { size: u32, offset: u32 }"),
        "Streamed 变体应为 {{ size: u32, offset: u32 }}"
    );
}

#[test]
fn audio_clip_field_uses_enum_type() {
    let src = generated_audio_clip_src();
    assert!(
        src.contains("pub m_audio_data: Option<AudioClipMAudioData>"),
        "m_audio_data 字段类型应为 Option<AudioClipMAudioData>"
    );
}

#[test]
fn pre_v5_branch_is_value_dependent() {
    let src = generated_audio_clip_src();

    // 值依赖分流：m_stream == Some(2) → Streamed { size, offset }
    assert!(
        src.contains("if m_stream == Some(2)"),
        "pre-5.0 分支应按 m_stream == Some(2) 分流"
    );
    // Streaming 路径读 u32 size; u32 offset;
    assert!(
        src.contains("let size = rdr.read_u32()?;"),
        "Streaming 路径应读 u32 size"
    );
    assert!(
        src.contains("let offset = rdr.read_u32()?;"),
        "Streaming 路径应读 u32 offset"
    );
    assert!(
        src.contains("AudioClipMAudioData::Streamed { size, offset }"),
        "Streaming 路径应构造 Streamed 变体"
    );
    // Inline 路径保留原内联数组读法。
    assert!(
        src.contains("AudioClipMAudioData::Inline(v)"),
        "非 Streaming 路径应构造 Inline 变体"
    );
}
