//! TPK 二进制读取器（小端序）。
//!
//! 详见 `docs/codegen-design.md` §6.1 与 AssetRipper Tpk 仓库的字节级清单。
//! 本模块**只读不写**。所有跨引用（字符串、节点）用 `u16` 索引，节点池扁平存储 + `sub_nodes` 递归表达树。
//!
//! # 文件布局
//!
//! ```text
//! TPK file (全程小端):
//!   u32 magic == 0x2A4B5054          // 'T' 'P' 'K' '*'
//!   u8  version == 1
//!   u8  compression (0/1/2/3)
//!   u8  dataType    (0 = TypeTree)
//!   u8  reserved
//!   u32 reserved
//!   i32 compressedSize
//!   i32 decompressedSize
//!   [u8; compressedSize] payload
//! ```
//!
//! 解压后（dataType == 0）即为 `TpkTypeTreeBlob` 的二进制序列化。

mod compression;
mod reader;

use std::collections::BTreeMap;
use std::path::Path;

use reader::LeReader;

pub use compression::CompressionType;

/// TPK 文件头部 magic（小端 u32，磁盘上是 'T' 'P' 'K' '*'）。
const MAGIC: u32 = 0x2A4B5054;
/// 当前支持的 TPK 版本。
const VERSION: u8 = 1;
/// TypeTree blob 的 dataType 标记。
const DATA_TYPE_TYPE_TREE: u8 = 0;

/// TPK 顶层文件：一个 header + 一段压缩 payload。
///
/// 调用 [`TpkFile::into_type_tree`] 拿到结构化的 TypeTree。
pub struct TpkFile {
    compression: CompressionType,
    data_type: u8,
    decompressed_size: usize,
    payload: Vec<u8>, // 压缩字节
}

impl TpkFile {
    /// 从文件路径读取。
    pub fn read_from_path(path: &Path) -> Result<Self, TpkError> {
        let bytes = std::fs::read(path).map_err(|e| TpkError::Io {
            path: path.display().to_string(),
            source: e,
        })?;
        Self::read_from_bytes(&bytes)
    }

    /// 从内存字节读取。
    pub fn read_from_bytes(bytes: &[u8]) -> Result<Self, TpkError> {
        let mut r = LeReader::new(bytes);

        let magic = r.read_u32("magic")?;
        if magic != MAGIC {
            return Err(TpkError::BadMagic { got: magic });
        }

        let version = r.read_u8("version")?;
        if version != VERSION {
            return Err(TpkError::UnsupportedVersion { got: version });
        }

        let compression = CompressionType::from_u8(r.read_u8("compressionType")?)?;
        let data_type = r.read_u8("dataType")?;
        if data_type != DATA_TYPE_TYPE_TREE {
            return Err(TpkError::UnsupportedDataType { got: data_type });
        }

        let _reserved1 = r.read_u8("reserved1")?;
        let _reserved2 = r.read_u32("reserved2")?;

        let compressed_size = r.read_i32("compressedSize")?;
        let decompressed_size = r.read_i32("decompressedSize")?;

        let payload = r.take(compressed_size as usize, "payload")?.to_vec();

        log::debug!(
            "TPK header: compression={compression:?}, dataType={data_type}, \
             compressed={compressed_size}, decompressed={decompressed_size}"
        );

        Ok(Self {
            compression,
            data_type,
            decompressed_size: decompressed_size as usize,
            payload,
        })
    }

    /// 解压 payload 并反序列化为 [`TpkTypeTree`]。消耗 self。
    pub fn into_type_tree(self) -> Result<TpkTypeTree, TpkError> {
        if self.data_type != DATA_TYPE_TYPE_TREE {
            return Err(TpkError::UnsupportedDataType {
                got: self.data_type,
            });
        }
        let decompressed = compression::decompress(
            self.compression,
            &self.payload,
            self.decompressed_size,
        )?;
        TpkTypeTree::read_from_bytes(&decompressed)
    }
}

// ────────────────────────────────────────────────────────────────────
// TypeTree blob（解压后的二进制反序列化产物）
// ────────────────────────────────────────────────────────────────────

