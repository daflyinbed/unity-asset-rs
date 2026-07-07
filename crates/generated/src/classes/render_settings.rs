//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::color_rgba::ColorRgba;
use crate::subclasses::spherical_harmonics_l2::SphericalHarmonicsL2;

#[derive(Debug, Clone)]
pub struct RenderSettings {
    pub m_fog: bool,
    pub m_fog_color: ColorRgba,
    pub m_fog_mode: i32,
    pub m_fog_density: f32,
    pub m_linear_fog_start: f32,
    pub m_linear_fog_end: f32,
    pub m_ambient_light: Option<ColorRgba>,
    pub m_ambient_sky_color: Option<ColorRgba>,
    pub m_skybox_material: PPtr,
    pub m_ambient_equator_color: Option<ColorRgba>,
    pub m_halo_strength: f32,
    pub m_ambient_ground_color: Option<ColorRgba>,
    pub m_flare_strength: f32,
    pub m_ambient_intensity: Option<f32>,
    pub m_halo_texture: PPtr,
    pub m_flare_fade_speed: Option<f32>,
    pub m_ambient_mode: Option<i32>,
    pub m_spot_cookie: PPtr,
    pub m_subtractive_shadow_color: Option<ColorRgba>,
    pub m_default_reflection_mode: Option<i32>,
    pub m_default_reflection_resolution: Option<i32>,
    pub m_reflection_bounces: Option<i32>,
    pub m_reflection_intensity: Option<f32>,
    pub m_custom_reflection: Option<PPtr>,
    pub m_ambient_probe: Option<SphericalHarmonicsL2>,
    pub m_ambient_probe_in_gamma: Option<SphericalHarmonicsL2>,
    pub m_generated_skybox_reflection: Option<PPtr>,
    pub m_sun: Option<PPtr>,
    pub m_indirect_specular_color: Option<ColorRgba>,
    pub m_use_radiance_ambient_probe: Option<bool>,
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
        let m_ambient_light = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(ColorRgba::parse(r, ver)?)
        } else {
            None
        };
        let m_ambient_sky_color = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(ColorRgba::parse(r, ver)?)
        } else {
            None
        };
        let m_skybox_material = crate::PPtr::parse(r, ver)?;
        let m_ambient_equator_color = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(ColorRgba::parse(r, ver)?)
        } else {
            None
        };
        let m_halo_strength = r.read_f32()?;
        let m_ambient_ground_color = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(ColorRgba::parse(r, ver)?)
        } else {
            None
        };
        let m_flare_strength = r.read_f32()?;
        let m_ambient_intensity = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_halo_texture = crate::PPtr::parse(r, ver)?;
        let m_flare_fade_speed = if *ver >= crate::UnityVersion::new(4, 3, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_ambient_mode = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = r.read_i32()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_spot_cookie = crate::PPtr::parse(r, ver)?;
        let m_subtractive_shadow_color = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            Some(ColorRgba::parse(r, ver)?)
        } else {
            None
        };
        let m_default_reflection_mode = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_default_reflection_resolution = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_reflection_bounces = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_reflection_intensity = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_custom_reflection = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(crate::PPtr::parse(r, ver)?)
        } else {
            None
        };
        let m_ambient_probe = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(SphericalHarmonicsL2::parse(r, ver)?)
        } else {
            None
        };
        let m_ambient_probe_in_gamma = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(SphericalHarmonicsL2::parse(r, ver)?)
        } else {
            None
        };
        let m_generated_skybox_reflection = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(crate::PPtr::parse(r, ver)?)
        } else {
            None
        };
        let m_sun = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(crate::PPtr::parse(r, ver)?)
        } else {
            None
        };
        let m_indirect_specular_color = if *ver >= crate::UnityVersion::new(5, 4, 0) {
            Some(ColorRgba::parse(r, ver)?)
        } else {
            None
        };
        let m_use_radiance_ambient_probe = if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };

        Ok(RenderSettings {
            m_fog,
            m_fog_color,
            m_fog_mode,
            m_fog_density,
            m_linear_fog_start,
            m_linear_fog_end,
            m_ambient_light,
            m_ambient_sky_color,
            m_skybox_material,
            m_ambient_equator_color,
            m_halo_strength,
            m_ambient_ground_color,
            m_flare_strength,
            m_ambient_intensity,
            m_halo_texture,
            m_flare_fade_speed,
            m_ambient_mode,
            m_spot_cookie,
            m_subtractive_shadow_color,
            m_default_reflection_mode,
            m_default_reflection_resolution,
            m_reflection_bounces,
            m_reflection_intensity,
            m_custom_reflection,
            m_ambient_probe,
            m_ambient_probe_in_gamma,
            m_generated_skybox_reflection,
            m_sun,
            m_indirect_specular_color,
            m_use_radiance_ambient_probe,
        })
    }
}
