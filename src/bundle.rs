use bytes::Bytes;

use crate::decompress::{CompressionType, Registry};
use crate::error::{
    BadSignatureSnafu, DecompressSnafu, Result, TruncatedSnafu, UnsupportedCompressionSnafu,
};

/// archive flag：低 6 位 = blocks-info 压缩类型。
const ARCHIVE_COMPRESSION_MASK: u32 = 0x3F;
/// 0x80：blocks-info 压缩块位于文件末尾。
const ARCHIVE_BLOCKS_INFO_AT_THE_END: u32 = 0x80;
/// 0x200：读 block 数据前需对齐到 16 字节。
const ARCHIVE_BLOCK_INFO_NEED_PADDING_AT_START: u32 = 0x200;

/// 单个 StorageBlock（压缩数据块描述符）。
#[derive(Debug, Clone)]
struct StorageBlock {
    uncompressed_size: u32,
    compressed_size: u32,
    flags: u16,
}

impl StorageBlock {
    fn compression(&self) -> CompressionType {
        CompressionType(self.flags & 0x3F)
    }
}

/// 目录节点：描述解压后拼接 blob 中的一段（对应一个内部文件）。
#[derive(Debug, Clone)]
struct Node {
    offset: i64,
    size: i64,
    flags: u32,
    path: String,
}

/// UnityFS 解析结果（内部文件条目）。
pub struct Bundle {
    /// 引擎版本（如 "2017.4.39f1"），供后续 SerializedFile 继承。
    pub unity_revision: String,
    /// bundle 版本（format，如 6/7/8）。
    pub format_version: u32,
    /// 解出的内部文件（.assets / .resS 等），字节已解压。
    pub files: Vec<BundleFile>,
}

pub struct BundleFile {
    pub name: String,
    pub bytes: Bytes,
    pub flags: u32,
}

/// 大端游标读取器（带边界检查）。
struct Cursor<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> Cursor<'a> {
    fn new(buf: &'a [u8]) -> Self {
        Self { buf, pos: 0 }
    }

    fn remaining(&self) -> usize {
        self.buf.len() - self.pos
    }

    fn need(&self, n: usize, what: &'static str) -> Result<()> {
        if self.remaining() < n {
            return Err(TruncatedSnafu { reason: what }.build());
        }
        Ok(())
    }

    fn take(&mut self, n: usize, what: &'static str) -> Result<&'a [u8]> {
        self.need(n, what)?;
        let s = &self.buf[self.pos..self.pos + n];
        self.pos += n;
        Ok(s)
    }

    fn read_u16(&mut self, what: &'static str) -> Result<u16> {
        Ok(u16::from_be_bytes(self.take(2, what)?.try_into().unwrap()))
    }

    fn read_u32(&mut self, what: &'static str) -> Result<u32> {
        Ok(u32::from_be_bytes(self.take(4, what)?.try_into().unwrap()))
    }

    fn read_i64(&mut self, what: &'static str) -> Result<i64> {
        Ok(i64::from_be_bytes(self.take(8, what)?.try_into().unwrap()))
    }

    /// 读到第一个 0x00 为止（不含终止符）。
    fn read_cstr(&mut self, what: &'static str) -> Result<String> {
        let start = self.pos;
        let end = self.buf[start..]
            .iter()
            .position(|&b| b == 0)
            .ok_or_else(|| TruncatedSnafu { reason: what }.build())?;
        let s = std::str::from_utf8(&self.buf[start..start + end])
            .map_err(|e| DecompressSnafu { reason: format!("非 UTF-8 字符串: {e}") }.build())?
            .to_owned();
        self.pos = start + end + 1;
        Ok(s)
    }
}

fn align_up(pos: usize, alignment: usize) -> usize {
    let r = pos % alignment;
    if r == 0 {
        pos
    } else {
        pos + (alignment - r)
    }
}

