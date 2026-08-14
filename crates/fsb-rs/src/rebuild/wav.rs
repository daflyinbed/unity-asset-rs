//! Minimal PCM WAV writer, ported from NAudio's `WaveFileWriter` +
//! `WaveFormat.Serialize`.
//!
//! NAudio writes a `fmt ` chunk for a PCM `WaveFormat` built via
//! `CreateCustomFormat(Pcm, ...)` as **18 bytes** — i.e. the 16-byte
//! `PCMWAVEFORMAT` fields followed by a 2-byte `cbSize = 0`. This module mirrors
//! that exact layout so output bodies line up byte-for-byte with Fmod5Sharp,
//! while also filling in the RIFF/data size fields correctly (the original C#
//! rebuilder leaves them zero in several code paths because of
//! `MemoryStream.GetBuffer()` / `ToArray()` dispose-ordering quirks).
//!
//! We only emit PCM, so there is no `fact` chunk (NAudio's `HasFactChunk()`
//! returns false for `WaveFormatEncoding.Pcm`).

use super::error::RebuildError;

/// PCM `WAVEFORMATEX` format tag.
const WAVE_FORMAT_PCM: u16 = 1;

/// Header size, in bytes, for the 18-byte `fmt ` chunk variant NAudio emits for
/// PCM `CreateCustomFormat` formats: `RIFF`(12) + `fmt `(8 + 18) = 38. The data
/// chunk body therefore always starts at offset 38.
pub(crate) const DATA_OFFSET: usize = 38;

/// Write a well-formed PCM WAV whose audio body is `data` (already in the target
/// bit depth / channel layout).
///
/// `bits_per_sample` is 8/16/32; `block_align = channels * (bits/8)` and
/// `avg_bytes_per_sec = sample_rate * block_align`, matching
/// `WaveFormat.CreateCustomFormat` / the `(rate, bits, channels)` constructor.
pub(crate) fn write_pcm_wav(
    data: &[u8],
    channels: u16,
    sample_rate: u32,
    bits_per_sample: u16,
) -> Result<Vec<u8>, RebuildError> {
    let bytes_per_sample = (bits_per_sample / 8) as u32;
    let block_align = (channels as u32)
        .checked_mul(bytes_per_sample)
        .ok_or(RebuildError::OutputTooLarge)?;
    let block_align_u16 = u16::try_from(block_align).map_err(|_| RebuildError::OutputTooLarge)?;
    let avg_bytes_per_sec = sample_rate
        .checked_mul(block_align)
        .ok_or(RebuildError::OutputTooLarge)?;

    let capacity = DATA_OFFSET
        .checked_add(data.len())
        .and_then(|n| n.checked_add(9))
        .ok_or(RebuildError::OutputTooLarge)?;
    let mut out = Vec::new();
    out.try_reserve_exact(capacity)
        .map_err(|_| RebuildError::OutputTooLarge)?;
    // --- RIFF header (size filled in at the end) ---
    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&0u32.to_le_bytes()); // placeholder
    out.extend_from_slice(b"WAVE");

    // --- fmt chunk: 18-byte PCM-with-cbSize, matching NAudio ---
    out.extend_from_slice(b"fmt ");
    out.extend_from_slice(&18u32.to_le_bytes()); // chunk size
    out.extend_from_slice(&WAVE_FORMAT_PCM.to_le_bytes());
    out.extend_from_slice(&channels.to_le_bytes());
    out.extend_from_slice(&sample_rate.to_le_bytes());
    out.extend_from_slice(&avg_bytes_per_sec.to_le_bytes());
    out.extend_from_slice(&block_align_u16.to_le_bytes());
    out.extend_from_slice(&bits_per_sample.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes()); // cbSize = 0

    debug_assert_eq!(out.len(), DATA_OFFSET, "data offset must be 38");

    // --- data chunk (size filled in at the end) ---
    let data_size_pos = out.len() + 4;
    out.extend_from_slice(b"data");
    out.extend_from_slice(&0u32.to_le_bytes()); // placeholder
    out.extend_from_slice(data);

    // Word-align the data chunk (NAudio pads odd-length data with one zero byte).
    if data.len() & 1 == 1 {
        out.push(0);
    }

    // --- Backfill size fields ---
    let riff_size = u32::try_from(out.len() - 8).map_err(|_| RebuildError::OutputTooLarge)?;
    out[4..8].copy_from_slice(&riff_size.to_le_bytes());
    let data_size = u32::try_from(data.len()).map_err(|_| RebuildError::OutputTooLarge)?;
    out[data_size_pos..data_size_pos + 4].copy_from_slice(&data_size.to_le_bytes());

    Ok(out)
}

