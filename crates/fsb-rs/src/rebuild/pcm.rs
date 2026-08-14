//! PCM rebuilder — straight port of `FmodPcmRebuilder.cs`.
//!
//! PCM is uncompressed: the sample bytes are already the audio body. The only
//! work is packaging them into a WAV at the right bit depth. PCM8/16/32 are
//! supported; anything else is `UnsupportedCodec`.

use super::error::RebuildError;
use super::wav::write_pcm_wav;
use crate::audio_type::AudioType;
use crate::sample::Sample;

/// Rebuild a PCM sample into a WAV byte vector.
///
/// Mirrors `FmodPcmRebuilder.Rebuild`: pick the byte width from the codec, then
/// write `channels * width * sample_count` bytes of the raw sample data (the C#
/// code uses `writer.Write(sample.SampleBytes, 0, numChannels * width * sampleCount)`).
pub(crate) fn rebuild(sample: &Sample<'_>, audio_type: AudioType) -> Result<Vec<u8>, RebuildError> {
    let width: usize = match audio_type {
        AudioType::Pcm8 => 1,
        AudioType::Pcm16 => 2,
        AudioType::Pcm32 => 4,
        _ => return Err(RebuildError::UnsupportedCodec(audio_type)),
    };

    // Match FmodPcmRebuilder exactly: only the packed stereo flag selects two
    // channels; every other channel code is rebuilt as mono.
    let channels = if sample.metadata.is_stereo {
        2usize
    } else {
        1usize
    };
    let sample_count = sample.metadata.sample_count as usize;
    let body_len = channels
        .checked_mul(width)
        .and_then(|p| p.checked_mul(sample_count))
        .ok_or(RebuildError::SampleDataTooShort)?;
    if sample.sample_bytes.len() < body_len {
        return Err(RebuildError::SampleDataTooShort);
    }
    let body = &sample.sample_bytes[..body_len];

    let bits = (width * 8) as u16;
    write_pcm_wav(body, channels as u16, sample.metadata.frequency(), bits)
}
