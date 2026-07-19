use std::collections::HashMap;
use std::sync::Arc;

use crate::error::{DecompressSnafu, Lz4SizeMismatchSnafu, Result, UnsupportedCompressionSnafu};

/// 压缩类型。低 6 位 flag 的编码。
///
/// 开放枚举：用户可通过 `Registry::register` 注册自定义槽位（如明日方舟私改 LZ4 占 flag 4/5）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CompressionType(pub u16);

impl CompressionType {
    pub const NONE: Self = Self(0);
    pub const LZMA: Self = Self(1);
    pub const LZ4: Self = Self(2);
    pub const LZ4HC: Self = Self(3);
}

/// 单个解压器 codec，无状态。
///
/// `expected` 为解压后大小提示：LZ4 必需；NONE 忽略。
pub trait Decompressor: Send + Sync {
    fn decompress(&self, input: &[u8], expected: usize) -> Result<Vec<u8>>;
}

/// 一个什么也不做的解压器（compression = NONE）。
pub struct IdentityDecompressor;

impl Decompressor for IdentityDecompressor {
    fn decompress(&self, input: &[u8], _expected: usize) -> Result<Vec<u8>> {
        Ok(input.to_vec())
    }
}

/// LZ4 / LZ4HC 共用解压器。LZ4HC 仅编码侧不同，解码路径相同。
pub struct Lz4Decompressor;

impl Decompressor for Lz4Decompressor {
    fn decompress(&self, input: &[u8], expected: usize) -> Result<Vec<u8>> {
        lz4_flex::block::decompress(input, expected)
            .map_err(|e| {
                DecompressSnafu {
                    reason: e.to_string(),
                }
                .build()
            })
            .and_then(|out| {
                if out.len() != expected {
                    return Err(Lz4SizeMismatchSnafu {
                        expected,
                        actual: out.len(),
                    }
                    .build());
                }
                Ok(out)
            })
    }
}

/// LZMA 解压器。Unity bundle 的 LZMA 块与标准 `.lzma` 不同：
/// 前 5 字节是 properties（1 byte lc/lp/pb + 4 byte dict size，LE），其后是
/// 压缩流，**流内不含 8 字节 unpacked-size 字段**，必须由调用方提供。
///
/// 故使用 `lzma_decompress_with_options` + `UnpackedSize::UseProvided(expected)`，
/// 与 `crates/codegen/src/tpk/compression.rs::decompress_lzma` 同源
/// （参考 AssetRipper `LzmaHandler.cs:24-28` + SharpCompress `LzmaStream`）。
pub struct LzmaDecompressor;

impl Decompressor for LzmaDecompressor {
    fn decompress(&self, input: &[u8], expected: usize) -> Result<Vec<u8>> {
        if input.len() < 5 {
            return Err(DecompressSnafu {
                reason: format!("LZMA 输入过短（{} 字节 < 5 字节 properties）", input.len()),
            }
            .build());
        }
        let mut out = Vec::with_capacity(expected);
        lzma_rs::lzma_decompress_with_options(
            &mut std::io::Cursor::new(input),
            &mut out,
            &lzma_rs::decompress::Options {
                unpacked_size: lzma_rs::decompress::UnpackedSize::UseProvided(Some(
                    expected as u64,
                )),
                allow_incomplete: false,
                memlimit: None,
            },
        )
        .map_err(|e| {
            DecompressSnafu {
                reason: e.to_string(),
            }
            .build()
        })?;
        Ok(out)
    }
}

/// 解压器注册表。按 `CompressionType` 键控，可覆盖任意槽位。
#[derive(Clone)]
pub struct Registry {
    entries: HashMap<CompressionType, Arc<dyn Decompressor>>,
}

impl Registry {
    /// 空注册表。
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    /// 默认注册 LZMA + LZ4 + LZ4HC + NONE（即 UnityFS 的 flag 0-3）。
    pub fn defaults() -> Self {
        let mut r = Self::new();
        let lz4: Arc<dyn Decompressor> = Arc::new(Lz4Decompressor);
        r.register(CompressionType::LZMA, Arc::new(LzmaDecompressor));
        r.register(CompressionType::LZ4, lz4.clone());
        r.register(CompressionType::LZ4HC, lz4);
        r.register(CompressionType::NONE, Arc::new(IdentityDecompressor));
        r
    }

    /// 注册（或覆盖）一个槽位。
    pub fn register(
        &mut self,
        ct: CompressionType,
        d: Arc<dyn Decompressor>,
    ) -> Option<Arc<dyn Decompressor>> {
        self.entries.insert(ct, d)
    }

    /// 取已注册的解压器；未注册返回 `UnsupportedCompression` 错误。
    pub fn get(&self, ct: CompressionType) -> Result<&dyn Decompressor> {
        self.entries.get(&ct).map(|d| d.as_ref()).ok_or_else(|| {
            log::warn!("compression type {:?} not registered", ct);
            UnsupportedCompressionSnafu { compression: ct.0 }.build()
        })
    }
}

impl Default for Registry {
    fn default() -> Self {
        Self::defaults()
    }
}
