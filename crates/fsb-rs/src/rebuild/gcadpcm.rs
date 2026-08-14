//! GCADPCM rebuilder — straight port of `FmodGcadPcmRebuilder.cs`.
//!
//! GCADPCM (Nintendo DSP ADPCM) stores samples in 8-byte frames: a 1-byte
//! scale/predictor header followed by 14 signed 4-bit nibbles. The predictor
//! selects two coefficients from a per-sample coefficient table carried in the
//! sample's `DSPCOEFF` chunk (16 big-endian `i16`s per channel). Decoding uses
//! a two-tap adaptive predictor with history.

use super::error::RebuildError;
use super::wav::write_pcm16_wav;
use crate::chunk::ChunkType;
use crate::chunk_data::ChunkData;
use crate::sample::Sample;

const SAMPLES_PER_FRAME: usize = 14;
const NIBBLES_PER_FRAME: usize = 16;

/// Signed-nibble lookup, matching `Utils.GetHigh/LowNibbleSigned`:
/// `{0,1,2,3,4,5,6,7,-8,-7,-6,-5,-4,-3,-2,-1}`.
const SIGNED_NIBBLES: [i8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, -8, -7, -6, -5, -4, -3, -2, -1];

fn high_nibble_signed(value: u8) -> i32 {
    SIGNED_NIBBLES[((value >> 4) & 0xF) as usize] as i32
}

fn low_nibble_signed(value: u8) -> i32 {
    SIGNED_NIBBLES[(value & 0xF) as usize] as i32
}

fn clamp16(value: i32) -> i16 {
    value.clamp(i16::MIN as i32, i16::MAX as i32) as i16
}

/// `NibbleCountToSampleCount` / `ByteCountToSampleCount` from the C# source.
fn byte_count_to_sample_count(byte_count: usize) -> usize {
    let nibble_count = byte_count * 2;
    let frames = nibble_count / NIBBLES_PER_FRAME;
    let extra_nibbles = nibble_count % NIBBLES_PER_FRAME;
    let extra_samples = extra_nibbles.saturating_sub(2);
    SAMPLES_PER_FRAME * frames + extra_samples
}

/// Extract the first channel's 16 coefficients from the sample's `DSPCOEFF` chunk.
fn first_channel_coeffs(sample: &Sample<'_>) -> Result<[i16; 16], RebuildError> {
    let chunk = sample
        .metadata
        .chunks
        .iter()
        .find(|c| c.chunk_type == ChunkType::DspCoeff)
        .ok_or(RebuildError::MissingDspCoeff)?;
    let coeffs = match &chunk.data {
        ChunkData::DspCoefficients { channel_data } => channel_data,
        _ => return Err(RebuildError::MissingDspCoeff),
    };
    let first = coeffs.first().ok_or(RebuildError::MissingDspCoeff)?;
    if first.len() < 16 {
        return Err(RebuildError::MissingDspCoeff);
    }
    let mut out = [0i16; 16];
    out.copy_from_slice(&first[..16]);
    Ok(out)
}

fn decode(sample: &Sample<'_>) -> Result<Vec<i16>, RebuildError> {
    let coeffs = first_channel_coeffs(sample)?;
    let adpcm = sample.sample_bytes;
    let sample_count = byte_count_to_sample_count(adpcm.len());
    let frame_count = sample_count.div_ceil(SAMPLES_PER_FRAME);

    let mut pcm = vec![0i16; sample_count];
    let mut current_sample = 0usize;
    let mut out_index = 0usize;
    let mut in_index = 0usize;
    let mut hist1: i16 = 0;
    let mut hist2: i16 = 0;

    for _ in 0..frame_count {
        if in_index >= adpcm.len() {
            return Err(RebuildError::SampleDataTooShort);
        }
        // Each byte is a scale and a predictor.
        let combined = adpcm[in_index];
        in_index += 1;
        let scale: i32 = 1 << (combined & 0xF);
        let predictor = (combined >> 4) as usize;
        // predictor selects coeff1 = coeffs[predictor*2], coeff2 = coeffs[predictor*2+1].
        // With only 16 coefficients (valid predictor <= 7), reject anything larger.
        let Some((pi1, pi2)) = predictor.checked_mul(2).map(|p| (p, p + 1)) else {
            return Err(RebuildError::SampleDataTooShort);
        };
        if pi2 >= coeffs.len() {
            return Err(RebuildError::SampleDataTooShort);
        }
        let coeff1 = coeffs[pi1] as i32;
        let coeff2 = coeffs[pi2] as i32;

        let samples_to_read = SAMPLES_PER_FRAME.min(sample_count - current_sample);
        for s in 0..samples_to_read {
            if in_index >= adpcm.len() {
                return Err(RebuildError::SampleDataTooShort);
            }
            // C#: even s -> high nibble (and doesn't advance index); odd s -> low nibble.
            let adpcm_sample = if s % 2 == 0 {
                high_nibble_signed(adpcm[in_index])
            } else {
                let v = low_nibble_signed(adpcm[in_index]);
                in_index += 1;
                v
            };

            // Adaptive processing.
            let adpcm_sample = (adpcm_sample * scale) << 11;
            // C# integer arithmetic is unchecked here. Use explicit wrapping
            // operations so malformed coefficients cannot panic in debug builds
            // and release/debug produce the same upstream-compatible result.
            let adpcm_sample = adpcm_sample
                .wrapping_add(1024)
                .wrapping_add(coeff1.wrapping_mul(hist1 as i32))
                .wrapping_add(coeff2.wrapping_mul(hist2 as i32))
                >> 11;
            let clamped = clamp16(adpcm_sample);

            hist2 = hist1;
            hist1 = clamped;

            pcm[out_index] = clamped;
            out_index += 1;
            current_sample += 1;
        }
    }

    Ok(pcm)
}

/// Rebuild a GCADPCM sample into a 16-bit PCM WAV.
///
/// Mirrors `FmodGcadPcmRebuilder.Rebuild`: decode to `i16`, then write a WAV
/// using `channels` (= 2 if stereo else 1) and the sample's frequency.
pub(crate) fn rebuild(sample: &Sample<'_>) -> Result<Vec<u8>, RebuildError> {
    let num_channels = if sample.metadata.is_stereo { 2 } else { 1 };
    let pcm = decode(sample)?;
    write_pcm16_wav(&pcm, num_channels, sample.metadata.frequency())
}
