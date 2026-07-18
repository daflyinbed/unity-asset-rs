//! UnityWebData（WebGL 数据格式）解析。
//!
//! 对齐 AssetRipper `WebFile`。与 bundle 不同——它是**小端序**，且不是 `BundleHeader`
//! 体系。结构：`"UnityWebData1.0\0"` magic + i32 header_length + entries[]，每个 entry
//! = i32 offset + i32 size + aligned_string(name)。数据按 entry.offset 定位读取。
//!
//! 参考 `AssetRipper.IO.Files/WebFiles/WebFile.cs`。

use bytes::Bytes;

use crate::bundle::RawEntry;
use crate::cursor::{Cursor, Endian};
use crate::error::{Result, TruncatedSnafu};

/// UnityWebData 的 magic（zero-term）。
const MAGIC: &[u8] = b"UnityWebData1.0\0";

/// 解析 UnityWebData，产出原始子条目列表。
///
/// 小端序（与 bundle 的大端不同）。magic 已由 `sniff` 校验，这里不重复检查。
pub fn parse_webfile(src: Bytes) -> Result<Vec<RawEntry>> {
    let total = src.len();
    let mut c = Cursor::new(&src, Endian::Little);

    // magic（zero-term，含 \0）
    let _signature = c.read_cstr("webfile signature")?;
    debug_assert!(_src_signature(&src) == MAGIC);

    let base = c.pos();
    let header_length = c.read_i32("header length")? as usize;
    if base + header_length > total {
        return Err(TruncatedSnafu {
            reason: format!(
                "webfile: header_length {header_length} 越界 (base={base} total={total})"
            ),
        }
        .build());
    }

    // 读 entries 直到 position >= base + header_length
    let mut entries_meta = Vec::new();
    while c.pos() < base + header_length {
        let offset = c.read_i32("entry offset")? as usize;
        let size = c.read_i32("entry size")? as usize;
        let name = c.read_aligned_string("entry name")?;
        log::debug!("webfile: entry offset={offset} size={size} name={name:?}");
        entries_meta.push((offset, size, name));
    }

    // 按 entry 取 data（offset 相对于文件起始）
    let mut entries = Vec::with_capacity(entries_meta.len());
    for (offset, size, name) in entries_meta {
        if offset + size > total {
            return Err(TruncatedSnafu {
                reason: format!(
                    "webfile: entry {name:?} 切片越界 (off={offset} sz={size} total={total})"
                ),
            }
            .build());
        }
        entries.push(RawEntry {
            name,
            bytes: src.slice(offset..offset + size),
            flags: 0,
        });
    }

    Ok(entries)
}

/// 取 src 开头的 signature（用于 debug_assert）。
fn _src_signature(src: &[u8]) -> &[u8] {
    &src[..MAGIC.len()]
}