/// 解析一个 UnityFS bundle。
///
/// `registry` 决定如何解压各压缩块；默认用 [`Registry::defaults`]。
pub fn parse(src: Bytes, registry: &Registry) -> Result<Bundle> {
    let total = src.len();

    let mut c = Cursor::new(&src);

    let signature = c.read_cstr("signature")?;
    if signature != "UnityFS" {
        return Err(BadSignatureSnafu { sig: signature }.build());
    }
    let format_version = c.read_u32("format version")?;
    if format_version < 6 {
        return Err(UnsupportedCompressionSnafu {
            compression: format_version as u16,
        }
        .build());
    }
    // unityVersion（player 版本）—— MVP 不使用，但需消费。
    let _unity_version = c.read_cstr("unity version")?;
    let unity_revision = c.read_cstr("unity revision")?;

    let _declared_size = c.read_i64("size")?;
    let compressed_blocks_info_size = c.read_u32("compressed blocks info size")? as usize;
    let uncompressed_blocks_info_size = c.read_u32("uncompressed blocks info size")? as usize;
    let archive_flags = c.read_u32("archive flags")?;

    let header_end = c.pos;

    log::debug!(
        "header: signature={signature} format_version={format_version} \
         unity_version={_unity_version} unity_revision={unity_revision} \
         declared_size={_declared_size} \
         compressed_blocks_info_size={compressed_blocks_info_size} \
         uncompressed_blocks_info_size={uncompressed_blocks_info_size} \
         archive_flags=0x{archive_flags:08x} (compression={} at_end={} pad={})",
        (archive_flags & ARCHIVE_COMPRESSION_MASK),
        archive_flags & ARCHIVE_BLOCKS_INFO_AT_THE_END != 0,
        archive_flags & ARCHIVE_BLOCK_INFO_NEED_PADDING_AT_START != 0,
    );

    // version >= 7：header 后对齐到 16。
    let blocks_info_pos = if format_version >= 7 {
        align_up(header_end, 16)
    } else {
        header_end
    };

    // 定位压缩的 blocks-info。
    let bi_start = if archive_flags & ARCHIVE_BLOCKS_INFO_AT_THE_END != 0 {
        total
            .checked_sub(compressed_blocks_info_size)
            .ok_or_else(|| {
                TruncatedSnafu {
                    reason: "blocks-info 末尾偏移越界",
                }
                .build()
            })?
    } else {
        blocks_info_pos
    };

    if bi_start + compressed_blocks_info_size > total {
        return Err(TruncatedSnafu {
            reason: "blocks-info 压缩块越界",
        }
        .build());
    }
    let bi_compressed = &src[bi_start..bi_start + compressed_blocks_info_size];

    // blocks-info 自身按 archive flags 低 6 位解压。
    let bi_ct = CompressionType((archive_flags & ARCHIVE_COMPRESSION_MASK) as u16);
    log::debug!(
        "blocks-info: at=0x{bi_start:x} compressed={compressed_blocks_info_size} \
         uncompressed={uncompressed_blocks_info_size} ct={:?}",
        bi_ct
    );
    let bi_bytes = registry
        .get(bi_ct)?
        .decompress(bi_compressed, uncompressed_blocks_info_size)?;

    let mut bi = Cursor::new(&bi_bytes);
    bi.take(16, "uncompressed data hash")?; // 16 字节 hash，忽略

    let block_count = bi.read_u32("block count")?;
    let mut blocks = Vec::with_capacity(block_count as usize);
    for i in 0..block_count {
        let uncompressed_size = bi.read_u32("block uncompressed size")?;
        let compressed_size = bi.read_u32("block compressed size")?;
        let flags = bi.read_u16("block flags")?;
        log::debug!(
            "  block[{i}]: uncompressed={uncompressed_size} compressed={compressed_size} \
             flags=0x{flags:04x} (compression={})",
            flags & 0x3F
        );
        blocks.push(StorageBlock {
            uncompressed_size,
            compressed_size,
            flags,
        });
    }

    let node_count = bi.read_u32("node count")?;
    let mut nodes = Vec::with_capacity(node_count as usize);
    for i in 0..node_count {
        let offset = bi.read_i64("node offset")?;
        let size = bi.read_i64("node size")?;
        let flags = bi.read_u32("node flags")?;
        let path = bi.read_cstr("node path")?;
        log::debug!(
            "  node[{i}]: offset={offset} size={size} flags=0x{flags:08x} path={path:?}"
        );
        nodes.push(Node {
            offset,
            size,
            flags,
            path,
        });
    }

    // block 数据起始位置：header 后（或末尾定位的 bi_start 之后；若 blocks-info 不在末尾则紧随其后）。
    // 参考 UnityPy/AssetStudio：若 blocks-info 紧随 header，block 数据从 bi_start + 压缩长度开始；
    // 若在末尾，则从 header 后开始。0x200 flag 时对齐到 16。
    let after_header = if archive_flags & ARCHIVE_BLOCKS_INFO_AT_THE_END != 0 {
        blocks_info_pos
    } else {
        bi_start + compressed_blocks_info_size
    };
    let block_data_start = if archive_flags & ARCHIVE_BLOCK_INFO_NEED_PADDING_AT_START != 0 {
        align_up(after_header, 16)
    } else {
        after_header
    };

    // 逐块解压、拼接。
    let total_uncompressed: u64 = blocks
        .iter()
        .map(|b| b.uncompressed_size as u64)
        .sum();
    log::debug!(
        "block data starts at 0x{block_data_start:x}; {} blocks, total uncompressed={total_uncompressed}",
        blocks.len()
    );
    let mut blob: Vec<u8> = Vec::with_capacity(total_uncompressed as usize);
    let mut data_pos = block_data_start;
    for (i, blk) in blocks.iter().enumerate() {
        let comp_ct = blk.compression();
        let comp_len = blk.compressed_size as usize;
        if data_pos + comp_len > total {
            return Err(TruncatedSnafu {
                reason: "block 数据越界",
            }
            .build());
        }
        let comp_bytes = &src[data_pos..data_pos + comp_len];
        log::trace!(
            "decompressing block[{i}] at file offset 0x{data_pos:x}: \
             {} compressed bytes -> {} uncompressed ({:?})",
            comp_len,
            blk.uncompressed_size,
            comp_ct
        );
        let out = registry.get(comp_ct)?.decompress(comp_bytes, blk.uncompressed_size as usize).map_err(
            |e| DecompressSnafu {
                reason: format!("block {i} 解压失败: {e}"),
            }
            .build(),
        )?;
        blob.extend_from_slice(&out);
        data_pos += comp_len;
    }

    // 按 node 切片。
    let blob_bytes = Bytes::from(blob);
    let mut files = Vec::with_capacity(nodes.len());
    for nd in nodes {
        let off = nd.offset as usize;
        let sz = nd.size as usize;
        if off + sz > blob_bytes.len() {
            return Err(TruncatedSnafu {
                reason: "node 切片越界",
            }
            .build());
        }
        files.push(BundleFile {
            name: nd.path,
            bytes: blob_bytes.slice(off..off + sz),
            flags: nd.flags,
        });
    }

    Ok(Bundle {
        unity_revision,
        format_version,
        files,
    })
}
