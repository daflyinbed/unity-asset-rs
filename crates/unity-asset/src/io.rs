//! io 层门面:UnityFS 解包的公开出口(高级用户用)。
//!
//! 默认入口 [`crate::Loader`] 内部已封装本模块;普通用户不需要直接访问。
//! 本模块提供给需要裸 [`LoadedFile`] 树、自定义 sniff 流程、或直接调
//! [`load_recursive`] 的高级用户。

pub use crate::bundle::{Bundle, RawEntry};
pub use crate::decompress::{
    CompressionType, Decompressor, IdentityDecompressor, Lz4Decompressor, LzmaDecompressor,
    Registry,
};
pub use crate::format::{load_recursive, sniff, FileFormat, LoadedFile};
pub use crate::serialized_file::{FileIdentifier, ObjectInfo, SerializedFile, SerializedType};
