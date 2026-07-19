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
