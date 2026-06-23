//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::gl_texture_settings::GlTextureSettings;
use crate::subclasses::texture_parameters::TextureParameters;
use super::texture::Texture;

#[derive(Debug, Clone)]
pub struct ProceduralTexture {
    pub parent: Texture,
    pub m_name: String,
    pub m_substance_material: Option<PPtr>,
    pub m_forced_fallback_format: Option<i32>,
    pub m_is_alpha_channel_optional: Option<bool>,
    pub m_substance_texture_uid: Option<u64>,
    pub m_downscale_fallback: Option<bool>,
    pub r#type: Option<i32>,
    pub alpha_source: Option<i32>,
    pub format: Option<i32>,
    pub m_alpha_source_uid: Option<u64>,
    pub m_texture_parameters: Option<TextureParameters>,
    pub m_texture_settings: Option<GlTextureSettings>,
    pub alpha_source_is_grayscale: Option<bool>,
    pub m_baked_data: Option<Vec<u8>>,
    pub m_mipmaps: Option<i32>,
    pub m_alpha_source_is_inverted: Option<bool>,
    pub m_baked_parameters: Option<TextureParameters>,
    pub m_lightmap_format: Option<i32>,
    pub m_color_space: Option<i32>,
}

impl crate::Parse for ProceduralTexture {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = Texture::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_substance_material = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(crate::PPtr::parse(r, ver)?)
        } else {
            None
        };
        let m_forced_fallback_format = if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_is_alpha_channel_optional = if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_substance_texture_uid = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_u64()?)
        } else {
            None
        };
        let m_downscale_fallback = if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let r#type = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let alpha_source = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let format = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_alpha_source_uid = if *ver >= crate::UnityVersion::new(4, 5, 0) {
            Some(r.read_u64()?)
        } else {
            None
        };
        let m_texture_parameters = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(TextureParameters::parse(r, ver)?)
        } else {
            None
        };
        let m_texture_settings = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(GlTextureSettings::parse(r, ver)?)
        } else {
            None
        };
        let alpha_source_is_grayscale = if *ver >= crate::UnityVersion::new(4, 5, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_baked_data = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u8() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_mipmaps = if *ver >= crate::UnityVersion::new(4, 5, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_alpha_source_is_inverted = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_baked_parameters = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(TextureParameters::parse(r, ver)?)
        } else {
            None
        };
        let m_lightmap_format = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_color_space = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };

        Ok(ProceduralTexture {
            parent,
            m_name,
            m_substance_material,
            m_forced_fallback_format,
            m_is_alpha_channel_optional,
            m_substance_texture_uid,
            m_downscale_fallback,
            r#type,
            alpha_source,
            format,
            m_alpha_source_uid,
            m_texture_parameters,
            m_texture_settings,
            alpha_source_is_grayscale,
            m_baked_data,
            m_mipmaps,
            m_alpha_source_is_inverted,
            m_baked_parameters,
            m_lightmap_format,
            m_color_space,
        })
    }
}
