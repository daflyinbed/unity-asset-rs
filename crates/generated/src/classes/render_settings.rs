//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::color_rgba::ColorRgba;
use crate::subclasses::spherical_harmonics_l2::SphericalHarmonicsL2;


/// 版本枚举：RenderSettingsMCustomReflectionVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum RenderSettingsMCustomReflectionVariant {
    Default(PPtr),
    V2021_2(PPtr),
}

impl crate::Parse for RenderSettingsMCustomReflectionVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            let v = crate::PPtr::parse(r, ver)?;
            Ok(Self::V2021_2(v))
        } else {
            let v = crate::PPtr::parse(r, ver)?;
            Ok(Self::Default(v))
        }
    }
}
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
    pub m_custom_reflection: Option<RenderSettingsMCustomReflectionVariant>,
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
        let mut m_fog = None;
        let mut m_fog_color = None;
        let mut m_fog_mode = None;
        let mut m_fog_density = None;
        let mut m_linear_fog_start = None;
        let mut m_linear_fog_end = None;
        let mut m_ambient_light = None;
        let mut m_ambient_sky_color = None;
        let mut m_skybox_material = None;
        let mut m_ambient_equator_color = None;
        let mut m_halo_strength = None;
        let mut m_ambient_ground_color = None;
        let mut m_flare_strength = None;
        let mut m_ambient_intensity = None;
        let mut m_halo_texture = None;
        let mut m_flare_fade_speed = None;
        let mut m_ambient_mode = None;
        let mut m_spot_cookie = None;
        let mut m_subtractive_shadow_color = None;
        let mut m_default_reflection_mode = None;
        let mut m_default_reflection_resolution = None;
        let mut m_reflection_bounces = None;
        let mut m_reflection_intensity = None;
        let mut m_custom_reflection = None;
        let mut m_ambient_probe = None;
        let mut m_ambient_probe_in_gamma = None;
        let mut m_generated_skybox_reflection = None;
        let mut m_sun = None;
        let mut m_indirect_specular_color = None;
        let mut m_use_radiance_ambient_probe = None;
            m_fog = Some(r.read_bool()?);
            r.align(4)?;
            m_fog_color = Some(ColorRgba::parse(r, ver)?);
            m_fog_mode = Some(r.read_i32()?);
            m_fog_density = Some(r.read_f32()?);
            m_linear_fog_start = Some(r.read_f32()?);
            m_linear_fog_end = Some(r.read_f32()?);
        if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_ambient_sky_color = Some(ColorRgba::parse(r, ver)?);
            m_ambient_equator_color = Some(ColorRgba::parse(r, ver)?);
            m_ambient_ground_color = Some(ColorRgba::parse(r, ver)?);
            m_ambient_intensity = Some(r.read_f32()?);
            m_ambient_mode = Some(r.read_i32()?);
            r.align(4)?;
            m_subtractive_shadow_color = Some(ColorRgba::parse(r, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_ambient_sky_color = Some(ColorRgba::parse(r, ver)?);
            m_ambient_equator_color = Some(ColorRgba::parse(r, ver)?);
            m_ambient_ground_color = Some(ColorRgba::parse(r, ver)?);
            m_ambient_intensity = Some(r.read_f32()?);
            m_ambient_mode = Some(r.read_i32()?);
            r.align(4)?;
        } else {
            m_ambient_light = Some(ColorRgba::parse(r, ver)?);
        }
            m_skybox_material = Some(crate::PPtr::parse(r, ver)?);
            m_halo_strength = Some(r.read_f32()?);
            m_flare_strength = Some(r.read_f32()?);
        if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_flare_fade_speed = Some(r.read_f32()?);
        } else {
        }
            m_halo_texture = Some(crate::PPtr::parse(r, ver)?);
            m_spot_cookie = Some(crate::PPtr::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            m_default_reflection_mode = Some(r.read_i32()?);
            m_default_reflection_resolution = Some(r.read_i32()?);
            m_reflection_bounces = Some(r.read_i32()?);
            m_reflection_intensity = Some(r.read_f32()?);
            m_custom_reflection = Some(RenderSettingsMCustomReflectionVariant::parse(r, ver)?);
            m_ambient_probe = Some(SphericalHarmonicsL2::parse(r, ver)?);
            m_generated_skybox_reflection = Some(crate::PPtr::parse(r, ver)?);
            m_sun = Some(crate::PPtr::parse(r, ver)?);
            m_indirect_specular_color = Some(ColorRgba::parse(r, ver)?);
            m_use_radiance_ambient_probe = Some(r.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            m_default_reflection_mode = Some(r.read_i32()?);
            m_default_reflection_resolution = Some(r.read_i32()?);
            m_reflection_bounces = Some(r.read_i32()?);
            m_reflection_intensity = Some(r.read_f32()?);
            m_custom_reflection = Some(RenderSettingsMCustomReflectionVariant::parse(r, ver)?);
            m_ambient_probe = Some(SphericalHarmonicsL2::parse(r, ver)?);
            m_generated_skybox_reflection = Some(crate::PPtr::parse(r, ver)?);
            m_sun = Some(crate::PPtr::parse(r, ver)?);
            m_indirect_specular_color = Some(ColorRgba::parse(r, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 3, 0) {
            m_default_reflection_mode = Some(r.read_i32()?);
            m_default_reflection_resolution = Some(r.read_i32()?);
            m_reflection_bounces = Some(r.read_i32()?);
            m_reflection_intensity = Some(r.read_f32()?);
            m_custom_reflection = Some(RenderSettingsMCustomReflectionVariant::parse(r, ver)?);
            m_ambient_probe = Some(SphericalHarmonicsL2::parse(r, ver)?);
            m_generated_skybox_reflection = Some(crate::PPtr::parse(r, ver)?);
            m_sun = Some(crate::PPtr::parse(r, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_default_reflection_mode = Some(r.read_i32()?);
            m_default_reflection_resolution = Some(r.read_i32()?);
            m_reflection_bounces = Some(r.read_i32()?);
            m_reflection_intensity = Some(r.read_f32()?);
            m_custom_reflection = Some(RenderSettingsMCustomReflectionVariant::parse(r, ver)?);
            m_ambient_probe = Some(SphericalHarmonicsL2::parse(r, ver)?);
            m_ambient_probe_in_gamma = Some(SphericalHarmonicsL2::parse(r, ver)?);
            m_generated_skybox_reflection = Some(crate::PPtr::parse(r, ver)?);
            m_sun = Some(crate::PPtr::parse(r, ver)?);
        } else {
        }

        Ok(RenderSettings {
            m_fog: m_fog.unwrap(),
            m_fog_color: m_fog_color.unwrap(),
            m_fog_mode: m_fog_mode.unwrap(),
            m_fog_density: m_fog_density.unwrap(),
            m_linear_fog_start: m_linear_fog_start.unwrap(),
            m_linear_fog_end: m_linear_fog_end.unwrap(),
            m_ambient_light,
            m_ambient_sky_color,
            m_skybox_material: m_skybox_material.unwrap(),
            m_ambient_equator_color,
            m_halo_strength: m_halo_strength.unwrap(),
            m_ambient_ground_color,
            m_flare_strength: m_flare_strength.unwrap(),
            m_ambient_intensity,
            m_halo_texture: m_halo_texture.unwrap(),
            m_flare_fade_speed,
            m_ambient_mode,
            m_spot_cookie: m_spot_cookie.unwrap(),
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
