//! Per-sample metadata (`FmodSampleMetadata` in the C# source).
//!
//! The metadata is bit-packed into a single 64-bit word. Bit layout (LSB-first),
//! verifiable against `FMOD::CodecFSB5::decodeSubSoundHeader` in fmod.dll:
//!
//! | bits   | field                       |
//! |--------|------------------------------|
//! | 0      | `has_any_chunks`            |
//! | 1–4    | `frequency_id` (4 bits)     |
//! | 5–6    | channel code (2 bits)       |
//! | 7–33   | `data_offset_raw` (27 bits) |
//! | 34–63  | `sample_count` (30 bits)    |
//!
//! `data_offset = data_offset_raw * 32`. The channel code maps
//! `0→1, 1→2, 2→6, 3→8` channels.

use crate::chunk::{Chunk, ChunkType};
use crate::chunk_data::ChunkData;
use crate::error::FsbError;
use crate::reader::Reader;

/// Standard FMOD frequency table keyed by id (mirrors `FsbLoader.Frequencies`).
///
/// When a [`ChunkData::Frequency`](crate::chunk_data::ChunkData::Frequency) chunk
/// overrides the id with a value not in this table, the id itself is the actual
/// frequency in Hz.
pub const FREQUENCIES: [(u32, u32); 11] = [
    (0, 4_000),
    (1, 8_000),
    (2, 11_000),
    (3, 12_000),
    (4, 16_000),
    (5, 22_050),
    (6, 24_000),
    (7, 32_000),
    (8, 44_100),
    (9, 48_000),
    (10, 96_000),
];

/// Look up an actual frequency (Hz) for a frequency id.
pub fn frequency_for(id: u32) -> u32 {
    FREQUENCIES
        .iter()
        .find(|(k, _)| *k == id)
        .map(|(_, v)| *v)
        .unwrap_or(id)
}

/// Metadata for a single sample.
#[derive(Debug)]
pub struct SampleMetadata {
    /// Number of audio samples (frames) in this sample.
    pub sample_count: u32,
    /// Channel count (1, 2, 6, or 8).
    pub channels: u32,
    /// Convenience: whether the sample is stereo (channels == 2).
    pub is_stereo: bool,
    /// Byte offset of the sample's data, relative to the bank's data region.
    pub data_offset: u32,
    /// Whether the sample declares any chunks in its header.
    pub has_any_chunks: bool,
    /// Raw frequency id (may be overridden by a `FREQUENCY` chunk).
    pub frequency_id: u32,
    /// Parsed chunks for this sample.
    pub chunks: Vec<Chunk>,
}

impl SampleMetadata {
    /// The effective sample rate in Hz, accounting for a `FREQUENCY` chunk override.
    ///
    /// A `FREQUENCY` chunk replaces `frequency_id`; the resulting frequency is then
    /// looked up in the standard table, falling back to the id itself as Hz.
    pub fn frequency(&self) -> u32 {
        frequency_for(self.frequency_id)
    }

    /// Move this metadata out of a slot, leaving a zeroed placeholder behind.
    ///
    /// Used by the loader to transfer each sample's metadata into the [`Sample`]
    /// while iterating over the header's vec by index (which requires reading the
    /// next sample's offset before this one is consumed).
    pub(crate) fn take_meta(&mut self) -> SampleMetadata {
        SampleMetadata {
            sample_count: std::mem::replace(&mut self.sample_count, 0),
            channels: std::mem::replace(&mut self.channels, 0),
            is_stereo: std::mem::replace(&mut self.is_stereo, false),
            data_offset: std::mem::replace(&mut self.data_offset, 0),
            has_any_chunks: std::mem::replace(&mut self.has_any_chunks, false),
            frequency_id: std::mem::replace(&mut self.frequency_id, 0),
            chunks: std::mem::take(&mut self.chunks),
        }
    }

