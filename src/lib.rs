pub use bytes::Bytes;
pub use bundle::{Bundle, BundleFile};
pub use cursor::Cursor;
pub use decompress::{CompressionType, Decompressor, IdentityDecompressor, Lz4Decompressor, LzmaDecompressor, Registry};
pub use error::{Error, Result};
pub use serialized_file::{ObjectInfo, SerializedFile, SerializedType};
pub use typetree::{build_tree, interpret, parse_blob, BlobNode, PPtrValue, RefTypeTable, TypeKind, TypeNode, Value};
pub use unity_core::{Endian, PPtr, UnityVersion};

mod bundle;
pub mod cursor;
pub mod decompress;
mod error;
pub mod object_parser;
mod serialized_file;
pub mod typetree;

// ── generated crate 桥接 ──
pub use generated::{ClassIDType, UnityObject};
pub use object_parser::{parse_object, ParsedObject};

/// 从文件路径加载 UnityFS bundle。
pub fn load(path: impl AsRef<std::path::Path>) -> Result<Bundle> {
    load_with(path, &Registry::defaults())
}

/// 用自定义解压器注册表加载。
pub fn load_with(path: impl AsRef<std::path::Path>, registry: &Registry) -> Result<Bundle> {
    let path = path.as_ref();
    let bytes = std::fs::read(path).map_err(|e| Error::Io { source: e })?;
    load_bytes_with(Bytes::from(bytes), registry)
}

/// 从内存字节加载 UnityFS bundle。
pub fn load_bytes(src: Bytes) -> Result<Bundle> {
    load_bytes_with(src, &Registry::defaults())
}

/// 用自定义解压器注册表从内存字节加载。
pub fn load_bytes_with(src: Bytes, registry: &Registry) -> Result<Bundle> {
    bundle::parse(src, registry)
}
