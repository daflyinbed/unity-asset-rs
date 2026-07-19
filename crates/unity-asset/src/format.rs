//! 格式闭集 enum + sniff + 递归再派发驱动器。
//!
//! 对齐 ADR-0003 的「递归再派发 + 三轴差异化扩展」：格式 = 闭集 → `enum FileFormat` +
//! `sniff` if/else + `match parse`（无 trait、无 `Vec`）。每种格式只「识别 + 解析自身结构
//! + 列出原始子字节」；驱动器看到 Container 就对每个子条目重新 sniff + parse，同一套派发
//! 服务顶层与子条目，任意嵌套（如 `brotli(UnityFS(.assets))`）免费获得。
//!
//! 参考 AssetRipper `SchemeReader`/`IScheme`（有序策略列表 + 两阶段解析），但 Rust 化为
//! enum 穷尽匹配（闭集无需 trait 对象）。

use bytes::Bytes;

use crate::bundle::{RawEntry, parse_unityfs};
use crate::containers::{compressed, rawweb, webfile};
use crate::decompress::Registry;
use crate::error::{Error, Result, UnsupportedFormatSnafu};

/// io 层支持的全部文件格式（闭集）。
///
/// 加格式 = 加一个枚举变体 + 一组 sniff 分支 + `load_recursive` 的一个 match 臂。
/// 顺序对齐 AssetRipper `SchemeReader` 的 sniff 优先级。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileFormat {
    /// 松散 SerializedFile（非 bundle 内嵌）。
    SerializedFile,
    /// UnityFS（现代主流容器，format 6/7/8）。
    FileStreamBundle,
    /// UnityWeb（旧版容器，LZMA 压缩）。
    WebBundle,
    /// UnityRaw（旧版容器，不压缩）。
    RawBundle,
    /// UnityWebData（WebGL 数据格式，小端）。
    WebFile,
    /// gzip 外层包裹。
    GZip,
    /// brotli 外层包裹。
    Brotli,
    /// UnityArchive（识别但不解析）。
    ArchiveBundle,
    /// 兜底：无法识别的裸资源（.resS / 纹理 / 音频等）。
    Resource,
}

/// io 层递归再派发的产物：一棵可能嵌套的文件树。
///
/// Container（bundle / webfile / 压缩壳）持有元信息 + 递归后的子树；
/// Leaf（SerializedFile 或裸资源）携带名字 + 解压后字节 + 可选引擎版本（供
/// `SerializedFile::parse` 继承）。
/// Failed（子条目递归 parse 失败）保留**原始字节** + 错误，不伪装成空 Resource。
#[derive(Debug)]
pub enum LoadedFile {
    Container {
        /// 引擎版本（bundle 有；webfile / compressed 无 → None）。
        unity_revision: Option<String>,
        /// bundle format version（如 UnityFS 的 6/7/8）。非 bundle 格式为 None。
        format_version: Option<u32>,
        /// 本层的格式（调试 / 日志用）。
        format: FileFormat,
        /// 递归后的子树。
        children: Vec<LoadedFile>,
    },
    /// 叶子：SerializedFile 或裸资源。
    Leaf {
        name: String,
        bytes: Bytes,
        /// 从父 Container 继承下来的引擎版本（顶层松散文件为 None）。
        unity_revision: Option<String>,
        /// sniff 结论（SerializedFile / Resource 等），供下游判断是否 parse。
        format: FileFormat,
    },
    /// 子条目递归 parse 失败。携带**原始字节**(可供诊断/重试)与错误。
    ///
    /// 旧实现把失败伪装成 `Leaf { format: Resource, bytes: Bytes::new() }`,
    /// 丢弃原始字节且只写日志——下游既看不到失败也拿不回字节。本变体让失败成为
    /// 显式 API 结果。
    Failed {
        name: String,
        bytes: Bytes,
        error: Error,
        /// 从父 Container 继承的引擎版本。
        unity_revision: Option<String>,
    },
}

/// 失败节点引用(view),由 [`LoadedFile::failures`] 产出。
#[derive(Debug, Clone, Copy)]
pub struct LoadedFailure<'a> {
    pub name: &'a str,
    pub bytes: &'a Bytes,
    pub error: &'a Error,
}

impl LoadedFile {
    /// 深度优先展平到叶子列表，返回 `(name, bytes, unity_revision)` 三元组引用。
    ///
    /// 供下游 `phase_a` 等消费——它们只关心最终的叶子文件，不关心 Container 层级。
    /// **不含** [`LoadedFile::Failed`] 节点(用 [`LoadedFile::failures`] 单独查)。
    pub fn leaves(&self) -> Vec<(&str, &Bytes, Option<&str>, FileFormat)> {
        let mut out = Vec::new();
        self.collect_leaves(&mut out);
        out
    }

