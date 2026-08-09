//! 导出层门面:纹理 + Sprite → PNG。
//!
//! **推荐入口**(阶段 7b 起):
//! - [`DecodedImage`]:解码后的中间图像(rgba + width + height)。
//! - [`decode_texture`]:统一 5 路分派(astc / bcn / crunch / etc / uncompressed),
//!   消除 main.rs / sprite_export 中重复的分派逻辑。
//! - [`Environment::decode_image`](`crate::Environment::decode_image`):最高层入口,
//!   自动识别 Texture2D / Sprite、取版本、解 atlas。
//!
//! 底层解码原语(`decode_astc` / `decode_bcn` / `decode_crunch` / `decode_etc` /
//! `decode_uncompressed`)留在 [`texture_export`] 模块,高级用户可直接调。
//!
//! 受 `export` feature 门控(默认开)。

use std::io::Write;

use generated::UnityObject;
use generated::classes::texture2_d::Texture2D;

use crate::UnityVersion;
use crate::environment::Environment;
pub use crate::texture_export::TextureExportError;

/// 解码后的中间图像:top-down RGBA8。统一纹理/Sprite 解码的中间表示,
/// 由 [`decode_texture`] / [`Environment::decode_image`] 产出,可 [`DecodedImage::write_png`]
/// / [`DecodedImage::write_png_path`] 落盘。
#[derive(Debug, Clone)]
pub struct DecodedImage {
    pub rgba: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

impl DecodedImage {
    pub fn new(rgba: Vec<u8>, width: u32, height: u32) -> Self {
        Self {
            rgba,
            width,
            height,
        }
    }

    /// 写入 PNG 到任意 `Write`。复用 [`texture_export::write_png`]。
    pub fn write_png<W: Write>(&self, w: &mut W) -> Result<(), TextureExportError> {
        crate::texture_export::write_png(self.rgba.clone(), self.width, self.height, w)
    }

    /// 创建文件并写入 PNG。文件名后缀不自动加,调用方负责。
    pub fn write_png_path(&self, path: &std::path::Path) -> Result<(), TextureExportError> {
        let mut f =
            std::fs::File::create(path).map_err(|e| TextureExportError::PngIo { source: e })?;
        self.write_png(&mut f)
    }
}

/// [`Environment::decode_image`] 的选项(留扩展点;当前为空结构)。
#[derive(Debug, Clone, Default)]
pub struct DecodeOptions {}

/// 统一 5 路纹理解码分派。对单个 [`Texture2D`] 取像素字节后,按 `TextureFormat`
/// 分派到 astc / bcn / crunch / etc / uncompressed 解码器,产出 [`DecodedImage`]。
///
/// 这取代了原本散在 `main.rs::export_one` 与 `sprite_export::decode_atlas_texture`
/// 的重复分派逻辑。crunch 路径的输出尺寸可能因 cubemap 纵向拼接而变,故尺寸来自
/// 解码结果而非输入 `(w, h)`。
pub fn decode_texture(
    t: &Texture2D,
    env: &Environment,
    ver: UnityVersion,
) -> Result<DecodedImage, TextureExportError> {
    use crate::texture_export::{
        TextureFormat, decode_astc, decode_bcn, decode_crunch, decode_etc, decode_uncompressed,
        texture_pixel_bytes,
    };
    let data = texture_pixel_bytes(t, env).ok_or(TextureExportError::NoImageData)?;
    let (w, h) = (t.m_width.max(0) as u32, t.m_height.max(0) as u32);
    let fmt = TextureFormat::from_i32(t.m_format).ok_or(TextureExportError::UnknownFormat {
        format_id: t.m_format,
    })?;
    // decode_crunch 自带输出尺寸(cubemap 纵向拼接改变高度),其余路径尺寸 = (w, h)。
    let (rgba, ow, oh) = if fmt.astc_block().is_some() {
        (decode_astc(&data, w, h, fmt)?, w, h)
    } else if fmt.is_bcn() {
        (decode_bcn(&data, w, h, fmt)?, w, h)
    } else if fmt.is_crunch() {
        decode_crunch(&data, fmt, ver)?
    } else if fmt.is_etc() {
        (decode_etc(&data, w, h, fmt)?, w, h)
    } else {
        (decode_uncompressed(&data, w, h, fmt)?, w, h)
    };
    Ok(DecodedImage::new(rgba, ow, oh))
}

impl Environment {
    /// 解码对象为 [`DecodedImage`]。自动识别 Texture2D / Sprite:Texture2D 直接解码;
    /// Sprite 解 atlas 纹理 + 裁 textureRect + 应用 packing rotation。
    ///
    /// `opts` 当前为空结构([`DecodeOptions`]),留未来扩展(如输出格式、resize)。
    #[cfg(feature = "export")]
    pub fn decode_image(
        &self,
        obj: &crate::EnvironmentObject,
        _opts: &DecodeOptions,
    ) -> Result<DecodedImage, DecodeImageError> {
        let ver = self
            .collections
            .get(obj.collection.to_index())
            .map(|c| c.unity_version)
            .unwrap_or_else(|| UnityVersion::new(0, 0, 0));
        match &obj.object {
            crate::ParsedObject::Typed(UnityObject::Texture2D(t)) => {
                Ok(decode_texture(t, self, ver)?)
            }
            crate::ParsedObject::Typed(UnityObject::Sprite(s)) => {
                let (rgba, w, h) = crate::sprite_export::sprite_pixel_bytes(self, obj, s, ver)?;
                Ok(DecodedImage::new(rgba, w, h))
            }
            _ => Err(DecodeImageError::UnsupportedClass {
                class_id: obj.class_id,
            }),
        }
    }
}

/// [`Environment::decode_image`] 的错误:统一 Texture2D / Sprite 两路。
#[derive(Debug)]
pub enum DecodeImageError {
    Texture(TextureExportError),
    Sprite(crate::sprite_export::SpriteExportError),
    /// 对象既非 Texture2D 也非 Sprite。
    UnsupportedClass {
        class_id: crate::ClassId,
    },
}

impl std::fmt::Display for DecodeImageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DecodeImageError::Texture(e) => write!(f, "texture: {e}"),
            DecodeImageError::Sprite(e) => write!(f, "sprite: {e}"),
            DecodeImageError::UnsupportedClass { class_id } => {
                write!(f, "unsupported class for decode_image: {class_id}")
            }
        }
    }
}
impl std::error::Error for DecodeImageError {}

impl From<TextureExportError> for DecodeImageError {
    fn from(e: TextureExportError) -> Self {
        DecodeImageError::Texture(e)
    }
}
impl From<crate::sprite_export::SpriteExportError> for DecodeImageError {
    fn from(e: crate::sprite_export::SpriteExportError) -> Self {
        DecodeImageError::Sprite(e)
    }
}

// 重新导出底层原语 + 高层导出函数,保持 `export` 模块作为单一入口。
pub use crate::audio_export::{AudioExportError, DecodedAudio, decode_audio};
pub use crate::sprite_export::{SpriteExportError, export_sprite, sprite_pixel_bytes};
pub use crate::texture_export::{
    TextureFormat, decode_astc, decode_bcn, decode_crunch, decode_etc, decode_uncompressed,
    texture_pixel_bytes, write_png,
};
