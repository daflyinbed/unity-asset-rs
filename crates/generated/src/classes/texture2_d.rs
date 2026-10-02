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

impl crate::ToValue for Texture2DMNameVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V4_2(v) => v.to_value(),
            Self::V5_0(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V4_2(v) => (v).inspect_value(path, offset, limit),
            Self::V5_0(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for Texture2DMDownscaleFallbackVariant {
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

impl crate::ToValue for Texture2DMCompleteImageSizeVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2020_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2020_1(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for Texture2DMIsReadableVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_5(v) => v.to_value(),
            Self::V2018_2(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_5(v) => (v).inspect_value(path, offset, limit),
            Self::V2018_2(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for Texture2D {
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
            crate::opt_field("m_DownscaleFallback", &self.m_downscale_fallback),
            Some((
                "m_CompleteImageSize".to_string(),
                self.m_complete_image_size.to_value(),
            )),
            Some(("m_Format".to_string(), self.m_format.to_value())),
            crate::opt_field("m_MipMap", &self.m_mip_map),
            crate::opt_field("m_MipCount", &self.m_mip_count),
            crate::opt_field("m_MipsStripped", &self.m_mips_stripped),
            Some(("m_IsReadable".to_string(), self.m_is_readable.to_value())),
            crate::opt_field("m_ReadAllowed", &self.m_read_allowed),
            Some(("m_Depth".to_string(), self.m_depth.to_value())),
            Some(("m_Dimension".to_string(), self.m_dimension.to_value())),
            Some((
                "m_TextureSettings".to_string(),
                self.m_texture_settings.to_value(),
            )),
            crate::opt_field("m_StreamingMipmaps", &self.m_streaming_mipmaps),
            crate::opt_field(
                "m_IgnoreMasterTextureLimit",
                &self.m_ignore_master_texture_limit,
            ),
            crate::opt_field("m_IsPreProcessed", &self.m_is_pre_processed),
            Some((
                "m_LightmapFormat".to_string(),
                self.m_lightmap_format.to_value(),
            )),
            crate::opt_field(
                "m_StreamingMipmapsPriority",
                &self.m_streaming_mipmaps_priority,
            ),
            crate::opt_field("m_IgnoreMipmapLimit", &self.m_ignore_mipmap_limit),
            crate::opt_field("m_ColorSpace", &self.m_color_space),
            crate::opt_field("m_MipmapLimitGroupName", &self.m_mipmap_limit_group_name),
            Some((
                "image data".to_string(),
                crate::Value::Bytes(self.image_data.clone()),
            )),
            crate::opt_field("m_StreamData", &self.m_stream_data),
            crate::opt_field("m_PlatformBlob", &self.m_platform_blob),
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
                "m_DownscaleFallback" => {
                    (self.m_downscale_fallback).inspect_value(path, offset, limit)
                }
                "m_CompleteImageSize" => {
                    (self.m_complete_image_size).inspect_value(path, offset, limit)
                }
                "m_Format" => (self.m_format).inspect_value(path, offset, limit),
                "m_MipMap" => (self.m_mip_map).inspect_value(path, offset, limit),
                "m_MipCount" => (self.m_mip_count).inspect_value(path, offset, limit),
                "m_MipsStripped" => (self.m_mips_stripped).inspect_value(path, offset, limit),
                "m_IsReadable" => (self.m_is_readable).inspect_value(path, offset, limit),
                "m_ReadAllowed" => (self.m_read_allowed).inspect_value(path, offset, limit),
                "m_Depth" => (self.m_depth).inspect_value(path, offset, limit),
                "m_Dimension" => (self.m_dimension).inspect_value(path, offset, limit),
                "m_TextureSettings" => (self.m_texture_settings).inspect_value(path, offset, limit),
                "m_StreamingMipmaps" => {
                    (self.m_streaming_mipmaps).inspect_value(path, offset, limit)
                }
                "m_IgnoreMasterTextureLimit" => {
                    (self.m_ignore_master_texture_limit).inspect_value(path, offset, limit)
                }
                "m_IsPreProcessed" => (self.m_is_pre_processed).inspect_value(path, offset, limit),
                "m_LightmapFormat" => (self.m_lightmap_format).inspect_value(path, offset, limit),
                "m_StreamingMipmapsPriority" => {
                    (self.m_streaming_mipmaps_priority).inspect_value(path, offset, limit)
                }
                "m_IgnoreMipmapLimit" => {
                    (self.m_ignore_mipmap_limit).inspect_value(path, offset, limit)
                }
                "m_ColorSpace" => (self.m_color_space).inspect_value(path, offset, limit),
                "m_MipmapLimitGroupName" => {
                    (self.m_mipmap_limit_group_name).inspect_value(path, offset, limit)
                }
                "image data" => crate::inspect_bytes(&(self.image_data), path, offset, limit),
                "m_StreamData" => (self.m_stream_data).inspect_value(path, offset, limit),
                "m_PlatformBlob" => (self.m_platform_blob).inspect_value(path, offset, limit),
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
            ((self.m_downscale_fallback).inspect_value(path, offset, limit))
                .map(|v| ("m_DownscaleFallback".to_string(), v)),
            ((self.m_complete_image_size).inspect_value(path, offset, limit))
                .map(|v| ("m_CompleteImageSize".to_string(), v)),
            ((self.m_format).inspect_value(path, offset, limit))
                .map(|v| ("m_Format".to_string(), v)),
            ((self.m_mip_map).inspect_value(path, offset, limit))
                .map(|v| ("m_MipMap".to_string(), v)),
            ((self.m_mip_count).inspect_value(path, offset, limit))
                .map(|v| ("m_MipCount".to_string(), v)),
            ((self.m_mips_stripped).inspect_value(path, offset, limit))
                .map(|v| ("m_MipsStripped".to_string(), v)),
            ((self.m_is_readable).inspect_value(path, offset, limit))
                .map(|v| ("m_IsReadable".to_string(), v)),
            ((self.m_read_allowed).inspect_value(path, offset, limit))
                .map(|v| ("m_ReadAllowed".to_string(), v)),
            ((self.m_depth).inspect_value(path, offset, limit)).map(|v| ("m_Depth".to_string(), v)),
            ((self.m_dimension).inspect_value(path, offset, limit))
                .map(|v| ("m_Dimension".to_string(), v)),
            ((self.m_texture_settings).inspect_value(path, offset, limit))
                .map(|v| ("m_TextureSettings".to_string(), v)),
            ((self.m_streaming_mipmaps).inspect_value(path, offset, limit))
                .map(|v| ("m_StreamingMipmaps".to_string(), v)),
            ((self.m_ignore_master_texture_limit).inspect_value(path, offset, limit))
                .map(|v| ("m_IgnoreMasterTextureLimit".to_string(), v)),
            ((self.m_is_pre_processed).inspect_value(path, offset, limit))
                .map(|v| ("m_IsPreProcessed".to_string(), v)),
            ((self.m_lightmap_format).inspect_value(path, offset, limit))
                .map(|v| ("m_LightmapFormat".to_string(), v)),
            ((self.m_streaming_mipmaps_priority).inspect_value(path, offset, limit))
                .map(|v| ("m_StreamingMipmapsPriority".to_string(), v)),
            ((self.m_ignore_mipmap_limit).inspect_value(path, offset, limit))
                .map(|v| ("m_IgnoreMipmapLimit".to_string(), v)),
            ((self.m_color_space).inspect_value(path, offset, limit))
                .map(|v| ("m_ColorSpace".to_string(), v)),
            ((self.m_mipmap_limit_group_name).inspect_value(path, offset, limit))
                .map(|v| ("m_MipmapLimitGroupName".to_string(), v)),
            (crate::inspect_bytes(&(self.image_data), path, offset, limit))
                .map(|v| ("image data".to_string(), v)),
            ((self.m_stream_data).inspect_value(path, offset, limit))
                .map(|v| ("m_StreamData".to_string(), v)),
            ((self.m_platform_blob).inspect_value(path, offset, limit))
                .map(|v| ("m_PlatformBlob".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
