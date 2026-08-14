//! Errors raised by the rebuild layer (decoding + WAV repackaging).

use crate::AudioType;

/// Errors that can occur while rebuilding a sample into a standard file format.
#[derive(Debug, thiserror::Error)]
pub enum RebuildError {
    /// The sample's codec is not one this crate can rebuild yet.
    #[error("unsupported codec for rebuild: {0:?}")]
    UnsupportedCodec(AudioType),

    /// A GCADPCM sample is missing the `DSPCOEFF` chunk its decoder requires.
    #[error("GCADPCM sample is missing its DSPCOEFF chunk")]
    MissingDspCoeff,

    /// A Vorbis sample does not carry the setup-header CRC chunk.
    #[error("Vorbis sample is missing its VORBISDATA chunk")]
    MissingVorbisData,

    /// The embedded setup-header table has no entry for this FMOD CRC.
    #[error("no embedded Vorbis setup header for CRC32 {0:#010x}")]
    UnknownVorbisSetup(u32),

    /// Embedded or sample Vorbis data is malformed or truncated.
    #[error("invalid Vorbis data: {0}")]
    InvalidVorbisData(&'static str),

    /// Ogg output could not be written.
    #[error("failed to write Ogg stream")]
    OggWrite,

    /// A chunk carried fewer bytes than the decoder needs.
    #[error("sample data too short for its declared codec")]
    SampleDataTooShort,

    /// The sample's channel count is not valid for its codec.
    #[error("invalid channel count {0} for codec")]
    InvalidChannelCount(u32),

    /// The requested sample index does not exist in the bank.
    #[error("sample index {0} is out of bounds")]
    InvalidSampleIndex(usize),

    /// A declared size or WAV format field cannot be represented safely.
    #[error("rebuilt output is too large or contains overflowing format fields")]
    OutputTooLarge,
}
