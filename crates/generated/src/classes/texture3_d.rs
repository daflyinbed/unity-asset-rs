//! 自动生成。请勿手动修改。

use crate::subclasses::gl_texture_settings::GlTextureSettings;
use crate::subclasses::streaming_info::StreamingInfo;


/// 版本枚举：Texture3DMDownscaleFallbackVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum Texture3DMDownscaleFallbackVariant {
    Default(bool),
    V2020_2(bool),
}

impl crate::Parse for Texture3DMDownscaleFallbackVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2020_2(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：Texture3DMFormatVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum Texture3DMFormatVariant {
    Default(u32),
    V5_0(i32),
}

impl crate::Parse for Texture3DMFormatVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = rdr.read_i32()?;
            Ok(Self::V5_0(v))
        } else {
            let v = rdr.read_u32()?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：Texture3DMMipMapVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum Texture3DMMipMapVariant {
    Default(bool),
    V4_0(bool),
}

impl crate::Parse for Texture3DMMipMapVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(4, 0, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V4_0(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：Texture3DMIsReadableVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum Texture3DMIsReadableVariant {
    Default(bool),
    V5_4(bool),
}

impl crate::Parse for Texture3DMIsReadableVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V5_4(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct Texture3D {
    pub m_name: String,
    pub m_width: i32,
    pub m_forced_fallback_format: Option<i32>,
    pub m_is_alpha_channel_optional: Option<bool>,
    pub m_height: i32,
    pub m_color_space: Option<i32>,
    pub m_downscale_fallback: Option<Texture3DMDownscaleFallbackVariant>,
    pub m_complete_image_size: Option<i32>,
    pub m_depth: Option<i32>,
    pub m_format: Option<Texture3DMFormatVariant>,
    pub m_texture_format: Option<i32>,
    pub m_mip_map: Option<Texture3DMMipMapVariant>,
    pub m_mip_count: Option<i32>,
    pub m_is_readable: Option<Texture3DMIsReadableVariant>,
    pub m_data_size: Option<u32>,
    pub m_read_allowed: Option<bool>,
    pub m_texture_settings: GlTextureSettings,
    pub m_image_count: Option<i32>,
    pub image_data: Vec<u8>,
    pub m_texture_dimension: Option<i32>,
    pub m_stream_data: Option<StreamingInfo>,
    pub m_usage_mode: Option<i32>,
    pub m_lightmap_format: Option<i32>,
}

impl crate::Parse for Texture3D {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut m_width = None;
        let mut m_forced_fallback_format = None;
        let mut m_is_alpha_channel_optional = None;
        let mut m_height = None;
        let mut m_color_space = None;
        let mut m_downscale_fallback = None;
        let mut m_complete_image_size = None;
        let mut m_depth = None;
        let mut m_format = None;
        let mut m_texture_format = None;
        let mut m_mip_map = None;
        let mut m_mip_count = None;
        let mut m_is_readable = None;
        let mut m_data_size = None;
        let mut m_read_allowed = None;
        let mut m_texture_settings = None;
        let mut m_image_count = None;
        let mut image_data = None;
        let mut m_texture_dimension = None;
        let mut m_stream_data = None;
        let mut m_usage_mode = None;
        let mut m_lightmap_format = None;
            m_name = Some(rdr.read_aligned_string()?);
        if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            m_is_alpha_channel_optional = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_color_space = Some(rdr.read_i32()?);
            m_format = Some(Texture3DMFormatVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            m_forced_fallback_format = Some(rdr.read_i32()?);
            m_downscale_fallback = Some(Texture3DMDownscaleFallbackVariant::parse(rdr, ver)?);
            m_is_alpha_channel_optional = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_color_space = Some(rdr.read_i32()?);
            m_format = Some(Texture3DMFormatVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_forced_fallback_format = Some(rdr.read_i32()?);
            m_downscale_fallback = Some(Texture3DMDownscaleFallbackVariant::parse(rdr, ver)?);
            m_color_space = Some(rdr.read_i32()?);
            m_format = Some(Texture3DMFormatVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            m_forced_fallback_format = Some(rdr.read_i32()?);
            m_downscale_fallback = Some(Texture3DMDownscaleFallbackVariant::parse(rdr, ver)?);
        } else {
        }
            m_width = Some(rdr.read_i32()?);
            m_height = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_depth = Some(rdr.read_i32()?);
            m_mip_count = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_data_size = Some(rdr.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(5, 2, 0) {
            m_depth = Some(rdr.read_i32()?);
            m_format = Some(Texture3DMFormatVariant::parse(rdr, ver)?);
            m_mip_count = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_data_size = Some(rdr.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_depth = Some(rdr.read_i32()?);
            m_format = Some(Texture3DMFormatVariant::parse(rdr, ver)?);
            m_mip_map = Some(Texture3DMMipMapVariant::parse(rdr, ver)?);
            m_data_size = Some(rdr.read_u32()?);
        } else {
            m_complete_image_size = Some(rdr.read_i32()?);
            m_texture_format = Some(rdr.read_i32()?);
            m_mip_map = Some(Texture3DMMipMapVariant::parse(rdr, ver)?);
            m_is_readable = Some(Texture3DMIsReadableVariant::parse(rdr, ver)?);
            m_read_allowed = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_image_count = Some(rdr.read_i32()?);
            m_texture_dimension = Some(rdr.read_i32()?);
        }
            m_texture_settings = Some(GlTextureSettings::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            m_usage_mode = Some(rdr.read_i32()?);
            m_is_readable = Some(Texture3DMIsReadableVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            m_is_readable = Some(Texture3DMIsReadableVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_lightmap_format = Some(rdr.read_i32()?);
            m_color_space = Some(rdr.read_i32()?);
        } else {
            m_lightmap_format = Some(rdr.read_i32()?);
        }
            image_data = Some(( { let n = rdr.read_i32()? as usize; rdr.read_bytes(n) } )?);
            rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_stream_data = Some(StreamingInfo::parse(rdr, ver)?);
        } else {
        }

        Ok(Texture3D {
            m_name: m_name.unwrap(),
            m_width: m_width.unwrap(),
            m_forced_fallback_format,
            m_is_alpha_channel_optional,
            m_height: m_height.unwrap(),
            m_color_space,
            m_downscale_fallback,
            m_complete_image_size,
            m_depth,
            m_format,
            m_texture_format,
            m_mip_map,
            m_mip_count,
            m_is_readable,
            m_data_size,
            m_read_allowed,
            m_texture_settings: m_texture_settings.unwrap(),
            m_image_count,
            image_data: image_data.unwrap(),
            m_texture_dimension,
            m_stream_data,
            m_usage_mode,
            m_lightmap_format,
        })
    }
}
