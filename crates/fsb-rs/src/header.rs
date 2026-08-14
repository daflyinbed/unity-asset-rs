//! FSB5 file header (`FmodAudioHeader` in the C# source).
//!
//! All multi-byte fields are little-endian. Layout:
//!
//! | offset | field                     |
//! |--------|----------------------------|
//! | 0x00   | magic `"FSB5"` (4 bytes)   |
//! | 0x04   | version (u32)              |
//! | 0x08   | num_samples (u32)          |
//! | 0x0C   | size_of_sample_headers     |
//! | 0x10   | size_of_name_table         |
//! | 0x14   | size_of_data               |
//! | 0x18   | audio_type (u32)           |
//! | 0x1C   | always 0 (skipped)         |
//! | 0x20+  | version-0 extra field (u32, only if version == 0), flags (u32),
//! |        | 128-bit hash (2× u64), unknown (u64) |
//!
//! `header_size` is `0x40` when `version == 0`, otherwise `0x3C`. Sample headers
//! (each 8 bytes + any chunks) follow immediately.

use crate::audio_type::AudioType;
use crate::error::FsbError;
use crate::metadata::{SampleMetadata, read_metadata_with_chunks};
use crate::reader::Reader;

/// Parsed FSB5 header: the public summary fields plus the size fields the loader
/// needs to locate sample data and names.
#[derive(Debug)]
pub struct Header {
    /// FSB format version (0 or 1 observed).
    pub version: u32,
    /// Number of samples in the bank.
    pub num_samples: u32,
    /// The bank's codec, shared by all samples.
    pub audio_type: AudioType,
    /// Total size of the FSB5 header in bytes (0x3C or 0x40).
    pub header_size: u32,
    /// Size of the sample-headers region in bytes.
    pub size_of_sample_headers: u32,
    /// Size of the name-table region in bytes.
    pub size_of_name_table: u32,
    /// Size of the sample-data region in bytes.
    pub size_of_data: u32,
    /// Per-sample metadata, in declaration order.
    pub(crate) samples_meta: Vec<SampleMetadata>,
}

impl Header {
    pub(crate) fn read(reader: &mut Reader<'_>) -> Result<Self, FsbError> {
        // Magic.
        let magic = reader.read_bytes(4)?;
        if magic != b"FSB5" {
            return Err(FsbError::BadMagic);
        }

        let version = reader.read_u32_le()?;
        let num_samples = reader.read_u32_le()?;
        let size_of_sample_headers = reader.read_u32_le()?;
        let size_of_name_table = reader.read_u32_le()?;
        let size_of_data = reader.read_u32_le()?;
        let audio_type = AudioType::from_u32(reader.read_u32_le()?);

        reader.read_u32_le()?; // 0x1C — always 0, skipped.

        // Version 0 carries an extra u32 at 0x20 before the flags field, and the
        // whole header is 0x40 bytes; otherwise 0x3C.
        let header_size = if version == 0 {
            reader.read_u32_le()?; // extra field at 0x20
            0x40
        } else {
            0x3C
        };

        reader.read_u32_le()?; // flags — skipped
        reader.read_u64_le()?; // hash lower 64 bits — skipped
        reader.read_u64_le()?; // hash upper 64 bits — skipped
        reader.read_u64_le()?; // unknown u64 — skipped

        // Parse sample headers through a reader restricted to their declared
        // region. This prevents a malformed chunk chain from consuming the name
        // table or sample data.
        let sample_headers_start = reader.position();
        let sample_headers_len = size_of_sample_headers as usize;
        let sample_headers_end = sample_headers_start
            .checked_add(sample_headers_len)
            .filter(|&end| end <= reader.full().len())
            .ok_or(FsbError::BadSampleHeaderBounds)?;
        let minimum_metadata_bytes = (num_samples as usize)
            .checked_mul(8)
            .ok_or(FsbError::BadSampleHeaderBounds)?;
        if minimum_metadata_bytes > sample_headers_len {
            return Err(FsbError::BadSampleHeaderBounds);
        }

        let mut sample_reader =
            Reader::new(&reader.full()[sample_headers_start..sample_headers_end]);
        let mut samples_meta = Vec::with_capacity(num_samples as usize);
        for _ in 0..num_samples {
            samples_meta.push(read_metadata_with_chunks(&mut sample_reader)?);
        }

        Ok(Self {
            version,
            num_samples,
            audio_type,
            header_size,
            size_of_sample_headers,
            size_of_name_table,
            size_of_data,
            samples_meta,
        })
    }
}
