//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::color_rgb_af::ColorRgbAf;
use crate::subclasses::spherical_harmonics_l2::SphericalHarmonicsL2;

/// 版本枚举：RenderSettingsMCustomReflectionVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum RenderSettingsMCustomReflectionVariant {
    Default(PPtr),
    V2021_2(PPtr),
}

impl crate::Parse for RenderSettingsMCustomReflectionVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            let v = crate::PPtr::parse(rdr, ver)?;
            Ok(Self::V2021_2(v))
        } else {
            let v = crate::PPtr::parse(rdr, ver)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for RenderSettingsMCustomReflectionVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2021_2(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2021_2(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct RenderSettings {
    pub m_fog: bool,
    pub m_fog_color: ColorRgbAf,
    pub m_fog_mode: i32,
    pub m_fog_density: f32,
    pub m_linear_fog_start: f32,
    pub m_linear_fog_end: f32,
    pub m_ambient_light: Option<ColorRgbAf>,
    pub m_ambient_sky_color: Option<ColorRgbAf>,
    pub m_skybox_material: PPtr,
    pub m_ambient_equator_color: Option<ColorRgbAf>,
    pub m_halo_strength: f32,
    pub m_ambient_ground_color: Option<ColorRgbAf>,
    pub m_flare_strength: f32,
    pub m_ambient_intensity: Option<f32>,
    pub m_halo_texture: PPtr,
    pub m_flare_fade_speed: Option<f32>,
    pub m_ambient_mode: Option<i32>,
    pub m_spot_cookie: PPtr,
    pub m_subtractive_shadow_color: Option<ColorRgbAf>,
    pub m_default_reflection_mode: Option<i32>,
    pub m_default_reflection_resolution: Option<i32>,
    pub m_reflection_bounces: Option<i32>,
    pub m_reflection_intensity: Option<f32>,
    pub m_custom_reflection: Option<RenderSettingsMCustomReflectionVariant>,
    pub m_ambient_probe: Option<SphericalHarmonicsL2>,
    pub m_ambient_probe_in_gamma: Option<SphericalHarmonicsL2>,
    pub m_generated_skybox_reflection: Option<PPtr>,
    pub m_sun: Option<PPtr>,
    pub m_indirect_specular_color: Option<ColorRgbAf>,
    pub m_use_radiance_ambient_probe: Option<bool>,
}

impl crate::Parse for RenderSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
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
        m_fog = Some(rdr.read_bool()?);
        rdr.align(4)?;
        m_fog_color = Some(ColorRgbAf::parse(rdr, ver)?);
        m_fog_mode = Some(rdr.read_i32()?);
        m_fog_density = Some(rdr.read_f32()?);
        m_linear_fog_start = Some(rdr.read_f32()?);
        m_linear_fog_end = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_ambient_sky_color = Some(ColorRgbAf::parse(rdr, ver)?);
            m_ambient_equator_color = Some(ColorRgbAf::parse(rdr, ver)?);
            m_ambient_ground_color = Some(ColorRgbAf::parse(rdr, ver)?);
            m_ambient_intensity = Some(rdr.read_f32()?);
            m_ambient_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_subtractive_shadow_color = Some(ColorRgbAf::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_ambient_sky_color = Some(ColorRgbAf::parse(rdr, ver)?);
            m_ambient_equator_color = Some(ColorRgbAf::parse(rdr, ver)?);
            m_ambient_ground_color = Some(ColorRgbAf::parse(rdr, ver)?);
            m_ambient_intensity = Some(rdr.read_f32()?);
            m_ambient_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
        } else {
            m_ambient_light = Some(ColorRgbAf::parse(rdr, ver)?);
        }
        m_skybox_material = Some(crate::PPtr::parse(rdr, ver)?);
        m_halo_strength = Some(rdr.read_f32()?);
        m_flare_strength = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_flare_fade_speed = Some(rdr.read_f32()?);
        } else {
        }
        m_halo_texture = Some(crate::PPtr::parse(rdr, ver)?);
        m_spot_cookie = Some(crate::PPtr::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            m_default_reflection_mode = Some(rdr.read_i32()?);
            m_default_reflection_resolution = Some(rdr.read_i32()?);
            m_reflection_bounces = Some(rdr.read_i32()?);
            m_reflection_intensity = Some(rdr.read_f32()?);
            m_custom_reflection = Some(RenderSettingsMCustomReflectionVariant::parse(rdr, ver)?);
            m_ambient_probe = Some(SphericalHarmonicsL2::parse(rdr, ver)?);
            m_generated_skybox_reflection = Some(crate::PPtr::parse(rdr, ver)?);
            m_sun = Some(crate::PPtr::parse(rdr, ver)?);
            m_indirect_specular_color = Some(ColorRgbAf::parse(rdr, ver)?);
            m_use_radiance_ambient_probe = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            m_default_reflection_mode = Some(rdr.read_i32()?);
            m_default_reflection_resolution = Some(rdr.read_i32()?);
            m_reflection_bounces = Some(rdr.read_i32()?);
            m_reflection_intensity = Some(rdr.read_f32()?);
            m_custom_reflection = Some(RenderSettingsMCustomReflectionVariant::parse(rdr, ver)?);
            m_ambient_probe = Some(SphericalHarmonicsL2::parse(rdr, ver)?);
            m_generated_skybox_reflection = Some(crate::PPtr::parse(rdr, ver)?);
            m_sun = Some(crate::PPtr::parse(rdr, ver)?);
            m_indirect_specular_color = Some(ColorRgbAf::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 3, 0) {
            m_default_reflection_mode = Some(rdr.read_i32()?);
            m_default_reflection_resolution = Some(rdr.read_i32()?);
            m_reflection_bounces = Some(rdr.read_i32()?);
            m_reflection_intensity = Some(rdr.read_f32()?);
            m_custom_reflection = Some(RenderSettingsMCustomReflectionVariant::parse(rdr, ver)?);
            m_ambient_probe = Some(SphericalHarmonicsL2::parse(rdr, ver)?);
            m_generated_skybox_reflection = Some(crate::PPtr::parse(rdr, ver)?);
            m_sun = Some(crate::PPtr::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_default_reflection_mode = Some(rdr.read_i32()?);
            m_default_reflection_resolution = Some(rdr.read_i32()?);
            m_reflection_bounces = Some(rdr.read_i32()?);
            m_reflection_intensity = Some(rdr.read_f32()?);
            m_custom_reflection = Some(RenderSettingsMCustomReflectionVariant::parse(rdr, ver)?);
            m_ambient_probe = Some(SphericalHarmonicsL2::parse(rdr, ver)?);
            m_ambient_probe_in_gamma = Some(SphericalHarmonicsL2::parse(rdr, ver)?);
            m_generated_skybox_reflection = Some(crate::PPtr::parse(rdr, ver)?);
            m_sun = Some(crate::PPtr::parse(rdr, ver)?);
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

impl crate::ToValue for RenderSettings {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Fog".to_string(), self.m_fog.to_value())),
            Some(("m_FogColor".to_string(), self.m_fog_color.to_value())),
            Some(("m_FogMode".to_string(), self.m_fog_mode.to_value())),
            Some(("m_FogDensity".to_string(), self.m_fog_density.to_value())),
            Some((
                "m_LinearFogStart".to_string(),
                self.m_linear_fog_start.to_value(),
            )),
            Some((
                "m_LinearFogEnd".to_string(),
                self.m_linear_fog_end.to_value(),
            )),
            crate::opt_field("m_AmbientLight", &self.m_ambient_light),
            crate::opt_field("m_AmbientSkyColor", &self.m_ambient_sky_color),
            Some((
                "m_SkyboxMaterial".to_string(),
                self.m_skybox_material.to_value(),
            )),
            crate::opt_field("m_AmbientEquatorColor", &self.m_ambient_equator_color),
            Some((
                "m_HaloStrength".to_string(),
                self.m_halo_strength.to_value(),
            )),
            crate::opt_field("m_AmbientGroundColor", &self.m_ambient_ground_color),
            Some((
                "m_FlareStrength".to_string(),
                self.m_flare_strength.to_value(),
            )),
            crate::opt_field("m_AmbientIntensity", &self.m_ambient_intensity),
            Some(("m_HaloTexture".to_string(), self.m_halo_texture.to_value())),
            crate::opt_field("m_FlareFadeSpeed", &self.m_flare_fade_speed),
            crate::opt_field("m_AmbientMode", &self.m_ambient_mode),
            Some(("m_SpotCookie".to_string(), self.m_spot_cookie.to_value())),
            crate::opt_field("m_SubtractiveShadowColor", &self.m_subtractive_shadow_color),
            crate::opt_field("m_DefaultReflectionMode", &self.m_default_reflection_mode),
            crate::opt_field(
                "m_DefaultReflectionResolution",
                &self.m_default_reflection_resolution,
            ),
            crate::opt_field("m_ReflectionBounces", &self.m_reflection_bounces),
            crate::opt_field("m_ReflectionIntensity", &self.m_reflection_intensity),
            crate::opt_field("m_CustomReflection", &self.m_custom_reflection),
            crate::opt_field("m_AmbientProbe", &self.m_ambient_probe),
            crate::opt_field("m_AmbientProbeInGamma", &self.m_ambient_probe_in_gamma),
            crate::opt_field(
                "m_GeneratedSkyboxReflection",
                &self.m_generated_skybox_reflection,
            ),
            crate::opt_field("m_Sun", &self.m_sun),
            crate::opt_field("m_IndirectSpecularColor", &self.m_indirect_specular_color),
            crate::opt_field(
                "m_UseRadianceAmbientProbe",
                &self.m_use_radiance_ambient_probe,
            ),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Fog" => (self.m_fog).inspect_value(path, offset, limit),
                "m_FogColor" => (self.m_fog_color).inspect_value(path, offset, limit),
                "m_FogMode" => (self.m_fog_mode).inspect_value(path, offset, limit),
                "m_FogDensity" => (self.m_fog_density).inspect_value(path, offset, limit),
                "m_LinearFogStart" => (self.m_linear_fog_start).inspect_value(path, offset, limit),
                "m_LinearFogEnd" => (self.m_linear_fog_end).inspect_value(path, offset, limit),
                "m_AmbientLight" => (self.m_ambient_light).inspect_value(path, offset, limit),
                "m_AmbientSkyColor" => {
                    (self.m_ambient_sky_color).inspect_value(path, offset, limit)
                }
                "m_SkyboxMaterial" => (self.m_skybox_material).inspect_value(path, offset, limit),
                "m_AmbientEquatorColor" => {
                    (self.m_ambient_equator_color).inspect_value(path, offset, limit)
                }
                "m_HaloStrength" => (self.m_halo_strength).inspect_value(path, offset, limit),
                "m_AmbientGroundColor" => {
                    (self.m_ambient_ground_color).inspect_value(path, offset, limit)
                }
                "m_FlareStrength" => (self.m_flare_strength).inspect_value(path, offset, limit),
                "m_AmbientIntensity" => {
                    (self.m_ambient_intensity).inspect_value(path, offset, limit)
                }
                "m_HaloTexture" => (self.m_halo_texture).inspect_value(path, offset, limit),
                "m_FlareFadeSpeed" => (self.m_flare_fade_speed).inspect_value(path, offset, limit),
                "m_AmbientMode" => (self.m_ambient_mode).inspect_value(path, offset, limit),
                "m_SpotCookie" => (self.m_spot_cookie).inspect_value(path, offset, limit),
                "m_SubtractiveShadowColor" => {
                    (self.m_subtractive_shadow_color).inspect_value(path, offset, limit)
                }
                "m_DefaultReflectionMode" => {
                    (self.m_default_reflection_mode).inspect_value(path, offset, limit)
                }
                "m_DefaultReflectionResolution" => {
                    (self.m_default_reflection_resolution).inspect_value(path, offset, limit)
                }
                "m_ReflectionBounces" => {
                    (self.m_reflection_bounces).inspect_value(path, offset, limit)
                }
                "m_ReflectionIntensity" => {
                    (self.m_reflection_intensity).inspect_value(path, offset, limit)
                }
                "m_CustomReflection" => {
                    (self.m_custom_reflection).inspect_value(path, offset, limit)
                }
                "m_AmbientProbe" => (self.m_ambient_probe).inspect_value(path, offset, limit),
                "m_AmbientProbeInGamma" => {
                    (self.m_ambient_probe_in_gamma).inspect_value(path, offset, limit)
                }
                "m_GeneratedSkyboxReflection" => {
                    (self.m_generated_skybox_reflection).inspect_value(path, offset, limit)
                }
                "m_Sun" => (self.m_sun).inspect_value(path, offset, limit),
                "m_IndirectSpecularColor" => {
                    (self.m_indirect_specular_color).inspect_value(path, offset, limit)
                }
                "m_UseRadianceAmbientProbe" => {
                    (self.m_use_radiance_ambient_probe).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_fog).inspect_value(path, offset, limit)).map(|v| ("m_Fog".to_string(), v)),
            ((self.m_fog_color).inspect_value(path, offset, limit))
                .map(|v| ("m_FogColor".to_string(), v)),
            ((self.m_fog_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_FogMode".to_string(), v)),
            ((self.m_fog_density).inspect_value(path, offset, limit))
                .map(|v| ("m_FogDensity".to_string(), v)),
            ((self.m_linear_fog_start).inspect_value(path, offset, limit))
                .map(|v| ("m_LinearFogStart".to_string(), v)),
            ((self.m_linear_fog_end).inspect_value(path, offset, limit))
                .map(|v| ("m_LinearFogEnd".to_string(), v)),
            ((self.m_ambient_light).inspect_value(path, offset, limit))
                .map(|v| ("m_AmbientLight".to_string(), v)),
            ((self.m_ambient_sky_color).inspect_value(path, offset, limit))
                .map(|v| ("m_AmbientSkyColor".to_string(), v)),
            ((self.m_skybox_material).inspect_value(path, offset, limit))
                .map(|v| ("m_SkyboxMaterial".to_string(), v)),
            ((self.m_ambient_equator_color).inspect_value(path, offset, limit))
                .map(|v| ("m_AmbientEquatorColor".to_string(), v)),
            ((self.m_halo_strength).inspect_value(path, offset, limit))
                .map(|v| ("m_HaloStrength".to_string(), v)),
            ((self.m_ambient_ground_color).inspect_value(path, offset, limit))
                .map(|v| ("m_AmbientGroundColor".to_string(), v)),
            ((self.m_flare_strength).inspect_value(path, offset, limit))
                .map(|v| ("m_FlareStrength".to_string(), v)),
            ((self.m_ambient_intensity).inspect_value(path, offset, limit))
                .map(|v| ("m_AmbientIntensity".to_string(), v)),
            ((self.m_halo_texture).inspect_value(path, offset, limit))
                .map(|v| ("m_HaloTexture".to_string(), v)),
            ((self.m_flare_fade_speed).inspect_value(path, offset, limit))
                .map(|v| ("m_FlareFadeSpeed".to_string(), v)),
            ((self.m_ambient_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_AmbientMode".to_string(), v)),
            ((self.m_spot_cookie).inspect_value(path, offset, limit))
                .map(|v| ("m_SpotCookie".to_string(), v)),
            ((self.m_subtractive_shadow_color).inspect_value(path, offset, limit))
                .map(|v| ("m_SubtractiveShadowColor".to_string(), v)),
            ((self.m_default_reflection_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_DefaultReflectionMode".to_string(), v)),
            ((self.m_default_reflection_resolution).inspect_value(path, offset, limit))
                .map(|v| ("m_DefaultReflectionResolution".to_string(), v)),
            ((self.m_reflection_bounces).inspect_value(path, offset, limit))
                .map(|v| ("m_ReflectionBounces".to_string(), v)),
            ((self.m_reflection_intensity).inspect_value(path, offset, limit))
                .map(|v| ("m_ReflectionIntensity".to_string(), v)),
            ((self.m_custom_reflection).inspect_value(path, offset, limit))
                .map(|v| ("m_CustomReflection".to_string(), v)),
            ((self.m_ambient_probe).inspect_value(path, offset, limit))
                .map(|v| ("m_AmbientProbe".to_string(), v)),
            ((self.m_ambient_probe_in_gamma).inspect_value(path, offset, limit))
                .map(|v| ("m_AmbientProbeInGamma".to_string(), v)),
            ((self.m_generated_skybox_reflection).inspect_value(path, offset, limit))
                .map(|v| ("m_GeneratedSkyboxReflection".to_string(), v)),
            ((self.m_sun).inspect_value(path, offset, limit)).map(|v| ("m_Sun".to_string(), v)),
            ((self.m_indirect_specular_color).inspect_value(path, offset, limit))
                .map(|v| ("m_IndirectSpecularColor".to_string(), v)),
            ((self.m_use_radiance_ambient_probe).inspect_value(path, offset, limit))
                .map(|v| ("m_UseRadianceAmbientProbe".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
