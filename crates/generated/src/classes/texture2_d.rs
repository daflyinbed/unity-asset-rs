//! 自动生成。请勿手动修改。

use crate::subclasses::gl_texture_settings::GlTextureSettings;
use crate::subclasses::streaming_info::StreamingInfo;

/// 版本枚举：Texture2DMNameVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum Texture2DMNameVariant {
    Default(String),
    V4_2(String),
    V5_0(String),
}

impl crate::Parse for Texture2DMNameVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = rdr.read_aligned_string()?;
            Ok(Self::V5_0(v))
        } else if *ver >= crate::UnityVersion::new(4, 2, 0) {
            let v = rdr.read_aligned_string()?;
            rdr.align(4)?;
            Ok(Self::V4_2(v))
        } else {
            let v = rdr.read_aligned_string()?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：Texture2DMDownscaleFallbackVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum Texture2DMDownscaleFallbackVariant {
    Default(bool),
    V2020_2(bool),
}

impl crate::Parse for Texture2DMDownscaleFallbackVariant {
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

/// 版本枚举：Texture2DMCompleteImageSizeVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum Texture2DMCompleteImageSizeVariant {
    Default(i32),
    V2020_1(u32),
}

impl crate::Parse for Texture2DMCompleteImageSizeVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = rdr.read_u32()?;
            Ok(Self::V2020_1(v))
        } else {
            let v = rdr.read_i32()?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：Texture2DMIsReadableVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum Texture2DMIsReadableVariant {
    Default(bool),
    V5_5(bool),
    V2018_2(bool),
}

impl crate::Parse for Texture2DMIsReadableVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2018_2(v))
        } else if *ver >= crate::UnityVersion::new(5, 5, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V5_5(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct Texture2D {
    pub m_name: Texture2DMNameVariant,
    pub m_width: i32,
    pub m_forced_fallback_format: Option<i32>,
    pub m_is_alpha_channel_optional: Option<bool>,
    pub m_height: i32,
    pub m_downscale_fallback: Option<Texture2DMDownscaleFallbackVariant>,
    pub m_complete_image_size: Texture2DMCompleteImageSizeVariant,
    pub m_format: i32,
    pub m_mip_map: Option<bool>,
    pub m_mip_count: Option<i32>,
    pub m_mips_stripped: Option<i32>,
    pub m_is_readable: Texture2DMIsReadableVariant,
    pub m_read_allowed: Option<bool>,
    pub m_depth: i32,
    pub m_dimension: i32,
    pub m_texture_settings: GlTextureSettings,
    pub m_streaming_mipmaps: Option<bool>,
    pub m_ignore_master_texture_limit: Option<bool>,
    pub m_is_pre_processed: Option<bool>,
    pub m_lightmap_format: i32,
    pub m_streaming_mipmaps_priority: Option<i32>,
    pub m_ignore_mipmap_limit: Option<bool>,
    pub m_color_space: Option<i32>,
    pub m_mipmap_limit_group_name: Option<String>,
    pub image_data: Vec<u8>,
    pub m_stream_data: Option<StreamingInfo>,
    pub m_platform_blob: Option<Vec<u8>>,
}

impl crate::Parse for Texture2D {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut m_width = None;
        let mut m_forced_fallback_format = None;
        let mut m_is_alpha_channel_optional = None;
        let mut m_height = None;
        let mut m_downscale_fallback = None;
        let mut m_complete_image_size = None;
        let mut m_format = None;
        let mut m_mip_map = None;
        let mut m_mip_count = None;
        let mut m_mips_stripped = None;
        let mut m_is_readable = None;
        let mut m_read_allowed = None;
        let mut m_depth = None;
        let mut m_dimension = None;
        let mut m_texture_settings = None;
        let mut m_streaming_mipmaps = None;
        let mut m_ignore_master_texture_limit = None;
        let mut m_is_pre_processed = None;
        let mut m_lightmap_format = None;
        let mut m_streaming_mipmaps_priority = None;
        let mut m_ignore_mipmap_limit = None;
        let mut m_color_space = None;
        let mut m_mipmap_limit_group_name = None;
        let mut image_data = None;
        let mut m_stream_data = None;
        let mut m_platform_blob = None;
        m_name = Some(Texture2DMNameVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            m_is_alpha_channel_optional = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            m_forced_fallback_format = Some(rdr.read_i32()?);
            m_downscale_fallback = Some(Texture2DMDownscaleFallbackVariant::parse(rdr, ver)?);
            m_is_alpha_channel_optional = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            m_forced_fallback_format = Some(rdr.read_i32()?);
            m_downscale_fallback = Some(Texture2DMDownscaleFallbackVariant::parse(rdr, ver)?);
        } else {
        }
        m_width = Some(rdr.read_i32()?);
        m_height = Some(rdr.read_i32()?);
        m_complete_image_size = Some(Texture2DMCompleteImageSizeVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            m_mips_stripped = Some(rdr.read_i32()?);
        } else {
        }
        m_format = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(5, 2, 0) {
            m_mip_count = Some(rdr.read_i32()?);
        } else {
            m_mip_map = Some(rdr.read_bool()?);
        }
        m_is_readable = Some(Texture2DMIsReadableVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            m_is_pre_processed = Some(rdr.read_bool()?);
            m_ignore_mipmap_limit = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_mipmap_limit_group_name = Some(rdr.read_aligned_string()?);
            m_streaming_mipmaps = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_streaming_mipmaps_priority = Some(rdr.read_i32()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            m_is_pre_processed = Some(rdr.read_bool()?);
            m_ignore_master_texture_limit = Some(rdr.read_bool()?);
            m_streaming_mipmaps = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_streaming_mipmaps_priority = Some(rdr.read_i32()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 4, 9) {
            m_ignore_master_texture_limit = Some(rdr.read_bool()?);
            m_is_pre_processed = Some(rdr.read_bool()?);
            m_streaming_mipmaps = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_streaming_mipmaps_priority = Some(rdr.read_i32()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            m_ignore_master_texture_limit = Some(rdr.read_bool()?);
            m_streaming_mipmaps = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_streaming_mipmaps_priority = Some(rdr.read_i32()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            m_streaming_mipmaps = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_streaming_mipmaps_priority = Some(rdr.read_i32()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 5, 0) {
        } else {
            m_read_allowed = Some(rdr.read_bool()?);
            rdr.align(4)?;
        }
        m_depth = Some(rdr.read_i32()?);
        m_dimension = Some(rdr.read_i32()?);
        m_texture_settings = Some(GlTextureSettings::parse(rdr, ver)?);
        m_lightmap_format = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            m_color_space = Some(rdr.read_i32()?);
            m_platform_blob = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_color_space = Some(rdr.read_i32()?);
        } else {
        }
        image_data = Some(
            ({
                let n = rdr.read_i32()? as usize;
                rdr.read_bytes(n)
            })?,
        );
        rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(5, 3, 0) {
            m_stream_data = Some(StreamingInfo::parse(rdr, ver)?);
        } else {
        }

        Ok(Texture2D {
            m_name: m_name.unwrap(),
            m_width: m_width.unwrap(),
            m_forced_fallback_format,
            m_is_alpha_channel_optional,
            m_height: m_height.unwrap(),
            m_downscale_fallback,
            m_complete_image_size: m_complete_image_size.unwrap(),
            m_format: m_format.unwrap(),
            m_mip_map,
            m_mip_count,
            m_mips_stripped,
            m_is_readable: m_is_readable.unwrap(),
            m_read_allowed,
            m_depth: m_depth.unwrap(),
            m_dimension: m_dimension.unwrap(),
            m_texture_settings: m_texture_settings.unwrap(),
            m_streaming_mipmaps,
            m_ignore_master_texture_limit,
            m_is_pre_processed,
            m_lightmap_format: m_lightmap_format.unwrap(),
            m_streaming_mipmaps_priority,
            m_ignore_mipmap_limit,
            m_color_space,
            m_mipmap_limit_group_name,
            image_data: image_data.unwrap(),
            m_stream_data,
            m_platform_blob,
        })
    }
}
