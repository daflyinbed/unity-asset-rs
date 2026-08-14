//! A single sample within a sound bank (`FmodSample` in the C# source).

use crate::metadata::SampleMetadata;

/// One sample: its metadata, raw (still-encoded) data bytes, and optional name.
///
/// `sample_bytes` is a zero-copy borrow into the original input buffer. Use
/// [`Sample::rebuild`](crate::Sample::rebuild) to decode it into a WAV.
#[derive(Debug)]
pub struct Sample<'a> {
    /// Bit-packed metadata and parsed chunks.
    pub metadata: SampleMetadata,
    /// Raw encoded sample data (codec-dependent).
    pub sample_bytes: &'a [u8],
    /// Sample name from the name table, if the bank declared one.
    pub name: Option<String>,
}

impl<'a> Sample<'a> {
    pub(crate) fn new(
        metadata: SampleMetadata,
        sample_bytes: &'a [u8],
        name: Option<String>,
    ) -> Self {
        Self {
            metadata,
            sample_bytes,
            name,
        }
    }
}
