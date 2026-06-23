//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::aabb::Aabb;
use crate::subclasses::color_rgba::ColorRgba;
use crate::subclasses::static_batch_info::StaticBatchInfo;

#[derive(Debug, Clone)]
pub struct SpriteShapeRenderer {
    pub m_object_hide_flags: u32,
    pub m_prefab_parent_object: PPtr,
    pub m_prefab_internal: PPtr,
    pub m_game_object: PPtr,
    pub m_enabled: bool,
    pub m_cast_shadows: u8,
    pub m_receive_shadows: u8,
    pub m_dynamic_occludee: u8,
    pub m_motion_vectors: u8,
    pub m_light_probe_usage: u8,
    pub m_reflection_probe_usage: u8,
    pub m_rendering_layer_mask: u32,
    pub m_materials: Vec<PPtr>,
    pub m_static_batch_info: StaticBatchInfo,
    pub m_static_batch_root: PPtr,
    pub m_probe_anchor: PPtr,
    pub m_light_probe_volume_override: PPtr,
    pub m_scale_in_lightmap: f32,
    pub m_preserve_u_vs: bool,
    pub m_ignore_normals_for_chart_detection: bool,
    pub m_important_gi: bool,
    pub m_stitch_lightmap_seams: bool,
    pub m_selected_editor_render_state: i32,
    pub m_minimum_chart_size: i32,
    pub m_auto_uv_max_distance: f32,
    pub m_auto_uv_max_angle: f32,
    pub m_lightmap_parameters: PPtr,
    pub m_sorting_layer_id: i32,
    pub m_sorting_layer: i16,
    pub m_sorting_order: i16,
    pub m_color: ColorRgba,
    pub m_mask_interaction: i32,
    pub m_shape_texture: PPtr,
    pub m_sprites: Vec<PPtr>,
    pub m_local_aabb: Aabb,
}

impl crate::Parse for SpriteShapeRenderer {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_prefab_parent_object = crate::PPtr::parse(r, ver)?;
        let m_prefab_internal = crate::PPtr::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_bool()?;
        let m_cast_shadows = r.read_u8()?;
        let m_receive_shadows = r.read_u8()?;
        let m_dynamic_occludee = r.read_u8()?;
        let m_motion_vectors = r.read_u8()?;
        let m_light_probe_usage = r.read_u8()?;
        let m_reflection_probe_usage = r.read_u8()?;
        r.align(4)?;
        let m_rendering_layer_mask = r.read_u32()?;
        let m_materials = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_static_batch_info = StaticBatchInfo::parse(r, ver)?;
        let m_static_batch_root = crate::PPtr::parse(r, ver)?;
        let m_probe_anchor = crate::PPtr::parse(r, ver)?;
        let m_light_probe_volume_override = crate::PPtr::parse(r, ver)?;
        let m_scale_in_lightmap = r.read_f32()?;
        let m_preserve_u_vs = r.read_bool()?;
        let m_ignore_normals_for_chart_detection = r.read_bool()?;
        let m_important_gi = r.read_bool()?;
        let m_stitch_lightmap_seams = r.read_bool()?;
        r.align(4)?;
        let m_selected_editor_render_state = r.read_i32()?;
        let m_minimum_chart_size = r.read_i32()?;
        let m_auto_uv_max_distance = r.read_f32()?;
        let m_auto_uv_max_angle = r.read_f32()?;
        let m_lightmap_parameters = crate::PPtr::parse(r, ver)?;
        r.align(4)?;
        let m_sorting_layer_id = r.read_i32()?;
        let m_sorting_layer = r.read_i16()?;
        let m_sorting_order = r.read_i16()?;
        r.align(4)?;
        let m_color = ColorRgba::parse(r, ver)?;
        let m_mask_interaction = r.read_i32()?;
        let m_shape_texture = crate::PPtr::parse(r, ver)?;
        let m_sprites = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_local_aabb = Aabb::parse(r, ver)?;

        Ok(SpriteShapeRenderer {
            m_object_hide_flags,
            m_prefab_parent_object,
            m_prefab_internal,
            m_game_object,
            m_enabled,
            m_cast_shadows,
            m_receive_shadows,
            m_dynamic_occludee,
            m_motion_vectors,
            m_light_probe_usage,
            m_reflection_probe_usage,
            m_rendering_layer_mask,
            m_materials,
            m_static_batch_info,
            m_static_batch_root,
            m_probe_anchor,
            m_light_probe_volume_override,
            m_scale_in_lightmap,
            m_preserve_u_vs,
            m_ignore_normals_for_chart_detection,
            m_important_gi,
            m_stitch_lightmap_seams,
            m_selected_editor_render_state,
            m_minimum_chart_size,
            m_auto_uv_max_distance,
            m_auto_uv_max_angle,
            m_lightmap_parameters,
            m_sorting_layer_id,
            m_sorting_layer,
            m_sorting_order,
            m_color,
            m_mask_interaction,
            m_shape_texture,
            m_sprites,
            m_local_aabb,
        })
    }
}
