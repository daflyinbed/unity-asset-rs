//! Texture2D → 图片导出（受 `export` feature 门控）。
//!
//! 现状：支持 ASTC LDR + ETC1/ETC2/EAC（通过 path 依赖 [`texture_decoder_rs`]），输出 PNG。
//! 像素数据来源选择对齐 UnityPy `_Texture2d_get_image_data`：
//! `image_data` 非空用它，否则走 `Environment::streaming_data` 取 `.resS` 流数据。
//!
//! Y 翻转：Unity 纹理原点在左下、PNG 在左上，导出时垂直翻转（对齐 UnityPy 默认
//! `FLIP_TOP_BOTTOM`）。通道顺序：`texture_decoder_rs` 的 ASTC 与 ETC 解码器均输出 RGBA
//! （ASTC 走 RGBA swizzle；ETC `write_pixel` 直接写 `[R,G,B]`+alpha），与 UnityPy 的
//! `texture2ddecoder`(C++) BGRA 输出不同——这里**不做** B/R 交换。
//!
//! # ASTC HDR 未做
//!
//! Unity 的 `ASTC_HDR_*`（format 66-71）暂不支持，遇之报 unsupported。根因：ASTC 的
//! HDR block（CEM 2/3/7/11/14/15）里 alpha 通道语义**不在 16 字节 block 内**，要由
//! 外部 decode profile（LDR / LDR_SRGB / HDR）补全——block 自己无从得知。`texture_decoder_rs`
//! 是 LDR-only：CEM 2/3/7/11 无条件硬编码 alpha=0x780（等价永远假设 HDR profile），
//! 且缺 LDR profile 的 error 色分支，输出也恒为 RGBA8（`floor(f*255)` 把 fp16 量化掉）。
//! 要做真 HDR 需重写整个插值/输出路径（统一 16-bit 域插值 + per-lane mask 分流 LNS/unorm +
//! fp16/fp32 输出 + EXR 编码）。**以后再说。**详见
//! [`../docs/astc-hdr-alpha-handling.md`](../../docs/astc-hdr-alpha-handling.md)。
//!
//! 路线图：BCn/Crunch/PVRTC/ATC + ASTC HDR + Sprite Sheet + 多输出格式。

use std::io::Write;

use bytes::Bytes;
use snafu::Snafu;

use crate::Environment;
use generated::classes::texture2_d::Texture2D;

/// Unity `TextureFormat` 枚举（仅列出已遇到/支持的值）。
///
/// 数字值来自 `UnityPy/enums/TextureFormat.py`（权威对照，bit-for-bit）。
/// ASTC 覆盖全部 6 个方形 footprint（4×4/5×5/6×6/8×8/10×10/12×12）× RGB/RGBA
/// 共 12 个值；RGB 与 RGBA 共用同一解码（ASTC block 头携带 alpha）。HDR ASTC（66-71）
/// 不支持（decoder 为 LDR-only）。
/// ETC1/ETC2/EAC（34/41-47）通过 `texture_decoder_rs` 的 `etc_decoder` 模块解码。
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureFormat {
    // ── ETC1 / ETC2 / EAC（34, 41-47）──
    /// ETC1（ETC_RGB4 = 34）。
    EtcRgb4 = 34,
    /// EAC R11 unsigned（EAC_R = 41）。
    EacR = 41,
    /// EAC R11 signed（EAC_R_SIGNED = 42）。
    EacRSigned = 42,
    /// EAC RG11 unsigned（EAC_RG = 43）。
    EacRg = 43,
    /// EAC RG11 signed（EAC_RG_SIGNED = 44）。
    EacRgSigned = 44,
    /// ETC2 RGB（ETC2_RGB = 45）。
    Etc2Rgb = 45,
    /// ETC2 RGB + 1-bit alpha punch-through（ETC2_RGBA1 = 46）。
    Etc2Rgba1 = 46,
    /// ETC2 RGBA8（ETC2_RGBA8 = 47），color + EAC alpha。
    Etc2Rgba8 = 47,
    // ── ASTC RGB（48-53）──
    AstcRgb4x4 = 48,
    AstcRgb5x5 = 49,
    AstcRgb6x6 = 50,
    AstcRgb8x8 = 51,
    AstcRgb10x10 = 52,
    AstcRgb12x12 = 53,
    // ── ASTC RGBA（54-59；与 RGB 同 footprint 共用解码）──
    AstcRgba4x4 = 54,
    AstcRgba5x5 = 55,
    AstcRgba6x6 = 56,
    AstcRgba8x8 = 57,
    AstcRgba10x10 = 58,
    AstcRgba12x12 = 59,
}

