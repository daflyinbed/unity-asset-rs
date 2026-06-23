//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::bit_field::BitField;
use crate::subclasses::color_rgba::ColorRgba;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct ReflectionProbe {
    pub m_object_hide_flags: u32,
    pub m_prefab_parent_object: PPtr,
    pub m_prefab_internal: PPtr,
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_type: i32,
    pub m_mode: i32,
    pub m_refresh_mode: i32,
    pub m_time_slicing_mode: i32,
    pub m_resolution: i32,
    pub m_update_frequency: i32,
    pub m_importance: i32,
    pub m_box_size: Vector3f,
    pub m_box_offset: Vector3f,
    pub m_near_clip: f32,
    pub m_far_clip: f32,
    pub m_shadow_distance: f32,
    pub m_clear_flags: u32,
    pub m_back_ground_color: ColorRgba,
    pub m_culling_mask: BitField,
    pub m_intensity_multiplier: f32,
    pub m_hdr: bool,
    pub m_box_projection: bool,
    pub m_render_dynamic_objects: bool,
    pub m_use_occlusion_culling: bool,
    pub m_custom_baked_texture: PPtr,
}

impl crate::Parse for ReflectionProbe {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_prefab_parent_object = crate::PPtr::parse(r, ver)?;
        let m_prefab_internal = crate::PPtr::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_u8()?;
        r.align(4)?;
        let m_type = r.read_i32()?;
        let m_mode = r.read_i32()?;
        let m_refresh_mode = r.read_i32()?;
        let m_time_slicing_mode = r.read_i32()?;
        let m_resolution = r.read_i32()?;
        let m_update_frequency = r.read_i32()?;
        let m_importance = r.read_i32()?;
        let m_box_size = Vector3f::parse(r, ver)?;
        let m_box_offset = Vector3f::parse(r, ver)?;
        let m_near_clip = r.read_f32()?;
        let m_far_clip = r.read_f32()?;
        let m_shadow_distance = r.read_f32()?;
        let m_clear_flags = r.read_u32()?;
        let m_back_ground_color = ColorRgba::parse(r, ver)?;
        let m_culling_mask = BitField::parse(r, ver)?;
        let m_intensity_multiplier = r.read_f32()?;
        let m_hdr = r.read_bool()?;
        let m_box_projection = r.read_bool()?;
        let m_render_dynamic_objects = r.read_bool()?;
        let m_use_occlusion_culling = r.read_bool()?;
        r.align(4)?;
        let m_custom_baked_texture = crate::PPtr::parse(r, ver)?;

        Ok(ReflectionProbe {
            m_object_hide_flags,
            m_prefab_parent_object,
            m_prefab_internal,
            m_game_object,
            m_enabled,
            m_type,
            m_mode,
            m_refresh_mode,
            m_time_slicing_mode,
            m_resolution,
            m_update_frequency,
            m_importance,
            m_box_size,
            m_box_offset,
            m_near_clip,
            m_far_clip,
            m_shadow_distance,
            m_clear_flags,
            m_back_ground_color,
            m_culling_mask,
            m_intensity_multiplier,
            m_hdr,
            m_box_projection,
            m_render_dynamic_objects,
            m_use_occlusion_culling,
            m_custom_baked_texture,
        })
    }
}
