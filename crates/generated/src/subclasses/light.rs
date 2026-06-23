//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::bit_field::BitField;
use crate::subclasses::color_rgba::ColorRgba;
use crate::subclasses::shadow_settings::ShadowSettings;

#[derive(Debug, Clone)]
pub struct Light {
    pub m_object_hide_flags: u32,
    pub m_extension_ptr: PPtr,
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_type: i32,
    pub m_color: ColorRgba,
    pub m_intensity: f32,
    pub m_range: f32,
    pub m_spot_angle: f32,
    pub m_cookie_size: f32,
    pub m_shadows: ShadowSettings,
    pub m_cookie: PPtr,
    pub m_draw_halo: bool,
    pub m_actually_lightmapped: bool,
    pub m_flare: PPtr,
    pub m_render_mode: i32,
    pub m_culling_mask: BitField,
    pub m_lightmapping: i32,
    pub m_shadow_samples: i32,
    pub m_shadow_radius: f32,
    pub m_shadow_angle: f32,
    pub m_indirect_intensity: f32,
}

impl crate::Parse for Light {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_extension_ptr = crate::PPtr::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_u8()?;
        r.align(4)?;
        let m_type = r.read_i32()?;
        let m_color = ColorRgba::parse(r, ver)?;
        let m_intensity = r.read_f32()?;
        let m_range = r.read_f32()?;
        let m_spot_angle = r.read_f32()?;
        let m_cookie_size = r.read_f32()?;
        let m_shadows = ShadowSettings::parse(r, ver)?;
        let m_cookie = crate::PPtr::parse(r, ver)?;
        let m_draw_halo = r.read_bool()?;
        let m_actually_lightmapped = r.read_bool()?;
        r.align(4)?;
        let m_flare = crate::PPtr::parse(r, ver)?;
        let m_render_mode = r.read_i32()?;
        let m_culling_mask = BitField::parse(r, ver)?;
        let m_lightmapping = r.read_i32()?;
        let m_shadow_samples = r.read_i32()?;
        let m_shadow_radius = r.read_f32()?;
        let m_shadow_angle = r.read_f32()?;
        let m_indirect_intensity = r.read_f32()?;

        Ok(Light {
            m_object_hide_flags,
            m_extension_ptr,
            m_game_object,
            m_enabled,
            m_type,
            m_color,
            m_intensity,
            m_range,
            m_spot_angle,
            m_cookie_size,
            m_shadows,
            m_cookie,
            m_draw_halo,
            m_actually_lightmapped,
            m_flare,
            m_render_mode,
            m_culling_mask,
            m_lightmapping,
            m_shadow_samples,
            m_shadow_radius,
            m_shadow_angle,
            m_indirect_intensity,
        })
    }
}
