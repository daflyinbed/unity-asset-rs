//! `fsb-rs` — an FMOD 5 sound bank (FSB5) container parser and rebuilder.
//!
//! This crate parses the FSB5 container format (header, per-sample metadata,
//! chunks, raw sample-data slicing, name table) and rebuilds supported samples
//! into playable WAV or Ogg files.
//!
//! Ported from [Fmod5Sharp](https://github.com/Samboy063/Fmod5Sharp).
//!
//! # Example
//!
//! ```no_run
//! use fsb_rs::parse;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let bytes = std::fs::read("sound_bank.fsb")?;
//! let bank = parse(&bytes)?;
//!
//! println!("codec: {:?}, {} samples", bank.header.audio_type, bank.header.num_samples);
//! // Rebuild the first sample into WAV or Ogg, according to the bank codec.
//! if let Ok(audio) = bank.rebuild_sample(0) {
//!     let extension = match &audio {
//!         fsb_rs::RebuiltAudio::Wav(_) => "wav",
//!         fsb_rs::RebuiltAudio::Ogg(_) => "ogg",
//!     };
//!     std::fs::write(format!("sample0.{extension}"), audio.into_bytes())?;
//! }
//! # Ok(())
//! # }
//! ```
//!
//! `SoundBank` borrows the input: keep the original `Vec<u8>` alive for as long
//! as you need to inspect `sample_bytes`.

mod audio_type;
mod chunk;
mod chunk_data;
mod error;
mod header;
mod metadata;
mod reader;
mod rebuild;
mod sample;
mod sound_bank;

pub use audio_type::AudioType;
pub use chunk::{Chunk, ChunkType};
pub use chunk_data::ChunkData;
pub use error::FsbError;
pub use header::Header;
pub use metadata::SampleMetadata;
pub use rebuild::{RebuildError, RebuiltAudio};
pub use sample::Sample;
pub use sound_bank::SoundBank;

use crate::reader::Reader;
use std::str;

/// Parse an FSB5 sound bank from a byte slice.
///
/// Returns a [`SoundBank`] whose `sample_bytes` borrow `input`. The input must
/// begin with the `FSB5` magic (callers that may have a leading prefix should
/// locate `"FSB5"` themselves before calling).
pub fn parse(input: &[u8]) -> Result<SoundBank<'_>, FsbError> {
    let mut reader = Reader::new(input);

    // 1. Header + sample metadata (including chunks).
    let mut header = Header::read(&mut reader)?;

    // 2. Locate each sample's raw data, zero-copy slices into `input`.
    let name_table_start = (header.header_size as usize)
        .checked_add(header.size_of_sample_headers as usize)
        .ok_or(FsbError::BadNameBounds)?;
    let data_start = name_table_start
        .checked_add(header.size_of_name_table as usize)
        .ok_or(FsbError::BadSampleBounds)?;
    let data_end = data_start
        .checked_add(header.size_of_data as usize)
        .filter(|&end| end <= input.len())
        .ok_or(FsbError::BadSampleBounds)?;

    let n = header.samples_meta.len();
    if header
        .samples_meta
        .iter()
        .any(|meta| meta.data_offset > header.size_of_data)
    {
        return Err(FsbError::BadSampleBounds);
    }
    let mut samples = Vec::with_capacity(n);
    for i in 0..n {
        let first = header.samples_meta[i].data_offset as usize;
        let last = if i + 1 < n {
            header.samples_meta[i + 1].data_offset as usize
        } else {
            header.size_of_data as usize
        };

        if first > last {
            return Err(FsbError::BadSampleBounds);
        }
        let start = data_start
            .checked_add(first)
            .ok_or(FsbError::BadSampleBounds)?;
        let end = data_start
            .checked_add(last)
            .ok_or(FsbError::BadSampleBounds)?;
        if end > data_end {
            return Err(FsbError::BadSampleBounds);
        }
        let sample_bytes = &input[start..end];

        // 3. Optional name from the name table.
        let name = if header.size_of_name_table > 0 {
            read_sample_name(input, &header, i)?
        } else {
            None
        };

        // Move this sample's metadata out of the header vec. We replace it with a
        // cheap placeholder; the vec is never exposed publicly and is dropped with
        // the header, so the placeholders are harmless.
        let meta = header.samples_meta[i].take_meta();
        samples.push(Sample::new(meta, sample_bytes, name));
    }

    Ok(SoundBank { header, samples })
}

/// Read sample `i`'s name from the name table (see `FsbLoader.LoadInternal`).
///
/// The name-offset table sits right after the sample-headers region, at
/// `header_size + size_of_sample_headers`; each entry is a u32 giving the name's
/// offset *relative to the start of the name table region's base*
/// (`header_size + size_of_sample_headers`). The name is null-terminated UTF-8.
fn read_sample_name(input: &[u8], header: &Header, i: usize) -> Result<Option<String>, FsbError> {
    let base = (header.header_size as usize)
        .checked_add(header.size_of_sample_headers as usize)
        .ok_or(FsbError::BadNameBounds)?;
    let name_region_end = base
        .checked_add(header.size_of_name_table as usize)
        .filter(|&end| end <= input.len())
        .ok_or(FsbError::BadNameBounds)?;
    let entry_pos = base
        .checked_add(i.checked_mul(4).ok_or(FsbError::BadNameBounds)?)
        .ok_or(FsbError::BadNameBounds)?;
    if entry_pos
        .checked_add(4)
        .is_none_or(|end| end > name_region_end)
    {
        return Err(FsbError::BadNameBounds);
    }
    let name_offset =
        u32::from_le_bytes(input[entry_pos..entry_pos + 4].try_into().unwrap()) as usize;
    let name_start = base
        .checked_add(name_offset)
        .filter(|&start| start < name_region_end)
        .ok_or(FsbError::BadNameBounds)?;

    // Read until 0x00 or end of the name-table region.
    let name_end = input[name_start..name_region_end]
        .iter()
        .position(|&b| b == 0)
        .map(|p| name_start + p)
        .ok_or(FsbError::BadNameBounds)?;
    let bytes = &input[name_start..name_end];
    Ok(Some(str::from_utf8(bytes)?.to_owned()))
}