    /// Read the 8-byte bit-packed metadata word (without chunks).
    pub(crate) fn read_raw(reader: &mut Reader<'_>) -> Result<Self, FsbError> {
        let encoded = reader.read_u64_le()?;

        let has_any_chunks = (encoded & 1) == 1;
        let frequency_id = bits(encoded, 1, 4) as u32;
        let channel_code = bits(encoded, 5, 2) as u32;
        let channels = match channel_code {
            0 => 1,
            1 => 2,
            2 => 6,
            3 => 8,
            _ => 0, // unreachable: 2 bits can't exceed 3
        };
        let data_offset = (bits(encoded, 7, 27) as u32) * 32;
        let sample_count = bits(encoded, 34, 30) as u32;

        Ok(Self {
            sample_count,
            channels,
            is_stereo: channels == 2,
            data_offset,
            has_any_chunks,
            frequency_id,
            chunks: Vec::new(),
        })
    }
}

/// Bitfield extraction matching C# `Extensions.Bits(ulong, lowestBit, numBits)`:
/// build a `numBits`-wide mask, shift it to `lowestBit`, mask, then shift back down.
fn bits(raw: u64, lowest_bit: u32, num_bits: u32) -> u64 {
    let mask = if num_bits >= 64 {
        u64::MAX
    } else {
        (1u64 << num_bits) - 1
    };
    (raw & (mask << lowest_bit)) >> lowest_bit
}

/// Read the metadata word and, if it declares chunks, the chunk stream.
///
/// Mirrors the loop in `FmodAudioHeader`'s constructor: read chunk header words
/// until `more_chunks` is false. If a `FREQUENCY` chunk is present, it overrides
/// `frequency_id`.
///
/// `channels` is passed explicitly into the chunk reader — the C# source uses a
/// static `FmodSampleChunk.CurrentSample` field for this, which is neither thread-
/// safe nor idiomatic; here it's a plain parameter.
pub(crate) fn read_metadata_with_chunks(
    reader: &mut Reader<'_>,
) -> Result<SampleMetadata, FsbError> {
    let mut meta = SampleMetadata::read_raw(reader)?;

    if meta.has_any_chunks {
        loop {
            let chunk = read_chunk(reader, meta.channels)?;
            let more = chunk.more;
            meta.chunks.push(chunk);
            if !more {
                break;
            }
        }

        // A FREQUENCY chunk overrides the frequency id from the metadata word.
        if let Some(chunk) = meta
            .chunks
            .iter()
            .find(|c| c.chunk_type == ChunkType::Frequency)
            && let ChunkData::Frequency { frequency_id } = &chunk.data
        {
            meta.frequency_id = *frequency_id;
        }
    }

    Ok(meta)
}

fn read_chunk(reader: &mut Reader<'_>, channels: u32) -> Result<Chunk, FsbError> {
    let raw = reader.read_u32_le()?;
    let more = (raw & 1) == 1;
    let chunk_size = bits(raw as u64, 1, 24) as u32;
    let chunk_type_raw = bits(raw as u64, 25, 7) as u8;
    let chunk_type = ChunkType::from_u8(chunk_type_raw);

    let (data, read) = ChunkData::read(reader, &chunk_type, channels, chunk_size)?;

    // C# throws if a chunk doesn't consume exactly its declared size.
    if read != chunk_size {
        return Err(FsbError::ChunkSizeMismatch {
            expected: chunk_size,
            read,
        });
    }

    Ok(Chunk {
        chunk_type,
        more,
        data,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bits_matches_csharp_semantics() {
        // C# Bits(low, num) builds a numBits-wide mask, shifts to low, masks, shifts down.
        // 0b1011_0010_0000_0000: bit9 is set; bits 7-10 (LSB-first) => 2^(9-7) = 4.
        assert_eq!(bits(0b1011_0010_0000_0000u64, 7, 4), 4);
        assert_eq!(bits(u64::MAX, 34, 30), (1u64 << 30) - 1);
        assert_eq!(bits(0, 0, 1), 0);
        assert_eq!(bits(1, 0, 1), 1);
    }

    #[test]
    fn frequency_table_and_fallback() {
        assert_eq!(frequency_for(8), 44_100);
        assert_eq!(frequency_for(9), 48_000);
        // Ids outside the table are the literal Hz value.
        assert_eq!(frequency_for(22_050), 22_050);
    }
}
