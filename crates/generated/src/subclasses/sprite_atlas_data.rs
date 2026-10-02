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

impl crate::ToValue for SpriteAtlasData {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("texture".to_string(), self.texture.to_value())),
            Some(("alphaTexture".to_string(), self.alpha_texture.to_value())),
            Some(("textureRect".to_string(), self.texture_rect.to_value())),
            Some((
                "textureRectOffset".to_string(),
                self.texture_rect_offset.to_value(),
            )),
            Some(("uvTransform".to_string(), self.uv_transform.to_value())),
            crate::opt_field("atlasRectOffset", &self.atlas_rect_offset),
            Some((
                "downscaleMultiplier".to_string(),
                self.downscale_multiplier.to_value(),
            )),
            Some(("settingsRaw".to_string(), self.settings_raw.to_value())),
            crate::opt_field("secondaryTextures", &self.secondary_textures),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "texture" => (self.texture).inspect_value(path, offset, limit),
                "alphaTexture" => (self.alpha_texture).inspect_value(path, offset, limit),
                "textureRect" => (self.texture_rect).inspect_value(path, offset, limit),
                "textureRectOffset" => {
                    (self.texture_rect_offset).inspect_value(path, offset, limit)
                }
                "uvTransform" => (self.uv_transform).inspect_value(path, offset, limit),
                "atlasRectOffset" => (self.atlas_rect_offset).inspect_value(path, offset, limit),
                "downscaleMultiplier" => {
                    (self.downscale_multiplier).inspect_value(path, offset, limit)
                }
                "settingsRaw" => (self.settings_raw).inspect_value(path, offset, limit),
                "secondaryTextures" => (self.secondary_textures).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.texture).inspect_value(path, offset, limit)).map(|v| ("texture".to_string(), v)),
            ((self.alpha_texture).inspect_value(path, offset, limit))
                .map(|v| ("alphaTexture".to_string(), v)),
            ((self.texture_rect).inspect_value(path, offset, limit))
                .map(|v| ("textureRect".to_string(), v)),
            ((self.texture_rect_offset).inspect_value(path, offset, limit))
                .map(|v| ("textureRectOffset".to_string(), v)),
            ((self.uv_transform).inspect_value(path, offset, limit))
                .map(|v| ("uvTransform".to_string(), v)),
            ((self.atlas_rect_offset).inspect_value(path, offset, limit))
                .map(|v| ("atlasRectOffset".to_string(), v)),
            ((self.downscale_multiplier).inspect_value(path, offset, limit))
                .map(|v| ("downscaleMultiplier".to_string(), v)),
            ((self.settings_raw).inspect_value(path, offset, limit))
                .map(|v| ("settingsRaw".to_string(), v)),
            ((self.secondary_textures).inspect_value(path, offset, limit))
                .map(|v| ("secondaryTextures".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
