//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::bit_field::BitField;
use crate::subclasses::color_rgba::ColorRgba;
use crate::subclasses::falloff_table::FalloffTable;
use crate::subclasses::light_baking_output::LightBakingOutput;
use crate::subclasses::shadow_settings::ShadowSettings;
use crate::subclasses::vector2f::Vector2f;
use crate::subclasses::vector4f::Vector4f;
use super::behaviour::Behaviour;

#[derive(Debug, Clone)]
pub struct Light {
    pub parent: Behaviour,
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_type: i32,
    pub m_color: ColorRgba,
    pub m_shape: Option<i32>,
    pub m_intensity: f32,
    pub m_range: f32,
    pub m_spot_angle: f32,
    pub m_cookie_size: Option<f32>,
    pub m_inner_spot_angle: Option<f32>,
    pub m_shadows: ShadowSettings,
    pub m_cookie_size2_d: Option<Vector2f>,
    pub m_cookie: PPtr,
    pub m_draw_halo: bool,
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
    pub m_cct: Option<f32>,
    pub m_color_temperature: Option<f32>,
    pub m_falloff_table: Option<FalloffTable>,
    pub m_use_color_temperature: Option<bool>,
    pub m_bounding_sphere_override: Option<Vector4f>,
    pub m_use_bounding_sphere_override: Option<bool>,
    pub m_use_view_frustum_for_shadow_caster_cull: Option<bool>,
    pub m_force_visible: Option<bool>,
    pub m_light_unit: Option<i32>,
    pub m_shape_radius: Option<f32>,
    pub m_lux_at_distance: Option<f32>,
    pub m_enable_spot_reflector: Option<bool>,
}

impl crate::Parse for Light {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = Behaviour::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_u8()?;
        r.align(4)?;
        let m_type = r.read_i32()?;
        let m_color = ColorRgba::parse(r, ver)?;
        let m_shape = if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_intensity = r.read_f32()?;
        let m_range = r.read_f32()?;
        let m_spot_angle = r.read_f32()?;
        let m_cookie_size = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_inner_spot_angle = if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_shadows = ShadowSettings::parse(r, ver)?;
        let m_cookie_size2_d = if *ver >= crate::UnityVersion::new(6000, 3, 0) {
            Some(Vector2f::parse(r, ver)?)
        } else {
            None
        };
        let m_cookie = crate::PPtr::parse(r, ver)?;
        let m_draw_halo = r.read_bool()?;
        let m_actually_lightmapped = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_baked_index = if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = r.read_i32()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_baking_output = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            Some(LightBakingOutput::parse(r, ver)?)
        } else {
            None
        };
        let m_flare = crate::PPtr::parse(r, ver)?;
        let m_render_mode = r.read_i32()?;
        let m_culling_mask = BitField::parse(r, ver)?;
        let m_lightmapping = r.read_i32()?;
        let m_bounce_intensity = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_area_size = if *ver >= crate::UnityVersion::new(5, 4, 0) {
            Some(Vector2f::parse(r, ver)?)
        } else {
            None
        };
        let m_light_shadow_caster_mode = if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_rendering_layer_mask = if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            Some(r.read_u32()?)
        } else {
            None
        };
        let m_cct = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = r.read_f32()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_color_temperature = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_falloff_table = if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            Some(FalloffTable::parse(r, ver)?)
        } else {
            None
        };
        let m_use_color_temperature = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_bounding_sphere_override = if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            Some(Vector4f::parse(r, ver)?)
        } else {
            None
        };
        let m_use_bounding_sphere_override = if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_use_view_frustum_for_shadow_caster_cull = if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_force_visible = if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_light_unit = if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_shape_radius = if *ver >= crate::UnityVersion::new(6000, 4, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_lux_at_distance = if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_enable_spot_reflector = if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(Light {
            parent,
            m_game_object,
            m_enabled,
            m_type,
            m_color,
            m_shape,
            m_intensity,
            m_range,
            m_spot_angle,
            m_cookie_size,
            m_inner_spot_angle,
            m_shadows,
            m_cookie_size2_d,
            m_cookie,
            m_draw_halo,
            m_actually_lightmapped,
            m_baked_index,
            m_baking_output,
            m_flare,
            m_render_mode,
            m_culling_mask,
            m_lightmapping,
            m_bounce_intensity,
            m_area_size,
            m_light_shadow_caster_mode,
            m_rendering_layer_mask,
            m_cct,
            m_color_temperature,
            m_falloff_table,
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
