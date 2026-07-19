//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::static_batch_info::StaticBatchInfo;
use crate::subclasses::vector4f::Vector4f;

#[derive(Debug, Clone)]
pub struct RenderAs2D {
    pub m_game_object: PPtr,
    pub m_enabled: bool,
    pub m_cast_shadows: u8,
    pub m_receive_shadows: u8,
    pub m_dynamic_occludee: u8,
    pub m_static_shadow_caster: u8,
    pub m_motion_vectors: u8,
    pub m_light_probe_usage: u8,
    pub m_reflection_probe_usage: u8,
    pub m_ray_tracing_mode: u8,
    pub m_ray_trace_procedural: u8,
    pub m_ray_tracing_accel_struct_build_flags_override: u8,
    pub m_ray_tracing_accel_struct_build_flags: u8,
    pub m_small_mesh_culling: u8,
    pub m_force_mesh_lod: i16,
    pub m_mesh_lod_selection_bias: f32,
    pub m_rendering_layer_mask: u32,
    pub m_renderer_priority: i32,
    pub m_lightmap_index: u16,
    pub m_lightmap_index_dynamic: u16,
    pub m_lightmap_tiling_offset: Vector4f,
    pub m_lightmap_tiling_offset_dynamic: Vector4f,
    pub m_materials: Vec<PPtr>,
    pub m_static_batch_info: StaticBatchInfo,
    pub m_static_batch_root: PPtr,
    pub m_probe_anchor: PPtr,
    pub m_light_probe_volume_override: PPtr,
    pub m_sorting_layer_id: i32,
    pub m_sorting_layer: i16,
    pub m_sorting_order: i16,
    pub m_mask_interaction: i32,
    pub m_owning_sorting_group: PPtr,
}

impl crate::Parse for RenderAs2D {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(rdr, ver)?;
        let m_enabled = rdr.read_bool()?;
        let m_cast_shadows = rdr.read_u8()?;
        let m_receive_shadows = rdr.read_u8()?;
        let m_dynamic_occludee = rdr.read_u8()?;
        let m_static_shadow_caster = rdr.read_u8()?;
        let m_motion_vectors = rdr.read_u8()?;
        let m_light_probe_usage = rdr.read_u8()?;
        let m_reflection_probe_usage = rdr.read_u8()?;
        let m_ray_tracing_mode = rdr.read_u8()?;
        let m_ray_trace_procedural = rdr.read_u8()?;
        let m_ray_tracing_accel_struct_build_flags_override = rdr.read_u8()?;
        let m_ray_tracing_accel_struct_build_flags = rdr.read_u8()?;
        let m_small_mesh_culling = rdr.read_u8()?;
        rdr.align(4)?;
        let m_force_mesh_lod = rdr.read_i16()?;
        rdr.align(4)?;
        let m_mesh_lod_selection_bias = rdr.read_f32()?;
        let m_rendering_layer_mask = rdr.read_u32()?;
        let m_renderer_priority = rdr.read_i32()?;
        let m_lightmap_index = rdr.read_u16()?;
        let m_lightmap_index_dynamic = rdr.read_u16()?;
        let m_lightmap_tiling_offset = Vector4f::parse(rdr, ver)?;
        let m_lightmap_tiling_offset_dynamic = Vector4f::parse(rdr, ver)?;
        let m_materials = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| crate::PPtr::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let m_static_batch_info = StaticBatchInfo::parse(rdr, ver)?;
        let m_static_batch_root = crate::PPtr::parse(rdr, ver)?;
        let m_probe_anchor = crate::PPtr::parse(rdr, ver)?;
        let m_light_probe_volume_override = crate::PPtr::parse(rdr, ver)?;
        rdr.align(4)?;
        let m_sorting_layer_id = rdr.read_i32()?;
        let m_sorting_layer = rdr.read_i16()?;
        let m_sorting_order = rdr.read_i16()?;
        rdr.align(4)?;
        let m_mask_interaction = rdr.read_i32()?;
        rdr.align(4)?;
        let m_owning_sorting_group = crate::PPtr::parse(rdr, ver)?;

        Ok(RenderAs2D {
            m_game_object,
            m_enabled,
            m_cast_shadows,
            m_receive_shadows,
            m_dynamic_occludee,
            m_static_shadow_caster,
            m_motion_vectors,
            m_light_probe_usage,
            m_reflection_probe_usage,
            m_ray_tracing_mode,
            m_ray_trace_procedural,
            m_ray_tracing_accel_struct_build_flags_override,
            m_ray_tracing_accel_struct_build_flags,
            m_small_mesh_culling,
            m_force_mesh_lod,
            m_mesh_lod_selection_bias,
            m_rendering_layer_mask,
            m_renderer_priority,
            m_lightmap_index,
            m_lightmap_index_dynamic,
            m_lightmap_tiling_offset,
            m_lightmap_tiling_offset_dynamic,
            m_materials,
            m_static_batch_info,
            m_static_batch_root,
            m_probe_anchor,
            m_light_probe_volume_override,
            m_sorting_layer_id,
            m_sorting_layer,
            m_sorting_order,
            m_mask_interaction,
            m_owning_sorting_group,
        })
    }
}
