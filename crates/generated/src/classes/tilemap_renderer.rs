//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::int3_storage::int3_storage;
use crate::subclasses::static_batch_info::StaticBatchInfo;
use crate::subclasses::vector3f::Vector3f;
use crate::subclasses::vector4f::Vector4f;
use super::renderer::Renderer;

#[derive(Debug, Clone)]
pub struct TilemapRenderer {
    pub parent: Renderer,
    pub m_game_object: PPtr,
    pub m_enabled: bool,
    pub m_cast_shadows: u8,
    pub m_receive_shadows: u8,
    pub m_dynamic_occludee: u8,
    pub m_motion_vectors: u8,
    pub m_static_shadow_caster: Option<u8>,
    pub m_light_probe_usage: u8,
    pub m_reflection_probe_usage: u8,
    pub m_lightmap_index: u16,
    pub m_rendering_layer_mask: Option<u32>,
    pub m_ray_tracing_mode: Option<u8>,
    pub m_lightmap_index_dynamic: u16,
    pub m_renderer_priority: Option<i32>,
    pub m_ray_trace_procedural: Option<u8>,
    pub m_lightmap_tiling_offset: Vector4f,
    pub m_lightmap_tiling_offset_dynamic: Vector4f,
    pub m_ray_tracing_accel_struct_build_flags_override: Option<u8>,
    pub m_materials: Vec<PPtr>,
    pub m_ray_tracing_accel_struct_build_flags: Option<u8>,
    pub m_static_batch_info: StaticBatchInfo,
    pub m_small_mesh_culling: Option<u8>,
    pub m_static_batch_root: PPtr,
    pub m_force_mesh_lod: Option<i16>,
    pub m_probe_anchor: PPtr,
    pub m_mesh_lod_selection_bias: Option<f32>,
    pub m_light_probe_volume_override: PPtr,
    pub m_sorting_layer_id: i32,
    pub m_sorting_layer: i16,
    pub m_sorting_order: i16,
    pub m_chunk_size: int3_storage,
    pub m_max_chunk_count: u32,
    pub m_chunk_culling_bounds: Option<Vector3f>,
    pub m_max_frame_age: u32,
    pub m_sort_order: i32,
    pub m_mask_interaction: i32,
    pub m_detect_chunk_culling_bounds: Option<i32>,
    pub m_mode: Option<i32>,
}

impl crate::Parse for TilemapRenderer {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = Renderer::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_bool()?;
        let m_cast_shadows = r.read_u8()?;
        let m_receive_shadows = r.read_u8()?;
        let m_dynamic_occludee = r.read_u8()?;
        let m_motion_vectors = r.read_u8()?;
        let m_static_shadow_caster = if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            Some(r.read_u8()?)
        } else {
            None
        };
        let m_light_probe_usage = r.read_u8()?;
        let m_reflection_probe_usage = r.read_u8()?;
        r.align(4)?;
        let m_lightmap_index = r.read_u16()?;
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
        let m_lightmap_index_dynamic = r.read_u16()?;
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
        let m_lightmap_tiling_offset = Vector4f::parse(r, ver)?;
        let m_lightmap_tiling_offset_dynamic = Vector4f::parse(r, ver)?;
        let m_ray_tracing_accel_struct_build_flags_override = if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            Some(r.read_u8()?)
        } else {
            None
        };
        let m_materials = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_ray_tracing_accel_struct_build_flags = if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            let v = r.read_u8()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_static_batch_info = StaticBatchInfo::parse(r, ver)?;
        let m_small_mesh_culling = if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            let v = r.read_u8()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_static_batch_root = crate::PPtr::parse(r, ver)?;
        let m_force_mesh_lod = if *ver >= crate::UnityVersion::new(6000, 2, 0) {
            let v = r.read_i16()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_probe_anchor = crate::PPtr::parse(r, ver)?;
        let m_mesh_lod_selection_bias = if *ver >= crate::UnityVersion::new(6000, 2, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_light_probe_volume_override = crate::PPtr::parse(r, ver)?;
        r.align(4)?;
        let m_sorting_layer_id = r.read_i32()?;
        let m_sorting_layer = r.read_i16()?;
        let m_sorting_order = r.read_i16()?;
        r.align(4)?;
        let m_chunk_size = int3_storage::parse(r, ver)?;
        let m_max_chunk_count = r.read_u32()?;
        let m_chunk_culling_bounds = if *ver >= crate::UnityVersion::new(2017, 4, 33) {
            Some(Vector3f::parse(r, ver)?)
        } else {
            None
        };
        let m_max_frame_age = r.read_u32()?;
        let m_sort_order = r.read_i32()?;
        let m_mask_interaction = r.read_i32()?;
        r.align(4)?;
        let m_detect_chunk_culling_bounds = if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_mode = if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };

        Ok(TilemapRenderer {
            parent,
            m_game_object,
            m_enabled,
            m_cast_shadows,
            m_receive_shadows,
            m_dynamic_occludee,
            m_motion_vectors,
            m_static_shadow_caster,
            m_light_probe_usage,
            m_reflection_probe_usage,
            m_lightmap_index,
            m_rendering_layer_mask,
            m_ray_tracing_mode,
            m_lightmap_index_dynamic,
            m_renderer_priority,
            m_ray_trace_procedural,
            m_lightmap_tiling_offset,
            m_lightmap_tiling_offset_dynamic,
            m_ray_tracing_accel_struct_build_flags_override,
            m_materials,
            m_ray_tracing_accel_struct_build_flags,
            m_static_batch_info,
            m_small_mesh_culling,
            m_static_batch_root,
            m_force_mesh_lod,
            m_probe_anchor,
            m_mesh_lod_selection_bias,
            m_light_probe_volume_override,
            m_sorting_layer_id,
            m_sorting_layer,
            m_sorting_order,
            m_chunk_size,
            m_max_chunk_count,
            m_chunk_culling_bounds,
            m_max_frame_age,
            m_sort_order,
            m_mask_interaction,
            m_detect_chunk_culling_bounds,
            m_mode,
        })
    }
}
