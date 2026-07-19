//! Texture2D → 图片导出（受 `export` feature 门控）。
//!
//! 现状：支持 ASTC LDR + ETC1/ETC2/EAC + BCn/DXT + Crunch（DXT1/DXT5/ETC1/ETC2A crunched，
//! 原始 + Unity 变体；通过 path 依赖 [`texture_decoder_rs`]，后者需 `crn` + `crn-unity`
//! feature）+ 全部未压缩格式（纯字节重排，不经 decoder），输出 PNG。Crunch 路径额外支持
//! cubemap（6 面纵向拼接，对齐 AssetRipper）+ 多 mip（只导出 level 0），详见 [`decode_crunch`]。
//! 像素数据来源选择对齐 UnityPy `_Texture2d_get_image_data`：`image_data` 非空用它，否则
//! 走 `Environment::streaming_data` 取 `.resS` 流数据。
//!
//! Y 翻转：Unity 纹理原点在左下、PNG 在左上，导出时垂直翻转（对齐 UnityPy 默认
//! `FLIP_TOP_BOTTOM`）。通道顺序：本 crate 全路径输出 RGBA——`texture_decoder_rs` 的 ASTC
//! 与 ETC 解码器均输出 RGBA（ASTC 走 RGBA swizzle；ETC `write_pixel` 直接写 `[R,G,B]`+alpha），
//! 与 UnityPy 的 `texture2ddecoder`(C++) BGRA 输出不同，这里**不做** B/R 交换；未压缩路径
//! 的公式已把 AssetStudio 的 BGRA 输出翻转成 RGBA。
//!
//! # 未压缩格式的数值映射
//!
//! 全部对齐 AssetRipper `NumericConversion`（`RgbConverter` 的逐通道标量变换）。AssetRipper
//! 是这套语义的权威实现——UnityPy 的 signed/fp32 路径目前因引用了不存在的 Pillow codec
//! 而不可用（详见 `CONV_TABLE` 里的 `R;8s` / `RF` 等字符串在标准 Pillow 中无定义）。
//!
//! - **8-bit signed（75-78）**：`byte_out = i8_in ^ 0x80`——**XOR 符号位**（offset-binary），
//!   不是 SNORM 归一化也不是 reinterpret cast。`-1 → 127`、`0 → 128`、`127 → 255`。
//! - **16-bit signed（79-82）**：`v = i16_in ^ 0x8000`，再走和 unsigned 同样的 `downscale16`。
//!   `-1 → 127`、`0 → 127`、`32767 → 255`。
//! - **16-bit unsigned 多通道（72-74）**：直接 `downscale16`。
//! - **fp16（15-17）/ fp32（6, 18-20, 23）/ RGB9e5（22）**：`byte_out = (v * 255.0) as u8`
//!   （Rust `as u8` 饱和截断，等价 AssetRipper 的 `clamp + (byte)`）。**无 tone mapping / 无
//!   Reinhard / 无 exposure**——HDR 值 >1.0 直接饱和到 255，负值饱和到 0。RGB9e5 共享指数
//!   解包：`mantissa9 * 2^(exp5 - 24)`，alpha 硬编码 255。
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
//! 路线图：PVRTC/ATC + ASTC HDR + Sprite Sheet + 多输出格式。
//!
//! # Crunch（原始 + Unity 变体）
//!
//! 支持 **原始 Crunch**（CRN v1.04，`crn` 模块，对齐 UnityPy `unpack_crunch`）与
//! **Unity 变体 Crunch**（Unity fork，`crn_unity` 模块，对齐 UnityPy `unpack_unity_crunch`）
//! 两条路径，由 [`TextureFormat::is_unity_crunch`]（版本分水岭）在 [`decode_crunch`] 内
//! 自动分派。覆盖 DXT1Crunched(28) / DXT5Crunched(29) / ETC_RGB4Crunched(64) /
//! ETC2_RGBA8Crunched(65) 四值。
//!
//! **版本分水岭**（对齐 AssetRipper `CrunchHandler.IsUseUnityCrunch`）：`version >= 2017.3`
//! → Unity 变体（Unity 在 2017.3 切换了 Crunch fork，bitstream 从「chunk encoding」改成
//! 不兼容的「reference encoding」）；或 format 为 ETC crunch（64/65，任意版本都走 Unity
//! 变体——ETC crunch 无原始变体）；否则（<2017.3 的 DXT crunched）走原始 crunch。两者 sig
//! 都是 `0x4878` "Hx"——原始解码器能跑完 Unity 变体数据却输出乱码，故**不能**靠 sig 区分，
//! 必须按版本 + format 分派。
//!
//! 两条路径结构同构：第一段（crn / crn_unity）把 CRN 解成含全部 face × mip 的 `Crn*Texture`
//! （每 mip 持原始 BC1/BC3/ETC1/ETC2_RGBA8 block），第二段对 level 0 走 `bcn`/`etc` 转
//! RGBA8。block 格式分派由 CRN 头的 `format` 字段决定（权威）。cubemap 纵向拼接 + level-0-only
//! + Y 翻转规则（2D 翻、cubemap 不翻）两路径共用。

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
/// 未压缩格式不经 `texture_decoder_rs`，由 [`decode_uncompressed`] 做纯字节重排/位宽
/// 转换；数值映射对齐 AssetRipper `NumericConversion`（8/16-bit signed 走 XOR 符号位，
/// 16-bit downscale，fp16/fp32 走线性 ×255 饱和，无 tone mapping）。
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureFormat {
    // ── 未压缩 8-bit（1, 3-5, 7-9, 14, 62-63）──
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
    /// R8_SIGNED（format 75，1 字节 i8；XOR 0x80 → u8）。
    R8Sign = 75,
    /// RG16_SIGNED（format 76，2 字节 i8×2；各 XOR 0x80）。
    Rg16Sign = 76,
    /// RGB24_SIGNED（format 77，3 字节 i8×3；各 XOR 0x80）。
    Rgb24Sign = 77,
    /// RGBA32_SIGNED（format 78，4 字节 i8×4；各 XOR 0x80）。
    Rgba32Sign = 78,
    /// R16_SIGNED（format 79，2 字节 LE i16；XOR 0x8000 后 downscale16）。
    R16Sign = 79,
    /// RG32_SIGNED（format 80，4 字节 LE i16×2；各 XOR 0x8000 后 downscale16）。
    Rg32Sign = 80,
    /// RGB48_SIGNED（format 81，6 字节 LE i16×3；各 XOR 0x8000 后 downscale16）。
    Rgb48Sign = 81,
    /// RGBA64_SIGNED（format 82，8 字节 LE i16×4；各 XOR 0x8000 后 downscale16）。
    Rgba64Sign = 82,
    // ── 未压缩 16-bit 多通道（72-74；与对应 SIGNED 共用 downscale16）──
    /// RG32（format 72，2 字节 LE u16×2；各 downscale16）。
    Rg32 = 72,
    /// RGB48（format 73，3 字节 LE u16×3；各 downscale16）。
    Rgb48 = 73,
    /// RGBA64（format 74，4 字节 LE u16×4；各 downscale16）。
    Rgba64 = 74,
    // ── 未压缩 HDR 浮点（6, 15-20, 22, 23）──
    /// ARGBFloat（format 6，16 字节 fp32×4，ARGB 序；线性 ×255 饱和）。
    ArgbFloat = 6,
    /// RHalf（format 15，2 字节 fp16；线性 ×255 饱和）。
    RHalf = 15,
    /// RGHalf（format 16，4 字节 fp16×2）。
    RgHalf = 16,
    /// RGBAHalf（format 17，8 字节 fp16×4）。
    RgbaHalf = 17,
    /// RFloat（format 18，4 字节 fp32）。
    RFloat = 18,
    /// RGFloat（format 19，8 字节 fp32×2）。
    RgFloat = 19,
    /// RGBAFloat（format 20，16 字节 fp32×4）。
    RgbaFloat = 20,
    /// RGB9e5Float（format 22，4 字节 packed；共享 5-bit 指数 + 3×9-bit 尾数）。
    Rgb9e5Float = 22,
    /// RGBFloat（format 23，12 字节 fp32×3）。
    RgbFloat = 23,
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
    // ── Crunch（28, 29, 64, 65；sig 0x4878，原始/Unity 变体由版本分水岭决定）──
    /// DXT1Crunched（format 28）→ crunch 解出 BC1 block 再转 RGBA。
    /// 版本分水岭（对齐 AssetRipper `IsUseUnityCrunch`）：Unity <2017.3 走**原始** crunch
    /// （`crn` 模块），≥2017.3 走 **Unity 变体**（`crn_unity` 模块）——两者 sig 都是 `0x4878`
    /// 但 bitstream 不兼容，原始解码器能跑完 Unity 变体却输出乱码。
    /// 支持 cubemap（6 面纵向拼接）+ 多 mip（只导出 level 0），见 [`decode_crunch`]。
    Dxt1Crunched = 28,
    /// DXT5Crunched（format 29）→ crunch 解出 BC3 block 再转 RGBA。版本分水岭同
    /// [`Dxt1Crunched`]。支持 cubemap + level-0-only，见 [`decode_crunch`]。
    Dxt5Crunched = 29,
    /// ETC_RGB4Crunched（format 64）→ **恒走 Unity 变体**（`crn_unity`），解出 ETC1 block
    /// 再转 RGBA。ETC crunch 无原始变体。支持 cubemap + level-0-only，见 [`decode_crunch`]。
    EtcRgb4Crunched = 64,
    /// ETC2_RGBA8Crunched（format 65）→ **恒走 Unity 变体**（`crn_unity`），解出 ETC2_RGBA8
    /// block 再转 RGBA。支持 cubemap + level-0-only，见 [`decode_crunch`]。
    Etc2Rgba8Crunched = 65,
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
            // 未压缩 8-bit
            1 => Some(Self::Alpha8),
            2 => Some(Self::Argb4444),
            3 => Some(Self::Rgb24),
            4 => Some(Self::Rgba32),
            5 => Some(Self::Argb32),
            7 => Some(Self::Rgb565),
            8 => Some(Self::Bgr24),
            9 => Some(Self::R16),
            13 => Some(Self::Rgba4444),
            14 => Some(Self::Bgra32),
            62 => Some(Self::Rg16),
            63 => Some(Self::R8),
            // 未压缩 8-bit signed
            75 => Some(Self::R8Sign),
            76 => Some(Self::Rg16Sign),
            77 => Some(Self::Rgb24Sign),
            78 => Some(Self::Rgba32Sign),
            // 未压缩 16-bit 多通道（unsigned）
            72 => Some(Self::Rg32),
            73 => Some(Self::Rgb48),
            74 => Some(Self::Rgba64),
            // 未压缩 16-bit 多通道（signed）
            79 => Some(Self::R16Sign),
            80 => Some(Self::Rg32Sign),
            81 => Some(Self::Rgb48Sign),
            82 => Some(Self::Rgba64Sign),
            // 未压缩 HDR 浮点
            6 => Some(Self::ArgbFloat),
            15 => Some(Self::RHalf),
            16 => Some(Self::RgHalf),
            17 => Some(Self::RgbaHalf),
            18 => Some(Self::RFloat),
            19 => Some(Self::RgFloat),
            20 => Some(Self::RgbaFloat),
            22 => Some(Self::Rgb9e5Float),
            23 => Some(Self::RgbFloat),
            // BCn / DXT
            10 => Some(Self::Dxt1),
            11 => Some(Self::Dxt3),
            12 => Some(Self::Dxt5),
            // Crunch（原始 / Unity 变体由 is_unity_crunch 按版本判定）
            28 => Some(Self::Dxt1Crunched),
            29 => Some(Self::Dxt5Crunched),
            64 => Some(Self::EtcRgb4Crunched),
            65 => Some(Self::Etc2Rgba8Crunched),
            24 => Some(Self::Bc6H),
            25 => Some(Self::Bc7),
            26 => Some(Self::Bc4),
            27 => Some(Self::Bc5),
            // ETC1 / ETC2 / EAC
            34 => Some(Self::EtcRgb4),
            41 => Some(Self::EacR),
            42 => Some(Self::EacRSigned),
            43 => Some(Self::EacRg),
            44 => Some(Self::EacRgSigned),
            45 => Some(Self::Etc2Rgb),
            46 => Some(Self::Etc2Rgba1),
            47 => Some(Self::Etc2Rgba8),
            // ASTC RGB / RGBA
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

    /// 是否为本模块支持的格式（ASTC LDR / ETC1/ETC2/EAC / BCn/DXT / Crunch / 未压缩）。
    pub fn is_supported(self) -> bool {
        self.astc_block().is_some()
            || self.is_etc()
            || self.is_bcn()
            || self.is_crunch()
            || self.is_uncompressed()
    }

    /// 是否为未压缩格式（纯字节重排/标量变换，不经 `texture_decoder_rs`）。
    /// 含 8-bit / 16-bit 整数（unsigned + signed）/ HDR 浮点（fp16/fp32/RGB9e5）。
    pub fn is_uncompressed(self) -> bool {
        matches!(
            self,
            // 8-bit unsigned / signed
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
                | Self::R8Sign
                | Self::Rg16Sign
                | Self::Rgb24Sign
                | Self::Rgba32Sign
                // 16-bit 多通道（unsigned + signed 共用 downscale16）
                | Self::Rg32
                | Self::Rgb48
                | Self::Rgba64
                | Self::R16Sign
                | Self::Rg32Sign
                | Self::Rgb48Sign
                | Self::Rgba64Sign
                // HDR 浮点
                | Self::ArgbFloat
                | Self::RHalf
                | Self::RgHalf
                | Self::RgbaHalf
                | Self::RFloat
                | Self::RgFloat
                | Self::RgbaFloat
                | Self::Rgb9e5Float
                | Self::RgbFloat
        )
    }

    /// 是否为 BCn/DXT 家族（4×4 块，由 `bcn_decoder` 处理）。
    pub fn is_bcn(self) -> bool {
        matches!(
            self,
            Self::Dxt1 | Self::Dxt3 | Self::Dxt5 | Self::Bc4 | Self::Bc5 | Self::Bc6H | Self::Bc7
        )
    }

    /// 是否为 Crunch 压缩（sig `0x4878`；原始或 Unity 变体由 [`Self::is_unity_crunch`]
    /// 按版本判定）。
    ///
    /// 覆盖 DXT1Crunched(28) / DXT5Crunched(29) / ETC_RGB4Crunched(64) /
    /// ETC2_RGBA8Crunched(65)。两段式解码：先解出含全部 face / mip 的 `Crn*Texture`
    /// （每 mip 持原始 BC1/BC3/ETC1/ETC2_RGBA8 block），再对 level 0 走 `bcn`/`etc`
    /// 转 RGBA。cubemap（6 面）纵向拼接成一张图、只导出最大 mip——详见 [`decode_crunch`]。
    pub fn is_crunch(self) -> bool {
        matches!(
            self,
            Self::Dxt1Crunched
                | Self::Dxt5Crunched
                | Self::EtcRgb4Crunched
                | Self::Etc2Rgba8Crunched
        )
    }

    /// 是否走 **Unity 变体** crunch（对齐 AssetRipper `CrunchHandler.IsUseUnityCrunch`）。
    ///
    /// 版本分水岭：`version >= 2017.3` → Unity 变体（含该版本的 DXT crunched，因 Unity
    /// 在 2017.3 切换了 Crunch fork）；或 format 为 ETC crunch（64/65，任意版本都走 Unity
    /// 变体——ETC crunch 无原始变体）。否则（<2017.3 的 DXT crunched）走**原始** crunch。
    /// 两者 sig 都是 `0x4878` 但 bitstream 不兼容，[`decode_crunch`] 据此分派到 `crn`
    /// / `crn_unity` 模块。
    pub fn is_unity_crunch(self, ver: crate::UnityVersion) -> bool {
        ver >= crate::UnityVersion::new(2017, 3, 0)
            || matches!(self, Self::EtcRgb4Crunched | Self::Etc2Rgba8Crunched)
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
            // 未压缩 8-bit
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
            Self::R8Sign => "r8s",
            Self::Rg16Sign => "rg16s",
            Self::Rgb24Sign => "rgb24s",
            Self::Rgba32Sign => "rgba32s",
            // 未压缩 16-bit 多通道（unsigned）
            Self::Rg32 => "rg32",
            Self::Rgb48 => "rgb48",
            Self::Rgba64 => "rgba64",
            // 未压缩 16-bit 多通道（signed）
            Self::R16Sign => "r16s",
            Self::Rg32Sign => "rg32s",
            Self::Rgb48Sign => "rgb48s",
            Self::Rgba64Sign => "rgba64s",
            // 未压缩 HDR 浮点
            Self::ArgbFloat => "argbf",
            Self::RHalf => "rh",
            Self::RgHalf => "rgh",
            Self::RgbaHalf => "rgbah",
            Self::RFloat => "rf",
            Self::RgFloat => "rgf",
            Self::RgbaFloat => "rgbaf",
            Self::Rgb9e5Float => "rgb9e5f",
            Self::RgbFloat => "rgbf",
            // BCn / DXT
            Self::Dxt1 => "bc1",
            Self::Dxt3 => "bc2",
            Self::Dxt5 => "bc3",
            Self::Dxt1Crunched => "bc1crn",
            Self::Dxt5Crunched => "bc3crn",
            // ETC crunch 恒为 Unity 变体（无原始变体），label 带 `u` 与 DXT 的原始/Unity
            // 通用 `crn` 区分（DXT 的原始 vs Unity 由版本分水岭决定，label 不区分）。
            Self::EtcRgb4Crunched => "etc1crnu",
            Self::Etc2Rgba8Crunched => "etc2a8crnu",
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
    Decode {
        err: texture_decoder_rs::DecodeError,
    },

    /// Crunch 第一段解码失败（CRN 头解析 / Huffman / palette 损坏）。
    /// `CrnDecodeError` 与 `DecodeError` 是不同类型（同属 `texture_decoder_rs`
    /// 但各自独立），故单独成变体。BC1/BC3 第二段解码失败仍走 [`Decode`]。
    #[snafu(display("Crunch 解码失败: {err:?}"))]
    CrunchDecode {
        err: texture_decoder_rs::crn::CrnDecodeError,
    },

    /// Unity 变体 Crunch 第一段解码失败（Unity fork 的 reference-encoding bitstream
    /// 解析失败）。`CrnUnityDecodeError` 又是另一独立类型，故单独成变体。第二段 BC/ETC
    /// 解码失败仍走 [`Decode`]。
    #[snafu(display("Unity Crunch 解码失败: {err:?}"))]
    CrunchUnityDecode {
        err: texture_decoder_rs::crn_unity::CrnUnityDecodeError,
    },

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
        TextureFormat::Etc2Rgba1 => {
            texture_decoder_rs::etc::decode_etc2_rgb8a1(data, w, h, &mut out)
        }
        TextureFormat::Etc2Rgba8 => {
            texture_decoder_rs::etc::decode_etc2_rgba8(data, w, h, &mut out)
        }
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

/// 解码 Crunch 纹理（DXT1/DXT5/ETC1/ETC2A crunched）为 top-down RGBA8。
///
/// **统一入口 + 版本分水岭**：内部按 AssetRipper `CrunchHandler.IsUseUnityCrunch`
/// （即 [`TextureFormat::is_unity_crunch`]）分派到两条同构的两段式路径：
///
/// - **原始 Crunch**（`texture_decoder_rs::crn`，对齐 UnityPy `unpack_crunch`）：
///   Unity <2017.3 的 DXT1Crunched/DXT5Crunched。CRN v1.04「chunk encoding」bitstream。
/// - **Unity 变体 Crunch**（`texture_decoder_rs::crn_unity`，对齐 UnityPy `unpack_unity_crunch`）：
///   Unity ≥2017.3 的 DXT crunched（Unity 在 2017.3 切换了 Crunch fork，bitstream 改成
///   不兼容的「reference encoding」）+ **任意版本**的 ETC_RGB4Crunched/ETC2_RGBA8Crunched
///   （ETC crunch 无原始变体）。
///
/// 两者 sig 都是 `0x4878` "Hx"——原始解码器能跑完 Unity 变体数据却输出乱码，故**不能**靠
/// sig 区分，必须按版本 + format 分派（`fmt` 与 `ver` 共同决定走哪条路）。
///
/// # 两段式
///
/// 第一段（crn / crn_unity）把 CRN 解成含全部 face × mip 的 `Crn*Texture`（每 mip 持有
/// **原始 BC1/BC3/ETC1/ETC2_RGBA8 block 字节**，两模块都**不输出 RGBA**）；第二段对目标
/// mip 的 block 走 `bcn`/`etc` 转 RGBA8。block 格式（BC1/BC3/ETC1/ETC2_RGBA8）分派由
/// **CRN 头的 `format` 字段决定**（权威，无需外部传 `TextureFormat`），块边缘裁剪由第二段
/// 解码器内部处理（非 4 倍数尺寸安全）。
///
/// # Mipmap
///
/// CRN 通常带多级 mip（level 0 最大，逐级减半）。本函数**只导出 level 0**（最大分辨率），
/// 其余 mip 丢弃——对齐 AssetRipper 导出器只渲染基础层的语义。
///
/// # Cubemap
///
/// CRN 头 `faces` 字段区分 2D（1 面）与 cubemap（6 面，序 +X/-X/+Y/-Y/+Z/-Z）。遇
/// cubemap 时参照 **AssetRipper**（`TextureConverter.cs:144-181` + `DirectBitmap` 的
/// 各 `Save*` 把 `Height*Depth` 当整图高度）：6 个面**纵向拼接**成一张图——输出宽 =
/// 面宽、高 = 面高 × 6，面 i 落在行 `[i*面高, (i+1)*面高)`。且**不做 Y 翻转**（AssetRipper
/// 对 `ICubemap` 跳过 `FlipY`，源码注释「cubemaps dont need flipping, for some reason」）。
/// 单面 2D 纹理照常 Y 翻转（Unity bottom-up → PNG top-down）。
///
/// # 返回值
///
/// 因 cubemap 纵向拼接会改变输出尺寸，本函数返回 `(rgba, width, height)`——caller 应
/// 用返回的尺寸喂 [`write_png`]，而非 Texture2D 声明的 `m_width`/`m_height`（cubemap
/// 下后者只是单面尺寸）。
///
/// 通道顺序：第二段 BCn/ETC 解码均输出 RGBA，与 [`decode_bcn`] / [`decode_etc`] 路径
/// 一致，无需 B/R 交换。
pub fn decode_crunch(
    data: &[u8],
    fmt: TextureFormat,
    ver: crate::UnityVersion,
) -> Result<(Vec<u8>, u32, u32), TextureExportError> {
    if fmt.is_unity_crunch(ver) {
        decode_crunch_unity(data)
    } else {
        decode_crunch_legacy(data)
    }
}

/// 第二段 block → RGBA8 解码器签名（bcn / etc 家族共用同一签名）。
type BlockDecode =
    fn(&[u8], usize, usize, &mut [u8]) -> Result<(), texture_decoder_rs::DecodeError>;

/// 把每面 level-0 的 block 字节逐面解 RGBA8、纵向拼接、按 cubemap 规则决定是否 Y 翻转。
///
/// `face_mips[face]` 为该面的 level-0 block 字节；`face_w`/`face_h` 为单面像素尺寸。
/// 原始 crunch 与 Unity 变体共用此尾部管线（两路径只在前段 decode + format 分派不同）。
fn stitch_crunch_faces(
    face_mips: &[&[u8]],
    face_w: usize,
    face_h: usize,
    block_decode: BlockDecode,
) -> Result<(Vec<u8>, u32, u32), TextureExportError> {
    let n_faces = face_mips.len();
    let is_cubemap = n_faces == 6;
    let out_w = face_w;
    let out_h = face_h * n_faces;
    let mut out = vec![0u8; out_w * out_h * 4];
    let face_stride = face_w * face_h * 4;

    for (i, blocks) in face_mips.iter().enumerate() {
        let slot = &mut out[i * face_stride..(i + 1) * face_stride];
        block_decode(blocks, face_w, face_h, slot)
            .map_err(|err| TextureExportError::Decode { err })?;
    }

    // 单面 2D：Y 翻转（Unity bottom-up → PNG top-down，对齐其他解码路径）。
    // Cubemap：不翻转（对齐 AssetRipper `ICubemap` 跳过 FlipY）。
    if !is_cubemap {
        flip_vertical_in_place(&mut out, out_w, out_h);
    }

    Ok((out, out_w as u32, out_h as u32))
}

/// 原始 Crunch（`crn` 模块）解码：CRN v1.04 → BC1/BC3 block → RGBA8。
fn decode_crunch_legacy(data: &[u8]) -> Result<(Vec<u8>, u32, u32), TextureExportError> {
    use texture_decoder_rs::crn::{CrnDecodeError, CrnFormat};

    let tex = texture_decoder_rs::crn::decode(data)
        .map_err(|err| TextureExportError::CrunchDecode { err })?;

    let missing = || TextureExportError::CrunchDecode {
        err: CrnDecodeError::InvalidHeader,
    };
    let base_mip = tex
        .faces
        .first()
        .and_then(|f| f.levels.first())
        .ok_or_else(missing)?;
    let (face_w, face_h) = (base_mip.width as usize, base_mip.height as usize);

    // BC1/BC3 分派由 CRN 头 format 决定（权威）。原始 crunch 只产 BC1/BC3。
    let block_decode: BlockDecode = match tex.format {
        CrnFormat::Bc1 => texture_decoder_rs::bcn::decode_bc1,
        CrnFormat::Bc3 => texture_decoder_rs::bcn::decode_bc3,
    };

    // 逐面取 level-0 block 字节（只导出最大 mip，丢弃其余）。
    let face_mips: Vec<&[u8]> = tex
        .faces
        .iter()
        .map(|f| {
            f.levels
                .first()
                .map(|m| m.data.as_slice())
                .ok_or_else(missing)
        })
        .collect::<Result<_, _>>()?;

    stitch_crunch_faces(&face_mips, face_w, face_h, block_decode)
}

/// Unity 变体 Crunch（`crn_unity` 模块）解码：Unity fork CRN →
/// BC1/BC3/ETC1/ETC2_RGBA8 block → RGBA8。
fn decode_crunch_unity(data: &[u8]) -> Result<(Vec<u8>, u32, u32), TextureExportError> {
    use texture_decoder_rs::crn_unity::{CrnUnityDecodeError, CrnUnityFormat};

    let tex = texture_decoder_rs::crn_unity::decode(data)
        .map_err(|err| TextureExportError::CrunchUnityDecode { err })?;

    let missing = || TextureExportError::CrunchUnityDecode {
        err: CrnUnityDecodeError::InvalidHeader,
    };
    let base_mip = tex
        .faces
        .first()
        .and_then(|f| f.levels.first())
        .ok_or_else(missing)?;
    let (face_w, face_h) = (base_mip.width as usize, base_mip.height as usize);

    // block 格式分派由 CRN 头 format 决定（权威）。Unity 变体覆盖 BC1/BC3/ETC1/ETC2_RGBA8。
    let block_decode: BlockDecode = match tex.format {
        CrnUnityFormat::Bc1 => texture_decoder_rs::bcn::decode_bc1,
        CrnUnityFormat::Bc3 => texture_decoder_rs::bcn::decode_bc3,
        CrnUnityFormat::Etc1Rgb4 => texture_decoder_rs::etc::decode_etc1_rgb8,
        CrnUnityFormat::Etc2Rgba8 => texture_decoder_rs::etc::decode_etc2_rgba8,
    };

    let face_mips: Vec<&[u8]> = tex
        .faces
        .iter()
        .map(|f| {
            f.levels
                .first()
                .map(|m| m.data.as_slice())
                .ok_or_else(missing)
        })
        .collect::<Result<_, _>>()?;

    stitch_crunch_faces(&face_mips, face_w, face_h, block_decode)
}

/// 解码未压缩纹理格式为 top-down RGBA8（已垂直翻转，可直接喂 PNG）。
///
/// 与 `decode_astc` / `decode_etc` / `decode_bcn` 同构：`data` 为 Unity 的裸像素流
/// （无 DDS/PKM/KTX 容器头），输出尺寸 = `width * height * 4`。**不经
/// `texture_decoder_rs`**——未压缩格式是纯字节重排/位宽转换/标量映射，无块解码。
///
/// # 数值映射（对齐 AssetRipper `NumericConversion`）
///
/// AssetRipper 是这套语义的权威实现（UnityPy 的 signed/fp32 路径目前引用了不存在的
/// Pillow codec 而不可用）。通道顺序：AssetRipper 输出 BGRA，这里翻成 RGBA。
/// 关键约定：
///
/// - **ARGB4444 / RGBA4444**：2 字节/像素 LE u16，每通道 4-bit，扩展成 8-bit 用 `nibble * 17`
///   （即 `(n << 4) | n`，复制高位到低位）。
/// - **RGB565**：2 字节/像素 LE u16，16-bit 值布局 `RRRRRGGGGGGBBBBB`（R=15-11, G=10-5, B=4-0），
///   **位复制扩展**（非 `*255//31` 缩放）：5-bit→8-bit 用 `(v << 3) | (v >> 2)`、
///   6-bit→8-bit 用 `(v << 2) | (v >> 4)`。
/// - **R16 / 16-bit 多通道（72-74, 79-82）**：`downscale16`：`((v as u32 * 255 + 32895) >> 16)
///   as u8`（对齐 AssetRipper `ConvertUInt16<byte>`，四舍五入 `≈ round(v*255/65535)`）。
/// - **SIGNED 变体（75-82）**：**XOR 符号位**（offset-binary / SNORM → unorm），不是 reinterpret
///   cast。8-bit：`b ^ 0x80`（`-1`→127、`0`→128、`127`→255）；16-bit：`v ^ 0x8000` 再走
///   `downscale16`（`-1`→127、`0`→127、`32767`→255）。对齐 AssetRipper `ChangeSign`。
/// - **HDR 浮点（6, 15-20, 22, 23）**：`byte_out = (v * 255.0) as u8`（Rust `as u8` 饱和截断，
///   等价 AssetRipper `clamp((byte)(v*255), 0, 255)`）。**无 tone mapping / 无 Reinhard / 无
///   exposure**——HDR 值 >1.0 饱和到 255，负值饱和到 0。fp16 经 `half` crate 转 f32。
///   RGB9e5 共享指数解包：`mantissa9 * 2^(exp5 - 24)`，alpha 硬编码 255。
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

    /// 16-bit 无符号 → 8-bit 四舍五入 downscale（对齐 AssetRipper `ConvertUInt16<byte>`）。
    #[inline]
    fn downscale16(v: u16) -> u8 {
        ((v as u32 * 255 + 32895) >> 16) as u8
    }

    /// f32 → u8 线性饱和（对齐 AssetRipper `ConvertSingle<byte>`：`(byte)(v*255)` + clamp）。
    /// Rust `as u8` 对 f32 自 1.45 起为饱和截断，等价 C# 的 `clamp + (byte)`。
    #[inline]
    fn f32_to_u8(v: f32) -> u8 {
        (v * 255.0) as u8
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
        TextureFormat::Rgb24 => {
            if data.len() < n * 3 {
                bail_short!();
            }
            for i in 0..n {
                out[i * 4] = data[i * 3];
                out[i * 4 + 1] = data[i * 3 + 1];
                out[i * 4 + 2] = data[i * 3 + 2];
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
        TextureFormat::Rgba32 => {
            if data.len() < n * 4 {
                bail_short!();
            }
            for i in 0..n {
                let b = i * 4;
                out[b] = data[b];
                out[b + 1] = data[b + 1];
                out[b + 2] = data[b + 2];
                out[b + 3] = data[b + 3];
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
        // ── 8-bit SIGNED：XOR 0x80（offset-binary / SNORM → unorm），对齐 AssetRipper ChangeSign。
        TextureFormat::R8Sign => {
            if data.len() < n {
                bail_short!();
            }
            for i in 0..n {
                out[i * 4] = data[i] ^ 0x80;
                out[i * 4 + 1] = 0;
                out[i * 4 + 2] = 0;
                out[i * 4 + 3] = 255;
            }
        }
        TextureFormat::Rg16Sign => {
            if data.len() < n * 2 {
                bail_short!();
            }
            for i in 0..n {
                out[i * 4] = data[i * 2] ^ 0x80;
                out[i * 4 + 1] = data[i * 2 + 1] ^ 0x80;
                out[i * 4 + 2] = 0;
                out[i * 4 + 3] = 255;
            }
        }
        TextureFormat::Rgb24Sign => {
            if data.len() < n * 3 {
                bail_short!();
            }
            for i in 0..n {
                out[i * 4] = data[i * 3] ^ 0x80;
                out[i * 4 + 1] = data[i * 3 + 1] ^ 0x80;
                out[i * 4 + 2] = data[i * 3 + 2] ^ 0x80;
                out[i * 4 + 3] = 255;
            }
        }
        TextureFormat::Rgba32Sign => {
            if data.len() < n * 4 {
                bail_short!();
            }
            for i in 0..n {
                let b = i * 4;
                out[b] = data[b] ^ 0x80;
                out[b + 1] = data[b + 1] ^ 0x80;
                out[b + 2] = data[b + 2] ^ 0x80;
                out[b + 3] = data[b + 3] ^ 0x80;
            }
        }
        // ── 16-bit 多通道 unsigned（72-74）：直接 downscale16。
        TextureFormat::Rg32 => {
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
        TextureFormat::Rgb48 => {
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
        TextureFormat::Rgba64 => {
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
        // ── 16-bit SIGNED（79-82）：XOR 0x8000 再 downscale16，对齐 AssetRipper ChangeSign。
        TextureFormat::R16Sign => {
            if data.len() < n * 2 {
                bail_short!();
            }
            for i in 0..n {
                let v = u16::from_le_bytes([data[i * 2], data[i * 2 + 1]]) ^ 0x8000;
                out[i * 4] = downscale16(v);
                out[i * 4 + 1] = 0;
                out[i * 4 + 2] = 0;
                out[i * 4 + 3] = 255;
            }
        }
        TextureFormat::Rg32Sign => {
            if data.len() < n * 4 {
                bail_short!();
            }
            for i in 0..n {
                let r = u16::from_le_bytes([data[i * 4], data[i * 4 + 1]]) ^ 0x8000;
                let g = u16::from_le_bytes([data[i * 4 + 2], data[i * 4 + 3]]) ^ 0x8000;
                out[i * 4] = downscale16(r);
                out[i * 4 + 1] = downscale16(g);
                out[i * 4 + 2] = 0;
                out[i * 4 + 3] = 255;
            }
        }
        TextureFormat::Rgb48Sign => {
            if data.len() < n * 6 {
                bail_short!();
            }
            for i in 0..n {
                let r = u16::from_le_bytes([data[i * 6], data[i * 6 + 1]]) ^ 0x8000;
                let g = u16::from_le_bytes([data[i * 6 + 2], data[i * 6 + 3]]) ^ 0x8000;
                let b = u16::from_le_bytes([data[i * 6 + 4], data[i * 6 + 5]]) ^ 0x8000;
                out[i * 4] = downscale16(r);
                out[i * 4 + 1] = downscale16(g);
                out[i * 4 + 2] = downscale16(b);
                out[i * 4 + 3] = 255;
            }
        }
        TextureFormat::Rgba64Sign => {
            if data.len() < n * 8 {
                bail_short!();
            }
            for i in 0..n {
                let r = u16::from_le_bytes([data[i * 8], data[i * 8 + 1]]) ^ 0x8000;
                let g = u16::from_le_bytes([data[i * 8 + 2], data[i * 8 + 3]]) ^ 0x8000;
                let b = u16::from_le_bytes([data[i * 8 + 4], data[i * 8 + 5]]) ^ 0x8000;
                let a = u16::from_le_bytes([data[i * 8 + 6], data[i * 8 + 7]]) ^ 0x8000;
                out[i * 4] = downscale16(r);
                out[i * 4 + 1] = downscale16(g);
                out[i * 4 + 2] = downscale16(b);
                out[i * 4 + 3] = downscale16(a);
            }
        }
        // ── HDR 浮点（6, 15-20, 22, 23）：线性 ×255 饱和，无 tone mapping。
        // fp16 经 half crate 转 f32；fp32 直接读。RGB9e5 共享指数解包。
        TextureFormat::ArgbFloat => {
            // 输入 [A,R,G,B] fp32×4 → 输出 [R,G,B,A]，每通道 ×255 饱和。
            if data.len() < n * 16 {
                bail_short!();
            }
            for i in 0..n {
                let b = i * 16;
                let a = f32::from_le_bytes([data[b], data[b + 1], data[b + 2], data[b + 3]]);
                let r = f32::from_le_bytes([data[b + 4], data[b + 5], data[b + 6], data[b + 7]]);
                let g = f32::from_le_bytes([data[b + 8], data[b + 9], data[b + 10], data[b + 11]]);
                let bl =
                    f32::from_le_bytes([data[b + 12], data[b + 13], data[b + 14], data[b + 15]]);
                let o = i * 4;
                out[o] = f32_to_u8(r);
                out[o + 1] = f32_to_u8(g);
                out[o + 2] = f32_to_u8(bl);
                out[o + 3] = f32_to_u8(a);
            }
        }
        TextureFormat::RHalf => {
            if data.len() < n * 2 {
                bail_short!();
            }
            for i in 0..n {
                let bits = u16::from_le_bytes([data[i * 2], data[i * 2 + 1]]);
                let r = half::f16::from_bits(bits).to_f32();
                out[i * 4] = f32_to_u8(r);
                out[i * 4 + 1] = 0;
                out[i * 4 + 2] = 0;
                out[i * 4 + 3] = 255;
            }
        }
        TextureFormat::RgHalf => {
            if data.len() < n * 4 {
                bail_short!();
            }
            for i in 0..n {
                let b = i * 4;
                let r = half::f16::from_bits(u16::from_le_bytes([data[b], data[b + 1]])).to_f32();
                let g =
                    half::f16::from_bits(u16::from_le_bytes([data[b + 2], data[b + 3]])).to_f32();
                let o = i * 4;
                out[o] = f32_to_u8(r);
                out[o + 1] = f32_to_u8(g);
                out[o + 2] = 0;
                out[o + 3] = 255;
            }
        }
        TextureFormat::RgbaHalf => {
            if data.len() < n * 8 {
                bail_short!();
            }
            for i in 0..n {
                let b = i * 8;
                let r = half::f16::from_bits(u16::from_le_bytes([data[b], data[b + 1]])).to_f32();
                let g =
                    half::f16::from_bits(u16::from_le_bytes([data[b + 2], data[b + 3]])).to_f32();
                let bl =
                    half::f16::from_bits(u16::from_le_bytes([data[b + 4], data[b + 5]])).to_f32();
                let a =
                    half::f16::from_bits(u16::from_le_bytes([data[b + 6], data[b + 7]])).to_f32();
                let o = i * 4;
                out[o] = f32_to_u8(r);
                out[o + 1] = f32_to_u8(g);
                out[o + 2] = f32_to_u8(bl);
                out[o + 3] = f32_to_u8(a);
            }
        }
        TextureFormat::RFloat => {
            if data.len() < n * 4 {
                bail_short!();
            }
            for i in 0..n {
                let b = i * 4;
                let r = f32::from_le_bytes([data[b], data[b + 1], data[b + 2], data[b + 3]]);
                out[i * 4] = f32_to_u8(r);
                out[i * 4 + 1] = 0;
                out[i * 4 + 2] = 0;
                out[i * 4 + 3] = 255;
            }
        }
        TextureFormat::RgFloat => {
            if data.len() < n * 8 {
                bail_short!();
            }
            for i in 0..n {
                let b = i * 8;
                let r = f32::from_le_bytes([data[b], data[b + 1], data[b + 2], data[b + 3]]);
                let g = f32::from_le_bytes([data[b + 4], data[b + 5], data[b + 6], data[b + 7]]);
                let o = i * 4;
                out[o] = f32_to_u8(r);
                out[o + 1] = f32_to_u8(g);
                out[o + 2] = 0;
                out[o + 3] = 255;
            }
        }
        TextureFormat::RgbaFloat => {
            if data.len() < n * 16 {
                bail_short!();
            }
            for i in 0..n {
                let b = i * 16;
                let r = f32::from_le_bytes([data[b], data[b + 1], data[b + 2], data[b + 3]]);
                let g = f32::from_le_bytes([data[b + 4], data[b + 5], data[b + 6], data[b + 7]]);
                let bl = f32::from_le_bytes([data[b + 8], data[b + 9], data[b + 10], data[b + 11]]);
                let a =
                    f32::from_le_bytes([data[b + 12], data[b + 13], data[b + 14], data[b + 15]]);
                let o = i * 4;
                out[o] = f32_to_u8(r);
                out[o + 1] = f32_to_u8(g);
                out[o + 2] = f32_to_u8(bl);
                out[o + 3] = f32_to_u8(a);
            }
        }
        TextureFormat::RgbFloat => {
            if data.len() < n * 12 {
                bail_short!();
            }
            for i in 0..n {
                let b = i * 12;
                let r = f32::from_le_bytes([data[b], data[b + 1], data[b + 2], data[b + 3]]);
                let g = f32::from_le_bytes([data[b + 4], data[b + 5], data[b + 6], data[b + 7]]);
                let bl = f32::from_le_bytes([data[b + 8], data[b + 9], data[b + 10], data[b + 11]]);
                let o = i * 4;
                out[o] = f32_to_u8(r);
                out[o + 1] = f32_to_u8(g);
                out[o + 2] = f32_to_u8(bl);
                out[o + 3] = 255;
            }
        }
        TextureFormat::Rgb9e5Float => {
            // 32-bit LE packed：[5-bit exp@27-31][9-bit B@18-26][9-bit G@9-17][9-bit R@0-8]。
            // value = mantissa9 * 2^(exp - 24)；alpha 硬编码 255。对齐 AssetRipper ColorRGB9e5。
            if data.len() < n * 4 {
                bail_short!();
            }
            for i in 0..n {
                let b = i * 4;
                let packed = u32::from_le_bytes([data[b], data[b + 1], data[b + 2], data[b + 3]]);
                let exp = ((packed >> 27) & 0x1F) as i32 - 24;
                let scale = f32::powi(2.0, exp);
                let r = (packed & 0x1FF) as f32 * scale;
                let g = ((packed >> 9) & 0x1FF) as f32 * scale;
                let bl = ((packed >> 18) & 0x1FF) as f32 * scale;
                let o = i * 4;
                out[o] = f32_to_u8(r);
                out[o + 1] = f32_to_u8(g);
                out[o + 2] = f32_to_u8(bl);
                out[o + 3] = 255;
            }
        }
        _ => return UnsupportedFormatSnafu { fmt }.fail(),
    }

    // Unity bottom-up → PNG top-down。
    flip_vertical_in_place(&mut out, w, h);
    Ok(out)
}

/// 逐行 reverse 实现垂直翻转（in-place，无额外分配）。
///
/// `pub(crate)` 以便 [`crate::sprite_export`] 在裁剪/旋转时复用（外部不暴露）。
pub(crate) fn flip_vertical_in_place(buf: &mut [u8], width: usize, height: usize) {
    let stride = width * 4;
    let mut top = 0;
    let mut bottom = height.saturating_sub(1);
    while top < bottom {
        let (a, b) = buf.split_at_mut(bottom * stride);
        a[top * stride..top * stride + stride].swap_with_slice(&mut b[..stride]);
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
    let img: image::RgbaImage = image::ImageBuffer::from_raw(width, height, rgba).ok_or(
        TextureExportError::BufferSize {
            width,
            height,
            expected,
            actual: 0,
        },
    )?;
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
