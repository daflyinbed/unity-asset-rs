//! Unity AssetBundle (UnityFS) 解析库。
//!
//! # 推荐入口
//!
//! ```no_run
//! # use unity_asset::{Loader, Environment};
//! let report = Loader::builder()
//!     .path("main.ab")
//!     .build()?; // 顶层 IO 错误在此返回
//! let env: &Environment = report.environment();
//! for d in report.diagnostics() {
//!     eprintln!("{d}");
//! }
//! # Ok::<(), unity_asset::Error>(())
//! ```
//!
//! 高级底层入口(非默认):[`io`](mod@io) 模块(裸 [`LoadedFile`] 树)、
//! [`typetree`] 模块(TypeTree 解释器)、[`types`] 模块(codegen typed struct 出口)。

pub use bytes::Bytes;
pub use error::{Error, Result};
pub use unity_core::{Endian, PPtr, UnityVersion};

// ── 模块树 ──
mod bundle;
mod container;
mod containers;
mod cursor;
pub mod decompress;
mod detect;
pub mod environment;
mod error;
pub mod format;
pub mod loader;
pub mod names;
pub mod object_parser;
mod serialized_file;
pub mod typetree;

// ── 高级门面模块 ──
#[cfg(feature = "export")]
pub mod export;
pub mod io;
pub mod types;

#[cfg(feature = "export")]
pub mod sprite_export;
#[cfg(feature = "export")]
pub mod texture_export;

// ── 顶层 re-export:常用类型直转,避免用户写全路径 ──
pub use container::{ContainerEntry, ContainerIndex, ContainerKind};
pub use environment::{
    AssetCollection, ClassId, CollectionId, DependencyProvider, DependencyRequest, Environment,
    EnvironmentObject, NamedBytes, ObjectId, ObjectRef, PPtrLike, ParseFailure, ProviderError,
    ResourceFile, ResourceProvider, ResourceRequest, SourceId,
};
#[cfg(feature = "export")]
pub use export::{DecodeImageError, DecodeOptions, DecodedImage, decode_texture};
pub use format::{FileFormat, LoadedFile, load_recursive, sniff};
pub use loader::{Diagnostic, DiagnosticKind, LoadReport, Loader, LoaderBuilder, Severity};
pub use object_parser::{MonoBehaviour, ParsedObject};
pub use typetree::{PPtrValue, RefTypeTable, Value};

/// 从文件路径加载 Unity 资产文件(递归再派发)。
///
/// 自动嗅探格式(UnityFS / UnityWeb / UnityRaw / UnityWebData / gzip / brotli /
/// 松散 SerializedFile),递归解包到叶子文件。返回 [`LoadedFile`] 树。
///
/// 这是 **io 层**入口(不解对象);构造聚合 [`Environment`] 用 [`Loader::builder`]。
pub fn load(path: impl AsRef<std::path::Path>) -> Result<LoadedFile> {
    load_with(path, &Registry::defaults())
}

/// 用自定义解压器注册表加载(递归再派发)。
pub fn load_with(path: impl AsRef<std::path::Path>, registry: &Registry) -> Result<LoadedFile> {
    let path = path.as_ref();
    let bytes = std::fs::read(path).map_err(|e| Error::Io { source: e })?;
    load_bytes_with(Bytes::from(bytes), registry)
}

/// 从内存字节加载(递归再派发)。
pub fn load_bytes(src: Bytes) -> Result<LoadedFile> {
    load_bytes_with(src, &Registry::defaults())
}

/// 用自定义解压器注册表从内存字节加载(递归再派发)。
pub fn load_bytes_with(src: Bytes, registry: &Registry) -> Result<LoadedFile> {
    let name = "root";
    load_recursive(src, name, registry, None)
}

// Registry/Decompressor 经 decompress 模块导出,但顶层也直转(常用)。
pub use decompress::{
    CompressionType, Decompressor, IdentityDecompressor, Lz4Decompressor, LzmaDecompressor,
    Registry,
};
