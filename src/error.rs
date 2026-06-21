use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("io error: {source}"))]
    Io { source: std::io::Error },

    #[snafu(display("data truncated: {reason}"))]
    Truncated { reason: &'static str },

    #[snafu(display("bad signature: {sig:?} (expected \"UnityFS\")"))]
    BadSignature { sig: String },

    #[snafu(display("unsupported bundle format version: {version}"))]
    UnsupportedVersion { version: u32 },

    #[snafu(display("unsupported compression type: {compression}"))]
    UnsupportedCompression { compression: u16 },

    #[snafu(display("decompress failed: {reason}"))]
    Decompress { reason: String },

    #[snafu(display("lz4 decompress failed: expected {expected} bytes, got {actual}"))]
    Lz4SizeMismatch { expected: usize, actual: usize },
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
