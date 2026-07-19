//! TPK payload 解压。支持 None / LZ4 / LZMA / Brotli。
//!
//! 参考 AssetRipper `Tpk/Compression/*`：
//! - LZ4 是 **block 编码**（非 frame），调用方必须提供输出大小。
//! - LZMA 的前 5 字节是 properties，其后才是压缩流；同样需要 decompressedSize。
//! - Brotli 用标准 brotli stream，不需要 decompressedSize。

use std::io::Read;

use super::TpkError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionType {
    None = 0,
    Lz4 = 1,
    Lzma = 2,
    Brotli = 3,
}

impl CompressionType {
    pub fn from_u8(b: u8) -> Result<Self, TpkError> {
        match b {
            0 => Ok(Self::None),
            1 => Ok(Self::Lz4),
            2 => Ok(Self::Lzma),
            3 => Ok(Self::Brotli),
            _ => Err(TpkError::UnsupportedCompression { got: b }),
        }
    }
}

pub fn decompress(ct: CompressionType, input: &[u8], expected: usize) -> Result<Vec<u8>, TpkError> {
    match ct {
        CompressionType::None => {
            if expected != 0 && input.len() != expected {
                log::warn!(
                    "None 压缩：输入 {} 字节但 declared_size={expected}",
                    input.len()
                );
            }
            Ok(input.to_vec())
        }
        CompressionType::Lz4 => decompress_lz4(input, expected),
        CompressionType::Lzma => decompress_lzma(input, expected),
        CompressionType::Brotli => decompress_brotli(input),
    }
}

fn decompress_lz4(input: &[u8], expected: usize) -> Result<Vec<u8>, TpkError> {
    let mut out = vec![0u8; expected];
    // lz4_flex::block::decompress_into 返回解压后的字节数（应 == expected）
    let written = lz4_flex::block::decompress_into(input, &mut out)
        .map_err(|e| TpkError::Lz4(e.to_string()))?;
    if written != expected {
        log::warn!("LZ4 解压字节数 {written} 与 declared_size {expected} 不一致");
        out.truncate(written);
    }
    Ok(out)
}

fn decompress_lzma(input: &[u8], expected: usize) -> Result<Vec<u8>, TpkError> {
    // LZMA stream: 前 5 字节是 properties（1 byte lc/lp/pb + 4 byte dict size, little-endian）
    // 后面才是压缩数据。lzma-rs 的 decompress_with_options 处理这个格式。
    // 参考 AssetRipper LzmaHandler.cs:24-28 + SharpCompress LzmaStream。
    if input.len() < 5 {
        return Err(TpkError::Lzma(format!(
            "LZMA 输入过短（{} 字节 < 5 字节 properties）",
            input.len()
        )));
    }
    let mut out = Vec::with_capacity(expected);
    lzma_rs::lzma_decompress_with_options(
        &mut std::io::Cursor::new(input),
        &mut out,
        &lzma_rs::decompress::Options {
            unpacked_size: lzma_rs::decompress::UnpackedSize::UseProvided(Some(expected as u64)),
            allow_incomplete: false,
            memlimit: None,
        },
    )
    .map_err(|e| TpkError::Lzma(e.to_string()))?;
    Ok(out)
}

fn decompress_brotli(input: &[u8]) -> Result<Vec<u8>, TpkError> {
    let mut decoder = brotli::Decompressor::new(input, 4096);
    let mut out = Vec::new();
    decoder
        .read_to_end(&mut out)
        .map_err(|e| TpkError::Brotli(e.to_string()))?;
    Ok(out)
}
