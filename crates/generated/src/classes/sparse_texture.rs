//! 自动生成。请勿手动修改。

use crate::subclasses::gl_texture_settings::GlTextureSettings;


/// 版本枚举：SparseTextureMDownscaleFallbackVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SparseTextureMDownscaleFallbackVariant {
    Default(bool),
    V2020_2(bool),
}

impl crate::Parse for SparseTextureMDownscaleFallbackVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            let v = r.read_bool()?;
            Ok(Self::V2020_2(v))
        } else {
            let v = r.read_bool()?;
            r.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：SparseTextureMFormatVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SparseTextureMFormatVariant {
    Default(u32),
    V5_0(i32),
}

impl crate::Parse for SparseTextureMFormatVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = r.read_i32()?;
            Ok(Self::V5_0(v))
        } else {
            let v = r.read_u32()?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct SparseTexture {
    pub m_name: String,
    pub m_width: i32,
    pub m_forced_fallback_format: Option<i32>,
    pub m_is_alpha_channel_optional: Option<bool>,
    pub m_height: i32,
    pub m_color_space: i32,
    pub m_downscale_fallback: Option<SparseTextureMDownscaleFallbackVariant>,
    pub m_mip_count: i32,
    pub m_format: SparseTextureMFormatVariant,
    pub m_texture_settings: GlTextureSettings,
}

impl crate::Parse for SparseTexture {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut m_width = None;
        let mut m_forced_fallback_format = None;
        let mut m_is_alpha_channel_optional = None;
        let mut m_height = None;
        let mut m_color_space = None;
        let mut m_downscale_fallback = None;
        let mut m_mip_count = None;
        let mut m_format = None;
        let mut m_texture_settings = None;
            m_name = Some(r.read_aligned_string()?);
            r.align(4)?;
        if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            m_is_alpha_channel_optional = Some(r.read_bool()?);
            r.align(4)?;
            m_color_space = Some(r.read_i32()?);
            m_format = Some(SparseTextureMFormatVariant::parse(r, ver)?);
        } else if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            m_forced_fallback_format = Some(r.read_i32()?);
            m_downscale_fallback = Some(SparseTextureMDownscaleFallbackVariant::parse(r, ver)?);
            m_is_alpha_channel_optional = Some(r.read_bool()?);
            r.align(4)?;
            m_color_space = Some(r.read_i32()?);
            m_format = Some(SparseTextureMFormatVariant::parse(r, ver)?);
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_forced_fallback_format = Some(r.read_i32()?);
            m_downscale_fallback = Some(SparseTextureMDownscaleFallbackVariant::parse(r, ver)?);
            m_color_space = Some(r.read_i32()?);
            m_format = Some(SparseTextureMFormatVariant::parse(r, ver)?);
        } else if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            m_forced_fallback_format = Some(r.read_i32()?);
            m_downscale_fallback = Some(SparseTextureMDownscaleFallbackVariant::parse(r, ver)?);
        } else {
        }
            m_width = Some(r.read_i32()?);
            m_height = Some(r.read_i32()?);
            m_mip_count = Some(r.read_i32()?);
        if *ver >= crate::UnityVersion::new(2019, 1, 0) {
        } else {
            m_format = Some(SparseTextureMFormatVariant::parse(r, ver)?);
            m_color_space = Some(r.read_i32()?);
        }
            m_texture_settings = Some(GlTextureSettings::parse(r, ver)?);

        Ok(SparseTexture {
            m_name: m_name.unwrap(),
            m_width: m_width.unwrap(),
            m_forced_fallback_format,
            m_is_alpha_channel_optional,
            m_height: m_height.unwrap(),
            m_color_space: m_color_space.unwrap(),
            m_downscale_fallback,
            m_mip_count: m_mip_count.unwrap(),
            m_format: m_format.unwrap(),
            m_texture_settings: m_texture_settings.unwrap(),
        })
    }
}