/// 一个 classID → 跨版本布局列表 + 全局字符串/节点池。
pub struct TpkTypeTree {
    /// TPK 内出现的 Unity 版本（升序，去重）。
    pub versions: Vec<UnityVersion>,
    /// classID → 该 class 的版本分段信息。
    pub classes: BTreeMap<i32, TpkClassInformation>,
    /// 全局字符串池（u16 索引引用）。
    pub string_pool: Vec<String>,
    /// 全局节点池（u16 索引引用）。
    pub node_pool: Vec<TpkUnityNode>,
}

impl TpkTypeTree {
    pub fn read_from_bytes(bytes: &[u8]) -> Result<Self, TpkError> {
        let mut r = LeReader::new(bytes);

        // TpkTypeTreeBlob.Read 顺序：
        // i64 creationTime
        let _creation_time = r.read_i64("creationTime")?;

        // Versions[]
        let version_count = r.read_i32("versionCount")? as usize;
        let mut versions = Vec::with_capacity(version_count);
        for _ in 0..version_count {
            versions.push(UnityVersion::from_bits(r.read_u64("version")?));
        }

        // ClassInformation[]
        let class_count = r.read_i32("classCount")? as usize;
        let mut classes = BTreeMap::new();
        for _ in 0..class_count {
            let id = r.read_i32("classID")?;
            let entry_count = r.read_i32("entryCount")? as usize;
            let mut entries = Vec::with_capacity(entry_count);
            for _ in 0..entry_count {
                let version = UnityVersion::from_bits(r.read_u64("entry version")?);
                let has_class = r.read_u8("hasClassData")?;
                let class = if has_class != 0 {
                    Some(TpkUnityClass::read(&mut r)?)
                } else {
                    None
                };
                entries.push(VersionedClass { version, class });
            }
            classes.insert(
                id,
                TpkClassInformation {
                    id,
                    classes: entries,
                },
            );
        }

        // CommonString
        //   i32 versionCount, repeat { u64 version; u8 stringCount }
        //   i32 indicesCount, [u16 stringBufferIndex; indicesCount]
        // 首版不使用 common string（运行时不需要），但必须消费。
        let cs_version_count = r.read_i32("commonString versionCount")? as usize;
        for _ in 0..cs_version_count {
            let _v = r.read_u64("commonString version")?;
            let _n = r.read_u8("commonString stringCount")?;
        }
        let cs_indices_count = r.read_i32("commonString indicesCount")? as usize;
        for _ in 0..cs_indices_count {
            let _idx = r.read_u16("commonString stringBufferIndex")?;
        }

        // NodeBuffer
        let node_count = r.read_i32("nodeCount")? as usize;
        let mut node_pool = Vec::with_capacity(node_count);
        for _ in 0..node_count {
            node_pool.push(TpkUnityNode::read(&mut r)?);
        }

        // StringBuffer
        let string_count = r.read_i32("stringCount")? as usize;
        let mut string_pool = Vec::with_capacity(string_count);
        for _ in 0..string_count {
            string_pool.push(r.read_7bit_string("string")?);
        }

        Ok(Self {
            versions,
            classes,
            string_pool,
            node_pool,
        })
    }
}

/// 一个 classID 的所有版本变体。
pub struct TpkClassInformation {
    pub id: i32,
    /// 按版本升序排列；每段表示"从该版本起，直到下一段开始之前"的布局。
    /// 若 `class` 为 `None`，表示该版本区间内 class 不存在。
    pub classes: Vec<VersionedClass>,
}

/// `TpkClassInformation.classes[i]`，`(起始版本, 该版本的布局)`。
pub struct VersionedClass {
    pub version: UnityVersion,
    pub class: Option<TpkUnityClass>,
}

/// 一个 class 在某版本区间的元数据（不含节点本身，只存索引）。
pub struct TpkUnityClass {
    /// class 名在 [`TpkTypeTree::string_pool`] 的索引。
    pub name_index: u16,
    /// 基类名在 string_pool 的索引（`None` 表示无基类）。
    pub base_index: Option<u16>,
    pub flags: TpkUnityClassFlags,
    /// Editor root node 在 [`TpkTypeTree::node_pool`] 的索引（若无则 `None`）。
    pub editor_root: Option<u16>,
    /// Release root node 在 node_pool 的索引（若无则 `None`）。
    pub release_root: Option<u16>,
}

