use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("io error: {source}"))]
    Io { source: std::io::Error },

    #[snafu(display("data truncated: {reason}"))]
    Truncated { reason: String },

    #[snafu(display("bad signature: {sig:?} (expected \"UnityFS\")"))]
    BadSignature { sig: String },

    #[snafu(display("unsupported bundle format version: {version}"))]
    UnsupportedVersion { version: u32 },

    #[snafu(display("unsupported file format: {format:?} (recognized but not implemented)"))]
    UnsupportedFormat { format: crate::format::FileFormat },

    #[snafu(display("unsupported compression type: {compression}"))]
    UnsupportedCompression { compression: u16 },

    #[snafu(display("decompress failed: {reason}"))]
    Decompress { reason: String },

    #[snafu(display("lz4 decompress failed: expected {expected} bytes, got {actual}"))]
    Lz4SizeMismatch { expected: usize, actual: usize },

    #[snafu(display("serialized file error: {reason}"))]
    SerializedFile { reason: String },

    #[snafu(display("typetree: expected to read {expected} bytes, got {actual}"))]
    TypeTreeSizeMismatch { expected: usize, actual: usize },

    #[snafu(display("typetree: negative array length {len}"))]
    NegativeArrayLen { len: i32 },

    #[snafu(display("referenced type not found: {class} {ns} {asm}"))]
    RefTypeNotFound { class: String, ns: String, asm: String },

    #[snafu(display("empty type tree"))]
    EmptyTypeTree,

    #[snafu(display("typetree error: {reason}"))]
    TypeTree { reason: String },

    #[snafu(display("typed parse error: {reason}"))]
    TypedParse { reason: String },

    #[snafu(display("unknown class id: {id}"))]
    UnknownClassId { id: i32 },

    #[snafu(display("MonoBehaviour has no embedded TypeTree, cannot parse"))]
    MonoBehaviourWithoutTypeTree,

    #[snafu(display("MonoBehaviour shell field `{field}` missing or wrong type: {reason}"))]
    MonoBehaviourShell { field: String, reason: String },
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