impl TextureFormat {
    /// 从 `Texture2D.m_format`（裸 `i32`）解析。
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            34 => Some(Self::EtcRgb4),
            41 => Some(Self::EacR),
            42 => Some(Self::EacRSigned),
            43 => Some(Self::EacRg),
            44 => Some(Self::EacRgSigned),
            45 => Some(Self::Etc2Rgb),
            46 => Some(Self::Etc2Rgba1),
            47 => Some(Self::Etc2Rgba8),
            48 => Some(Self::AstcRgb4x4),
            49 => Some(Self::AstcRgb5x5),
            50 => Some(Self::AstcRgb6x6),
            51 => Some(Self::AstcRgb8x8),
            52 => Some(Self::AstcRgb10x10),
            53 => Some(Self::AstcRgb12x12),
            54 => Some(Self::AstcRgba4x4),
            55 => Some(Self::AstcRgba5x5),
            56 => Some(Self::AstcRgba6x6),
            57 => Some(Self::AstcRgba8x8),
            58 => Some(Self::AstcRgba10x10),
            59 => Some(Self::AstcRgba12x12),
            _ => None,
        }
    }

    /// 是否为本模块支持的格式（ASTC LDR 或 ETC1/ETC2/EAC）。
    pub fn is_supported(self) -> bool {
        self.astc_block().is_some() || self.is_etc()
    }

    /// 是否为 ETC/EAC 家族（4×4 块，由 `etc_decoder` 处理）。
    pub fn is_etc(self) -> bool {
        matches!(
            self,
            Self::EtcRgb4
                | Self::EacR
                | Self::EacRSigned
                | Self::EacRg
                | Self::EacRgSigned
                | Self::Etc2Rgb
                | Self::Etc2Rgba1
                | Self::Etc2Rgba8
        )
    }

    /// CLI 展示用的短码（如 `astc6x6` / `etc1` / `etc2rgba8` / `eac_r11`）。
    /// ASTC 的 label 与历史输出 `astc{bxby}` 一致，保持向后兼容。
    pub fn label(self) -> &'static str {
        match self {
            // ETC / EAC
            Self::EtcRgb4 => "etc1",
            Self::Etc2Rgb => "etc2rgb",
            Self::Etc2Rgba1 => "etc2rgba1",
            Self::Etc2Rgba8 => "etc2rgba8",
            Self::EacR => "eac_r11",
            Self::EacRSigned => "eac_r11_signed",
            Self::EacRg => "eac_rg11",
            Self::EacRgSigned => "eac_rg11_signed",
            // ASTC：复用 footprint 拼成 `astc{w}x{h}`
            Self::AstcRgb4x4 | Self::AstcRgba4x4 => "astc4x4",
            Self::AstcRgb5x5 | Self::AstcRgba5x5 => "astc5x5",
            Self::AstcRgb6x6 | Self::AstcRgba6x6 => "astc6x6",
            Self::AstcRgb8x8 | Self::AstcRgba8x8 => "astc8x8",
            Self::AstcRgb10x10 | Self::AstcRgba10x10 => "astc10x10",
            Self::AstcRgb12x12 | Self::AstcRgba12x12 => "astc12x12",
        }
    }

    /// ASTC block footprint（仅 ASTC 格式有意义；RGB/RGBA 同尺寸返回同一值）。
    pub fn astc_block(self) -> Option<(u32, u32)> {
        match self {
            Self::AstcRgb4x4 | Self::AstcRgba4x4 => Some((4, 4)),
            Self::AstcRgb5x5 | Self::AstcRgba5x5 => Some((5, 5)),
            Self::AstcRgb6x6 | Self::AstcRgba6x6 => Some((6, 6)),
            Self::AstcRgb8x8 | Self::AstcRgba8x8 => Some((8, 8)),
            Self::AstcRgb10x10 | Self::AstcRgba10x10 => Some((10, 10)),
            Self::AstcRgb12x12 | Self::AstcRgba12x12 => Some((12, 12)),
            _ => None,
        }
    }
}

