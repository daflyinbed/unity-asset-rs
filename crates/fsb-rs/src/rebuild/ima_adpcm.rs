//! IMA ADPCM rebuilder — straight port of `FmodImaAdPcmRebuilder.cs`.
//!
//! FMOD ships two IMA-ADPCM layouts:
//! - **Xbox IMA** (mono): each 0x24-byte frame is a 4-byte header (predictor,
//!   step-index) followed by 64 nibbles.
//! - **FSB IMA** (stereo): frames are laid out per-channel within a 0x24*ch
//!   frame, with a more elaborate nibble-stride formula.
//!
//! The dispatcher (`Rebuild`) selects the mono path when `!is_stereo`. The
//! stereo path's C# source contains a dead `numChannels == 0` branch; it is not
//! ported here.

use super::error::RebuildError;
use super::wav::write_pcm16_wav;
use crate::sample::Sample;

/// 64 samples per frame, per channel (`SamplesPerFramePerChannel`).
const SAMPLES_PER_FRAME: usize = 0x40;

/// Standard 89-entry IMA ADPCM step table (`ADPCMTable`).
const STEP_TABLE: [i32; 89] = [
    7, 8, 9, 10, 11, 12, 13, 14, 16, 17, 19, 21, 23, 25, 28, 31, 34, 37, 41, 45, 50, 55, 60, 66,
    73, 80, 88, 97, 107, 118, 130, 143, 157, 173, 190, 209, 230, 253, 279, 307, 337, 371, 408, 449,
    494, 544, 598, 658, 724, 796, 876, 963, 1060, 1166, 1282, 1411, 1552, 1707, 1878, 2066, 2272,
    2499, 2749, 3024, 3327, 3660, 4026, 4428, 4871, 5358, 5894, 6484, 7132, 7845, 8630, 9493,
    10442, 11487, 12635, 13899, 15289, 16818, 18500, 20350, 22385, 24623, 27086, 29794, 32767,
];

/// Standard 16-entry IMA index adaptation table (`IMA_IndexTable`).
const INDEX_TABLE: [i32; 16] = [-1, -1, -1, -1, 2, 4, 6, 8, -1, -1, -1, -1, 2, 4, 6, 8];

fn clamp_to_short(value: i32) -> i16 {
    value.clamp(i16::MIN as i32, i16::MAX as i32) as i16
}

/// Decode a single nibble, advancing the channel's predictor/step state.
///
/// Direct translation of `ExpandNibble`: the magnitude bits (0–2) add scaled
/// deltas, bit 3 is the sign; the predictor is saturated (not wrapped), and the
/// step index is clamped to `0..=88`.
fn expand_nibble(data: &[u8], byte_offset: usize, nibble_shift: u32, state: &mut ImaState) {
    let sample_nibble = ((data[byte_offset] >> nibble_shift) & 0xF) as usize;

    let mut sample_decoded = state.hist as i32;
    let step = STEP_TABLE[state.step_index as usize];
    let mut delta = step >> 3;
    if sample_nibble & 1 != 0 {
        delta += step >> 2;
    }
    if sample_nibble & 2 != 0 {
        delta += step >> 1;
    }
    if sample_nibble & 4 != 0 {
        delta += step;
    }
    if sample_nibble & 8 != 0 {
        delta = -delta;
    }
    sample_decoded += delta;
    state.hist = clamp_to_short(sample_decoded);

    state.step_index += INDEX_TABLE[sample_nibble];
    state.step_index = state.step_index.clamp(0, 88);
}

struct ImaState {
    hist: i16,
    step_index: i32,
}

fn zeroed_output(len: usize) -> Result<Vec<i16>, RebuildError> {
    let mut output = Vec::new();
    output
        .try_reserve_exact(len)
        .map_err(|_| RebuildError::OutputTooLarge)?;
    output.resize(len, 0);
    Ok(output)
}