impl TpkUnityClass {
    fn read(r: &mut LeReader) -> Result<Self, TpkError> {
        let name_index = r.read_u16("class.nameIndex")?;
        let base_index_raw = r.read_u16("class.baseIndex")?;
        let base_index = (base_index_raw != u16::MAX).then_some(base_index_raw);
        let flags_raw = r.read_u8("class.flags")?;
        let flags = TpkUnityClassFlags::from_bits_truncate(flags_raw);

        // 仅当 flags 标记 has_editor_root / has_release_root 时才读对应字段。
        // （参考 AssetRipper TpkUnityClass.cs:36-37）
        let editor_root = if flags.has_editor_root() {
            let idx = r.read_u16("class.editorRoot")?;
            (idx != u16::MAX).then_some(idx)
        } else {
            None
        };
        let release_root = if flags.has_release_root() {
            let idx = r.read_u16("class.releaseRoot")?;
            (idx != u16::MAX).then_some(idx)
        } else {
            None
        };

        Ok(Self {
            name_index,
            base_index,
            flags,
            editor_root,
            release_root,
        })
    }
}

/// class 的 8 个 bool 打包成 1 字节 flags。
///
/// 参考 AssetRipper `TpkUnityClassFlags.cs:9-41`。
/// bit 1 = IsAbstract, 2 = IsSealed, 4 = IsEditorOnly, 8 = IsReleaseOnly,
/// 16 = IsStripped, 32 = Reserved, 64 = HasEditorRootNode, 128 = HasReleaseRootNode。
#[derive(Debug, Clone, Copy, Default)]
pub struct TpkUnityClassFlags(pub u8);

impl TpkUnityClassFlags {
    pub fn from_bits_truncate(b: u8) -> Self {
        Self(b)
    }
    pub fn is_abstract(self) -> bool {
        self.0 & 0x01 != 0
    }
    pub fn is_sealed(self) -> bool {
        self.0 & 0x02 != 0
    }
    pub fn is_editor_only(self) -> bool {
        self.0 & 0x04 != 0
    }
    pub fn is_release_only(self) -> bool {
        self.0 & 0x08 != 0
    }
    pub fn is_stripped(self) -> bool {
        self.0 & 0x10 != 0
    }
    pub fn has_editor_root(self) -> bool {
        self.0 & 0x40 != 0
    }
    pub fn has_release_root(self) -> bool {
        self.0 & 0x80 != 0
    }
}

/// TypeTree 节点池里的一个节点。
///
/// 字符串和子节点都用索引表达，不内嵌。
/// `Level` / `Index` 不存（解码时重算，参考 `TpkUnityNode.cs:7,9` 的 "redundant" 注释）。
pub struct TpkUnityNode {
    /// 类型名在 string_pool 的索引。
    pub type_name_index: u16,
    /// 字段名在 string_pool 的索引。
    pub name_index: u16,
    /// 该字段在磁盘上的字节大小（可能负或对齐不准）。
    pub byte_size: i32,
    /// 节点 schema 版本（不是 Unity 版本）。
    pub node_version: i16,
    /// 原始 type flags，bit0 = "Array/TypelessData"。
    pub type_flags: u8,
    /// 原始 meta flag，位域（0x4000 = AlignBytes, 0x8000 = AnyChildUsesAlignBytes, ...）。
    pub meta_flag: u32,
    /// 子节点在 node_pool 的索引列表。
    pub sub_nodes: Vec<u16>,
}

impl TpkUnityNode {
    fn read(r: &mut LeReader) -> Result<Self, TpkError> {
        let type_name_index = r.read_u16("node.typeNameIndex")?;
        let name_index = r.read_u16("node.nameIndex")?;
        let byte_size = r.read_i32("node.byteSize")?;
        let node_version = r.read_i16("node.version")?;
        let type_flags = r.read_u8("node.typeFlags")?;
        let meta_flag = r.read_u32("node.metaFlag")?;

        let sub_count = r.read_u16("node.subCount")? as usize;
        let mut sub_nodes = Vec::with_capacity(sub_count);
        for _ in 0..sub_count {
            sub_nodes.push(r.read_u16("node.subIndex")?);
        }

        Ok(Self {
            type_name_index,
            name_index,
            byte_size,
            node_version,
            type_flags,
            meta_flag,
            sub_nodes,
        })
    }
}

