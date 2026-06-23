//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::static_batch_info::StaticBatchInfo;
use crate::subclasses::vector4f::Vector4f;
use super::renderer::Renderer;

#[derive(Debug, Clone)]
pub struct SpriteMask {
    pub parent: Renderer,
    pub m_game_object: PPtr,
    pub m_enabled: bool,
    pub m_cast_shadows: u8,
    pub m_receive_shadows: u8,
    pub m_motion_vectors: u8,
    pub m_dynamic_occludee: Option<u8>,
    pub m_light_probe_usage: u8,
    pub m_static_shadow_caster: Option<u8>,
    pub m_reflection_probe_usage: u8,
    pub m_lightmap_index: u16,
    pub m_lightmap_index_dynamic: u16,
    pub m_rendering_layer_mask: Option<u32>,
    pub m_ray_tracing_mode: Option<u8>,
    pub m_lightmap_tiling_offset: Vector4f,
    pub m_renderer_priority: Option<i32>,
    pub m_ray_trace_procedural: Option<u8>,
    pub m_lightmap_tiling_offset_dynamic: Vector4f,
    pub m_materials: Vec<PPtr>,
    pub m_ray_tracing_accel_struct_build_flags_override: Option<u8>,
    pub m_static_batch_info: StaticBatchInfo,
    pub m_ray_tracing_accel_struct_build_flags: Option<u8>,
    pub m_static_batch_root: PPtr,
    pub m_small_mesh_culling: Option<u8>,
    pub m_probe_anchor: PPtr,
    pub m_force_mesh_lod: Option<i16>,
    pub m_light_probe_volume_override: PPtr,
    pub m_mesh_lod_selection_bias: Option<f32>,
    pub m_sorting_layer_id: i32,
    pub m_sorting_layer: i16,
    pub m_sorting_order: i16,
    pub m_sprite: PPtr,
    pub m_mask_alpha_cutoff: f32,
    pub m_front_sorting_layer: i16,
    pub m_back_sorting_layer: i16,
    pub m_front_sorting_order: i16,
    pub m_back_sorting_order: i16,
    pub m_front_sorting_layer_id: Option<i32>,
    pub m_is_custom_range_active: bool,
    pub m_back_sorting_layer_id: Option<i32>,
    pub m_sprite_sort_point: Option<i32>,
    pub m_mask_interaction: Option<i32>,
    pub m_mask_source: Option<i32>,
}

impl crate::Parse for SpriteMask {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = Renderer::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_bool()?;
        let m_cast_shadows = r.read_u8()?;
        let m_receive_shadows = r.read_u8()?;
        let m_motion_vectors = r.read_u8()?;
        let m_dynamic_occludee = if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            Some(r.read_u8()?)
        } else {
            None
        };
        let m_light_probe_usage = r.read_u8()?;
        let m_static_shadow_caster = if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            Some(r.read_u8()?)
        } else {
            None
        };
        let m_reflection_probe_usage = r.read_u8()?;
        r.align(4)?;
        let m_lightmap_index = r.read_u16()?;
        let m_lightmap_index_dynamic = r.read_u16()?;
        let m_rendering_layer_mask = if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            Some(r.read_u32()?)
        } else {
            None
        };
        let m_ray_tracing_mode = if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            let v = r.read_u8()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_lightmap_tiling_offset = Vector4f::parse(r, ver)?;
        let m_renderer_priority = if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_ray_trace_procedural = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = r.read_u8()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_lightmap_tiling_offset_dynamic = Vector4f::parse(r, ver)?;
        let m_materials = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_ray_tracing_accel_struct_build_flags_override = if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            Some(r.read_u8()?)
        } else {
            None
        };
        let m_static_batch_info = StaticBatchInfo::parse(r, ver)?;
        let m_ray_tracing_accel_struct_build_flags = if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            let v = r.read_u8()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_static_batch_root = crate::PPtr::parse(r, ver)?;
        let m_small_mesh_culling = if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            let v = r.read_u8()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_probe_anchor = crate::PPtr::parse(r, ver)?;
        let m_force_mesh_lod = if *ver >= crate::UnityVersion::new(6000, 2, 0) {
            let v = r.read_i16()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_light_probe_volume_override = crate::PPtr::parse(r, ver)?;
        r.align(4)?;
        let m_mesh_lod_selection_bias = if *ver >= crate::UnityVersion::new(6000, 2, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_sorting_layer_id = r.read_i32()?;
        let m_sorting_layer = r.read_i16()?;
        let m_sorting_order = r.read_i16()?;
        r.align(4)?;
        let m_sprite = crate::PPtr::parse(r, ver)?;
        let m_mask_alpha_cutoff = r.read_f32()?;
        let m_front_sorting_layer = r.read_i16()?;
        let m_back_sorting_layer = r.read_i16()?;
        let m_front_sorting_order = r.read_i16()?;
        let m_back_sorting_order = r.read_i16()?;
        let m_front_sorting_layer_id = if *ver >= crate::UnityVersion::new(2018, 4, 5) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_is_custom_range_active = r.read_bool()?;
        r.align(4)?;
        let m_back_sorting_layer_id = if *ver >= crate::UnityVersion::new(2018, 4, 5) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_sprite_sort_point = if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            let v = r.read_i32()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_mask_interaction = if *ver >= crate::UnityVersion::new(6000, 3, 0) {
            let v = r.read_i32()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_mask_source = if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            let v = r.read_i32()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(SpriteMask {
            parent,
            m_game_object,
            m_enabled,
            m_cast_shadows,
            m_receive_shadows,
            m_motion_vectors,
            m_dynamic_occludee,
            m_light_probe_usage,
            m_static_shadow_caster,
            m_reflection_probe_usage,
            m_lightmap_index,
            m_lightmap_index_dynamic,
            m_rendering_layer_mask,
            m_ray_tracing_mode,
            m_lightmap_tiling_offset,
            m_renderer_priority,
            m_ray_trace_procedural,
            m_lightmap_tiling_offset_dynamic,
            m_materials,
            m_ray_tracing_accel_struct_build_flags_override,
            m_static_batch_info,
            m_ray_tracing_accel_struct_build_flags,
            m_static_batch_root,
            m_small_mesh_culling,
            m_probe_anchor,
            m_force_mesh_lod,
            m_light_probe_volume_override,
            m_mesh_lod_selection_bias,
            m_sorting_layer_id,
            m_sorting_layer,
            m_sorting_order,
            m_sprite,
            m_mask_alpha_cutoff,
            m_front_sorting_layer,
            m_back_sorting_layer,
            m_front_sorting_order,
            m_back_sorting_order,
            m_front_sorting_layer_id,
            m_is_custom_range_active,
            m_back_sorting_layer_id,
            m_sprite_sort_point,
            m_mask_interaction,
            m_mask_source,
        })
    }
}
