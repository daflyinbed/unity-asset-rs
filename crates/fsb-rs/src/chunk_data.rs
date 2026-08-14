//! Decoded payloads for sample chunks (`ChunkData/` in the C# source).
//!
//! Each variant corresponds to a `IChunkData` implementation. Parsed by
//! [`ChunkData::read`], which consumes exactly `expected_size` bytes from the
//! reader (the same invariant the C# `FmodSampleChunk.Read` enforces).

use crate::error::FsbError;
use crate::reader::Reader;

/// Type-specific payload of a sample chunk.
#[derive(Debug)]
pub enum ChunkData {
    /// `VORBISDATA` — the CRC32 used to look up the vorbis setup header.
    Vorbis {
        /// CRC32 of the vorbis setup header (key into the known-headers table).
        crc32: u32,
    },
    /// `FREQUENCY` — overrides the sample's frequency.
    Frequency {
        /// Raw frequency id. When no entry exists in the standard frequency
        /// table, this id is treated as the actual frequency in Hz.
        frequency_id: u32,
    },
    /// `CHANNELS` — overrides the sample's channel count.
    Channels { num_channels: u8 },
    /// `LOOP` — loop region in samples.
    Loop { loop_start: u32, loop_end: u32 },
    /// `DSPCOEFF` — GCADPCM predictor coefficients, per channel.
    ///
    /// Each channel has exactly 16 `i16` coefficients, read **big-endian**
    /// (see `DspCoefficientsBlockData` — the FSB5 format stores these big-endian
    /// even though the rest of the container is little-endian).
    DspCoefficients { channel_data: Vec<Vec<i16>> },
    /// Catch-all for chunk types without a dedicated parser.
    Unknown(Vec<u8>),
}

impl ChunkData {
    /// Parse `expected_size` bytes of payload for a chunk of `chunk_type`.
    ///
    /// Returns the decoded payload *and* the number of bytes consumed, so the
    /// caller can enforce the C# invariant that the two must match.
    pub(crate) fn read(
        reader: &mut Reader<'_>,
        chunk_type: &crate::chunk::ChunkType,
        channels: u32,
        expected_size: u32,
    ) -> Result<(ChunkData, u32), FsbError> {
        let start = reader.position();
        let data = match chunk_type {
            crate::chunk::ChunkType::VorbisData => {
                // C# reads the u32 CRC then discards the rest of the chunk.
                let crc32 = reader.read_u32_le()?;
                reader.skip(expected_size.saturating_sub(4) as usize)?;
                ChunkData::Vorbis { crc32 }
            }
            crate::chunk::ChunkType::Frequency => {
                let frequency_id = reader.read_u32_le()?;
                ChunkData::Frequency { frequency_id }
            }
            crate::chunk::ChunkType::Channels => {
                let num_channels = reader.read_u8()?;
                ChunkData::Channels { num_channels }
            }
            crate::chunk::ChunkType::Loop => {
                let loop_start = reader.read_u32_le()?;
                let loop_end = reader.read_u32_le()?;
                ChunkData::Loop {
                    loop_start,
                    loop_end,
                }
            }
            crate::chunk::ChunkType::DspCoeff => {
                // 0x2E bytes per channel: 0x20 (16 big-endian i16) + 0x0E padding.
                let mut channel_data = Vec::with_capacity(channels as usize);
                for _ in 0..channels {
                    let mut coeffs = Vec::with_capacity(16);
                    for _ in 0..16 {
                        coeffs.push(reader.read_i16_be()?);
                    }
                    reader.skip(0x0E)?; // Int64 + Int32 + Int16
                    channel_data.push(coeffs);
                }
                ChunkData::DspCoefficients { channel_data }
            }
            _ => {
                let bytes = reader.read_bytes(expected_size as usize)?.to_vec();
                ChunkData::Unknown(bytes)
            }
        };
        let read = (reader.position() - start) as u32;
        Ok((data, read))
    }
}
