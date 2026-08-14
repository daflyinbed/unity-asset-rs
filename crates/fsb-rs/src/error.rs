//! Error types for FSB5 parsing.

/// Errors that can occur while parsing an FSB5 sound bank.
#[derive(Debug, thiserror::Error)]
pub enum FsbError {
    /// The file does not start with the `FSB5` magic.
    #[error("not an FSB5 file (bad magic)")]
    BadMagic,

    /// The input ended before a complete value could be read.
    #[error("unexpected end of input")]
    UnexpectedEof,

    /// A sample chunk consumed a different number of bytes than its declared size.
    #[error("chunk size mismatch: expected {expected}, read {read}")]
    ChunkSizeMismatch { expected: u32, read: u32 },

    /// The declared sample-header region cannot contain the advertised metadata.
    #[error("invalid sample-header region")]
    BadSampleHeaderBounds,

    /// A sample's data offset/length fell outside the bank's data region.
    #[error("invalid offset/length while reading sample data")]
    BadSampleBounds,

    /// A sample-name offset or string fell outside the bank's name-table region.
    #[error("invalid offset/string while reading sample name")]
    BadNameBounds,

    /// A string was not valid UTF-8.
    #[error("invalid UTF-8: {0}")]
    BadUtf8(#[from] std::str::Utf8Error),
}
