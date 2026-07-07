//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::color_rgba::ColorRgba;
use crate::subclasses::gl_texture_settings::GlTextureSettings;
use crate::subclasses::update_zone_info::UpdateZoneInfo;

#[derive(Debug, Clone)]
pub struct CustomRenderTexture {
    pub m_name: String,
    pub m_width: i32,
    pub m_forced_fallback_format: Option<i32>,
    pub m_is_alpha_channel_optional: Option<bool>,
    pub m_height: i32,
    pub m_downscale_fallback: Option<bool>,
    pub m_anti_aliasing: i32,
    pub m_depth_format: Option<i32>,
    pub m_color_format: i32,
    pub m_mip_count: Option<i32>,
    pub m_mip_map: bool,
    pub m_depth_stencil_format: Option<i32>,
    pub m_generate_mips: bool,
    pub m_srgb: bool,
    pub m_texture_settings: GlTextureSettings,
    pub m_dimension: i32,
    pub m_volume_depth: i32,
    pub m_use_dynamic_scale: Option<bool>,
    pub m_material: PPtr,
    pub m_bind_ms: Option<bool>,
    pub m_use_dynamic_scale_explicit: Option<bool>,
    pub m_init_material: PPtr,
    pub m_init_source: Option<i32>,
    pub m_enable_compatible_format: Option<bool>,
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
        let m_anti_aliasing = r.read_i32()?;
        let m_depth_format = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_color_format = r.read_i32()?;
        let m_mip_count = if *ver >= crate::UnityVersion::new(2019, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_mip_map = r.read_bool()?;
        let m_depth_stencil_format = if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_generate_mips = r.read_bool()?;
        let m_srgb = r.read_bool()?;
        let m_texture_settings = GlTextureSettings::parse(r, ver)?;
        let m_dimension = r.read_i32()?;
        let m_volume_depth = r.read_i32()?;
        let m_use_dynamic_scale = if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_material = crate::PPtr::parse(r, ver)?;
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
        let m_init_material = crate::PPtr::parse(r, ver)?;
        let m_init_source = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_enable_compatible_format = if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_init_color = ColorRgba::parse(r, ver)?;
        let m_enable_random_write = if *ver >= crate::UnityVersion::new(2021, 3, 29) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_init_texture = crate::PPtr::parse(r, ver)?;
        let m_update_mode = r.read_i32()?;
        let m_initialization_mode = r.read_i32()?;
        let m_update_zone_space = r.read_i32()?;
        let m_shadow_sampling_mode = if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_current_update_zone_space = r.read_i32()?;
        let m_update_zones = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { UpdateZoneInfo::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_update_period = r.read_f32()?;
        r.align(4)?;
        let m_shader_pass = r.read_u32()?;
        let m_cubemap_face_mask = r.read_u32()?;
        let m_double_buffered = r.read_bool()?;
        r.align(4)?;
        let m_wrap_update_zones = r.read_bool()?;
        r.align(4)?;

        Ok(CustomRenderTexture {
            m_name,
            m_width,
            m_forced_fallback_format,
            m_is_alpha_channel_optional,
            m_height,
            m_downscale_fallback,
            m_anti_aliasing,
            m_depth_format,
            m_color_format,
            m_mip_count,
            m_mip_map,
            m_depth_stencil_format,
            m_generate_mips,
            m_srgb,
            m_texture_settings,
            m_dimension,
            m_volume_depth,
            m_use_dynamic_scale,
            m_material,
            m_bind_ms,
            m_use_dynamic_scale_explicit,
            m_init_material,
            m_init_source,
            m_enable_compatible_format,
            m_init_color,
            m_enable_random_write,
            m_init_texture,
            m_update_mode,
            m_initialization_mode,
            m_update_zone_space,
            m_shadow_sampling_mode,
            m_current_update_zone_space,
            m_update_zones,
            m_update_period,
            m_shader_pass,
            m_cubemap_face_mask,
            m_double_buffered,
            m_wrap_update_zones,
        })
    }
}
