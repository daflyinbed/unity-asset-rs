//! 自动生成。请勿手动修改。

use crate::subclasses::gl_texture_settings::GlTextureSettings;

/// 版本枚举：RenderTextureMDownscaleFallbackVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum RenderTextureMDownscaleFallbackVariant {
    Default(bool),
    V2020_2(bool),
}

impl crate::Parse for RenderTextureMDownscaleFallbackVariant {
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

impl crate::ToValue for RenderTextureMDownscaleFallbackVariant {
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

/// 版本枚举：RenderTextureMMipMapVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum RenderTextureMMipMapVariant {
    Default(bool),
    V3_5(bool),
}

impl crate::Parse for RenderTextureMMipMapVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V3_5(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for RenderTextureMMipMapVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V3_5(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V3_5(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：RenderTextureMSrgbVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum RenderTextureMSrgbVariant {
    Default(bool),
    V2017_3(bool),
}

impl crate::Parse for RenderTextureMSrgbVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2017_3(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for RenderTextureMSrgbVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_3(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_3(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：RenderTextureMBindMsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum RenderTextureMBindMsVariant {
    Default(bool),
    V2019_1(bool),
}

impl crate::Parse for RenderTextureMBindMsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2019_1(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for RenderTextureMBindMsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2019_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2019_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：RenderTextureMEnableCompatibleFormatVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum RenderTextureMEnableCompatibleFormatVariant {
    Default(bool),
    V2021_3_29(bool),
    V2022_1(bool),
    V2022_3_6(bool),
    V2023_1(bool),
    V2023_1_5(bool),
}

impl crate::Parse for RenderTextureMEnableCompatibleFormatVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2023, 1, 5) {
            let v = rdr.read_bool()?;
            Ok(Self::V2023_1_5(v))
        } else if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V2023_1(v))
        } else if *ver >= crate::UnityVersion::new(2022, 3, 6) {
            let v = rdr.read_bool()?;
            Ok(Self::V2022_3_6(v))
        } else if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V2022_1(v))
        } else if *ver >= crate::UnityVersion::new(2021, 3, 29) {
            let v = rdr.read_bool()?;
            Ok(Self::V2021_3_29(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for RenderTextureMEnableCompatibleFormatVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2021_3_29(v) => v.to_value(),
            Self::V2022_1(v) => v.to_value(),
            Self::V2022_3_6(v) => v.to_value(),
            Self::V2023_1(v) => v.to_value(),
            Self::V2023_1_5(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2021_3_29(v) => (v).inspect_value(path, offset, limit),
            Self::V2022_1(v) => (v).inspect_value(path, offset, limit),
            Self::V2022_3_6(v) => (v).inspect_value(path, offset, limit),
            Self::V2023_1(v) => (v).inspect_value(path, offset, limit),
            Self::V2023_1_5(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct RenderTexture {
    pub m_name: String,
    pub m_width: i32,
    pub m_forced_fallback_format: Option<i32>,
    pub m_is_alpha_channel_optional: Option<bool>,
    pub m_height: i32,
    pub m_downscale_fallback: Option<RenderTextureMDownscaleFallbackVariant>,
    pub m_depth_format: Option<i32>,
    pub m_anti_aliasing: Option<i32>,
    pub m_color_format: i32,
    pub m_is_power_of_two: Option<bool>,
    pub m_is_cubemap: Option<bool>,
    pub m_mip_map: RenderTextureMMipMapVariant,
    pub m_mip_count: Option<i32>,
    pub m_srgb: Option<RenderTextureMSrgbVariant>,
    pub m_depth_stencil_format: Option<i32>,
    pub m_texture_settings: GlTextureSettings,
    pub m_generate_mips: Option<bool>,
    pub m_dimension: Option<i32>,
    pub m_volume_depth: Option<i32>,
    pub m_use_dynamic_scale: Option<bool>,
    pub m_bind_ms: Option<RenderTextureMBindMsVariant>,
    pub m_use_dynamic_scale_explicit: Option<bool>,
    pub m_enable_compatible_format: Option<RenderTextureMEnableCompatibleFormatVariant>,
    pub m_enable_random_write: Option<bool>,
    pub m_shadow_sampling_mode: Option<i32>,
}

impl crate::Parse for RenderTexture {
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
        let mut m_depth_format = None;
        let mut m_anti_aliasing = None;
        let mut m_color_format = None;
        let mut m_is_power_of_two = None;
        let mut m_is_cubemap = None;
        let mut m_mip_map = None;
        let mut m_mip_count = None;
        let mut m_srgb = None;
        let mut m_depth_stencil_format = None;
        let mut m_texture_settings = None;
        let mut m_generate_mips = None;
        let mut m_dimension = None;
        let mut m_volume_depth = None;
        let mut m_use_dynamic_scale = None;
        let mut m_bind_ms = None;
        let mut m_use_dynamic_scale_explicit = None;
        let mut m_enable_compatible_format = None;
        let mut m_enable_random_write = None;
        let mut m_shadow_sampling_mode = None;
        m_name = Some(rdr.read_aligned_string()?);
        if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            m_is_alpha_channel_optional = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            m_forced_fallback_format = Some(rdr.read_i32()?);
            m_downscale_fallback = Some(RenderTextureMDownscaleFallbackVariant::parse(rdr, ver)?);
            m_is_alpha_channel_optional = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            m_forced_fallback_format = Some(rdr.read_i32()?);
            m_downscale_fallback = Some(RenderTextureMDownscaleFallbackVariant::parse(rdr, ver)?);
        } else {
        }
        m_width = Some(rdr.read_i32()?);
        m_height = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            m_anti_aliasing = Some(rdr.read_i32()?);
            m_mip_count = Some(rdr.read_i32()?);
            m_depth_stencil_format = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2019, 2, 0) {
            m_anti_aliasing = Some(rdr.read_i32()?);
            m_mip_count = Some(rdr.read_i32()?);
            m_depth_format = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(4, 2, 0) {
            m_anti_aliasing = Some(rdr.read_i32()?);
            m_depth_format = Some(rdr.read_i32()?);
        } else {
            m_depth_format = Some(rdr.read_i32()?);
        }
        m_color_format = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(4, 0, 0) {
        } else if *ver >= crate::UnityVersion::new(3, 4, 1) {
            m_is_cubemap = Some(rdr.read_bool()?);
        } else {
            m_is_power_of_two = Some(rdr.read_bool()?);
            m_is_cubemap = Some(rdr.read_bool()?);
        }
        m_mip_map = Some(RenderTextureMMipMapVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            m_generate_mips = Some(rdr.read_bool()?);
            m_srgb = Some(RenderTextureMSrgbVariant::parse(rdr, ver)?);
            m_use_dynamic_scale = Some(rdr.read_bool()?);
            m_use_dynamic_scale_explicit = Some(rdr.read_bool()?);
            m_bind_ms = Some(RenderTextureMBindMsVariant::parse(rdr, ver)?);
            m_enable_compatible_format = Some(RenderTextureMEnableCompatibleFormatVariant::parse(
                rdr, ver,
            )?);
            m_enable_random_write = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2023, 1, 5) {
            m_generate_mips = Some(rdr.read_bool()?);
            m_srgb = Some(RenderTextureMSrgbVariant::parse(rdr, ver)?);
            m_use_dynamic_scale = Some(rdr.read_bool()?);
            m_bind_ms = Some(RenderTextureMBindMsVariant::parse(rdr, ver)?);
            m_enable_compatible_format = Some(RenderTextureMEnableCompatibleFormatVariant::parse(
                rdr, ver,
            )?);
            m_enable_random_write = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            m_generate_mips = Some(rdr.read_bool()?);
            m_srgb = Some(RenderTextureMSrgbVariant::parse(rdr, ver)?);
            m_use_dynamic_scale = Some(rdr.read_bool()?);
            m_bind_ms = Some(RenderTextureMBindMsVariant::parse(rdr, ver)?);
            m_enable_compatible_format = Some(RenderTextureMEnableCompatibleFormatVariant::parse(
                rdr, ver,
            )?);
        } else if *ver >= crate::UnityVersion::new(2022, 3, 6) {
            m_generate_mips = Some(rdr.read_bool()?);
            m_srgb = Some(RenderTextureMSrgbVariant::parse(rdr, ver)?);
            m_use_dynamic_scale = Some(rdr.read_bool()?);
            m_bind_ms = Some(RenderTextureMBindMsVariant::parse(rdr, ver)?);
            m_enable_compatible_format = Some(RenderTextureMEnableCompatibleFormatVariant::parse(
                rdr, ver,
            )?);
            m_enable_random_write = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            m_generate_mips = Some(rdr.read_bool()?);
            m_srgb = Some(RenderTextureMSrgbVariant::parse(rdr, ver)?);
            m_use_dynamic_scale = Some(rdr.read_bool()?);
            m_bind_ms = Some(RenderTextureMBindMsVariant::parse(rdr, ver)?);
            m_enable_compatible_format = Some(RenderTextureMEnableCompatibleFormatVariant::parse(
                rdr, ver,
            )?);
        } else if *ver >= crate::UnityVersion::new(2021, 3, 29) {
            m_generate_mips = Some(rdr.read_bool()?);
            m_srgb = Some(RenderTextureMSrgbVariant::parse(rdr, ver)?);
            m_use_dynamic_scale = Some(rdr.read_bool()?);
            m_bind_ms = Some(RenderTextureMBindMsVariant::parse(rdr, ver)?);
            m_enable_compatible_format = Some(RenderTextureMEnableCompatibleFormatVariant::parse(
                rdr, ver,
            )?);
            m_enable_random_write = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_generate_mips = Some(rdr.read_bool()?);
            m_srgb = Some(RenderTextureMSrgbVariant::parse(rdr, ver)?);
            m_use_dynamic_scale = Some(rdr.read_bool()?);
            m_bind_ms = Some(RenderTextureMBindMsVariant::parse(rdr, ver)?);
            m_enable_compatible_format = Some(RenderTextureMEnableCompatibleFormatVariant::parse(
                rdr, ver,
            )?);
        } else if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            m_generate_mips = Some(rdr.read_bool()?);
            m_srgb = Some(RenderTextureMSrgbVariant::parse(rdr, ver)?);
            m_use_dynamic_scale = Some(rdr.read_bool()?);
            m_bind_ms = Some(RenderTextureMBindMsVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_generate_mips = Some(rdr.read_bool()?);
            m_srgb = Some(RenderTextureMSrgbVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_srgb = Some(RenderTextureMSrgbVariant::parse(rdr, ver)?);
        } else {
        }
        m_texture_settings = Some(GlTextureSettings::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            m_dimension = Some(rdr.read_i32()?);
            m_volume_depth = Some(rdr.read_i32()?);
            m_shadow_sampling_mode = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_dimension = Some(rdr.read_i32()?);
            m_volume_depth = Some(rdr.read_i32()?);
        } else {
        }

        Ok(RenderTexture {
            m_name: m_name.unwrap(),
            m_width: m_width.unwrap(),
            m_forced_fallback_format,
            m_is_alpha_channel_optional,
            m_height: m_height.unwrap(),
            m_downscale_fallback,
            m_depth_format,
            m_anti_aliasing,
            m_color_format: m_color_format.unwrap(),
            m_is_power_of_two,
            m_is_cubemap,
            m_mip_map: m_mip_map.unwrap(),
            m_mip_count,
            m_srgb,
            m_depth_stencil_format,
            m_texture_settings: m_texture_settings.unwrap(),
            m_generate_mips,
            m_dimension,
            m_volume_depth,
            m_use_dynamic_scale,
            m_bind_ms,
            m_use_dynamic_scale_explicit,
            m_enable_compatible_format,
            m_enable_random_write,
            m_shadow_sampling_mode,
        })
    }
}

impl crate::ToValue for RenderTexture {
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
            crate::opt_field("m_DepthFormat", &self.m_depth_format),
            crate::opt_field("m_AntiAliasing", &self.m_anti_aliasing),
            Some(("m_ColorFormat".to_string(), self.m_color_format.to_value())),
            crate::opt_field("m_IsPowerOfTwo", &self.m_is_power_of_two),
            crate::opt_field("m_IsCubemap", &self.m_is_cubemap),
            Some(("m_MipMap".to_string(), self.m_mip_map.to_value())),
            crate::opt_field("m_MipCount", &self.m_mip_count),
            crate::opt_field("m_SRGB", &self.m_srgb),
            crate::opt_field("m_DepthStencilFormat", &self.m_depth_stencil_format),
            Some((
                "m_TextureSettings".to_string(),
                self.m_texture_settings.to_value(),
            )),
            crate::opt_field("m_GenerateMips", &self.m_generate_mips),
            crate::opt_field("m_Dimension", &self.m_dimension),
            crate::opt_field("m_VolumeDepth", &self.m_volume_depth),
            crate::opt_field("m_UseDynamicScale", &self.m_use_dynamic_scale),
            crate::opt_field("m_BindMS", &self.m_bind_ms),
            crate::opt_field(
                "m_UseDynamicScaleExplicit",
                &self.m_use_dynamic_scale_explicit,
            ),
            crate::opt_field("m_EnableCompatibleFormat", &self.m_enable_compatible_format),
            crate::opt_field("m_EnableRandomWrite", &self.m_enable_random_write),
            crate::opt_field("m_ShadowSamplingMode", &self.m_shadow_sampling_mode),
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
                "m_DepthFormat" => (self.m_depth_format).inspect_value(path, offset, limit),
                "m_AntiAliasing" => (self.m_anti_aliasing).inspect_value(path, offset, limit),
                "m_ColorFormat" => (self.m_color_format).inspect_value(path, offset, limit),
                "m_IsPowerOfTwo" => (self.m_is_power_of_two).inspect_value(path, offset, limit),
                "m_IsCubemap" => (self.m_is_cubemap).inspect_value(path, offset, limit),
                "m_MipMap" => (self.m_mip_map).inspect_value(path, offset, limit),
                "m_MipCount" => (self.m_mip_count).inspect_value(path, offset, limit),
                "m_SRGB" => (self.m_srgb).inspect_value(path, offset, limit),
                "m_DepthStencilFormat" => {
                    (self.m_depth_stencil_format).inspect_value(path, offset, limit)
                }
                "m_TextureSettings" => (self.m_texture_settings).inspect_value(path, offset, limit),
                "m_GenerateMips" => (self.m_generate_mips).inspect_value(path, offset, limit),
                "m_Dimension" => (self.m_dimension).inspect_value(path, offset, limit),
                "m_VolumeDepth" => (self.m_volume_depth).inspect_value(path, offset, limit),
                "m_UseDynamicScale" => {
                    (self.m_use_dynamic_scale).inspect_value(path, offset, limit)
                }
                "m_BindMS" => (self.m_bind_ms).inspect_value(path, offset, limit),
                "m_UseDynamicScaleExplicit" => {
                    (self.m_use_dynamic_scale_explicit).inspect_value(path, offset, limit)
                }
                "m_EnableCompatibleFormat" => {
                    (self.m_enable_compatible_format).inspect_value(path, offset, limit)
                }
                "m_EnableRandomWrite" => {
                    (self.m_enable_random_write).inspect_value(path, offset, limit)
                }
                "m_ShadowSamplingMode" => {
                    (self.m_shadow_sampling_mode).inspect_value(path, offset, limit)
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
            ((self.m_downscale_fallback).inspect_value(path, offset, limit))
                .map(|v| ("m_DownscaleFallback".to_string(), v)),
            ((self.m_depth_format).inspect_value(path, offset, limit))
                .map(|v| ("m_DepthFormat".to_string(), v)),
            ((self.m_anti_aliasing).inspect_value(path, offset, limit))
                .map(|v| ("m_AntiAliasing".to_string(), v)),
            ((self.m_color_format).inspect_value(path, offset, limit))
                .map(|v| ("m_ColorFormat".to_string(), v)),
            ((self.m_is_power_of_two).inspect_value(path, offset, limit))
                .map(|v| ("m_IsPowerOfTwo".to_string(), v)),
            ((self.m_is_cubemap).inspect_value(path, offset, limit))
                .map(|v| ("m_IsCubemap".to_string(), v)),
            ((self.m_mip_map).inspect_value(path, offset, limit))
                .map(|v| ("m_MipMap".to_string(), v)),
            ((self.m_mip_count).inspect_value(path, offset, limit))
                .map(|v| ("m_MipCount".to_string(), v)),
            ((self.m_srgb).inspect_value(path, offset, limit)).map(|v| ("m_SRGB".to_string(), v)),
            ((self.m_depth_stencil_format).inspect_value(path, offset, limit))
                .map(|v| ("m_DepthStencilFormat".to_string(), v)),
            ((self.m_texture_settings).inspect_value(path, offset, limit))
                .map(|v| ("m_TextureSettings".to_string(), v)),
            ((self.m_generate_mips).inspect_value(path, offset, limit))
                .map(|v| ("m_GenerateMips".to_string(), v)),
            ((self.m_dimension).inspect_value(path, offset, limit))
                .map(|v| ("m_Dimension".to_string(), v)),
            ((self.m_volume_depth).inspect_value(path, offset, limit))
                .map(|v| ("m_VolumeDepth".to_string(), v)),
            ((self.m_use_dynamic_scale).inspect_value(path, offset, limit))
                .map(|v| ("m_UseDynamicScale".to_string(), v)),
            ((self.m_bind_ms).inspect_value(path, offset, limit))
                .map(|v| ("m_BindMS".to_string(), v)),
            ((self.m_use_dynamic_scale_explicit).inspect_value(path, offset, limit))
                .map(|v| ("m_UseDynamicScaleExplicit".to_string(), v)),
            ((self.m_enable_compatible_format).inspect_value(path, offset, limit))
                .map(|v| ("m_EnableCompatibleFormat".to_string(), v)),
            ((self.m_enable_random_write).inspect_value(path, offset, limit))
                .map(|v| ("m_EnableRandomWrite".to_string(), v)),
            ((self.m_shadow_sampling_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_ShadowSamplingMode".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
