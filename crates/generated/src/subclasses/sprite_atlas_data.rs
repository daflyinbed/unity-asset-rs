//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::rectf::Rectf;
use crate::subclasses::vector2f::Vector2f;
use crate::subclasses::vector4f::Vector4f;

#[derive(Debug, Clone)]
pub struct SpriteAtlasData {
    pub texture: PPtr,
    pub alpha_texture: PPtr,
    pub texture_rect: Rectf,
    pub texture_rect_offset: Vector2f,
    pub uv_transform: Vector4f,
    pub downscale_multiplier: f32,
    pub settings_raw: u32,
}

impl crate::Parse for SpriteAtlasData {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let texture = crate::PPtr::parse(r, ver)?;
        let alpha_texture = crate::PPtr::parse(r, ver)?;
        let texture_rect = Rectf::parse(r, ver)?;
        let texture_rect_offset = Vector2f::parse(r, ver)?;
        let uv_transform = Vector4f::parse(r, ver)?;
        let downscale_multiplier = r.read_f32()?;
        let settings_raw = r.read_u32()?;

        Ok(SpriteAtlasData {
            texture,
            alpha_texture,
            texture_rect,
            texture_rect_offset,
            uv_transform,
            downscale_multiplier,
            settings_raw,
        })
    }
}
