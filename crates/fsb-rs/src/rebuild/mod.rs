//! Sample rebuilding — decoding encoded sample bytes back into a standard file
//! format (WAV). Ported from Fmod5Sharp's `CodecRebuilders/`.
//!
//! This layer is the counterpart to the container parser: [`Sample::rebuild`]
//! takes a parsed sample and the bank's [`AudioType`] and produces a playable
//! file.
//!
//! Mirrors `FmodSample.RebuildAsStandardFileFormat`, which dispatches on the
//! bank's codec:
//!
//! | codec        | rebuilder              | output |
//! |--------------|------------------------|--------|
//! | PCM8/16/32   | `FmodPcmRebuilder`     | WAV    |
//! | GCADPCM      | `FmodGcadPcmRebuilder` | WAV    |
//! | IMAADPCM     | `FmodImaAdPcmRebuilder`| WAV    |
//! | FADPCM       | `FmodFadPcmRebuilder`  | WAV    |
//! | VORBIS       | `vorbis`               | Ogg    |

pub mod error;
mod fadpcm;
mod gcadpcm;
mod ima_adpcm;
mod pcm;
mod vorbis;
mod wav;

use crate::audio_type::AudioType;
use crate::sample::Sample;
use crate::sound_bank::SoundBank;

pub use error::RebuildError;

/// The result of rebuilding a sample into a standard file format.
#[derive(Debug, Clone)]
pub enum RebuiltAudio {
    /// A WAV (RIFF) file. Produced by all currently-supported codecs.
    Wav(Vec<u8>),
    /// An Ogg Vorbis file.
    Ogg(Vec<u8>),
}

impl RebuiltAudio {
    /// The rebuilt file's bytes, regardless of container.
    pub fn into_bytes(self) -> Vec<u8> {
        match self {
            RebuiltAudio::Wav(b) | RebuiltAudio::Ogg(b) => b,
        }
    }
}

impl Sample<'_> {
    /// Rebuild this sample into a standard file format (WAV), using `audio_type`
    /// to select the decoder.
    ///
    /// `audio_type` is taken from the enclosing bank's header (the C# source
    /// reaches it via `FmodSample.MyBank`); pass [`SoundBank::rebuild_sample`]
    /// to avoid supplying it by hand.
    pub fn rebuild(&self, audio_type: AudioType) -> Result<RebuiltAudio, RebuildError> {
        if audio_type == AudioType::Vorbis {
            return Ok(RebuiltAudio::Ogg(vorbis::rebuild(self)?));
        }

        let bytes = match audio_type {
            AudioType::Pcm8 | AudioType::Pcm16 | AudioType::Pcm32 => {
                pcm::rebuild(self, audio_type)?
            }
            AudioType::Gcadpcm => gcadpcm::rebuild(self)?,
            AudioType::ImaAdpcm => ima_adpcm::rebuild(self)?,
            AudioType::Fadpcm => fadpcm::rebuild(self)?,
            other => return Err(RebuildError::UnsupportedCodec(other)),
        };
        Ok(RebuiltAudio::Wav(bytes))
    }
}

impl SoundBank<'_> {
    /// Rebuild sample `i` into a standard file format, using the bank's codec.
    ///
    /// Convenience over [`Sample::rebuild`] that supplies the bank's
    /// [`AudioType`].
    pub fn rebuild_sample(&self, i: usize) -> Result<RebuiltAudio, RebuildError> {
        self.samples
            .get(i)
            .ok_or(RebuildError::InvalidSampleIndex(i))?
            .rebuild(self.header.audio_type)
    }
}
