//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::bit_field::BitField;
use crate::subclasses::color_rgba::ColorRgba;
use crate::subclasses::vector3f::Vector3f;


/// 版本枚举：ReflectionProbeMImportanceVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ReflectionProbeMImportanceVariant {
    Default(i32),
    V5_4(i16),
}

impl crate::Parse for ReflectionProbeMImportanceVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = r.read_i16()?;
            r.align(4)?;
            Ok(Self::V5_4(v))
        } else {
            let v = r.read_i32()?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：ReflectionProbeMUseOcclusionCullingVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ReflectionProbeMUseOcclusionCullingVariant {
    Default(bool),
    V5_4(bool),
}

impl crate::Parse for ReflectionProbeMUseOcclusionCullingVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = r.read_bool()?;
            Ok(Self::V5_4(v))
        } else {
            let v = r.read_bool()?;
            r.align(4)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct ReflectionProbe {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_type: i32,
    pub m_mode: i32,
    pub m_refresh_mode: i32,
    pub m_time_slicing_mode: i32,
    pub m_resolution: i32,
    pub m_update_frequency: i32,
    pub m_importance: ReflectionProbeMImportanceVariant,
    pub m_box_size: Vector3f,
    pub m_box_offset: Vector3f,
    pub m_near_clip: f32,
    pub m_far_clip: f32,
    pub m_shadow_distance: f32,
    pub m_clear_flags: u32,
    pub m_back_ground_color: ColorRgba,
    pub m_culling_mask: BitField,
    pub m_intensity_multiplier: f32,
    pub m_blend_distance: Option<f32>,
    pub m_hdr: bool,
    pub m_box_projection: bool,
    pub m_render_dynamic_objects: bool,
    pub m_use_occlusion_culling: ReflectionProbeMUseOcclusionCullingVariant,
    pub m_custom_baked_texture: PPtr,
    pub m_baked_texture: PPtr,
}

impl crate::Parse for ReflectionProbe {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut m_enabled = None;
        let mut m_type = None;
        let mut m_mode = None;
        let mut m_refresh_mode = None;
        let mut m_time_slicing_mode = None;
        let mut m_resolution = None;
        let mut m_update_frequency = None;
        let mut m_importance = None;
        let mut m_box_size = None;
        let mut m_box_offset = None;
        let mut m_near_clip = None;
        let mut m_far_clip = None;
        let mut m_shadow_distance = None;
        let mut m_clear_flags = None;
        let mut m_back_ground_color = None;
        let mut m_culling_mask = None;
        let mut m_intensity_multiplier = None;
        let mut m_blend_distance = None;
        let mut m_hdr = None;
        let mut m_box_projection = None;
        let mut m_render_dynamic_objects = None;
        let mut m_use_occlusion_culling = None;
        let mut m_custom_baked_texture = None;
        let mut m_baked_texture = None;
            m_game_object = Some(crate::PPtr::parse(r, ver)?);
            m_enabled = Some(r.read_u8()?);
            r.align(4)?;
            m_type = Some(r.read_i32()?);
            m_mode = Some(r.read_i32()?);
            m_refresh_mode = Some(r.read_i32()?);
            m_time_slicing_mode = Some(r.read_i32()?);
            m_resolution = Some(r.read_i32()?);
            m_update_frequency = Some(r.read_i32()?);
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
        } else {
            m_importance = Some(ReflectionProbeMImportanceVariant::parse(r, ver)?);
        }
            m_box_size = Some(Vector3f::parse(r, ver)?);
            m_box_offset = Some(Vector3f::parse(r, ver)?);
            m_near_clip = Some(r.read_f32()?);
            m_far_clip = Some(r.read_f32()?);
            m_shadow_distance = Some(r.read_f32()?);
            m_clear_flags = Some(r.read_u32()?);
            m_back_ground_color = Some(ColorRgba::parse(r, ver)?);
            m_culling_mask = Some(BitField::parse(r, ver)?);
            m_intensity_multiplier = Some(r.read_f32()?);
        if *ver >= crate::UnityVersion::new(5, 2, 0) {
            m_blend_distance = Some(r.read_f32()?);
        } else {
        }
            m_hdr = Some(r.read_bool()?);
            m_box_projection = Some(r.read_bool()?);
            m_render_dynamic_objects = Some(r.read_bool()?);
            m_use_occlusion_culling = Some(ReflectionProbeMUseOcclusionCullingVariant::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            m_importance = Some(ReflectionProbeMImportanceVariant::parse(r, ver)?);
        } else {
        }
            m_custom_baked_texture = Some(crate::PPtr::parse(r, ver)?);
            m_baked_texture = Some(crate::PPtr::parse(r, ver)?);

        Ok(ReflectionProbe {
            m_game_object: m_game_object.unwrap(),
            m_enabled: m_enabled.unwrap(),
            m_type: m_type.unwrap(),
            m_mode: m_mode.unwrap(),
            m_refresh_mode: m_refresh_mode.unwrap(),
            m_time_slicing_mode: m_time_slicing_mode.unwrap(),
            m_resolution: m_resolution.unwrap(),
            m_update_frequency: m_update_frequency.unwrap(),
            m_importance: m_importance.unwrap(),
            m_box_size: m_box_size.unwrap(),
            m_box_offset: m_box_offset.unwrap(),
            m_near_clip: m_near_clip.unwrap(),
            m_far_clip: m_far_clip.unwrap(),
            m_shadow_distance: m_shadow_distance.unwrap(),
            m_clear_flags: m_clear_flags.unwrap(),
            m_back_ground_color: m_back_ground_color.unwrap(),
            m_culling_mask: m_culling_mask.unwrap(),
            m_intensity_multiplier: m_intensity_multiplier.unwrap(),
            m_blend_distance,
            m_hdr: m_hdr.unwrap(),
            m_box_projection: m_box_projection.unwrap(),
            m_render_dynamic_objects: m_render_dynamic_objects.unwrap(),
            m_use_occlusion_culling: m_use_occlusion_culling.unwrap(),
            m_custom_baked_texture: m_custom_baked_texture.unwrap(),
            m_baked_texture: m_baked_texture.unwrap(),
        })
    }
}