/// 纹理导出错误。`texture_decoder_rs::DecodeError` 是 `#![no_std]` 且未 impl
/// `std::error::Error`，这里用 `source(false)` + 自定义 display 透传。
#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum TextureExportError {
    /// 格式已知但不支持（如 ETC_RGB4）。
    #[snafu(display("不支持的纹理格式: {fmt:?}"))]
    UnsupportedFormat { fmt: TextureFormat },

    /// `m_format` 不在已知枚举内。
    #[snafu(display("未知纹理格式: format_id={format_id}"))]
    UnknownFormat { format_id: i32 },

    /// `image_data` 为空且 `.resS` 流数据也取不到。
    #[snafu(display("纹理无像素数据（image_data 空 + streaming 失败）"))]
    NoImageData,

    /// ASTC / ETC 解码失败（输入过短 / 非法块等）。`texture_decoder_rs` 不返回
    /// `std::error::Error`，故作为字段携带而非 source。两族解码器共用同一
    /// `DecodeError` 类型。
    #[snafu(display("解码失败: {err:?}"))]
    Decode { err: texture_decoder_rs::DecodeError },

    /// PNG 编码 / 写入失败。
    #[snafu(display("写 PNG 失败: {source}"))]
    PngIo { source: std::io::Error },

    /// 输出缓冲尺寸与图像不匹配（`image::ImageBuffer::from_raw` 返回 None）。
    #[snafu(display("输出缓冲尺寸不匹配: {width}x{height} 需 {expected} 字节, 实 {actual}"))]
    BufferSize {
        width: u32,
        height: u32,
        expected: usize,
        actual: usize,
    },
}

/// 取 Texture2D 的像素字节：`image_data` 非空用它，否则走 streaming。
///
/// 对齐 UnityPy `_Texture2d_get_image_data`。返回 [`Bytes`] 便于两种来源统一
/// （`image_data` 走 `Bytes::from(Vec)`；streaming 直接返回零拷贝 slice）。
pub fn texture_pixel_bytes(t: &Texture2D, env: &Environment) -> Option<Bytes> {
    if !t.image_data.is_empty() {
        Some(Bytes::from(t.image_data.clone()))
    } else {
        t.m_stream_data
            .as_ref()
            .and_then(|info| env.streaming_data(info))
    }
}

/// 解码 ASTC 纹理为 top-down RGBA8（已垂直翻转，可直接喂 PNG）。
///
/// `data` 为裸块流（Unity 的 `image_data`/streaming 本就不带 `.astc` 文件头，直接喂）。
/// 输出尺寸 = `width * height * 4`。
pub fn decode_astc(
    data: &[u8],
    width: u32,
    height: u32,
    fmt: TextureFormat,
) -> Result<Vec<u8>, TextureExportError> {
    let (w, h) = (width as usize, height as usize);
    let mut out = vec![0u8; w * h * 4];

    // texture_decoder_rs 按 footprint 命名函数，运行时分派；RGB/RGBA 同尺寸共用。
    let res = match fmt {
        TextureFormat::AstcRgb4x4 | TextureFormat::AstcRgba4x4 => {
            texture_decoder_rs::decode_ldr_4_4(data, w, h, &mut out)
        }
        TextureFormat::AstcRgb5x5 | TextureFormat::AstcRgba5x5 => {
            texture_decoder_rs::decode_ldr_5_5(data, w, h, &mut out)
        }
        TextureFormat::AstcRgb6x6 | TextureFormat::AstcRgba6x6 => {
            texture_decoder_rs::decode_ldr_6_6(data, w, h, &mut out)
        }
        TextureFormat::AstcRgb8x8 | TextureFormat::AstcRgba8x8 => {
            texture_decoder_rs::decode_ldr_8_8(data, w, h, &mut out)
        }
        TextureFormat::AstcRgb10x10 | TextureFormat::AstcRgba10x10 => {
            texture_decoder_rs::decode_ldr_10_10(data, w, h, &mut out)
        }
        TextureFormat::AstcRgb12x12 | TextureFormat::AstcRgba12x12 => {
            texture_decoder_rs::decode_ldr_12_12(data, w, h, &mut out)
        }
        _ => return UnsupportedFormatSnafu { fmt }.fail(),
    };
    res.map_err(|err| TextureExportError::Decode { err })?;

    // Unity bottom-up → PNG top-down。
    flip_vertical_in_place(&mut out, w, h);
    Ok(out)
}