/// Write 16-bit PCM samples as a WAV. Convenience wrapper around
/// [`write_pcm_wav`] that packs `samples` little-endian.
pub(crate) fn write_pcm16_wav(
    samples: &[i16],
    channels: u16,
    sample_rate: u32,
) -> Result<Vec<u8>, RebuildError> {
    let data_len = samples
        .len()
        .checked_mul(2)
        .ok_or(RebuildError::OutputTooLarge)?;
    let mut data = Vec::new();
    data.try_reserve_exact(data_len)
        .map_err(|_| RebuildError::OutputTooLarge)?;
    for &s in samples {
        data.extend_from_slice(&s.to_le_bytes());
    }
    write_pcm_wav(&data, channels, sample_rate, 16)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_layout_matches_naudio() {
        // One 16-bit mono sample: body is 2 bytes, total 48 bytes.
        let wav = write_pcm16_wav(&[0x1234], 1, 44_100).unwrap();
        assert_eq!(&wav[..4], b"RIFF");
        assert_eq!(&wav[8..12], b"WAVE");
        assert_eq!(&wav[12..16], b"fmt ");
        assert_eq!(u32::from_le_bytes(wav[16..20].try_into().unwrap()), 18); // fmt size
        assert_eq!(u16::from_le_bytes(wav[20..22].try_into().unwrap()), 1); // PCM tag
        assert_eq!(u16::from_le_bytes(wav[22..24].try_into().unwrap()), 1); // channels
        assert_eq!(u32::from_le_bytes(wav[24..28].try_into().unwrap()), 44_100); // rate
        assert_eq!(u16::from_le_bytes(wav[32..34].try_into().unwrap()), 2); // blockAlign
        assert_eq!(u16::from_le_bytes(wav[34..36].try_into().unwrap()), 16); // bits
        assert_eq!(u16::from_le_bytes(wav[36..38].try_into().unwrap()), 0); // cbSize
        assert_eq!(&wav[38..42], b"data");
        // Sizes are filled in correctly (unlike NAudio's dispose-ordering quirks).
        assert_eq!(u32::from_le_bytes(wav[4..8].try_into().unwrap()), 40); // riff = 48-8
        assert_eq!(u32::from_le_bytes(wav[42..46].try_into().unwrap()), 2); // data size
        assert_eq!(&wav[46..48], &0x1234_i16.to_le_bytes());
    }

    #[test]
    fn odd_data_gets_word_aligned() {
        let wav = write_pcm_wav(&[0xab], 1, 8_000, 8).unwrap();
        // 1 byte of data -> padded to 2 -> data size field still 1.
        assert_eq!(u32::from_le_bytes(wav[42..46].try_into().unwrap()), 1);
        // header(46) + 1 body byte + 1 pad byte.
        assert_eq!(wav.len(), 46 + 1 + 1);
        assert_eq!(wav[47], 0); // pad byte
    }

    #[test]
    fn rejects_overflowing_average_bytes_per_second() {
        let err = write_pcm_wav(&[], 1, u32::MAX, 16).unwrap_err();
        assert!(matches!(err, RebuildError::OutputTooLarge));
    }
}
