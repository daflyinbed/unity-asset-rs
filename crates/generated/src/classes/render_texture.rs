//! 自动生成。请勿手动修改。

use crate::subclasses::gl_texture_settings::GlTextureSettings;

#[derive(Debug, Clone)]
pub struct RenderTexture {
    pub m_name: String,
    pub m_width: i32,
    pub m_forced_fallback_format: Option<i32>,
    pub m_is_alpha_channel_optional: Option<bool>,
    pub m_height: i32,
    pub m_downscale_fallback: Option<bool>,
    pub m_depth_format: Option<i32>,
    pub m_anti_aliasing: Option<i32>,
    pub m_color_format: i32,
    pub m_is_power_of_two: Option<bool>,
    pub m_is_cubemap: Option<bool>,
    pub m_mip_map: bool,
    pub m_mip_count: Option<i32>,
    pub m_srgb: Option<bool>,
    pub m_depth_stencil_format: Option<i32>,
    pub m_texture_settings: GlTextureSettings,
    pub m_generate_mips: Option<bool>,
    pub m_dimension: Option<i32>,
    pub m_volume_depth: Option<i32>,
    pub m_use_dynamic_scale: Option<bool>,
    pub m_bind_ms: Option<bool>,
    pub m_use_dynamic_scale_explicit: Option<bool>,
    pub m_enable_compatible_format: Option<bool>,
    pub m_enable_random_write: Option<bool>,
    pub m_shadow_sampling_mode: Option<i32>,
}

impl crate::Parse for RenderTexture {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_width = r.read_i32()?;
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
        let m_height = r.read_i32()?;
        let m_downscale_fallback = if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_depth_format = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_anti_aliasing = if *ver >= crate::UnityVersion::new(4, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_color_format = r.read_i32()?;
        let m_is_power_of_two = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_is_cubemap = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_mip_map = r.read_bool()?;
        let m_mip_count = if *ver >= crate::UnityVersion::new(2019, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_srgb = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_depth_stencil_format = if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_texture_settings = GlTextureSettings::parse(r, ver)?;
        let m_generate_mips = if *ver >= crate::UnityVersion::new(4, 3, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_dimension = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_volume_depth = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_use_dynamic_scale = if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_bind_ms = if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_use_dynamic_scale_explicit = if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_enable_compatible_format = if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_enable_random_write = if *ver >= crate::UnityVersion::new(2021, 3, 29) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_shadow_sampling_mode = if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };

        Ok(RenderTexture {
            m_name,
            m_width,
            m_forced_fallback_format,
            m_is_alpha_channel_optional,
            m_height,
            m_downscale_fallback,
            m_depth_format,
            m_anti_aliasing,
            m_color_format,
            m_is_power_of_two,
            m_is_cubemap,
            m_mip_map,
            m_mip_count,
            m_srgb,
            m_depth_stencil_format,
            m_texture_settings,
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