// ────────────────────────────────────────────────────────────────────
// UnityVersion：8 字节小端 u64 拆位
// ────────────────────────────────────────────────────────────────────

/// Unity 版本号，从 TPK 的 8 字节小端 u64 拆位得到。
///
/// 参考 AssetRipper `UnityVersion.cs:8-14`：
/// ```text
/// bits 48..63 → Major   (u16)
/// bits 32..47 → Minor   (u16)
/// bits 16..31 → Build   (u16)
/// bits  8..15 → Type    (u8,  UnityVersionType enum)
/// bits  0..7  → TypeNumber (u8)
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UnityVersion {
    pub major: u16,
    pub minor: u16,
    pub build: u16,
    pub version_type: UnityVersionType,
    pub type_number: u8,
}

impl UnityVersion {
    pub fn from_bits(bits: u64) -> Self {
        Self {
            major: ((bits >> 48) & 0xFFFF) as u16,
            minor: ((bits >> 32) & 0xFFFF) as u16,
            build: ((bits >> 16) & 0xFFFF) as u16,
            version_type: UnityVersionType::from_u8(((bits >> 8) & 0xFF) as u8),
            type_number: (bits & 0xFF) as u8,
        }
    }

    /// 比较时只看 major.minor.build，忽略 type/typenumber（与 AssetRipper Pass000 折叠策略一致）。
    pub fn major_minor_build(&self) -> (u16, u16, u16) {
        (self.major, self.minor, self.build)
    }
}

impl PartialOrd for UnityVersion {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for UnityVersion {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.major_minor_build().cmp(&other.major_minor_build())
    }
}

impl std::fmt::Display for UnityVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self.version_type {
            UnityVersionType::Alpha => 'a',
            UnityVersionType::Beta => 'b',
            UnityVersionType::China => 'c',
            UnityVersionType::Final => 'f',
            UnityVersionType::Patch => 'p',
            UnityVersionType::Experimental => 'x',
        };
        write!(
            f,
            "{}.{}.{}{}{}",
            self.major, self.minor, self.build, c, self.type_number
        )
    }
}

/// Unity 版本类型枚举。参考 AssetRipper `UnityVersionType.cs:6-41`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnityVersionType {
    Alpha,       // 'a'
    Beta,        // 'b'
    China,       // 'c'
    Final,       // 'f'
    Patch,       // 'p'
    Experimental,// 'x'
}

impl UnityVersionType {
    pub fn from_u8(b: u8) -> Self {
        match b {
            0 => Self::Alpha,
            1 => Self::Beta,
            2 => Self::China,
            3 => Self::Final,
            4 => Self::Patch,
            5 => Self::Experimental,
            _ => {
                log::warn!("未知的 UnityVersionType 值 {b}, 默认当作 Final");
                Self::Final
            }
        }
    }
}

// ────────────────────────────────────────────────────────────────────
// 错误
// ────────────────────────────────────────────────────────────────────

#[derive(Debug, thiserror::Error)]
pub enum TpkError {
    #[error("读取 {path} 失败: {source}")]
    Io {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("TPK magic 不对：期望 0x2A4B5054 ('TPK*')，实际 0x{got:08X}")]
    BadMagic { got: u32 },

    #[error("不支持的 TPK 版本：期望 1，实际 {got}")]
    UnsupportedVersion { got: u8 },

    #[error("不支持的 dataType：期望 0 (TypeTree)，实际 {got}")]
    UnsupportedDataType { got: u8 },

    #[error("不支持的 compressionType：{got}")]
    UnsupportedCompression { got: u8 },

    #[error("数据截断：{reason}（剩余 {remaining} 字节，需要 {needed} 字节）")]
    Truncated {
        reason: &'static str,
        remaining: usize,
        needed: usize,
    },

    #[error("LZ4 解压失败：{0}")]
    Lz4(String),

    #[error("LZMA 解压失败：{0}")]
    Lzma(String),

    #[error("Brotli 解压失败：{0}")]
    Brotli(String),

    #[error("UTF-8 解码失败：{0}")]
    Utf8(#[from] std::str::Utf8Error),
}