    fn collect_leaves<'a>(
        &'a self,
        out: &mut Vec<(&'a str, &'a Bytes, Option<&'a str>, FileFormat)>,
    ) {
        match self {
            LoadedFile::Leaf {
                name,
                bytes,
                unity_revision,
                format,
            } => {
                out.push((name, bytes, unity_revision.as_deref(), *format));
            }
            LoadedFile::Container { children, .. } => {
                for child in children {
                    child.collect_leaves(out);
                }
            }
            LoadedFile::Failed { .. } => {}
        }
    }

    /// 深度优先收集所有 [`LoadedFile::Failed`] 节点。
    pub fn failures(&self) -> Vec<LoadedFailure<'_>> {
        let mut out = Vec::new();
        self.collect_failures(&mut out);
        out
    }

    fn collect_failures<'a>(&'a self, out: &mut Vec<LoadedFailure<'a>>) {
        match self {
            LoadedFile::Failed {
                name, bytes, error, ..
            } => out.push(LoadedFailure { name, bytes, error }),
            LoadedFile::Container { children, .. } => {
                for child in children {
                    child.collect_failures(out);
                }
            }
            LoadedFile::Leaf { .. } => {}
        }
    }

    /// 消费整棵树,把所有 [`LoadedFile::Failed`] 节点的 `name`/`error` 按值取出。
    ///
    /// `Error` 未实现 `Clone`(含 `io::Error`),借用版本 [`LoadedFile::failures`]
    /// 只能返回引用;调用方若需要拥有(如塞进 `ParseFailure`)用此版本。
    pub fn into_failures(self) -> Vec<(String, Error)> {
        fn go(f: LoadedFile, out: &mut Vec<(String, Error)>) {
            match f {
                LoadedFile::Failed { name, error, .. } => out.push((name, error)),
                LoadedFile::Container { children, .. } => {
                    for c in children {
                        go(c, out);
                    }
                }
                LoadedFile::Leaf { .. } => {}
            }
        }
        let mut out = Vec::new();
        go(self, &mut out);
        out
    }
}

// ── sniff ──────────────────────────────────────────────────────────

/// 各 bundle 签名（ASCII，大端）。
const MAGIC_UNITYFS: &[u8] = b"UnityFS";
const MAGIC_UNITY_RAW: &[u8] = b"UnityRaw";
const MAGIC_UNITY_WEB: &[u8] = b"UnityWeb";
const MAGIC_UNITY_ARCHIVE: &[u8] = b"UnityArchive";
const MAGIC_UNITY_WEB_DATA: &[u8] = b"UnityWebData1.0\0";

/// 嗅探字节切片的格式。if/else 链，顺序对齐 AssetRipper `SchemeReader` 优先级。
///
/// 优先级：各 bundle magic → WebFile → GZip → Brotli → SerializedFile（弱 sniff，靠后）
/// → Resource 兜底。sniff 只读头部少量字节，不消费整个流。
pub fn sniff(bytes: &[u8]) -> FileFormat {
    // 1. 各 bundle magic（zero-term string 前缀）
    if starts_with_cstr(bytes, MAGIC_UNITYFS) {
        return FileFormat::FileStreamBundle;
    }
    if starts_with_cstr(bytes, MAGIC_UNITY_RAW) {
        return FileFormat::RawBundle;
    }
    if starts_with_cstr(bytes, MAGIC_UNITY_WEB) {
        return FileFormat::WebBundle;
    }
    if starts_with_cstr(bytes, MAGIC_UNITY_ARCHIVE) {
        return FileFormat::ArchiveBundle;
    }
    // 2. WebFile（"UnityWebData1.0\0"）
    if bytes.len() >= MAGIC_UNITY_WEB_DATA.len()
        && &bytes[..MAGIC_UNITY_WEB_DATA.len()] == MAGIC_UNITY_WEB_DATA
    {
        return FileFormat::WebFile;
    }
    // 3. GZip（0x1F 0x8B）
    if bytes.len() >= 2 && bytes[0] == 0x1F && bytes[1] == 0x8B {
        return FileFormat::GZip;
    }
    // 4. Brotli（需帧头校验，见 compressed::looks_like_brotli）
    if compressed::looks_like_brotli(bytes) {
        return FileFormat::Brotli;
    }
    // 5. SerializedFile（弱 sniff：头部尺寸一致性校验，靠后避免误判）
    if looks_like_serialized_file(bytes) {
        return FileFormat::SerializedFile;
    }
    // 6. 兜底
    FileFormat::Resource
}

/// 判断 `bytes` 是否以 `magic\0` 开头（zero-term string 前缀匹配）。
///
/// bundle 签名是 zero-term string，故 magic 后必须紧跟 0x00（或 magic 本身含终止符）。
fn starts_with_cstr(bytes: &[u8], magic: &[u8]) -> bool {
    if bytes.len() < magic.len() + 1 {
        return false;
    }
    &bytes[..magic.len()] == magic && bytes[magic.len()] == 0
}

