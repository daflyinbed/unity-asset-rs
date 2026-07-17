//! UnityWeb / UnityRaw（旧版 bundle 容器）解析。
//!
//! 对齐 AssetRipper `RawWebBundleFile<THeader>` 泛型基类——两种格式共用同一个 header
//! 结构和 directory/node 逻辑，唯一区别在数据段：
//!
//! - **UnityRaw**：数据不压缩，directory + data 紧跟 header 后（裸读）。
//! - **UnityWeb**：整个 metadata + data blob 被打包成一个 LZMA 压缩流（`scenes[]`
//!   的最后一个 chunk），解压后从中读 directory + 按 node offset 取 data。
//!
//! 全程大端（与 UnityFS 一致）。参考 `AssetRipper.IO.Files/BundleFiles/RawWeb/`。

use bytes::Bytes;

use crate::bundle::{Bundle, RawEntry};
use crate::cursor::{Cursor, Endian};
use crate::decompress::Registry;
use crate::error::{DecompressSnafu, Result, TruncatedSnafu};
use crate::format::FileFormat;

/// BundleVersion 门槛值（对齐 AssetRipper `BundleVersion` 枚举）。
const BF_260_340: u32 = 2; // HasCompleteFileSize
const BF_350_4X: u32 = 3; // HasUncompressedBlocksInfoSize
const BF_520A1: u32 = 4; // HasHash

/// 解析 UnityWeb / UnityRaw bundle。
///
/// `fmt` 区分 WebBundle / RawBundle（由 `sniff` 判定）。产出 `Bundle`（与 UnityFS
/// 的 `parse_unityfs` 同构），供递归驱动器统一处理。
pub fn parse_rawweb(src: Bytes, fmt: FileFormat, _registry: &Registry) -> Result<Bundle> {
    let total = src.len();
    let mut c = Cursor::new(&src, Endian::Big);

    // ── Header（BundleHeader + RawWebBundleHeader）──
    let signature = c.read_cstr("signature")?;
    let version = c.read_u32("bundle version")?;
    let _unity_version = c.read_cstr("unity version")?;
    let unity_revision = c.read_cstr("unity revision")?;

    // v >= 4 (5.2.0+): Hash128 + Crc
    if version >= BF_520A1 {
        c.take(16, "hash")?; // 16 字节 hash，忽略
        let _crc = c.read_u32("crc")?;
    }

    let _minimum_streamed_bytes = c.read_u32("minimum streamed bytes")?;
    let _header_size = c.read_i32("header size")?;
    let _num_scenes_to_download = c.read_i32("number of scenes to download before streaming")?;

    // Scenes[]: count + [u32 compressed_size; u32 decompressed_size] × count
    let scene_count = c.read_i32("scene count")? as usize;
    let mut scenes = Vec::with_capacity(scene_count);
    for i in 0..scene_count {
        let compressed_size = c.read_u32("scene compressed size")?;
        let decompressed_size = c.read_u32("scene decompressed size")?;
        log::debug!(
            "rawweb: scene[{i}]: compressed={compressed_size} decompressed={decompressed_size}"
        );
        scenes.push((compressed_size as usize, decompressed_size as usize));
    }

    // v >= 2 (2.6.0+): CompleteFileSize
    if version >= BF_260_340 {
        let _complete_file_size = c.read_u32("complete file size")?;
    }
    // v >= 3 (3.5.0+): UncompressedBlocksInfoSize
    let uncompressed_blocks_info_size = if version >= BF_350_4X {
        c.read_u32("uncompressed blocks info size")? as usize
    } else {
        0
    };

    // header 末尾对齐到 4
    c.align(4)?;

    log::debug!(
        "rawweb: signature={signature} version={version} unity_revision={unity_revision} \
         scenes={} uncompressed_blocks_info_size={uncompressed_blocks_info_size}",
        scenes.len()
    );

    // ── 数据段 ──
    let entries = match fmt {
        FileFormat::RawBundle => {
            // UnityRaw：directory + data 紧跟 header（裸读，不压缩）。
            let metadata_offset = c.pos();
            read_directory_and_data(
                &src,
                metadata_offset,
                metadata_offset,
                uncompressed_blocks_info_size,
            )?
        }
        FileFormat::WebBundle => {
            // UnityWeb：整个 metadata + data blob 是 scenes[last] 的 LZMA 压缩流。
            let last = scenes.last().ok_or_else(|| {
                TruncatedSnafu {
                    reason: "UnityWeb: scenes 数组为空".to_string(),
                }
                .build()
            })?;
            let (compressed_size, decompressed_size) = *last;

            let comp_start = c.pos();
            if comp_start + compressed_size > total {
                return Err(TruncatedSnafu {
                    reason: "UnityWeb: LZMA chunk 越界".to_string(),
                }
                .build());
            }
            let comp_bytes = &src[comp_start..comp_start + compressed_size];
            let decompressed = decompress_lzma_size_stream(comp_bytes, decompressed_size)?;

            // 解压后的流：directory 在开头，data 紧随。metadata_offset = 0。
            let data_bytes = Bytes::from(decompressed);
            read_directory_and_data(
                &data_bytes,
                0,
                0,
                uncompressed_blocks_info_size,
            )?
        }
        _ => unreachable!("parse_rawweb 只处理 WebBundle / RawBundle"),
    };

    Ok(Bundle {
        unity_revision,
        format_version: version,
        entries,
    })
}

