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

impl crate::ToValue for Texture2DArrayMDownscaleFallbackVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2020_2(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2020_2(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for Texture2DArrayMIsReadableVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2023_2(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2023_2(v) => (v).inspect_value(path, offset, limit),
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
        image_data = Some(
            ({
                let n = rdr.read_i32()? as usize;
                rdr.read_bytes(n)
            })?,
        );
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

impl crate::ToValue for Texture2DArray {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some(("m_Width".to_string(), self.m_width.to_value())),
            crate::opt_field("m_ForcedFallbackFormat", &self.m_forced_fallback_format),
            crate::opt_field(
                "m_IsAlphaChannelOptional",
                &self.m_is_alpha_channel_optional,
            ),
            Some(("m_Height".to_string(), self.m_height.to_value())),
            Some(("m_ColorSpace".to_string(), self.m_color_space.to_value())),
            crate::opt_field("m_DownscaleFallback", &self.m_downscale_fallback),
            Some(("m_Depth".to_string(), self.m_depth.to_value())),
            Some(("m_Format".to_string(), self.m_format.to_value())),
            Some(("m_MipCount".to_string(), self.m_mip_count.to_value())),
            Some((
                "m_CompleteImageSize".to_string(),
                self.m_complete_image_size.to_value(),
            )),
            Some((
                "m_TextureSettings".to_string(),
                self.m_texture_settings.to_value(),
            )),
            crate::opt_field("m_MipsStripped", &self.m_mips_stripped),
            Some(("m_IsReadable".to_string(), self.m_is_readable.to_value())),
            Some((
                "image data".to_string(),
                crate::Value::Bytes(self.image_data.clone()),
            )),
            crate::opt_field("m_StreamData", &self.m_stream_data),
            crate::opt_field("m_UsageMode", &self.m_usage_mode),
            crate::opt_field("m_IgnoreMipmapLimit", &self.m_ignore_mipmap_limit),
            crate::opt_field("m_MipmapLimitGroupName", &self.m_mipmap_limit_group_name),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Name" => (self.m_name).inspect_value(path, offset, limit),
                "m_Width" => (self.m_width).inspect_value(path, offset, limit),
                "m_ForcedFallbackFormat" => {
                    (self.m_forced_fallback_format).inspect_value(path, offset, limit)
                }
                "m_IsAlphaChannelOptional" => {
                    (self.m_is_alpha_channel_optional).inspect_value(path, offset, limit)
                }
                "m_Height" => (self.m_height).inspect_value(path, offset, limit),
                "m_ColorSpace" => (self.m_color_space).inspect_value(path, offset, limit),
                "m_DownscaleFallback" => {
                    (self.m_downscale_fallback).inspect_value(path, offset, limit)
                }
                "m_Depth" => (self.m_depth).inspect_value(path, offset, limit),
                "m_Format" => (self.m_format).inspect_value(path, offset, limit),
                "m_MipCount" => (self.m_mip_count).inspect_value(path, offset, limit),
                "m_CompleteImageSize" => {
                    (self.m_complete_image_size).inspect_value(path, offset, limit)
                }
                "m_TextureSettings" => (self.m_texture_settings).inspect_value(path, offset, limit),
                "m_MipsStripped" => (self.m_mips_stripped).inspect_value(path, offset, limit),
                "m_IsReadable" => (self.m_is_readable).inspect_value(path, offset, limit),
                "image data" => crate::inspect_bytes(&(self.image_data), path, offset, limit),
                "m_StreamData" => (self.m_stream_data).inspect_value(path, offset, limit),
                "m_UsageMode" => (self.m_usage_mode).inspect_value(path, offset, limit),
                "m_IgnoreMipmapLimit" => {
                    (self.m_ignore_mipmap_limit).inspect_value(path, offset, limit)
                }
                "m_MipmapLimitGroupName" => {
                    (self.m_mipmap_limit_group_name).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_width).inspect_value(path, offset, limit)).map(|v| ("m_Width".to_string(), v)),
            ((self.m_forced_fallback_format).inspect_value(path, offset, limit))
                .map(|v| ("m_ForcedFallbackFormat".to_string(), v)),
            ((self.m_is_alpha_channel_optional).inspect_value(path, offset, limit))
                .map(|v| ("m_IsAlphaChannelOptional".to_string(), v)),
            ((self.m_height).inspect_value(path, offset, limit))
                .map(|v| ("m_Height".to_string(), v)),
            ((self.m_color_space).inspect_value(path, offset, limit))
                .map(|v| ("m_ColorSpace".to_string(), v)),
            ((self.m_downscale_fallback).inspect_value(path, offset, limit))
                .map(|v| ("m_DownscaleFallback".to_string(), v)),
            ((self.m_depth).inspect_value(path, offset, limit)).map(|v| ("m_Depth".to_string(), v)),
            ((self.m_format).inspect_value(path, offset, limit))
                .map(|v| ("m_Format".to_string(), v)),
            ((self.m_mip_count).inspect_value(path, offset, limit))
                .map(|v| ("m_MipCount".to_string(), v)),
            ((self.m_complete_image_size).inspect_value(path, offset, limit))
                .map(|v| ("m_CompleteImageSize".to_string(), v)),
            ((self.m_texture_settings).inspect_value(path, offset, limit))
                .map(|v| ("m_TextureSettings".to_string(), v)),
            ((self.m_mips_stripped).inspect_value(path, offset, limit))
                .map(|v| ("m_MipsStripped".to_string(), v)),
            ((self.m_is_readable).inspect_value(path, offset, limit))
                .map(|v| ("m_IsReadable".to_string(), v)),
            (crate::inspect_bytes(&(self.image_data), path, offset, limit))
                .map(|v| ("image data".to_string(), v)),
            ((self.m_stream_data).inspect_value(path, offset, limit))
                .map(|v| ("m_StreamData".to_string(), v)),
            ((self.m_usage_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_UsageMode".to_string(), v)),
            ((self.m_ignore_mipmap_limit).inspect_value(path, offset, limit))
                .map(|v| ("m_IgnoreMipmapLimit".to_string(), v)),
            ((self.m_mipmap_limit_group_name).inspect_value(path, offset, limit))
                .map(|v| ("m_MipmapLimitGroupName".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