/// SerializedFile 弱嗅探：头部尺寸一致性校验。
///
/// 对齐 AssetRipper `SerializedFileHeader.IsSerializedFileHeader`——SerializedFile 没有
/// magic，靠 metadata_size / file_size / version 等字段的合理性交叉验证。sniff 较弱，
/// 故在 `sniff` 中排在所有 bundle / 压缩格式之后。
fn looks_like_serialized_file(bytes: &[u8]) -> bool {
    // SerializedFile 头部至少需要：metadata_size(4) + fileSize(4) + version(4) +
    // data_offset(4) + endianness(1) + reserved(3) = 20 字节。
    if bytes.len() < 20 {
        return false;
    }
    // SerializedFile 前三个 u32 是大端（prologue 固定大端，读完 endian 字节才切换）。
    let metadata_size = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    let file_size = u32::from_be_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
    let version = u32::from_be_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);
    let data_offset = u32::from_be_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]);
    let endian = bytes[16];

    // version 合法范围（Unity SerializedFile 版本号，实测 6-22）。
    if !(6..=22).contains(&version) {
        return false;
    }
    // endian 只能是 0 或 1。
    if endian > 1 {
        return false;
    }
    // metadata_size 不能超过 file_size。
    if metadata_size > file_size {
        return false;
    }
    // data_offset 不能超过 file_size，且通常 >= header 大小。
    if data_offset > file_size {
        return false;
    }
    // file_size 不能超过实际字节（松散文件可能等于；bundle 内嵌可能小于）。
    // 放宽：只要求 file_size <= bytes.len() * 2（容忍一定误差，避免漏判）。
    if file_size as usize > bytes.len() * 2 {
        return false;
    }
    true
}

// ── 递归再派发驱动器 ───────────────────────────────────────────────

/// 递归再派发驱动器：sniff → parse → 若 Container 则对 children 逐个递归。
///
/// 这是 io 层的核心入口。同一套派发服务顶层与子条目，任意嵌套免费获得。
///
/// - `src`：待解析的字节。
/// - `name`：本层的文件名（用于 Leaf 的 name 字段 + 日志）。
/// - `registry`：解压器注册表（UnityFS 逐块解压 / gzip / brotli 等共用）。
/// - `parent_revision`：从父 Container 继承的引擎版本（顶层为 None）。
///
/// 单个子条目 parse 失败不中断整体——捕获错误后产出一个标记为 Resource 的 Leaf
/// （携带原始字节），让下游能继续处理其他条目。
pub fn load_recursive(
    src: Bytes,
    name: &str,
    registry: &Registry,
    parent_revision: Option<&str>,
) -> Result<LoadedFile> {
    let fmt = sniff(&src);
    log::debug!(
        "load_recursive: name={name:?} sniff={fmt:?} ({} bytes)",
        src.len()
    );

    match fmt {
        FileFormat::FileStreamBundle => {
            let bundle = parse_unityfs(src, registry)?;
            let ur = Some(bundle.unity_revision.clone());
            let fv = Some(bundle.format_version);
            let revision = bundle.unity_revision.as_str();
            let children = recurse_entries(bundle.entries, registry, Some(revision));
            Ok(LoadedFile::Container {
                unity_revision: ur,
                format_version: fv,
                format: fmt,
                children,
            })
        }
        FileFormat::RawBundle | FileFormat::WebBundle => {
            let bundle = rawweb::parse_rawweb(src, fmt, registry)?;
            let ur = Some(bundle.unity_revision.clone());
            let fv = Some(bundle.format_version);
            let revision = bundle.unity_revision.as_str();
            let children = recurse_entries(bundle.entries, registry, Some(revision));
            Ok(LoadedFile::Container {
                unity_revision: ur,
                format_version: fv,
                format: fmt,
                children,
            })
        }
        FileFormat::WebFile => {
            let entries = webfile::parse_webfile(src)?;
            // WebFile 没有 unity_revision（不是 bundle），继承父版本。
            let children = recurse_entries(entries, registry, parent_revision);
            Ok(LoadedFile::Container {
                unity_revision: parent_revision.map(|s| s.to_string()),
                format_version: None,
                format: fmt,
                children,
            })
        }
        FileFormat::GZip => {
            let decompressed = compressed::decompress_gzip(&src)?;
            // 解压后递归 sniff——内层可能是 UnityFS / WebFile / SerializedFile 等。
            let child = load_recursive(Bytes::from(decompressed), name, registry, parent_revision)?;
            Ok(LoadedFile::Container {
                unity_revision: parent_revision.map(|s| s.to_string()),
                format_version: None,
                format: fmt,
                children: vec![child],
            })
        }
        FileFormat::Brotli => {
            let decompressed = compressed::decompress_brotli(&src)?;
            let child = load_recursive(Bytes::from(decompressed), name, registry, parent_revision)?;
            Ok(LoadedFile::Container {
                unity_revision: parent_revision.map(|s| s.to_string()),
                format_version: None,
                format: fmt,
                children: vec![child],
            })
        }
        FileFormat::SerializedFile => Ok(LoadedFile::Leaf {
            name: name.to_string(),
            bytes: src,
            unity_revision: parent_revision.map(|s| s.to_string()),
            format: fmt,
        }),
        FileFormat::Resource => Ok(LoadedFile::Leaf {
            name: name.to_string(),
            bytes: src,
            unity_revision: parent_revision.map(|s| s.to_string()),
            format: fmt,
        }),
        FileFormat::ArchiveBundle => Err(UnsupportedFormatSnafu { format: fmt }.build()),
    }
}