/// 从 `data`（解压后的或原始的字节流）读取 directory（node 列表）+ 按 node 取 data。
///
/// - `data`：directory + data 所在的字节流。
/// - `directory_offset`：directory 在 `data` 中的偏移（UnityRaw = header 后；
///   UnityWeb = 0）。
/// - `data_base`：node.offset 的基准（与 directory_offset 相同）。
/// - `metadata_size`：directory 区域的声明大小（v >= 3 才有；0 表示不校验）。
fn read_directory_and_data(
    data: &[u8],
    directory_offset: usize,
    data_base: usize,
    metadata_size: usize,
) -> Result<Vec<RawEntry>> {
    let total = data.len();
    if directory_offset >= total {
        return Err(TruncatedSnafu {
            reason: format!("rawweb: directory 偏移 {directory_offset} 越界（total={total}）"),
        }
        .build());
    }

    let mut c = Cursor::new(&data[directory_offset..], Endian::Big);
    let metadata_start = c.pos();

    // DirectoryInfo: count + [cstr path; i32 offset; i32 size] × count
    let node_count = c.read_i32("node count")? as usize;
    let mut nodes = Vec::with_capacity(node_count);
    for i in 0..node_count {
        let path = c.read_cstr("node path")?;
        let offset = c.read_i32("node offset")? as i64;
        let size = c.read_i32("node size")? as i64;
        log::debug!("rawweb: node[{i}]: path={path:?} offset={offset} size={size}");
        nodes.push((path, offset, size));
    }
    // directory 末尾对齐到 4
    c.align(4)?;

    if metadata_size > 0 {
        let actual = c.pos() - metadata_start;
        if actual != metadata_size {
            log::warn!(
                "rawweb: directory 实际大小 {actual} != 声明 metadata_size {metadata_size}"
            );
        }
    }

    // 按 node 取 data
    let mut entries = Vec::with_capacity(nodes.len());
    for (path, offset, size) in nodes {
        let off = data_base + offset as usize;
        let sz = size as usize;
        if off + sz > total {
            return Err(TruncatedSnafu {
                reason: format!("rawweb: node {path:?} 切片越界 (off={off} sz={sz} total={total})"),
            }
            .build());
        }
        entries.push(RawEntry {
            name: path,
            bytes: Bytes::copy_from_slice(&data[off..off + sz]),
            flags: 0,
        });
    }

    Ok(entries)
}

/// UnityWeb 的 LZMA 解压变体（`DecompressLzmaSizeStream`）。
///
/// 与 UnityFS 的 LZMA（`DecompressLzmaStream`，5 字节 props + 无 unpacked size）不同：
/// UnityWeb chunk 布局为 `[5 字节 props][8 字节 LE unpacked size][LZMA payload]`。
///
/// 对齐 AssetRipper `LzmaCompression.DecompressLzmaSizeStream`。
fn decompress_lzma_size_stream(input: &[u8], expected: usize) -> Result<Vec<u8>> {
    const PROPS_SIZE: usize = 5;
    const SIZE_FIELD: usize = 8;
    const HEADER: usize = PROPS_SIZE + SIZE_FIELD; // 13

    if input.len() < HEADER {
        return Err(DecompressSnafu {
            reason: format!(
                "UnityWeb LZMA: 输入过短（{} 字节 < 13 字节 header）",
                input.len()
            ),
        }
        .build());
    }

    // 5 字节 props + 8 字节 LE unpacked size + payload
    let props = &input[..PROPS_SIZE];
    let size_bytes = &input[PROPS_SIZE..HEADER];
    let declared_size = u64::from_le_bytes(size_bytes.try_into().unwrap());
    let payload = &input[HEADER..];

    log::debug!(
        "UnityWeb LZMA: props={} declared_size={declared_size} expected={expected} payload={} bytes",
        props.len(),
        payload.len()
    );

    // 优先用流内声明的 size（对齐 AssetRipper 行为），回退到 expected。
    let out_size = if declared_size > 0 {
        declared_size as usize
    } else {
        expected
    };

    let mut out = Vec::with_capacity(out_size);
    // lzma-rs 需要完整的 properties+payload 作为输入流。
    // 重建：props(5) + payload（不含 8 字节 size 字段）。
    let mut stream_input = Vec::with_capacity(PROPS_SIZE + payload.len());
    stream_input.extend_from_slice(props);
    stream_input.extend_from_slice(payload);

    lzma_rs::lzma_decompress_with_options(
        &mut std::io::Cursor::new(stream_input),
        &mut out,
        &lzma_rs::decompress::Options {
            unpacked_size: lzma_rs::decompress::UnpackedSize::UseProvided(Some(out_size as u64)),
            allow_incomplete: false,
            memlimit: None,
        },
    )
    .map_err(|e| DecompressSnafu {
        reason: format!("UnityWeb LZMA 解压失败: {e}"),
    }
    .build())?;

    Ok(out)
}
