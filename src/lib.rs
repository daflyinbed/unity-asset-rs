pub use bytes::Bytes;
pub use bundle::{Bundle, RawEntry};
pub use decompress::{CompressionType, Decompressor, IdentityDecompressor, Lz4Decompressor, LzmaDecompressor, Registry};
pub use error::{Error, Result};
pub use format::{FileFormat, LoadedFile, load_recursive, sniff};
pub use serialized_file::{FileIdentifier, ObjectInfo, SerializedFile, SerializedType};
pub use typetree::{
    build_tree, interpret, interpret_mono_behaviour, parse_blob, BlobNode, PPtrValue, RefTypeTable,
    TypeKind, TypeNode, Value,
};
pub use unity_core::{Endian, PPtr, UnityVersion};

mod bundle;
mod cursor;
mod containers;
mod detect;
pub mod decompress;
pub mod environment;
mod error;
pub mod format;
pub mod names;
pub mod object_parser;
mod serialized_file;
#[cfg(feature = "export")]
pub mod sprite_export;
#[cfg(feature = "export")]
pub mod texture_export;
pub mod typetree;

// ── generated crate 桥接 ──
pub use generated::{ClassIDType, UnityObject};
pub use object_parser::{parse_object, MonoBehaviour, ParsedObject};
pub use environment::{
    AssetCollection, CollectionId, DependencyProvider, Environment, EnvironmentBuilder,
    EnvironmentObject, ObjectId, ObjectRef, ParseFailure, PPtrLike, ResourceFile,
    ResourceProvider,
};

/// 从文件路径加载 Unity 资产文件（递归再派发）。
///
/// 自动嗅探格式（UnityFS / UnityWeb / UnityRaw / UnityWebData / gzip / brotli /
/// 松散 SerializedFile），递归解包到叶子文件。返回 [`LoadedFile`] 树。
pub fn load(path: impl AsRef<std::path::Path>) -> Result<LoadedFile> {
    load_with(path, &Registry::defaults())
}

/// 用自定义解压器注册表加载（递归再派发）。
pub fn load_with(path: impl AsRef<std::path::Path>, registry: &Registry) -> Result<LoadedFile> {
    let path = path.as_ref();
    let bytes = std::fs::read(path).map_err(|e| Error::Io { source: e })?;
    load_bytes_with(Bytes::from(bytes), registry)
}

/// 从内存字节加载（递归再派发）。
pub fn load_bytes(src: Bytes) -> Result<LoadedFile> {
    load_bytes_with(src, &Registry::defaults())
}

/// 用自定义解压器注册表从内存字节加载（递归再派发）。
pub fn load_bytes_with(src: Bytes, registry: &Registry) -> Result<LoadedFile> {
    let name = "root";
    load_recursive(src, name, registry, None)
}
