//! FADPCM rebuilder — straight port of `FmodFadPcmRebuilder.cs`.
//!
//! FMOD FADPCM is a fixed-coefficient ADPCM variant. Each 0x8C-byte frame holds
//! a header (coefs/shifts lookup words + two history samples) followed by 8
//! groups of nibbles. Decoding is per the vgmstream reference credited in the
//! C# source.
//!
//! There is no FADPCM fixture in the upstream test set, so this module is
//! covered only by a unit test that checks its coefficient table and decode
//! shape against the C# values.

use super::error::RebuildError;
use super::wav::write_pcm16_wav;
use crate::sample::Sample;

const FRAME_SIZE: usize = 0x8C;
/// `(FrameSize - 0x0C) * 2` samples per frame.
const SAMPLES_PER_FRAME: usize = (FRAME_SIZE - 0x0C) * 2;

/// Fixed 8×2 FADPCM coefficient table (`FadpcmCoefs` in the C# source).
const FADPCM_COEFS: [[i16; 2]; 8] = [
    [0, 0],
    [60, 0],
    [122, 60],
    [115, 52],
    [98, 55],
    [0, 0],
    [0, 0],
    [0, 0],
];

fn clamp_to_short(value: i32) -> i16 {
    value.clamp(i16::MIN as i32, i16::MAX as i32) as i16
}

/// Decode a FADPCM sample to interleaved 16-bit PCM.
///
/// Direct port of `DecodeFadpcm`. The C# allocates `total_frames *
/// SamplesPerFrame` and guards each write with `outIndex < outputSpan.Length`;
/// we do the same with a bounds check.
fn decode(sample: &Sample<'_>) -> Result<Vec<i16>, RebuildError> {
    let bytes = sample.sample_bytes;
    let num_channels = sample.metadata.channels as usize;
    if num_channels == 0 {
        return Err(RebuildError::InvalidChannelCount(sample.metadata.channels));
    }
    let total_frames = bytes.len() / FRAME_SIZE;
    let mut output = vec![0i16; total_frames * SAMPLES_PER_FRAME];

    let mut hist1 = vec![0i32; num_channels];
    let mut hist2 = vec![0i32; num_channels];

    for f in 0..total_frames {
        let channel = f % num_channels;
        let frame_offset = f * FRAME_SIZE;
        let frame = &bytes[frame_offset..frame_offset + FRAME_SIZE];

        // Parse header.
        let coefs_lookup = u32::from_le_bytes(frame[0..4].try_into().unwrap());
        let shifts_lookup = u32::from_le_bytes(frame[4..8].try_into().unwrap());
        hist1[channel] = i16::from_le_bytes(frame[8..10].try_into().unwrap()) as i32;
        hist2[channel] = i16::from_le_bytes(frame[10..12].try_into().unwrap()) as i32;

        let frame_index_in_channel = f / num_channels;
        let frame_base_out_index =
            frame_index_in_channel * SAMPLES_PER_FRAME * num_channels + channel;

        // 8 groups of nibbles, each 0x10 bytes (4 × u32 × 8 nibbles).
        for i in 0..8 {
            // Each group has its own coefs/shifts (indexes > 7 wrap via %7).
            let index = ((coefs_lookup >> (i * 4)) & 0x0F) as usize % 0x07;
            let shift = ((shifts_lookup >> (i * 4)) & 0x0F) as i32;
            let coef1 = FADPCM_COEFS[index][0] as i32;
            let coef2 = FADPCM_COEFS[index][1] as i32;
            let final_shift = 22 - shift; // pre-adjust for 32-bit sign extend

            for j in 0..4 {
                let base = 0x0C + 0x10 * i + 0x04 * j;
                let nibbles = u32::from_le_bytes(frame[base..base + 4].try_into().unwrap());
                for k in 0..8 {
                    let mut sample_value = ((nibbles >> (k * 4)) & 0x0F) as i32;
                    sample_value = (sample_value << 28) >> final_shift; // 32-bit sign extend + scale
                    sample_value =
                        (sample_value - hist2[channel] * coef2 + hist1[channel] * coef1) >> 6;
                    let final_sample = clamp_to_short(sample_value);

                    let out_index = frame_base_out_index + (i * 32 + j * 8 + k) * num_channels;
                    if out_index < output.len() {
                        output[out_index] = final_sample;
                    }

                    hist2[channel] = hist1[channel];
                    hist1[channel] = final_sample as i32;
                }
            }
        }
    }

    Ok(output)
}

/// Rebuild a FADPCM sample into a 16-bit PCM WAV.
///
/// Mirrors `FmodFadPcmRebuilder.Rebuild`.
pub(crate) fn rebuild(sample: &Sample<'_>) -> Result<Vec<u8>, RebuildError> {
    let num_channels = sample.metadata.channels as u16;
    let pcm = decode(sample)?;
    write_pcm16_wav(&pcm, num_channels, sample.metadata.frequency())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coefficient_table_matches_csharp() {
        // FadpcmCoefs from FmodFadPcmRebuilder.cs.
        assert_eq!(FADPCM_COEFS[0], [0, 0]);
        assert_eq!(FADPCM_COEFS[1], [60, 0]);
        assert_eq!(FADPCM_COEFS[2], [122, 60]);
        assert_eq!(FADPCM_COEFS[3], [115, 52]);
        assert_eq!(FADPCM_COEFS[4], [98, 55]);
        assert_eq!(FADPCM_COEFS[5], [0, 0]);
        assert_eq!(FADPCM_COEFS[6], [0, 0]);
        assert_eq!(FADPCM_COEFS[7], [0, 0]);
    }

    #[test]
    fn samples_per_frame_matches_csharp() {
        assert_eq!(SAMPLES_PER_FRAME, (0x8C - 0x0C) * 2);
        assert_eq!(SAMPLES_PER_FRAME, 0x80 * 2);
    }
}
