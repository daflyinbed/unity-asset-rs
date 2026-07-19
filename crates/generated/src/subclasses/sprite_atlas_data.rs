//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::rectf::Rectf;
use crate::subclasses::secondary_sprite_texture::SecondarySpriteTexture;
use crate::subclasses::vector2f::Vector2f;
use crate::subclasses::vector4f::Vector4f;

#[derive(Debug, Clone)]
pub struct SpriteAtlasData {
    pub texture: PPtr,
    pub alpha_texture: PPtr,
    pub texture_rect: Rectf,
    pub texture_rect_offset: Vector2f,
    pub uv_transform: Vector4f,
    pub atlas_rect_offset: Option<Vector2f>,
    pub downscale_multiplier: f32,
    pub settings_raw: u32,
    pub secondary_textures: Option<Vec<SecondarySpriteTexture>>,
}

impl crate::Parse for SpriteAtlasData {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut texture = None;
        let mut alpha_texture = None;
        let mut texture_rect = None;
        let mut texture_rect_offset = None;
        let mut uv_transform = None;
        let mut atlas_rect_offset = None;
        let mut downscale_multiplier = None;
        let mut settings_raw = None;
        let mut secondary_textures = None;
        texture = Some(crate::PPtr::parse(rdr, ver)?);
        alpha_texture = Some(crate::PPtr::parse(rdr, ver)?);
        texture_rect = Some(Rectf::parse(rdr, ver)?);
        texture_rect_offset = Some(Vector2f::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2017, 1, 1) {
            atlas_rect_offset = Some(Vector2f::parse(rdr, ver)?);
        } else {
        }
        uv_transform = Some(Vector4f::parse(rdr, ver)?);
        downscale_multiplier = Some(rdr.read_f32()?);
        settings_raw = Some(rdr.read_u32()?);
        if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            secondary_textures = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| SecondarySpriteTexture::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
        } else {
        }

        Ok(SpriteAtlasData {
            texture: texture.unwrap(),
            alpha_texture: alpha_texture.unwrap(),
            texture_rect: texture_rect.unwrap(),
            texture_rect_offset: texture_rect_offset.unwrap(),
            uv_transform: uv_transform.unwrap(),
            atlas_rect_offset,
            downscale_multiplier: downscale_multiplier.unwrap(),
            settings_raw: settings_raw.unwrap(),
            secondary_textures,
        })
    }
}
