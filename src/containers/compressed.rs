//! gzip / brotli 外层包裹解压。
//!
//! 这两种是传输层压缩，独立于 bundle 格式——解压后递归驱动器再 sniff 内层格式
//! （可能是 UnityFS / WebFile / SerializedFile 等）。对齐 AssetRipper
//! `CompressedFile` 体系：`CompressedFile.ReadContents` 把解压出的 `ResourceFile`
//! 再次喂回 `SchemeReader.ReadFile`。
//!
//! 参考 `AssetRipper.IO.Files/CompressedFiles/`。

use std::io::Read;

use crate::error::{DecompressSnafu, Result};

/// Unity brotli 流的元数据签名（ASCII）。
const BROTLI_SIGNATURE: &[u8] = b"UnityWeb Compressed Content (brotli)";

/// 解压 gzip 流。
///
/// magic `0x1F 0x8B`。用 `flate2` 解压（标准 gzip stream）。
pub fn decompress_gzip(input: &[u8]) -> Result<Vec<u8>> {
    let mut decoder = flate2::read::GzDecoder::new(input);
    let mut out = Vec::new();
    decoder
        .read_to_end(&mut out)
        .map_err(|e| DecompressSnafu {
            reason: format!("gzip 解压失败: {e}"),
        }
        .build())?;
    log::debug!("gzip: {} bytes -> {} bytes", input.len(), out.len());
    Ok(out)
}

/// 解压 brotli 流。
///
/// 用 `brotli` crate 解压（标准 brotli stream）。magic 校验由 [`looks_like_brotli`]
/// 在 sniff 阶段完成，这里直接解压。
pub fn decompress_brotli(input: &[u8]) -> Result<Vec<u8>> {
    let mut decoder = brotli::Decompressor::new(input, 4096);
    let mut out = Vec::new();
    decoder
        .read_to_end(&mut out)
        .map_err(|e| DecompressSnafu {
            reason: format!("brotli 解压失败: {e}"),
        }
        .build())?;
    log::debug!("brotli: {} bytes -> {} bytes", input.len(), out.len());
    Ok(out)
}

/// 嗅探字节是否为 Unity brotli 流。
///
/// Unity 的 brotli 流以一个 **metadata block** 开头，其内容是固定签名
/// `"UnityWeb Compressed Content (brotli)"`。需要解析 brotli 帧头的变长长度前缀
/// 才能定位到签名字节——不能简单地按字节前缀匹配。
///
/// 对齐 AssetRipper `BrotliFile.IsBrotliFile`：brotli 帧第一字节的低 3 位是
/// `WBITS`（window bits 编码），随后是元数据块长度前缀（变长，1-5 字节），再后是
/// 元数据内容（即签名）。
pub fn looks_like_brotli(bytes: &[u8]) -> bool {
    if bytes.len() < 4 {
        return false;
    }

    // brotli 帧头：第 0 字节低 3 位 = WBITS。
    // 若 WBITS != 0，则 window base 偏移占 1-7 位（取决于 WBITS 值），之后才是
    // meta-block。AssetRipper 的实现简化处理：跳过第 0 字节，从第 1 字节读 3 位
    // 作为 sizeBytes（元数据块长度的变长编码字节数）。
    //
    // 这里直接对齐 AssetRipper 的位运算逻辑（IsBrotliFile）。
    let mut pos = 1; // 跳过第 0 字节
    let bt = bytes[pos];
    pos += 1;
    let size_bytes = (bt & 0x3) as usize;

    if pos + size_bytes > bytes.len() {
        return false;
    }

    // 解码元数据块长度（变长，size_bytes 个字节）
    let mut length: usize = 0;
    let mut cur_bt = bt;
    for i in 0..size_bytes {
        if pos >= bytes.len() {
            return false;
        }
        let nbt = bytes[pos];
        pos += 1;
        let bits = ((cur_bt >> 2) as usize) | (((nbt & 0x3) as usize) << 6);
        cur_bt = nbt;
        length |= bits << (8 * i);
    }

    // 长度必须等于签名长度，且后续字节足够
    if length != BROTLI_SIGNATURE.len() {
        return false;
    }
    if pos + length > bytes.len() {
        return false;
    }

    // 比较签名
    &bytes[pos..pos + length] == BROTLI_SIGNATURE
}
