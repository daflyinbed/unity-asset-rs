//! 明日方舟（Arknights）自定义 LZ4 变体解压。
//!
//! 与标准 LZ4 block 格式相比有两处位级反转（见各函数注释），其余完全一致。
//! 参考实现：外部 `../../lz4inv` crate（PyO3 绑定版本），本 crate 去掉 pyo3 依赖，
//! 保留纯 Rust `decompress` 函数，供主 crate 通过 `Registry` 注册到 flag 4/5。
//!
//! 用法：
//! ```no_run
//! // 在主 crate 注册（src/decompress.rs 的 Registry）：
//! // registry.register(unity_asset_rs::CompressionType(4), Box::new(Lz4Inv));
//! ```

use std::fmt;

#[derive(Debug)]
pub enum DecompressError {
    /// 提供的输出缓冲区不够大
    OutputTooSmall {
        /// 最小期望输出大小
        expected: usize,
        /// 实际输出大小
        actual: usize,
    },
    /// literal 越界（输入不足）
    LiteralOutOfBounds,
    /// 去重 offset 越界（不在已解压缓冲区内）
    OffsetOutOfBounds,
}

impl fmt::Display for DecompressError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DecompressError::OutputTooSmall { expected, actual } => {
                write!(f, "Output too small: expected {expected}, actual {actual}")
            }
            DecompressError::LiteralOutOfBounds => write!(f, "Literal out of bounds"),
            DecompressError::OffsetOutOfBounds => write!(f, "Offset out of bounds"),
        }
    }
}

impl std::error::Error for DecompressError {}

/// 解压明日方舟 LZ4 变体数据。
///
/// `src` 是压缩数据，`dst` 是预分配的输出缓冲区（大小 = 期望解压后大小）。
/// 返回实际写入的字节数。
///
/// # Errors
///
/// 当输出缓冲区不够大、literal 越界或 offset 越界时返回对应的 [`DecompressError`]。
pub fn decompress(src: &[u8], dst: &mut [u8]) -> Result<usize, DecompressError> {
    let mut src_pos = 0;
    let mut dst_pos = 0;

    while src_pos < src.len() && dst_pos < dst.len() {
        // 读 token
        let (mut literal_length, mut match_length) = get_literal_token(src, &mut src_pos);

        // 拷贝 literal 段
        literal_length = get_length(literal_length, src, &mut src_pos);
        if literal_length > src.len() - src_pos {
            return Err(DecompressError::LiteralOutOfBounds);
        }
        if literal_length > dst.len() - dst_pos {
            return Err(DecompressError::OutputTooSmall {
                expected: dst_pos + literal_length,
                actual: dst.len(),
            });
        }

        dst[dst_pos..dst_pos + literal_length]
            .copy_from_slice(&src[src_pos..src_pos + literal_length]);

        src_pos += literal_length;
        dst_pos += literal_length;

        if src_pos >= src.len() {
            break;
        }

        // 拷贝压缩（match）段
        let offset = get_chunk_end(src, &mut src_pos);

        match_length = get_length(match_length, src, &mut src_pos) + 4;

        // 复制的源开始的位置
        let (enc_pos, did_overflow) = dst_pos.overflowing_sub(offset);
        if did_overflow {
            return Err(DecompressError::OffsetOutOfBounds);
        }
        if dst_pos + match_length > dst.len() {
            return Err(DecompressError::OutputTooSmall {
                expected: dst_pos + match_length,
                actual: dst.len(),
            });
        }

        if match_length <= offset {
            dst.copy_within(enc_pos..enc_pos + match_length, dst_pos);
            dst_pos += match_length;
        } else {
            // 重叠拷贝
            let mut match_length_remain = match_length;
            let mut curr_enc_pos = enc_pos;
            let mut curr_dst_pos = dst_pos;

            while match_length_remain > 0 {
                dst[curr_dst_pos] = dst[curr_enc_pos];
                curr_enc_pos += 1;
                curr_dst_pos += 1;
                match_length_remain -= 1;
            }

            dst_pos = curr_dst_pos;
        }
    }

    Ok(dst_pos)
}

// |literal|match|
// |0000|0000|
// 标准 LZ4：高四位 = literal 长度，低四位 = match 长度；
// 明日方舟变体：反过来了——低四位 = literal，高四位 = match。
fn get_literal_token(src: &[u8], src_pos: &mut usize) -> (usize, usize) {
    let token = src[*src_pos];
    *src_pos += 1;
    ((token & 0xf) as usize, ((token >> 4) & 0xf) as usize)
}

// 标准 LZ4 读 little-endian u16；明日方舟变体读 big-endian u16。
fn get_chunk_end(src: &[u8], src_pos: &mut usize) -> usize {
    let high = src[*src_pos] as usize;
    *src_pos += 1;
    let low = src[*src_pos] as usize;
    *src_pos += 1;
    (high << 8) | low
}

// 读长度扩展（当 token 中的长度为 0xf 时，继续读后续字节累加，直到遇到非 0xff）。
fn get_length(mut length: usize, src: &[u8], src_pos: &mut usize) -> usize {
    if length == 0xf {
        let mut sum;
        loop {
            sum = src[*src_pos] as usize;
            *src_pos += 1;
            length += sum;
            if sum != 0xff {
                break;
            }
        }
    }
    length
}
