//! Texture2D → 图片导出（受 `export` feature 门控）。
//!
//! 现状：支持 ASTC LDR + ETC1/ETC2/EAC + BCn/DXT（通过 path 依赖
//! [`texture_decoder_rs`]）+ 未压缩格式（Alpha8/RGB24/RGBA32/ARGB32/BGRA32/ARGB4444/
//! RGBA4444/RGB565/R8/RG16/R16 及各 *_SIGNED 变体，纯字节重排，不经 decoder），输出 PNG。
//! 像素数据来源选择对齐 UnityPy `_Texture2d_get_image_data`：`image_data` 非空用它，否则
//! 走 `Environment::streaming_data` 取 `.resS` 流数据。
//!
//! Y 翻转：Unity 纹理原点在左下、PNG 在左上，导出时垂直翻转（对齐 UnityPy 默认
//! `FLIP_TOP_BOTTOM`）。通道顺序：本 crate 全路径输出 RGBA——`texture_decoder_rs` 的 ASTC
//! 与 ETC 解码器均输出 RGBA（ASTC 走 RGBA swizzle；ETC `write_pixel` 直接写 `[R,G,B]`+alpha），
//! 与 UnityPy 的 `texture2ddecoder`(C++) BGRA 输出不同，这里**不做** B/R 交换；未压缩路径
//! 的公式已把 AssetStudio 的 BGRA 输出翻转成 RGBA。
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
//! 路线图：Crunch/PVRTC/ATC + ASTC HDR + Sprite Sheet + 多输出格式。

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
/// ETC1/ETC2/EAC（34/41-47）通过 `texture_decoder_rs` 的 `etc` 模块解码。
/// BCn/DXT（10/11/12/24-27）通过 `texture_decoder_rs` 的 `bcn` 模块解码，
/// 4×4 block；Unity 的 `BC6H`（24）走无符号 UF16 路径（Unity 不区分 SF16）。
/// 未压缩格式（1/2/3/4/5/7/8/9/13/14/62/63/75-82）不经 `texture_decoder_rs`，由
/// [`decode_uncompressed`] 做纯字节重排/位宽转换；字节布局对齐 AssetStudio
/// `Texture2DConverter.cs` 的逐像素 Decode 函数（AssetStudio 输出 BGRA，这里翻成 RGBA）。
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureFormat {
    // ── 未压缩（1, 3-5, 7-9, 14, 62-63, 75-82）──
    /// Alpha8（format 1，1 字节/像素）→ `[0,0,0,A]`。
    Alpha8 = 1,
    /// ARGB4444（format 2，2 字节/像素 LE u16，nibble×17）。
    Argb4444 = 2,
    /// RGB24（format 3，3 字节/像素）→ `[R,G,B,255]`。
    Rgb24 = 3,
    /// RGBA32（format 4，4 字节/像素，原序）。
    Rgba32 = 4,
    /// ARGB32（format 5，4 字节/像素；A 在 byte0）。
    Argb32 = 5,
    /// RGB565（format 7，2 字节/像素 LE u16，位复制扩展）。
    Rgb565 = 7,
    /// BGR24（format 8，3 字节/像素）→ R/B 交换。
    Bgr24 = 8,
    /// R16（format 9，2 字节/像素 LE u16，downscale16）。
    R16 = 9,
    /// BGRA32（format 14，4 字节/像素；R/B 交换）。
    Bgra32 = 14,
    /// RG16（format 62，2 字节/像素）→ `[R,G,0,255]`。
    Rg16 = 62,
    /// R8（format 63，1 字节/像素）→ `[R,0,0,255]`。
    R8 = 63,
    /// R8_SIGNED（format 75，1 字节 i8；reinterpret 为 u8）。
    R8Signed = 75,
    /// RG16_SIGNED（format 76，2 字节 i8×2；reinterpret）。
    Rg16Signed = 76,
    /// RGB24_SIGNED（format 77，3 字节 i8×3；reinterpret）。
    Rgb24Signed = 77,
    /// RGBA32_SIGNED（format 78，4 字节 i8×4；reinterpret）。
    Rgba32Signed = 78,
    /// R16_SIGNED（format 79，2 字节 LE i16；reinterpret 后 downscale16）。
    R16Signed = 79,
    /// RG32_SIGNED（format 80，4 字节 LE i16×2；reinterpret 后每通道 downscale16）。
    Rg32Signed = 80,
    /// RGB48_SIGNED（format 81，6 字节 LE i16×3；reinterpret 后每通道 downscale16）。
    Rgb48Signed = 81,
    /// RGBA64_SIGNED（format 82，8 字节 LE i16×4；reinterpret 后每通道 downscale16）。
    Rgba64Signed = 82,
    // ── BCn / DXT（10-12, 24-27）──
    /// DXT1（= BC1，8 字节块；3-color 模式下 selector 3 解码为透明黑）。
    Dxt1 = 10,
    /// DXT3（= BC2，16 字节块；4-bit alpha 扩展为 `value * 17`）。
    Dxt3 = 11,
    /// DXT5（= BC3，16 字节块；8-entry alpha 插值）。
    Dxt5 = 12,
    /// RGBA4444（format 13，2 字节/像素 LE u16，nibble×17）。
    Rgba4444 = 13,
    /// BC6H（无符号 UF16，16 字节块；f16 → 8-bit 钳位映射，HDR >1 钳到 255）。
    Bc6H = 24,
    /// BC7（16 字节块；保留解码 RGBA）。
    Bc7 = 25,
    /// BC4（UNORM，8 字节块；单通道 `[R, 0, 0, 255]`）。
    Bc4 = 26,
    /// BC5（UNORM，16 字节块；双通道 `[R, G, 0, 255]`）。
    Bc5 = 27,
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
            1 => Some(Self::Alpha8),
            2 => Some(Self::Argb4444),
            3 => Some(Self::Rgb24),
            4 => Some(Self::Rgba32),
            5 => Some(Self::Argb32),
            7 => Some(Self::Rgb565),
            8 => Some(Self::Bgr24),
            9 => Some(Self::R16),
            14 => Some(Self::Bgra32),
            10 => Some(Self::Dxt1),
            11 => Some(Self::Dxt3),
            12 => Some(Self::Dxt5),
            13 => Some(Self::Rgba4444),
            24 => Some(Self::Bc6H),
            25 => Some(Self::Bc7),
            26 => Some(Self::Bc4),
            27 => Some(Self::Bc5),
            34 => Some(Self::EtcRgb4),
            41 => Some(Self::EacR),
            42 => Some(Self::EacRSigned),
            43 => Some(Self::EacRg),
            44 => Some(Self::EacRgSigned),
            45 => Some(Self::Etc2Rgb),
            46 => Some(Self::Etc2Rgba1),
            47 => Some(Self::Etc2Rgba8),
            62 => Some(Self::Rg16),
            63 => Some(Self::R8),
            75 => Some(Self::R8Signed),
            76 => Some(Self::Rg16Signed),
            77 => Some(Self::Rgb24Signed),
            78 => Some(Self::Rgba32Signed),
            79 => Some(Self::R16Signed),
            80 => Some(Self::Rg32Signed),
            81 => Some(Self::Rgb48Signed),
            82 => Some(Self::Rgba64Signed),
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

    /// 是否为本模块支持的格式（ASTC LDR / ETC1/ETC2/EAC / BCn/DXT / 未压缩）。
    pub fn is_supported(self) -> bool {
        self.astc_block().is_some() || self.is_etc() || self.is_bcn() || self.is_uncompressed()
    }

    /// 是否为未压缩格式（纯字节重排，不经 `texture_decoder_rs`）。
    pub fn is_uncompressed(self) -> bool {
        matches!(
            self,
            Self::Alpha8
                | Self::Argb4444
                | Self::Rgba4444
                | Self::Rgb24
                | Self::Rgba32
                | Self::Argb32
                | Self::Rgb565
                | Self::Bgr24
                | Self::R16
                | Self::Bgra32
                | Self::Rg16
                | Self::R8
                | Self::R8Signed
                | Self::Rg16Signed
                | Self::Rgb24Signed
                | Self::Rgba32Signed
                | Self::R16Signed
                | Self::Rg32Signed
                | Self::Rgb48Signed
                | Self::Rgba64Signed
        )
    }

    /// 是否为 BCn/DXT 家族（4×4 块，由 `bcn_decoder` 处理）。
    pub fn is_bcn(self) -> bool {
        matches!(
            self,
            Self::Dxt1 | Self::Dxt3 | Self::Dxt5 | Self::Bc4 | Self::Bc5 | Self::Bc6H | Self::Bc7
        )
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

    /// CLI 展示用的短码（如 `astc6x6` / `etc1` / `etc2rgba8` / `eac_r11` / `bc1`）。
    /// ASTC 的 label 与历史输出 `astc{bxby}` 一致，保持向后兼容。
    pub fn label(self) -> &'static str {
        match self {
            // 未压缩
            Self::Alpha8 => "a8",
            Self::Argb4444 => "argb4444",
            Self::Rgba4444 => "rgba4444",
            Self::Rgb24 => "rgb24",
            Self::Rgba32 => "rgba32",
            Self::Argb32 => "argb32",
            Self::Rgb565 => "rgb565",
            Self::Bgr24 => "bgr24",
            Self::R16 => "r16",
            Self::Bgra32 => "bgra32",
            Self::Rg16 => "rg16",
            Self::R8 => "r8",
            Self::R8Signed => "r8s",
            Self::Rg16Signed => "rg16s",
            Self::Rgb24Signed => "rgb24s",
            Self::Rgba32Signed => "rgba32s",
            Self::R16Signed => "r16s",
            Self::Rg32Signed => "rg32s",
            Self::Rgb48Signed => "rgb48s",
            Self::Rgba64Signed => "rgba64s",
            // BCn / DXT
            Self::Dxt1 => "bc1",
            Self::Dxt3 => "bc2",
            Self::Dxt5 => "bc3",
            Self::Bc4 => "bc4",
            Self::Bc5 => "bc5",
            Self::Bc6H => "bc6h",
            Self::Bc7 => "bc7",
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
            texture_decoder_rs::astc::decode_ldr_4_4(data, w, h, &mut out)
        }
        TextureFormat::AstcRgb5x5 | TextureFormat::AstcRgba5x5 => {
            texture_decoder_rs::astc::decode_ldr_5_5(data, w, h, &mut out)
        }
        TextureFormat::AstcRgb6x6 | TextureFormat::AstcRgba6x6 => {
            texture_decoder_rs::astc::decode_ldr_6_6(data, w, h, &mut out)
        }
        TextureFormat::AstcRgb8x8 | TextureFormat::AstcRgba8x8 => {
            texture_decoder_rs::astc::decode_ldr_8_8(data, w, h, &mut out)
        }
        TextureFormat::AstcRgb10x10 | TextureFormat::AstcRgba10x10 => {
            texture_decoder_rs::astc::decode_ldr_10_10(data, w, h, &mut out)
        }
        TextureFormat::AstcRgb12x12 | TextureFormat::AstcRgba12x12 => {
            texture_decoder_rs::astc::decode_ldr_12_12(data, w, h, &mut out)
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
        TextureFormat::EtcRgb4 => texture_decoder_rs::etc::decode_etc1_rgb8(data, w, h, &mut out),
        TextureFormat::Etc2Rgb => texture_decoder_rs::etc::decode_etc2_rgb8(data, w, h, &mut out),
        TextureFormat::Etc2Rgba1 => texture_decoder_rs::etc::decode_etc2_rgb8a1(data, w, h, &mut out),
        TextureFormat::Etc2Rgba8 => texture_decoder_rs::etc::decode_etc2_rgba8(data, w, h, &mut out),
        TextureFormat::EacR => texture_decoder_rs::etc::decode_eac_r11(data, w, h, &mut out),
        TextureFormat::EacRSigned => {
            texture_decoder_rs::etc::decode_eac_signed_r11(data, w, h, &mut out)
        }
        TextureFormat::EacRg => texture_decoder_rs::etc::decode_eac_rg11(data, w, h, &mut out),
        TextureFormat::EacRgSigned => {
            texture_decoder_rs::etc::decode_eac_signed_rg11(data, w, h, &mut out)
        }
        _ => return UnsupportedFormatSnafu { fmt }.fail(),
    };
    res.map_err(|err| TextureExportError::Decode { err })?;

    // Unity bottom-up → PNG top-down。
    flip_vertical_in_place(&mut out, w, h);
    Ok(out)
}

/// 解码 BCn/DXT 纹理为 top-down RGBA8（已垂直翻转，可直接喂 PNG）。
///
/// 与 [`decode_astc`] / [`decode_etc`] 同构：`data` 为裸块流（Unity 的
/// `image_data`/streaming 不带 DDS 容器头，直接喂），4×4 block，输出尺寸 =
/// `width * height * 4`。块边缘裁剪由 `texture_decoder_rs` 的 `decode_image` 内部处理
/// （非 4 倍数尺寸安全）。
///
/// 通道顺序：`texture_decoder_rs` 的 BCn 解码器统一输出 RGBA（BC1/BC2/BC3/BC7 全通道；
/// BC4 写 `[R,0,0,255]`、BC5 写 `[R,G,0,255]`、BC6H 写 `[R,G,B,255]`），与 ASTC/ETC
/// 路径一致，无需 B/R 交换。
///
/// Unity 的 `BC6H`（format 24）走无符号 UF16 路径——Unity 不区分 SF16，且 decoder 对
/// HDR >1 的值钳到 255（不做 tone mapping）。
pub fn decode_bcn(
    data: &[u8],
    width: u32,
    height: u32,
    fmt: TextureFormat,
) -> Result<Vec<u8>, TextureExportError> {
    let (w, h) = (width as usize, height as usize);
    let mut out = vec![0u8; w * h * 4];

    let res = match fmt {
        TextureFormat::Dxt1 => texture_decoder_rs::bcn::decode_bc1(data, w, h, &mut out),
        TextureFormat::Dxt3 => texture_decoder_rs::bcn::decode_bc2(data, w, h, &mut out),
        TextureFormat::Dxt5 => texture_decoder_rs::bcn::decode_bc3(data, w, h, &mut out),
        TextureFormat::Bc4 => texture_decoder_rs::bcn::decode_bc4(data, w, h, &mut out),
        TextureFormat::Bc5 => texture_decoder_rs::bcn::decode_bc5(data, w, h, &mut out),
        TextureFormat::Bc6H => texture_decoder_rs::bcn::decode_bc6h_uf16(data, w, h, &mut out),
        TextureFormat::Bc7 => texture_decoder_rs::bcn::decode_bc7(data, w, h, &mut out),
        _ => return UnsupportedFormatSnafu { fmt }.fail(),
    };
    res.map_err(|err| TextureExportError::Decode { err })?;

    // Unity bottom-up → PNG top-down。
    flip_vertical_in_place(&mut out, w, h);
    Ok(out)
}

/// 解码未压缩纹理格式为 top-down RGBA8（已垂直翻转，可直接喂 PNG）。
///
/// 与 `decode_astc` / `decode_etc` / `decode_bcn` 同构：`data` 为 Unity 的裸像素流
/// （无 DDS/PKM/KTX 容器头），输出尺寸 = `width * height * 4`。**不经
/// `texture_decoder_rs`**——未压缩格式是纯字节重排/位宽转换，无块解码。
///
/// # 字节布局
///
/// 所有公式对齐 AssetStudio `Texture2DConverter.cs` 的逐像素 `Decode*` 函数（AssetStudio
/// 输出 BGRA，这里翻成 RGBA）。关键约定：
///
/// - **ARGB4444 / RGBA4444**：2 字节/像素 LE u16，每通道 4-bit，扩展成 8-bit 用 `nibble * 17`
///   （即 `(n << 4) | n`，复制高位到低位，对齐 AssetStudio）。
/// - **RGB565**：2 字节/像素 LE u16，16-bit 值布局 `RRRRRGGGGGGBBBBB`（R=15-11, G=10-5, B=4-0），
///   **位复制扩展**（非 `*255//31` 缩放）：5-bit→8-bit 用 `(v << 3) | (v >> 2)`、
///   6-bit→8-bit 用 `(v << 2) | (v >> 4)`。与 Pillow 的缩放法仅差 ±1 LSB，视觉无差，选
///   AssetStudio 可重现路径。
/// - **R16 / 16-bit 多通道**：`downscale16`：`((v as u32 * 255 + 32895) >> 16) as u8`（对齐
///   AssetStudio `DownScaleFrom16BitTo8Bit`，四舍五入除法 `≈ round(v*255/65535)`）。
/// - **SIGNED 变体**（75-82）：Pillow 的 `";8s"`/`";16s"` raw mode 在新版已不可复现且语义随版本
///   漂移；这里取最简单可重现的 **reinterpret cast**——`i8 as u8`、`i16 的 bits as u16` 后走
///   对应 unsigned 路径（`-1` 的 i8 字节 `0xFF` → u8 `255`）。
pub fn decode_uncompressed(
    data: &[u8],
    width: u32,
    height: u32,
    fmt: TextureFormat,
) -> Result<Vec<u8>, TextureExportError> {
    let (w, h) = (width as usize, height as usize);
    let n = w * h;
    let mut out = vec![0u8; n * 4];

    /// 4-bit nibble 扩展成 8-bit：复制高位到低位（`0xA` → `0xAA`）。
    #[inline]
    fn expand4(nibble: u8) -> u8 {
        (nibble << 4) | nibble
    }

    /// 16-bit 无符号 → 8-bit 四舍五入 downscale（对齐 AssetStudio `DownScaleFrom16BitTo8Bit`）。
    #[inline]
    fn downscale16(v: u16) -> u8 {
        ((v as u32 * 255 + 32895) >> 16) as u8
    }

    macro_rules! bail_short {
        () => {
            return Err(TextureExportError::Decode {
                err: texture_decoder_rs::DecodeError::InputTooShort,
            })
        };
    }

    match fmt {
        TextureFormat::Alpha8 => {
            if data.len() < n {
                bail_short!();
            }
            for i in 0..n {
                out[i * 4] = 0;
                out[i * 4 + 1] = 0;
                out[i * 4 + 2] = 0;
                out[i * 4 + 3] = data[i];
            }
        }
        TextureFormat::R8 => {
            if data.len() < n {
                bail_short!();
            }
            for i in 0..n {
                out[i * 4] = data[i];
                out[i * 4 + 1] = 0;
                out[i * 4 + 2] = 0;
                out[i * 4 + 3] = 255;
            }
        }
        TextureFormat::Rg16 => {
            if data.len() < n * 2 {
                bail_short!();
            }
            for i in 0..n {
                out[i * 4] = data[i * 2];
                out[i * 4 + 1] = data[i * 2 + 1];
                out[i * 4 + 2] = 0;
                out[i * 4 + 3] = 255;
            }
        }
        TextureFormat::R16 => {
            if data.len() < n * 2 {
                bail_short!();
            }
            for i in 0..n {
                let v = u16::from_le_bytes([data[i * 2], data[i * 2 + 1]]);
                out[i * 4] = downscale16(v);
                out[i * 4 + 1] = 0;
                out[i * 4 + 2] = 0;
                out[i * 4 + 3] = 255;
            }
        }
        TextureFormat::Rgb24 | TextureFormat::Rgb24Signed => {
            if data.len() < n * 3 {
                bail_short!();
            }
            for i in 0..n {
                out[i * 4] = data[i * 3] as i8 as u8;
                out[i * 4 + 1] = data[i * 3 + 1] as i8 as u8;
                out[i * 4 + 2] = data[i * 3 + 2] as i8 as u8;
                out[i * 4 + 3] = 255;
            }
        }
        TextureFormat::Bgr24 => {
            if data.len() < n * 3 {
                bail_short!();
            }
            for i in 0..n {
                out[i * 4] = data[i * 3 + 2];
                out[i * 4 + 1] = data[i * 3 + 1];
                out[i * 4 + 2] = data[i * 3];
                out[i * 4 + 3] = 255;
            }
        }
        TextureFormat::Rgba32 | TextureFormat::Rgba32Signed => {
            if data.len() < n * 4 {
                bail_short!();
            }
            for i in 0..n {
                let b = i * 4;
                out[b] = data[b] as i8 as u8;
                out[b + 1] = data[b + 1] as i8 as u8;
                out[b + 2] = data[b + 2] as i8 as u8;
                out[b + 3] = data[b + 3] as i8 as u8;
            }
        }
        TextureFormat::Argb32 => {
            if data.len() < n * 4 {
                bail_short!();
            }
            for i in 0..n {
                let b = i * 4;
                // 输入 [A,R,G,B] → 输出 [R,G,B,A]
                out[b] = data[b + 1];
                out[b + 1] = data[b + 2];
                out[b + 2] = data[b + 3];
                out[b + 3] = data[b];
            }
        }
        TextureFormat::Bgra32 => {
            if data.len() < n * 4 {
                bail_short!();
            }
            for i in 0..n {
                let b = i * 4;
                // 输入 [B,G,R,A] → 输出 [R,G,B,A]
                out[b] = data[b + 2];
                out[b + 1] = data[b + 1];
                out[b + 2] = data[b];
                out[b + 3] = data[b + 3];
            }
        }
        TextureFormat::Argb4444 => {
            if data.len() < n * 2 {
                bail_short!();
            }
            for i in 0..n {
                let v = u16::from_le_bytes([data[i * 2], data[i * 2 + 1]]);
                // 16-bit 布局 AAAARRRRGGGGBBBB（A=15-12, R=11-8, G=7-4, B=3-0）。
                out[i * 4] = expand4(((v >> 8) & 0xF) as u8);
                out[i * 4 + 1] = expand4(((v >> 4) & 0xF) as u8);
                out[i * 4 + 2] = expand4((v & 0xF) as u8);
                out[i * 4 + 3] = expand4(((v >> 12) & 0xF) as u8);
            }
        }
        TextureFormat::Rgba4444 => {
            if data.len() < n * 2 {
                bail_short!();
            }
            for i in 0..n {
                let v = u16::from_le_bytes([data[i * 2], data[i * 2 + 1]]);
                // 16-bit 布局 RRRRGGGGBBBBAAAA（R=15-12, G=11-8, B=7-4, A=3-0）。
                out[i * 4] = expand4(((v >> 12) & 0xF) as u8);
                out[i * 4 + 1] = expand4(((v >> 8) & 0xF) as u8);
                out[i * 4 + 2] = expand4(((v >> 4) & 0xF) as u8);
                out[i * 4 + 3] = expand4((v & 0xF) as u8);
            }
        }
        TextureFormat::Rgb565 => {
            if data.len() < n * 2 {
                bail_short!();
            }
            for i in 0..n {
                let v = u16::from_le_bytes([data[i * 2], data[i * 2 + 1]]);
                // 16-bit 布局 RRRRRGGGGGGBBBBB（R=15-11, G=10-5, B=4-0），位复制扩展。
                let r5 = ((v >> 11) & 0x1F) as u8;
                let g6 = ((v >> 5) & 0x3F) as u8;
                let b5 = (v & 0x1F) as u8;
                out[i * 4] = (r5 << 3) | (r5 >> 2);
                out[i * 4 + 1] = (g6 << 2) | (g6 >> 4);
                out[i * 4 + 2] = (b5 << 3) | (b5 >> 2);
                out[i * 4 + 3] = 255;
            }
        }
        // ── SIGNED ── 全部 reinterpret 后走对应 unsigned 公式。
        TextureFormat::R8Signed => {
            if data.len() < n {
                bail_short!();
            }
            for i in 0..n {
                out[i * 4] = data[i] as i8 as u8;
                out[i * 4 + 1] = 0;
                out[i * 4 + 2] = 0;
                out[i * 4 + 3] = 255;
            }
        }
        TextureFormat::Rg16Signed => {
            if data.len() < n * 2 {
                bail_short!();
            }
            for i in 0..n {
                out[i * 4] = data[i * 2] as i8 as u8;
                out[i * 4 + 1] = data[i * 2 + 1] as i8 as u8;
                out[i * 4 + 2] = 0;
                out[i * 4 + 3] = 255;
            }
        }
        TextureFormat::R16Signed => {
            if data.len() < n * 2 {
                bail_short!();
            }
            for i in 0..n {
                let v = u16::from_le_bytes([data[i * 2], data[i * 2 + 1]]);
                out[i * 4] = downscale16(v);
                out[i * 4 + 1] = 0;
                out[i * 4 + 2] = 0;
                out[i * 4 + 3] = 255;
            }
        }
        TextureFormat::Rg32Signed => {
            if data.len() < n * 4 {
                bail_short!();
            }
            for i in 0..n {
                let r = u16::from_le_bytes([data[i * 4], data[i * 4 + 1]]);
                let g = u16::from_le_bytes([data[i * 4 + 2], data[i * 4 + 3]]);
                out[i * 4] = downscale16(r);
                out[i * 4 + 1] = downscale16(g);
                out[i * 4 + 2] = 0;
                out[i * 4 + 3] = 255;
            }
        }
        TextureFormat::Rgb48Signed => {
            if data.len() < n * 6 {
                bail_short!();
            }
            for i in 0..n {
                let r = u16::from_le_bytes([data[i * 6], data[i * 6 + 1]]);
                let g = u16::from_le_bytes([data[i * 6 + 2], data[i * 6 + 3]]);
                let b = u16::from_le_bytes([data[i * 6 + 4], data[i * 6 + 5]]);
                out[i * 4] = downscale16(r);
                out[i * 4 + 1] = downscale16(g);
                out[i * 4 + 2] = downscale16(b);
                out[i * 4 + 3] = 255;
            }
        }
        TextureFormat::Rgba64Signed => {
            if data.len() < n * 8 {
                bail_short!();
            }
            for i in 0..n {
                let r = u16::from_le_bytes([data[i * 8], data[i * 8 + 1]]);
                let g = u16::from_le_bytes([data[i * 8 + 2], data[i * 8 + 3]]);
                let b = u16::from_le_bytes([data[i * 8 + 4], data[i * 8 + 5]]);
                let a = u16::from_le_bytes([data[i * 8 + 6], data[i * 8 + 7]]);
                out[i * 4] = downscale16(r);
                out[i * 4 + 1] = downscale16(g);
                out[i * 4 + 2] = downscale16(b);
                out[i * 4 + 3] = downscale16(a);
            }
        }
        _ => return UnsupportedFormatSnafu { fmt }.fail(),
    }

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
