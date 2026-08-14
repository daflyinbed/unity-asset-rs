//! Sample chunk types (`FmodSampleChunkType` in the C# source).
//!
//! The chunk-type field occupies bits 25–31 of the chunk-header word (7 bits), so
//! the raw value is stored as a `u8`. Known variants map to named cases; anything
//! else falls through to [`ChunkType::Unknown`].

/// Identifies the kind of a sample chunk.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChunkType {
    /// `CHANNELS = 1` — number of channels override.
    Channels,
    /// `FREQUENCY = 2` — frequency override.
    Frequency,
    /// `LOOP = 3` — loop region.
    Loop,
    /// `COMMENT = 4`.
    Comment,
    /// `XMASEEK = 6`.
    XmaSeek,
    /// `DSPCOEFF = 7` — GCADPCM coefficients.
    DspCoeff,
    /// `ATRAC9CFG = 9`.
    Atrac9Cfg,
    /// `XWMADATA = 10`.
    XwmaData,
    /// `VORBISDATA = 11` — vorbis setup-header CRC32 lookup key.
    VorbisData,
    /// `PEAKVOLUME = 13`.
    PeakVolume,
    /// `VORBISINTRALAYERS = 14`.
    VorbisIntraLayers,
    /// `OPUSDATALEN = 15`.
    OpusDataLen,
    /// A chunk type not in the known list. Carries the raw 7-bit discriminant.
    Unknown(u8),
}

impl ChunkType {
    pub(crate) fn from_u8(raw: u8) -> Self {
        match raw {
            1 => Self::Channels,
            2 => Self::Frequency,
            3 => Self::Loop,
            4 => Self::Comment,
            6 => Self::XmaSeek,
            7 => Self::DspCoeff,
            9 => Self::Atrac9Cfg,
            10 => Self::XwmaData,
            11 => Self::VorbisData,
            13 => Self::PeakVolume,
            14 => Self::VorbisIntraLayers,
            15 => Self::OpusDataLen,
            other => Self::Unknown(other),
        }
    }
}

/// A parsed sample chunk: its declared type, whether more chunks follow, and the
/// decoded payload.
///
/// Mirrors `FmodSampleChunk` in the C# source. The `more` flag is the lowest bit
/// of the chunk-header word and signals whether additional chunks follow for the
/// same sample.
#[derive(Debug)]
pub struct Chunk {
    /// Decoded chunk-type discriminant.
    pub chunk_type: ChunkType,
    /// True when more chunks follow this one for the same sample.
    pub more: bool,
    /// Type-specific decoded payload.
    pub data: crate::chunk_data::ChunkData,
}