/// 对 `entries` 逐个递归 sniff + parse,错误隔离。
///
/// 单个 entry 失败不中断——产出 [`LoadedFile::Failed`],**保留原始字节** + 错误,
/// 让下游([`LoadedFile::failures`] / `Environment::failures`)能显式看到失败原因
/// 并(可选)取回字节重试。不再伪装成空 Resource、也不再只写 log。
fn recurse_entries(
    entries: Vec<RawEntry>,
    registry: &Registry,
    parent_revision: Option<&str>,
) -> Vec<LoadedFile> {
    entries
        .into_iter()
        .map(|e| {
            let name = e.name.clone();
            // Bytes 是 Arc 共享 slice,clone 廉价;失败时保留原始字节供诊断。
            let original = e.bytes.clone();
            match load_recursive(e.bytes, &name, registry, parent_revision) {
                Ok(loaded) => loaded,
                Err(err) => LoadedFile::Failed {
                    name,
                    bytes: original,
                    error: err,
                    unity_revision: parent_revision.map(|s| s.to_string()),
                },
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Failed 节点保留原始字节 + 错误;failures()/into_failures()/leaves() 行为正确。
    #[test]
    fn failed_node_preserves_bytes_and_error() {
        let archive_bytes = Bytes::from(b"UnityArchive\0garbage".to_vec());
        let tree = LoadedFile::Container {
            unity_revision: None,
            format_version: None,
            format: FileFormat::ArchiveBundle,
            children: vec![
                LoadedFile::Failed {
                    name: "bad.child".to_string(),
                    bytes: archive_bytes.clone(),
                    error: Error::UnsupportedFormat {
                        format: FileFormat::ArchiveBundle,
                    },
                    unity_revision: None,
                },
                LoadedFile::Leaf {
                    name: "ok.child".to_string(),
                    bytes: Bytes::new(),
                    unity_revision: None,
                    format: FileFormat::Resource,
                },
            ],
        };

        // failures() 借用,返回原始字节引用 + 错误引用。
        let fails = tree.failures();
        assert_eq!(fails.len(), 1, "应有 1 个 Failed 节点");
        assert_eq!(fails[0].name, "bad.child");
        assert_eq!(fails[0].bytes.as_ref(), archive_bytes.as_ref());

        // leaves() 不含 Failed,只含 ok.child。
        let leaves = tree.leaves();
        assert_eq!(leaves.len(), 1);
        assert_eq!(leaves[0].0, "ok.child");

        // into_failures() 消费树,取走 owning Error(Error 未实现 Clone)。
        let owned = tree.into_failures();
        assert_eq!(owned.len(), 1);
        assert_eq!(owned[0].0, "bad.child");
        assert!(matches!(
            owned[0].1,
            Error::UnsupportedFormat {
                format: FileFormat::ArchiveBundle
            }
        ));
    }

    /// `recurse_entries` 子条目失败时产 Failed,字节非空(回归旧实现丢字节的 bug)。
    #[test]
    fn recurse_entries_produces_failed_with_original_bytes() {
        // ArchiveBundle 是 sniff 能识别但 load_recursive 显式 Err 的唯一格式,
        // 经 recurse_entries 包装后应得到 Failed(而非顶层 Err)。
        let archive_bytes = Bytes::from(b"UnityArchive\0garbage".to_vec());
        let entry = RawEntry {
            name: "inner.archive".to_string(),
            bytes: archive_bytes.clone(),
            flags: 0,
        };
        let children = recurse_entries(vec![entry], &Registry::defaults(), None);
        assert_eq!(children.len(), 1);
        match &children[0] {
            LoadedFile::Failed { name, bytes, .. } => {
                assert_eq!(name, "inner.archive");
                // 关键回归断言:旧实现这里会是 Bytes::new()。
                assert_eq!(bytes.as_ref(), archive_bytes.as_ref());
            }
            other => panic!("期望 Failed,得到 {other:?}"),
        }
    }
}