/// 解码 ETC1/ETC2/EAC 纹理为 top-down RGBA8（已垂直翻转，可直接喂 PNG）。
///
/// 与 [`decode_astc`] 同构：`data` 为裸块流（Unity 的 `image_data`/streaming
/// 不带 PKM/KTX 容器头，直接喂），4×4 block，输出尺寸 = `width * height * 4`。
/// 块边缘裁剪由 `texture_decoder_rs` 的 `decode_image` 内部处理（非 4 倍数尺寸安全）。
///
/// 通道顺序：`texture_decoder_rs` 的 ETC 解码器 `write_pixel` 直接写 `[R,G,B]`+alpha
/// （RGBA），与 ASTC 路径一致，无需 B/R 交换（注意 UnityPy 的 ETC 路径用 `"BGRA"` 是
/// 因 Python 端 `texture2ddecoder`(C++) 输出 BGRA，与本 crate 无关）。
pub fn decode_etc(
    data: &[u8],
    width: u32,
    height: u32,
    fmt: TextureFormat,
) -> Result<Vec<u8>, TextureExportError> {
    let (w, h) = (width as usize, height as usize);
    let mut out = vec![0u8; w * h * 4];

    let res = match fmt {
        TextureFormat::EtcRgb4 => texture_decoder_rs::decode_etc1_rgb8(data, w, h, &mut out),
        TextureFormat::Etc2Rgb => texture_decoder_rs::decode_etc2_rgb8(data, w, h, &mut out),
        TextureFormat::Etc2Rgba1 => texture_decoder_rs::decode_etc2_rgb8a1(data, w, h, &mut out),
        TextureFormat::Etc2Rgba8 => texture_decoder_rs::decode_etc2_rgba8(data, w, h, &mut out),
        TextureFormat::EacR => texture_decoder_rs::decode_eac_r11(data, w, h, &mut out),
        TextureFormat::EacRSigned => {
            texture_decoder_rs::decode_eac_signed_r11(data, w, h, &mut out)
        }
        TextureFormat::EacRg => texture_decoder_rs::decode_eac_rg11(data, w, h, &mut out),
        TextureFormat::EacRgSigned => {
            texture_decoder_rs::decode_eac_signed_rg11(data, w, h, &mut out)
        }
        _ => return UnsupportedFormatSnafu { fmt }.fail(),
    };
    res.map_err(|err| TextureExportError::Decode { err })?;

    // Unity bottom-up → PNG top-down。
    flip_vertical_in_place(&mut out, w, h);
    Ok(out)
}

/// 逐行 reverse 实现垂直翻转（in-place，无额外分配）。
fn flip_vertical_in_place(buf: &mut [u8], width: usize, height: usize) {
    let stride = width * 4;
    let mut top = 0;
    let mut bottom = height.saturating_sub(1);
    while top < bottom {
        let (a, b) = buf.split_at_mut(bottom * stride);
        a[top * stride..top * stride + stride]
            .swap_with_slice(&mut b[..stride]);
        top += 1;
        bottom -= 1;
    }
}

/// 将 RGBA8 像素编码为 PNG 写入 `wtr`。
///
/// `rgba` 尺寸必须恰好 `width * height * 4`（由 [`decode_astc`] 保证）。零拷贝接管
/// `Vec<u8>`：`ImageBuffer::from_raw` 消费 `Vec`，不复制。
pub fn write_png(
    rgba: Vec<u8>,
    width: u32,
    height: u32,
    wtr: &mut impl Write,
) -> Result<(), TextureExportError> {
    let expected = width as usize * height as usize * 4;
    if rgba.len() != expected {
        return Err(TextureExportError::BufferSize {
            width,
            height,
            expected,
            actual: rgba.len(),
        });
    }
    let img: image::RgbaImage = image::ImageBuffer::from_raw(width, height, rgba)
        .ok_or(TextureExportError::BufferSize {
            width,
            height,
            expected,
            actual: 0,
        })?;
    // image::DynamicImage::write_to 需 `Write + Seek`，用 Cursor 包装提供 Seek，
    // 再把缓冲一次性写到底层 writer。
    let mut cur = std::io::Cursor::new(Vec::new());
    image::DynamicImage::ImageRgba8(img)
        .write_to(&mut cur, image::ImageFormat::Png)
        .map_err(|e| TextureExportError::PngIo {
            source: std::io::Error::other(e.to_string()),
        })?;
    wtr.write_all(&cur.into_inner())
        .map_err(|e| TextureExportError::PngIo { source: e })?;
    Ok(())
}