/// Decode a stereo (FSB-layout) IMA sample. Output is interleaved L,R,L,R,...
///
/// Mirrors `DecodeSamplesFsbIma`. `ret` is sized `sample_count * 2`; channel 0
/// writes even indices, channel 1 writes odd indices.
fn decode_fsb_ima(sample: &Sample<'_>) -> Result<Vec<i16>, RebuildError> {
    let num_channels = sample.metadata.channels as usize;
    if num_channels != 2 {
        return Err(RebuildError::InvalidChannelCount(sample.metadata.channels));
    }
    let data = sample.sample_bytes;
    let num_frames = sample.metadata.sample_count as usize / SAMPLES_PER_FRAME;
    let required_data_len = 0x24usize
        .checked_mul(num_channels)
        .and_then(|bytes_per_frame| bytes_per_frame.checked_mul(num_frames))
        .ok_or(RebuildError::OutputTooLarge)?;
    if data.len() < required_data_len {
        return Err(RebuildError::SampleDataTooShort);
    }
    let output_len = (sample.metadata.sample_count as usize)
        .checked_mul(2)
        .ok_or(RebuildError::OutputTooLarge)?;
    let mut ret = zeroed_output(output_len)?;

    for channel in 0..num_channels {
        let mut sample_index = channel;
        for frame_num in 0..num_frames {
            // Offset of this frame in the entire sample data.
            let frame_offset = 0x24usize
                .checked_mul(num_channels)
                .and_then(|p| p.checked_mul(frame_num))
                .ok_or(RebuildError::SampleDataTooShort)?;
            let header_index = frame_offset + 4 * channel;
            if header_index + 3 >= data.len() {
                return Err(RebuildError::SampleDataTooShort);
            }
            let hist = i16::from_le_bytes(data[header_index..header_index + 2].try_into().unwrap());
            let step_index_raw = data[header_index + 2] as i32;
            let step_index = step_index_raw.clamp(0, 88);
            let mut state = ImaState { hist, step_index };

            ret[sample_index] = hist;
            sample_index += num_channels;

            for sample_num in 1..=SAMPLES_PER_FRAME {
                // Offset of this sample's nibble within the frame.
                // C# line 105 (stereo branch — the numChannels==0 branch is dead code).
                let byte_offset = frame_offset
                    + 4 * 2
                    + 4 * (channel % 2)
                    + 4 * 2 * ((sample_num - 1) / 8)
                    + ((sample_num - 1) % 8) / 2;
                // Odd samples use the upper nibble, even samples the lower.
                let nibble_shift: u32 = if (sample_num - 1) & 1 != 0 { 4 } else { 0 };

                if sample_num < SAMPLES_PER_FRAME {
                    if byte_offset >= data.len() {
                        return Err(RebuildError::SampleDataTooShort);
                    }
                    expand_nibble(data, byte_offset, nibble_shift, &mut state);
                    ret[sample_index] = state.hist;
                    sample_index += num_channels;
                }
            }
        }
    }

    Ok(ret)
}

/// Decode a mono (Xbox-layout) IMA sample. Simplified because there is one
/// channel. Mirrors `DecodeSamplesXboxIma`.
fn decode_xbox_ima(sample: &Sample<'_>) -> Result<Vec<i16>, RebuildError> {
    const FRAME_SIZE: usize = 0x24;
    let data = sample.sample_bytes;
    let num_frames = sample.metadata.sample_count as usize / SAMPLES_PER_FRAME;
    let required_data_len = FRAME_SIZE
        .checked_mul(num_frames)
        .ok_or(RebuildError::OutputTooLarge)?;
    if data.len() < required_data_len {
        return Err(RebuildError::SampleDataTooShort);
    }
    let mut ret = zeroed_output(sample.metadata.sample_count as usize)?;
    let mut sample_index = 0usize;

    for frame_num in 0..num_frames {
        let frame_offset = FRAME_SIZE * frame_num;
        if frame_offset + 3 >= data.len() {
            return Err(RebuildError::SampleDataTooShort);
        }
        let hist = i16::from_le_bytes(data[frame_offset..frame_offset + 2].try_into().unwrap());
        let step_index_raw = data[frame_offset + 2] as i32;
        let step_index = step_index_raw.clamp(0, 88);
        let mut state = ImaState { hist, step_index };

        ret[sample_index] = hist;
        sample_index += 1;

        for sample_num in 1..=SAMPLES_PER_FRAME {
            let byte_offset = frame_offset + 4 + (sample_num - 1) / 2;
            let nibble_shift: u32 = if (sample_num - 1) & 1 != 0 { 4 } else { 0 };

            if sample_num < SAMPLES_PER_FRAME {
                if byte_offset >= data.len() {
                    return Err(RebuildError::SampleDataTooShort);
                }
                expand_nibble(data, byte_offset, nibble_shift, &mut state);
                ret[sample_index] = state.hist;
                sample_index += 1;
            }
        }
    }

    Ok(ret)
}

/// Rebuild an IMA-ADPCM sample into a 16-bit PCM WAV.
///
/// Mirrors `FmodImaAdPcmRebuilder.Rebuild`: mono samples use the Xbox path,
/// stereo samples use the FSB path.
pub(crate) fn rebuild(sample: &Sample<'_>) -> Result<Vec<u8>, RebuildError> {
    let num_channels = match (sample.metadata.channels, sample.metadata.is_stereo) {
        (1, false) => 1,
        (2, true) => 2,
        _ => return Err(RebuildError::InvalidChannelCount(sample.metadata.channels)),
    };
    let pcm = match num_channels {
        1 => decode_xbox_ima(sample)?,
        2 => decode_fsb_ima(sample)?,
        _ => unreachable!("validated IMA ADPCM channel count"),
    };
    write_pcm16_wav(&pcm, num_channels, sample.metadata.frequency())
}
