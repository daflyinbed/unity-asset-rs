//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::color_rgba::ColorRgba;

#[derive(Debug, Clone)]
pub struct RenderSettings {
    pub m_fog: bool,
    pub m_fog_color: ColorRgba,
    pub m_fog_mode: i32,
    pub m_fog_density: f32,
    pub m_linear_fog_start: f32,
    pub m_linear_fog_end: f32,
    pub m_ambient_light: ColorRgba,
    pub m_skybox_material: PPtr,
    pub m_halo_strength: f32,
    pub m_flare_strength: f32,
    pub m_halo_texture: PPtr,
    pub m_spot_cookie: PPtr,
    pub m_object_hide_flags: u32,
}

impl crate::Parse for RenderSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_fog = r.read_bool()?;
        r.align(4)?;
        let m_fog_color = ColorRgba::parse(r, ver)?;
        let m_fog_mode = r.read_i32()?;
        let m_fog_density = r.read_f32()?;
        let m_linear_fog_start = r.read_f32()?;
        let m_linear_fog_end = r.read_f32()?;
        let m_ambient_light = ColorRgba::parse(r, ver)?;
        let m_skybox_material = crate::PPtr::parse(r, ver)?;
        let m_halo_strength = r.read_f32()?;
        let m_flare_strength = r.read_f32()?;
        let m_halo_texture = crate::PPtr::parse(r, ver)?;
        let m_spot_cookie = crate::PPtr::parse(r, ver)?;
        let m_object_hide_flags = r.read_u32()?;

        Ok(RenderSettings {
            m_fog,
            m_fog_color,
            m_fog_mode,
            m_fog_density,
            m_linear_fog_start,
            m_linear_fog_end,
            m_ambient_light,
            m_skybox_material,
            m_halo_strength,
            m_flare_strength,
            m_halo_texture,
            m_spot_cookie,
            m_object_hide_flags,
        })
    }
}
