pub use bytes::Bytes;
pub use bundle::{Bundle, BundleFile};
pub use cursor::Cursor;
pub use decompress::{CompressionType, Decompressor, IdentityDecompressor, Lz4Decompressor, LzmaDecompressor, Registry};
pub use error::{Error, Result};
pub use serialized_file::{FileIdentifier, ObjectInfo, SerializedFile, SerializedType};
pub use typetree::{
    build_tree, interpret, interpret_mono_behaviour, parse_blob, BlobNode, PPtrValue, RefTypeTable,
    TypeKind, TypeNode, Value,
};
pub use unity_core::{Endian, PPtr, UnityVersion};

mod bundle;
pub mod cursor;
mod detect;
pub mod decompress;
pub mod environment;
mod error;
pub mod names;
pub mod object_parser;
mod serialized_file;
pub mod typetree;

// ── generated crate 桥接 ──
pub use generated::{ClassIDType, UnityObject};
pub use object_parser::{parse_object, MonoBehaviour, ParsedObject};
pub use environment::{
    AssetCollection, DependencyProvider, Environment, EnvironmentBuilder, EnvironmentObject,
    ParseFailure, PPtrLike, ResourceFile, ResourceProvider,
};

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

/// 便捷入口：load + `Environment::from_bundle`。
pub fn load_environment(path: impl AsRef<std::path::Path>) -> Result<Environment> {
    Ok(Environment::from_bundle(load(path)?))
}

/// 多 bundle 路径 → 一个 `Environment`（跨 bundle 依赖在 build 时全局解析）。
///
/// 内部走 [`EnvironmentBuilder`]：逐 bundle 做 Phase A（eager parse），全部完成后一次
/// Phase B（全局 `name_fixed` 匹配），故 bundle 之间的前向/后向 PPtr 引用都能命中。
pub fn load_environment_multi<I, P>(paths: I) -> Result<Environment>
where
    I: IntoIterator<Item = P>,
    P: AsRef<std::path::Path>,
{
    let mut builder = Environment::builder();
    for p in paths {
        builder = builder.path(p)?;
    }
    Ok(builder.build())
}

/// 递归扫描目录，嗅探 `UnityFS` 签名，把全部 bundle 加载进一个 `Environment`。
///
/// 对齐 AssetStudio `LoadFolder`：递归遍历目录树，对每个文件读头部判断是否为 UnityFS
/// bundle，命中才加载，否则静默跳过（`log::warn`）。gzip/brotli/`UnityWeb`/非 bundle 文件
/// 均跳过（gzip 路线图见 `docs/environment-layer.md`）。
pub fn load_environment_dir(path: impl AsRef<std::path::Path>) -> Result<Environment> {
    let path = path.as_ref();
    let mut builder = Environment::builder();
    let mut any = false;
    for entry in walk_files(path)? {
        if detect::looks_like_unityfs_path(&entry)? {
            builder = builder.path(&entry)?;
            any = true;
        } else {
            log::warn!("load_environment_dir: 跳过非 UnityFS 文件: {}", entry.display());
        }
    }
    if !any {
        log::warn!("load_environment_dir: {} 下未发现任何 UnityFS bundle", path.display());
    }
    Ok(builder.build())
}

/// 递归收集目录下全部文件路径（先序）。符号链接子目录跳过（避免循环）。
fn walk_files(root: &std::path::Path) -> Result<Vec<std::path::PathBuf>> {
    let mut out = Vec::new();
    walk_inner(root, &mut out)?;
    Ok(out)
}

fn walk_inner(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) -> Result<()> {
    for entry in std::fs::read_dir(dir).map_err(|e| Error::Io { source: e })? {
        let entry = entry.map_err(|e| Error::Io { source: e })?;
        let path = entry.path();
        let ft = entry.file_type().map_err(|e| Error::Io { source: e })?;
        if ft.is_dir() {
            // 不跟符号链接目录，防循环。
            if !path.is_symlink() {
                walk_inner(&path, out)?;
            }
        } else {
            out.push(path);
        }
    }
    Ok(())
}
