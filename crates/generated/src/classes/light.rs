//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::bit_field::BitField;
use crate::subclasses::color_rgb_af::ColorRgbAf;
use crate::subclasses::light_baking_output::LightBakingOutput;
use crate::subclasses::shadow_settings::ShadowSettings;
use crate::subclasses::vector2f::Vector2f;
use crate::subclasses::vector4f::Vector4f;

/// 版本枚举：LightMDrawHaloVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum LightMDrawHaloVariant {
    Default(bool),
    V5_4(bool),
}

impl crate::Parse for LightMDrawHaloVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V5_4(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：LightMUseBoundingSphereOverrideVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum LightMUseBoundingSphereOverrideVariant {
    Default(bool),
    V2020_2(bool),
}

impl crate::Parse for LightMUseBoundingSphereOverrideVariant {
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

/// 版本枚举：LightMUseViewFrustumForShadowCasterCullVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum LightMUseViewFrustumForShadowCasterCullVariant {
    Default(bool),
    V2023_3(bool),
}

impl crate::Parse for LightMUseViewFrustumForShadowCasterCullVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2023_3(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct Light {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_type: i32,
    pub m_color: ColorRgbAf,
    pub m_shape: Option<i32>,
    pub m_intensity: f32,
    pub m_range: f32,
    pub m_spot_angle: f32,
    pub m_cookie_size: Option<f32>,
    pub m_inner_spot_angle: Option<f32>,
    pub m_shadows: ShadowSettings,
    pub m_cookie_size2_d: Option<Vector2f>,
    pub m_cookie: PPtr,
    pub m_draw_halo: LightMDrawHaloVariant,
    pub m_actually_lightmapped: Option<bool>,
    pub m_baked_index: Option<i32>,
    pub m_baking_output: Option<LightBakingOutput>,
    pub m_flare: PPtr,
    pub m_render_mode: i32,
    pub m_culling_mask: BitField,
    pub m_lightmapping: i32,
    pub m_bounce_intensity: Option<f32>,
    pub m_area_size: Option<Vector2f>,
    pub m_light_shadow_caster_mode: Option<i32>,
    pub m_rendering_layer_mask: Option<u32>,
    pub m_color_temperature: Option<f32>,
    pub m_use_color_temperature: Option<bool>,
    pub m_bounding_sphere_override: Option<Vector4f>,
    pub m_use_bounding_sphere_override: Option<LightMUseBoundingSphereOverrideVariant>,
    pub m_use_view_frustum_for_shadow_caster_cull:
        Option<LightMUseViewFrustumForShadowCasterCullVariant>,
    pub m_force_visible: Option<bool>,
    pub m_light_unit: Option<i32>,
    pub m_shape_radius: Option<f32>,
    pub m_lux_at_distance: Option<f32>,
    pub m_enable_spot_reflector: Option<bool>,
}

impl crate::Parse for Light {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut m_enabled = None;
        let mut m_type = None;
        let mut m_color = None;
        let mut m_shape = None;
        let mut m_intensity = None;
        let mut m_range = None;
        let mut m_spot_angle = None;
        let mut m_cookie_size = None;
        let mut m_inner_spot_angle = None;
        let mut m_shadows = None;
        let mut m_cookie_size2_d = None;
        let mut m_cookie = None;
        let mut m_draw_halo = None;
        let mut m_actually_lightmapped = None;
        let mut m_baked_index = None;
        let mut m_baking_output = None;
        let mut m_flare = None;
        let mut m_render_mode = None;
        let mut m_culling_mask = None;
        let mut m_lightmapping = None;
        let mut m_bounce_intensity = None;
        let mut m_area_size = None;
        let mut m_light_shadow_caster_mode = None;
        let mut m_rendering_layer_mask = None;
        let mut m_color_temperature = None;
        let mut m_use_color_temperature = None;
        let mut m_bounding_sphere_override = None;
        let mut m_use_bounding_sphere_override = None;
        let mut m_use_view_frustum_for_shadow_caster_cull = None;
        let mut m_force_visible = None;
        let mut m_light_unit = None;
        let mut m_shape_radius = None;
        let mut m_lux_at_distance = None;
        let mut m_enable_spot_reflector = None;
        m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
        m_enabled = Some(rdr.read_u8()?);
        rdr.align(4)?;
        m_type = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(2023, 2, 0) {
        } else if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            m_shape = Some(rdr.read_i32()?);
        } else {
        }
        m_color = Some(ColorRgbAf::parse(rdr, ver)?);
        m_intensity = Some(rdr.read_f32()?);
        m_range = Some(rdr.read_f32()?);
        m_spot_angle = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(6000, 3, 0) {
            m_inner_spot_angle = Some(rdr.read_f32()?);
            m_cookie_size2_d = Some(Vector2f::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_inner_spot_angle = Some(rdr.read_f32()?);
            m_cookie_size = Some(rdr.read_f32()?);
        } else {
            m_cookie_size = Some(rdr.read_f32()?);
        }
        m_shadows = Some(ShadowSettings::parse(rdr, ver)?);
        m_cookie = Some(crate::PPtr::parse(rdr, ver)?);
        m_draw_halo = Some(LightMDrawHaloVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_baking_output = Some(LightBakingOutput::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            m_baked_index = Some(rdr.read_i32()?);
            rdr.align(4)?;
        } else {
            m_actually_lightmapped = Some(rdr.read_bool()?);
            rdr.align(4)?;
        }
        m_flare = Some(crate::PPtr::parse(rdr, ver)?);
        m_render_mode = Some(rdr.read_i32()?);
        m_culling_mask = Some(BitField::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_rendering_layer_mask = Some(rdr.read_u32()?);
        } else {
        }
        m_lightmapping = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(6000, 4, 0) {
            m_light_shadow_caster_mode = Some(rdr.read_i32()?);
            m_area_size = Some(Vector2f::parse(rdr, ver)?);
            m_bounce_intensity = Some(rdr.read_f32()?);
            m_color_temperature = Some(rdr.read_f32()?);
            m_use_color_temperature = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_bounding_sphere_override = Some(Vector4f::parse(rdr, ver)?);
            m_use_bounding_sphere_override =
                Some(LightMUseBoundingSphereOverrideVariant::parse(rdr, ver)?);
            m_use_view_frustum_for_shadow_caster_cull = Some(
                LightMUseViewFrustumForShadowCasterCullVariant::parse(rdr, ver)?,
            );
            m_force_visible = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_shape_radius = Some(rdr.read_f32()?);
            m_light_unit = Some(rdr.read_i32()?);
            m_lux_at_distance = Some(rdr.read_f32()?);
            m_enable_spot_reflector = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            m_light_shadow_caster_mode = Some(rdr.read_i32()?);
            m_area_size = Some(Vector2f::parse(rdr, ver)?);
            m_bounce_intensity = Some(rdr.read_f32()?);
            m_color_temperature = Some(rdr.read_f32()?);
            m_use_color_temperature = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_bounding_sphere_override = Some(Vector4f::parse(rdr, ver)?);
            m_use_bounding_sphere_override =
                Some(LightMUseBoundingSphereOverrideVariant::parse(rdr, ver)?);
            m_use_view_frustum_for_shadow_caster_cull = Some(
                LightMUseViewFrustumForShadowCasterCullVariant::parse(rdr, ver)?,
            );
            m_force_visible = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_light_unit = Some(rdr.read_i32()?);
            m_lux_at_distance = Some(rdr.read_f32()?);
            m_enable_spot_reflector = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            m_light_shadow_caster_mode = Some(rdr.read_i32()?);
            m_area_size = Some(Vector2f::parse(rdr, ver)?);
            m_bounce_intensity = Some(rdr.read_f32()?);
            m_color_temperature = Some(rdr.read_f32()?);
            m_use_color_temperature = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_bounding_sphere_override = Some(Vector4f::parse(rdr, ver)?);
            m_use_bounding_sphere_override =
                Some(LightMUseBoundingSphereOverrideVariant::parse(rdr, ver)?);
            m_use_view_frustum_for_shadow_caster_cull = Some(
                LightMUseViewFrustumForShadowCasterCullVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_light_shadow_caster_mode = Some(rdr.read_i32()?);
            m_area_size = Some(Vector2f::parse(rdr, ver)?);
            m_bounce_intensity = Some(rdr.read_f32()?);
            m_color_temperature = Some(rdr.read_f32()?);
            m_use_color_temperature = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_bounding_sphere_override = Some(Vector4f::parse(rdr, ver)?);
            m_use_bounding_sphere_override =
                Some(LightMUseBoundingSphereOverrideVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            m_light_shadow_caster_mode = Some(rdr.read_i32()?);
            m_area_size = Some(Vector2f::parse(rdr, ver)?);
            m_bounce_intensity = Some(rdr.read_f32()?);
            m_color_temperature = Some(rdr.read_f32()?);
            m_use_color_temperature = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_area_size = Some(Vector2f::parse(rdr, ver)?);
            m_bounce_intensity = Some(rdr.read_f32()?);
            m_color_temperature = Some(rdr.read_f32()?);
            m_use_color_temperature = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            m_area_size = Some(Vector2f::parse(rdr, ver)?);
            m_bounce_intensity = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_bounce_intensity = Some(rdr.read_f32()?);
        } else {
        }

        Ok(Light {
            m_game_object: m_game_object.unwrap(),
            m_enabled: m_enabled.unwrap(),
            m_type: m_type.unwrap(),
            m_color: m_color.unwrap(),
            m_shape,
            m_intensity: m_intensity.unwrap(),
            m_range: m_range.unwrap(),
            m_spot_angle: m_spot_angle.unwrap(),
            m_cookie_size,
            m_inner_spot_angle,
            m_shadows: m_shadows.unwrap(),
            m_cookie_size2_d,
            m_cookie: m_cookie.unwrap(),
            m_draw_halo: m_draw_halo.unwrap(),
            m_actually_lightmapped,
            m_baked_index,
            m_baking_output,
            m_flare: m_flare.unwrap(),
            m_render_mode: m_render_mode.unwrap(),
            m_culling_mask: m_culling_mask.unwrap(),
            m_lightmapping: m_lightmapping.unwrap(),
            m_bounce_intensity,
            m_area_size,
            m_light_shadow_caster_mode,
            m_rendering_layer_mask,
            m_color_temperature,
            m_use_color_temperature,
            m_bounding_sphere_override,
            m_use_bounding_sphere_override,
            m_use_view_frustum_for_shadow_caster_cull,
            m_force_visible,
            m_light_unit,
            m_shape_radius,
            m_lux_at_distance,
            m_enable_spot_reflector,
        })
    }
}
