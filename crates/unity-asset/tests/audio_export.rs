//! 音频导出层端到端测试：从真实 AssetBundle 加载 AudioClip，经 `.resource` 读取
//! FSB5，并把 sample 重建为 WAV / Ogg。
//!
//! fixture 来自同级 Unity 测试工程 `unity-projects/cst-audio2023`。只测试构建汇总中
//! 未发生平台 fallback 的 PCM16 / IMAADPCM / Vorbis；PCM8 / PCM32 / GCADPCM 在该
//! StandaloneLinux64 构建中实际均被 Unity 回退成 PCM16。

#![cfg(feature = "export")]

use std::path::{Path, PathBuf};

use generated::{ClassIDType, UnityObject};
use unity_asset::audio_export::{DecodedAudio, decode_audio};
use unity_asset::{Environment, Loader, ParsedObject};

const AUDIO_BUNDLE_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../../unity-projects/cst-audio2023/AssetBundles"
);

fn bundle_path(name: &str) -> PathBuf {
    Path::new(AUDIO_BUNDLE_DIR).join(name)
}

/// fixture 不存在时跳过，对齐现有 texture/environment 集成测试约定。
fn load_env(name: &str) -> Option<Environment> {
    let path = bundle_path(name);
    if !path.exists() {
        eprintln!("跳过：{} 不存在", path.display());
        return None;
    }
    Some(
        Loader::builder()
            .path(&path)
            .build()
            .unwrap_or_else(|e| panic!("加载 {} 失败: {e}", path.display()))
            .into_environment(),
    )
}

fn decode_single(name: &str) -> Option<DecodedAudio> {
    let env = load_env(name)?;
    let clips: Vec<_> = env.objects_of_class(ClassIDType::AudioClip).collect();
    assert_eq!(clips.len(), 1, "{name} 应恰好包含一个 AudioClip");

    let clip_obj = clips[0];
    let ParsedObject::Typed(UnityObject::AudioClip(_)) = &clip_obj.object else {
        panic!("{name} 的 AudioClip 应由 typed parser 解析");
    };
    let raw = env
        .audio_data(clip_obj)
        .expect("应能从 .resource 读取音频数据");
    assert!(raw.starts_with(b"FSB5"), "AudioClip 原始数据应为 FSB5");

    let mut decoded = decode_audio(&env, clip_obj).expect("FSB5 sample 应能重建");
    assert_eq!(decoded.len(), 1, "fixture 应恰好包含一个 FSB sample");
    Some(decoded.remove(0))
}

fn assert_pcm_wav(audio: &DecodedAudio) {
    assert_eq!(audio.extension, "wav");
    assert!(audio.data.len() >= 44, "WAV 至少应含完整 header");
    assert_eq!(&audio.data[..4], b"RIFF");
    assert_eq!(&audio.data[8..12], b"WAVE");
    assert_eq!(&audio.data[12..16], b"fmt ");
    assert_eq!(
        u16::from_le_bytes([audio.data[20], audio.data[21]]),
        1,
        "应为 PCM"
    );
    assert_eq!(
        u16::from_le_bytes([audio.data[22], audio.data[23]]),
        2,
        "应为双声道"
    );
    assert_eq!(
        u32::from_le_bytes(audio.data[24..28].try_into().unwrap()),
        44_100,
        "采样率应为 44100 Hz"
    );
    assert_eq!(
        u16::from_le_bytes([audio.data[34], audio.data[35]]),
        16,
        "重建输出应为 16-bit PCM"
    );
}

#[test]
fn pcm16_bundle_rebuilds_to_wav() {
    let Some(audio) = decode_single("cst_pcm16") else {
        return;
    };
    assert_pcm_wav(&audio);
}

#[test]
fn ima_adpcm_bundle_rebuilds_to_wav() {
    let Some(audio) = decode_single("cst_imaadpcm") else {
        return;
    };
    assert_pcm_wav(&audio);
}

#[test]
fn vorbis_bundle_rebuilds_to_ogg() {
    let Some(audio) = decode_single("cst_vorbis") else {
        return;
    };
    assert_eq!(audio.extension, "ogg");
    assert!(audio.data.len() > 64, "Ogg 输出不应只有 header");
    assert_eq!(&audio.data[..4], b"OggS");
    assert!(
        audio.data.windows(6).any(|window| window == b"vorbis"),
        "Ogg 应包含 Vorbis identification header"
    );
}
