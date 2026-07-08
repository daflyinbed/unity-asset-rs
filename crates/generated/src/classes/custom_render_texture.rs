//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::color_rgba::ColorRgba;
use crate::subclasses::gl_texture_settings::GlTextureSettings;
use crate::subclasses::update_zone_info::UpdateZoneInfo;


/// 版本枚举：CustomRenderTextureMDownscaleFallbackVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum CustomRenderTextureMDownscaleFallbackVariant {
    Default(bool),
    V2020_2(bool),
}

impl crate::Parse for CustomRenderTextureMDownscaleFallbackVariant {
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

/// 版本枚举：CustomRenderTextureMSrgbVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum CustomRenderTextureMSrgbVariant {
    Default(bool),
    V2017_3(bool),
}

impl crate::Parse for CustomRenderTextureMSrgbVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            let v = r.read_bool()?;
            Ok(Self::V2017_3(v))
        } else {
            let v = r.read_bool()?;
            r.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：CustomRenderTextureMBindMsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum CustomRenderTextureMBindMsVariant {
    Default(bool),
    V2019_1(bool),
}

impl crate::Parse for CustomRenderTextureMBindMsVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            let v = r.read_bool()?;
            Ok(Self::V2019_1(v))
        } else {
            let v = r.read_bool()?;
            r.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：CustomRenderTextureMEnableCompatibleFormatVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum CustomRenderTextureMEnableCompatibleFormatVariant {
    Default(bool),
    V2021_3_29(bool),
    V2022_1(bool),
    V2022_3_6(bool),
    V2023_1(bool),
    V2023_1_5(bool),
}

impl crate::Parse for CustomRenderTextureMEnableCompatibleFormatVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2023, 1, 5) {
            let v = r.read_bool()?;
            Ok(Self::V2023_1_5(v))
        } else if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Ok(Self::V2023_1(v))
        } else if *ver >= crate::UnityVersion::new(2022, 3, 6) {
            let v = r.read_bool()?;
            Ok(Self::V2022_3_6(v))
        } else if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Ok(Self::V2022_1(v))
        } else if *ver >= crate::UnityVersion::new(2021, 3, 29) {
            let v = r.read_bool()?;
            Ok(Self::V2021_3_29(v))
        } else {
            let v = r.read_bool()?;
            r.align(4)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct CustomRenderTexture {
    pub m_name: String,
    pub m_width: i32,
    pub m_forced_fallback_format: Option<i32>,
    pub m_is_alpha_channel_optional: Option<bool>,
    pub m_height: i32,
    pub m_downscale_fallback: Option<CustomRenderTextureMDownscaleFallbackVariant>,
    pub m_anti_aliasing: i32,
    pub m_depth_format: Option<i32>,
    pub m_color_format: i32,
    pub m_mip_count: Option<i32>,
    pub m_mip_map: bool,
    pub m_depth_stencil_format: Option<i32>,
    pub m_generate_mips: bool,
    pub m_srgb: CustomRenderTextureMSrgbVariant,
    pub m_texture_settings: GlTextureSettings,
    pub m_dimension: i32,
    pub m_volume_depth: i32,
    pub m_use_dynamic_scale: Option<bool>,
    pub m_material: PPtr,
    pub m_bind_ms: Option<CustomRenderTextureMBindMsVariant>,
    pub m_use_dynamic_scale_explicit: Option<bool>,
    pub m_init_source: i32,
    pub m_enable_compatible_format: Option<CustomRenderTextureMEnableCompatibleFormatVariant>,
    pub m_init_material: PPtr,
    pub m_init_color: ColorRgba,
    pub m_enable_random_write: Option<bool>,
    pub m_init_texture: PPtr,
    pub m_update_mode: i32,
    pub m_initialization_mode: i32,
    pub m_update_zone_space: i32,
    pub m_shadow_sampling_mode: Option<i32>,
    pub m_current_update_zone_space: i32,
    pub m_update_zones: Vec<UpdateZoneInfo>,
    pub m_update_period: f32,
    pub m_shader_pass: u32,
    pub m_cubemap_face_mask: u32,
    pub m_double_buffered: bool,
    pub m_wrap_update_zones: bool,
}

impl crate::Parse for CustomRenderTexture {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut m_width = None;
        let mut m_forced_fallback_format = None;
        let mut m_is_alpha_channel_optional = None;
        let mut m_height = None;
        let mut m_downscale_fallback = None;
        let mut m_anti_aliasing = None;
        let mut m_depth_format = None;
        let mut m_color_format = None;
        let mut m_mip_count = None;
        let mut m_mip_map = None;
        let mut m_depth_stencil_format = None;
        let mut m_generate_mips = None;
        let mut m_srgb = None;
        let mut m_texture_settings = None;
        let mut m_dimension = None;
        let mut m_volume_depth = None;
        let mut m_use_dynamic_scale = None;
        let mut m_material = None;
        let mut m_bind_ms = None;
        let mut m_use_dynamic_scale_explicit = None;
        let mut m_init_source = None;
        let mut m_enable_compatible_format = None;
        let mut m_init_material = None;
        let mut m_init_color = None;
        let mut m_enable_random_write = None;
        let mut m_init_texture = None;
        let mut m_update_mode = None;
        let mut m_initialization_mode = None;
        let mut m_update_zone_space = None;
        let mut m_shadow_sampling_mode = None;
        let mut m_current_update_zone_space = None;
        let mut m_update_zones = None;
        let mut m_update_period = None;
        let mut m_shader_pass = None;
        let mut m_cubemap_face_mask = None;
        let mut m_double_buffered = None;
        let mut m_wrap_update_zones = None;
            m_name = Some(r.read_aligned_string()?);
            r.align(4)?;
        if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            m_is_alpha_channel_optional = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            m_forced_fallback_format = Some(r.read_i32()?);
            m_downscale_fallback = Some(CustomRenderTextureMDownscaleFallbackVariant::parse(r, ver)?);
            m_is_alpha_channel_optional = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            m_forced_fallback_format = Some(r.read_i32()?);
            m_downscale_fallback = Some(CustomRenderTextureMDownscaleFallbackVariant::parse(r, ver)?);
        } else {
        }
            m_width = Some(r.read_i32()?);
            m_height = Some(r.read_i32()?);
            m_anti_aliasing = Some(r.read_i32()?);
        if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            m_mip_count = Some(r.read_i32()?);
            m_depth_stencil_format = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2019, 2, 0) {
            m_mip_count = Some(r.read_i32()?);
            m_depth_format = Some(r.read_i32()?);
        } else {
            m_depth_format = Some(r.read_i32()?);
        }
            m_color_format = Some(r.read_i32()?);
            m_mip_map = Some(r.read_bool()?);
            m_generate_mips = Some(r.read_bool()?);
            m_srgb = Some(CustomRenderTextureMSrgbVariant::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            m_use_dynamic_scale = Some(r.read_bool()?);
            m_use_dynamic_scale_explicit = Some(r.read_bool()?);
            m_bind_ms = Some(CustomRenderTextureMBindMsVariant::parse(r, ver)?);
            m_enable_compatible_format = Some(CustomRenderTextureMEnableCompatibleFormatVariant::parse(r, ver)?);
            m_enable_random_write = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2023, 1, 5) {
            m_use_dynamic_scale = Some(r.read_bool()?);
            m_bind_ms = Some(CustomRenderTextureMBindMsVariant::parse(r, ver)?);
            m_enable_compatible_format = Some(CustomRenderTextureMEnableCompatibleFormatVariant::parse(r, ver)?);
            m_enable_random_write = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            m_use_dynamic_scale = Some(r.read_bool()?);
            m_bind_ms = Some(CustomRenderTextureMBindMsVariant::parse(r, ver)?);
            m_enable_compatible_format = Some(CustomRenderTextureMEnableCompatibleFormatVariant::parse(r, ver)?);
        } else if *ver >= crate::UnityVersion::new(2022, 3, 6) {
            m_use_dynamic_scale = Some(r.read_bool()?);
            m_bind_ms = Some(CustomRenderTextureMBindMsVariant::parse(r, ver)?);
            m_enable_compatible_format = Some(CustomRenderTextureMEnableCompatibleFormatVariant::parse(r, ver)?);
            m_enable_random_write = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            m_use_dynamic_scale = Some(r.read_bool()?);
            m_bind_ms = Some(CustomRenderTextureMBindMsVariant::parse(r, ver)?);
            m_enable_compatible_format = Some(CustomRenderTextureMEnableCompatibleFormatVariant::parse(r, ver)?);
        } else if *ver >= crate::UnityVersion::new(2021, 3, 29) {
            m_use_dynamic_scale = Some(r.read_bool()?);
            m_bind_ms = Some(CustomRenderTextureMBindMsVariant::parse(r, ver)?);
            m_enable_compatible_format = Some(CustomRenderTextureMEnableCompatibleFormatVariant::parse(r, ver)?);
            m_enable_random_write = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_use_dynamic_scale = Some(r.read_bool()?);
            m_bind_ms = Some(CustomRenderTextureMBindMsVariant::parse(r, ver)?);
            m_enable_compatible_format = Some(CustomRenderTextureMEnableCompatibleFormatVariant::parse(r, ver)?);
        } else if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            m_use_dynamic_scale = Some(r.read_bool()?);
            m_bind_ms = Some(CustomRenderTextureMBindMsVariant::parse(r, ver)?);
        } else {
        }
            m_texture_settings = Some(GlTextureSettings::parse(r, ver)?);
            m_dimension = Some(r.read_i32()?);
            m_volume_depth = Some(r.read_i32()?);
        if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            m_shadow_sampling_mode = Some(r.read_i32()?);
        } else {
        }
            m_material = Some(crate::PPtr::parse(r, ver)?);
            m_init_source = Some(r.read_i32()?);
            m_init_material = Some(crate::PPtr::parse(r, ver)?);
            m_init_color = Some(ColorRgba::parse(r, ver)?);
            m_init_texture = Some(crate::PPtr::parse(r, ver)?);
            m_update_mode = Some(r.read_i32()?);
            m_initialization_mode = Some(r.read_i32()?);
            m_update_zone_space = Some(r.read_i32()?);
            m_current_update_zone_space = Some(r.read_i32()?);
            m_update_zones = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { UpdateZoneInfo::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_update_period = Some(r.read_f32()?);
            r.align(4)?;
            m_shader_pass = Some(r.read_u32()?);
            m_cubemap_face_mask = Some(r.read_u32()?);
            m_double_buffered = Some(r.read_bool()?);
            r.align(4)?;
            m_wrap_update_zones = Some(r.read_bool()?);
            r.align(4)?;

        Ok(CustomRenderTexture {
            m_name: m_name.unwrap(),
            m_width: m_width.unwrap(),
            m_forced_fallback_format,
            m_is_alpha_channel_optional,
            m_height: m_height.unwrap(),
            m_downscale_fallback,
            m_anti_aliasing: m_anti_aliasing.unwrap(),
            m_depth_format,
            m_color_format: m_color_format.unwrap(),
            m_mip_count,
            m_mip_map: m_mip_map.unwrap(),
            m_depth_stencil_format,
            m_generate_mips: m_generate_mips.unwrap(),
            m_srgb: m_srgb.unwrap(),
            m_texture_settings: m_texture_settings.unwrap(),
            m_dimension: m_dimension.unwrap(),
            m_volume_depth: m_volume_depth.unwrap(),
            m_use_dynamic_scale,
            m_material: m_material.unwrap(),
            m_bind_ms,
            m_use_dynamic_scale_explicit,
            m_init_source: m_init_source.unwrap(),
            m_enable_compatible_format,
            m_init_material: m_init_material.unwrap(),
            m_init_color: m_init_color.unwrap(),
            m_enable_random_write,
            m_init_texture: m_init_texture.unwrap(),
            m_update_mode: m_update_mode.unwrap(),
            m_initialization_mode: m_initialization_mode.unwrap(),
            m_update_zone_space: m_update_zone_space.unwrap(),
            m_shadow_sampling_mode,
            m_current_update_zone_space: m_current_update_zone_space.unwrap(),
            m_update_zones: m_update_zones.unwrap(),
            m_update_period: m_update_period.unwrap(),
            m_shader_pass: m_shader_pass.unwrap(),
            m_cubemap_face_mask: m_cubemap_face_mask.unwrap(),
            m_double_buffered: m_double_buffered.unwrap(),
            m_wrap_update_zones: m_wrap_update_zones.unwrap(),
        })
    }
}
