//! 自动生成。请勿手动修改。

use crate::subclasses::gl_texture_settings::GlTextureSettings;
use crate::subclasses::streaming_info::StreamingInfo;


/// 版本枚举：Texture2DArrayMDownscaleFallbackVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum Texture2DArrayMDownscaleFallbackVariant {
    Default(bool),
    V2020_2(bool),
}

impl crate::Parse for Texture2DArrayMDownscaleFallbackVariant {
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

/// 版本枚举：Texture2DArrayMIsReadableVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum Texture2DArrayMIsReadableVariant {
    Default(bool),
    V2023_2(bool),
}

impl crate::Parse for Texture2DArrayMIsReadableVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2023_2(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct Texture2DArray {
    pub m_name: String,
    pub m_width: i32,
    pub m_forced_fallback_format: Option<i32>,
    pub m_is_alpha_channel_optional: Option<bool>,
    pub m_height: i32,
    pub m_color_space: i32,
    pub m_downscale_fallback: Option<Texture2DArrayMDownscaleFallbackVariant>,
    pub m_depth: i32,
    pub m_format: i32,
    pub m_mip_count: i32,
    pub m_complete_image_size: u32,
    pub m_texture_settings: GlTextureSettings,
    pub m_mips_stripped: Option<i32>,
    pub m_is_readable: Texture2DArrayMIsReadableVariant,
    pub image_data: Vec<u8>,
    pub m_stream_data: Option<StreamingInfo>,
    pub m_usage_mode: Option<i32>,
    pub m_ignore_mipmap_limit: Option<bool>,
    pub m_mipmap_limit_group_name: Option<String>,
}

impl crate::Parse for Texture2DArray {
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
        let mut m_depth = None;
        let mut m_format = None;
        let mut m_mip_count = None;
        let mut m_complete_image_size = None;
        let mut m_texture_settings = None;
        let mut m_mips_stripped = None;
        let mut m_is_readable = None;
        let mut image_data = None;
        let mut m_stream_data = None;
        let mut m_usage_mode = None;
        let mut m_ignore_mipmap_limit = None;
        let mut m_mipmap_limit_group_name = None;
            m_name = Some(rdr.read_aligned_string()?);
        if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            m_is_alpha_channel_optional = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_color_space = Some(rdr.read_i32()?);
            m_format = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            m_forced_fallback_format = Some(rdr.read_i32()?);
            m_downscale_fallback = Some(Texture2DArrayMDownscaleFallbackVariant::parse(rdr, ver)?);
            m_is_alpha_channel_optional = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_color_space = Some(rdr.read_i32()?);
            m_format = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_forced_fallback_format = Some(rdr.read_i32()?);
            m_downscale_fallback = Some(Texture2DArrayMDownscaleFallbackVariant::parse(rdr, ver)?);
            m_color_space = Some(rdr.read_i32()?);
            m_format = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            m_forced_fallback_format = Some(rdr.read_i32()?);
            m_downscale_fallback = Some(Texture2DArrayMDownscaleFallbackVariant::parse(rdr, ver)?);
        } else {
        }
            m_width = Some(rdr.read_i32()?);
            m_height = Some(rdr.read_i32()?);
            m_depth = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(2019, 1, 0) {
        } else {
            m_format = Some(rdr.read_i32()?);
        }
            m_mip_count = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            m_mips_stripped = Some(rdr.read_i32()?);
        } else {
        }
            m_complete_image_size = Some(rdr.read_u32()?);
            m_texture_settings = Some(GlTextureSettings::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            m_usage_mode = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
        } else {
            m_color_space = Some(rdr.read_i32()?);
        }
            m_is_readable = Some(Texture2DArrayMIsReadableVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            m_ignore_mipmap_limit = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_mipmap_limit_group_name = Some(rdr.read_aligned_string()?);
        } else {
        }
            image_data = Some(( { let n = rdr.read_i32()? as usize; rdr.read_bytes(n) } )?);
            rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_stream_data = Some(StreamingInfo::parse(rdr, ver)?);
        } else {
        }

        Ok(Texture2DArray {
            m_name: m_name.unwrap(),
            m_width: m_width.unwrap(),
            m_forced_fallback_format,
            m_is_alpha_channel_optional,
            m_height: m_height.unwrap(),
            m_color_space: m_color_space.unwrap(),
            m_downscale_fallback,
            m_depth: m_depth.unwrap(),
            m_format: m_format.unwrap(),
            m_mip_count: m_mip_count.unwrap(),
            m_complete_image_size: m_complete_image_size.unwrap(),
            m_texture_settings: m_texture_settings.unwrap(),
            m_mips_stripped,
            m_is_readable: m_is_readable.unwrap(),
            image_data: image_data.unwrap(),
            m_stream_data,
            m_usage_mode,
            m_ignore_mipmap_limit,
            m_mipmap_limit_group_name,
        })
    }
}
