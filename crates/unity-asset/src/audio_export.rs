//! AudioClip 音频拆分（受 `export` feature 门控）。
//!
//! 数据定位复用 [`Environment::audio_data`]；FSB5 解析与 codec 重建委托给
//! `fsb-rs`（Fmod5Sharp 的 Rust port）。每个 FSB sample 独立输出为 WAV/Ogg。

use std::error::Error;
use std::fmt;

use bytes::Bytes;
use generated::UnityObject;

use crate::{Environment, EnvironmentObject, ParsedObject};

/// 一个从 AudioClip 拆出的、可直接写盘的音频文件。
#[derive(Debug, Clone)]
pub struct DecodedAudio {
    /// FSB name table 中的 sample 名；无 name table 或非 FSB 时为 `None`。
    pub sample_name: Option<String>,
    /// 不带点的文件扩展名。
    pub extension: &'static str,
    /// 标准音频容器字节，或旧版 AudioClip 已经是标准格式的原始字节。
    pub data: Vec<u8>,
}

/// AudioClip 拆分失败。
#[derive(Debug)]
pub enum AudioExportError {
    NotAudioClip,
    MissingData,
    UnsupportedFormat {
        magic: [u8; 4],
    },
    Fsb(fsb_rs::FsbError),
    Rebuild {
        sample: usize,
        source: fsb_rs::RebuildError,
    },
}

impl fmt::Display for AudioExportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotAudioClip => f.write_str("对象不是 AudioClip"),
            Self::MissingData => f.write_str("AudioClip 音频数据缺失"),
            Self::UnsupportedFormat { magic } => write!(
                f,
                "无法识别音频格式（前四字节 {:02X}{:02X}{:02X}{:02X}）",
                magic[0], magic[1], magic[2], magic[3]
            ),
            Self::Fsb(e) => write!(f, "FSB5 解析失败: {e}"),
            Self::Rebuild { sample, source } => {
                write!(f, "FSB5 sample {sample} 重建失败: {source}")
            }
        }
    }
}

impl Error for AudioExportError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Fsb(e) => Some(e),
            Self::Rebuild { source, .. } => Some(source),
            _ => None,
        }
    }
}

/// 拆分一个 AudioClip。
///
/// 对齐 AssetRipper `AudioClipDecoder`：pre-5.0 的 `m_Type` 表示原始文件类型，直接
/// 保留；post-5.0 的 FSB5 交给 Fmod5Sharp port 重建；tracker 格式按 magic 保留。
/// 与 AssetRipper 只接受单 sample 不同，这里会返回 FSB 中的全部 sample。
pub fn decode_audio(
    env: &Environment,
    obj: &EnvironmentObject,
) -> Result<Vec<DecodedAudio>, AudioExportError> {
    let ParsedObject::Typed(UnityObject::AudioClip(clip)) = &obj.object else {
        return Err(AudioExportError::NotAudioClip);
    };
    let raw = env.audio_data(obj).ok_or(AudioExportError::MissingData)?;
    if raw.is_empty() {
        return Err(AudioExportError::MissingData);
    }

    // pre-5.0：AssetRipper 优先相信序列化的 FMOD_SOUND_TYPE，不尝试拆 FSB。
    if let Some(sound_type) = clip.m_type {
        return Ok(vec![DecodedAudio {
            sample_name: None,
            extension: sound_type_extension(sound_type),
            data: raw.to_vec(),
        }]);
    }

    decode_modern(&raw)
}

fn decode_modern(raw: &Bytes) -> Result<Vec<DecodedAudio>, AudioExportError> {
    if raw.starts_with(b"FSB5") {
        let bank = fsb_rs::parse(raw).map_err(AudioExportError::Fsb)?;
        let mut decoded = Vec::with_capacity(bank.samples.len());
        for (index, sample) in bank.samples.iter().enumerate() {
            let rebuilt =
                bank.rebuild_sample(index)
                    .map_err(|source| AudioExportError::Rebuild {
                        sample: index,
                        source,
                    })?;
            let extension = match &rebuilt {
                fsb_rs::RebuiltAudio::Wav(_) => "wav",
                fsb_rs::RebuiltAudio::Ogg(_) => "ogg",
            };
            decoded.push(DecodedAudio {
                sample_name: sample.name.clone(),
                extension,
                data: rebuilt.into_bytes(),
            });
        }
        return Ok(decoded);
    }

    let extension = if raw.starts_with(b"IMPM") {
        Some("it")
    } else if raw.starts_with(b"Extended Module: ") {
        Some("xm")
    } else if magic_at(raw, 156, b"SCRM") {
        Some("s3m")
    } else if [
        b"M.K.", b"M!K!", b"FLT4", b"FLT8", b"4CHN", b"6CHN", b"8CHN",
    ]
    .iter()
    .any(|magic| magic_at(raw, 1080, *magic))
    {
        Some("mod")
    } else {
        None
    };
    if let Some(extension) = extension {
        return Ok(vec![DecodedAudio {
            sample_name: None,
            extension,
            data: raw.to_vec(),
        }]);
    }

    let mut magic = [0; 4];
    let count = raw.len().min(4);
    magic[..count].copy_from_slice(&raw[..count]);
    Err(AudioExportError::UnsupportedFormat { magic })
}

fn magic_at(data: &[u8], offset: usize, magic: &[u8]) -> bool {
    data.get(offset..offset.saturating_add(magic.len())) == Some(magic)
}

// Unity pre-5.0 AudioType / AssetRipper FmodSoundType 数值。
fn sound_type_extension(sound_type: i32) -> &'static str {
    match sound_type {
        1 => "m4a",
        2 => "aif",
        10 => "it",
        12 => "mod",
        13 => "mp3",
        14 => "ogg",
        17 => "s3m",
        20 | 22 => "wav",
        21 => "xm",
        23 => "vag",
        24 => "fsb",
        _ => "audioClip",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tracker_magic_matches_assetripper_offsets() {
        let it = decode_modern(&Bytes::from_static(b"IMPMrest")).unwrap();
        assert_eq!(it[0].extension, "it");

        let mut s3m = vec![0; 160];
        s3m[156..].copy_from_slice(b"SCRM");
        assert_eq!(
            decode_modern(&Bytes::from(s3m)).unwrap()[0].extension,
            "s3m"
        );

        let mut module = vec![0; 1084];
        module[1080..].copy_from_slice(b"M.K.");
        assert_eq!(
            decode_modern(&Bytes::from(module)).unwrap()[0].extension,
            "mod"
        );
    }

    #[test]
    fn old_sound_type_extensions_match_assetripper() {
        assert_eq!(sound_type_extension(13), "mp3");
        assert_eq!(sound_type_extension(14), "ogg");
        assert_eq!(sound_type_extension(999), "audioClip");
    }
}
