//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::static_batch_info::StaticBatchInfo;
use crate::subclasses::vector4f::Vector4f;


/// 版本枚举：UiRenderer_1931382933MRayTracingAccelStructBuildFlagsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum UiRenderer_1931382933MRayTracingAccelStructBuildFlagsVariant {
    Default(u8),
    V2023_3(u8),
}

impl crate::Parse for UiRenderer_1931382933MRayTracingAccelStructBuildFlagsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            let v = rdr.read_u8()?;
            Ok(Self::V2023_3(v))
        } else {
            let v = rdr.read_u8()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct UiRenderer_1931382933 {
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
    pub m_ray_tracing_accel_struct_build_flags: UiRenderer_1931382933MRayTracingAccelStructBuildFlagsVariant,
    pub m_rendering_layer_mask: u32,
    pub m_small_mesh_culling: Option<u8>,
    pub m_renderer_priority: i32,
    pub m_force_mesh_lod: Option<i16>,
    pub m_lightmap_index: u16,
    pub m_mesh_lod_selection_bias: Option<f32>,
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
    pub m_mask_interaction: Option<i32>,
}

impl crate::Parse for UiRenderer_1931382933 {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut m_enabled = None;
        let mut m_cast_shadows = None;
        let mut m_receive_shadows = None;
        let mut m_dynamic_occludee = None;
        let mut m_static_shadow_caster = None;
        let mut m_motion_vectors = None;
        let mut m_light_probe_usage = None;
        let mut m_reflection_probe_usage = None;
        let mut m_ray_tracing_mode = None;
        let mut m_ray_trace_procedural = None;
        let mut m_ray_tracing_accel_struct_build_flags_override = None;
        let mut m_ray_tracing_accel_struct_build_flags = None;
        let mut m_rendering_layer_mask = None;
        let mut m_small_mesh_culling = None;
        let mut m_renderer_priority = None;
        let mut m_force_mesh_lod = None;
        let mut m_lightmap_index = None;
        let mut m_mesh_lod_selection_bias = None;
        let mut m_lightmap_index_dynamic = None;
        let mut m_lightmap_tiling_offset = None;
        let mut m_lightmap_tiling_offset_dynamic = None;
        let mut m_materials = None;
        let mut m_static_batch_info = None;
        let mut m_static_batch_root = None;
        let mut m_probe_anchor = None;
        let mut m_light_probe_volume_override = None;
        let mut m_sorting_layer_id = None;
        let mut m_sorting_layer = None;
        let mut m_sorting_order = None;
        let mut m_mask_interaction = None;
            m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
            m_enabled = Some(rdr.read_bool()?);
            m_cast_shadows = Some(rdr.read_u8()?);
            m_receive_shadows = Some(rdr.read_u8()?);
            m_dynamic_occludee = Some(rdr.read_u8()?);
            m_static_shadow_caster = Some(rdr.read_u8()?);
            m_motion_vectors = Some(rdr.read_u8()?);
            m_light_probe_usage = Some(rdr.read_u8()?);
            m_reflection_probe_usage = Some(rdr.read_u8()?);
            m_ray_tracing_mode = Some(rdr.read_u8()?);
            m_ray_trace_procedural = Some(rdr.read_u8()?);
            m_ray_tracing_accel_struct_build_flags_override = Some(rdr.read_u8()?);
            m_ray_tracing_accel_struct_build_flags = Some(UiRenderer_1931382933MRayTracingAccelStructBuildFlagsVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(6000, 2, 0) {
            m_small_mesh_culling = Some(rdr.read_u8()?);
            rdr.align(4)?;
            m_force_mesh_lod = Some(rdr.read_i16()?);
            rdr.align(4)?;
            m_mesh_lod_selection_bias = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            m_small_mesh_culling = Some(rdr.read_u8()?);
            rdr.align(4)?;
        } else {
        }
            m_rendering_layer_mask = Some(rdr.read_u32()?);
            m_renderer_priority = Some(rdr.read_i32()?);
            m_lightmap_index = Some(rdr.read_u16()?);
            m_lightmap_index_dynamic = Some(rdr.read_u16()?);
            m_lightmap_tiling_offset = Some(Vector4f::parse(rdr, ver)?);
            m_lightmap_tiling_offset_dynamic = Some(Vector4f::parse(rdr, ver)?);
            m_materials = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_static_batch_info = Some(StaticBatchInfo::parse(rdr, ver)?);
            m_static_batch_root = Some(crate::PPtr::parse(rdr, ver)?);
            m_probe_anchor = Some(crate::PPtr::parse(rdr, ver)?);
            m_light_probe_volume_override = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
            m_sorting_layer_id = Some(rdr.read_i32()?);
            m_sorting_layer = Some(rdr.read_i16()?);
            m_sorting_order = Some(rdr.read_i16()?);
            rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(6000, 3, 0) {
            m_mask_interaction = Some(rdr.read_i32()?);
            rdr.align(4)?;
        } else {
        }

        Ok(UiRenderer_1931382933 {
            m_game_object: m_game_object.unwrap(),
            m_enabled: m_enabled.unwrap(),
            m_cast_shadows: m_cast_shadows.unwrap(),
            m_receive_shadows: m_receive_shadows.unwrap(),
            m_dynamic_occludee: m_dynamic_occludee.unwrap(),
            m_static_shadow_caster: m_static_shadow_caster.unwrap(),
            m_motion_vectors: m_motion_vectors.unwrap(),
            m_light_probe_usage: m_light_probe_usage.unwrap(),
            m_reflection_probe_usage: m_reflection_probe_usage.unwrap(),
            m_ray_tracing_mode: m_ray_tracing_mode.unwrap(),
            m_ray_trace_procedural: m_ray_trace_procedural.unwrap(),
            m_ray_tracing_accel_struct_build_flags_override: m_ray_tracing_accel_struct_build_flags_override.unwrap(),
            m_ray_tracing_accel_struct_build_flags: m_ray_tracing_accel_struct_build_flags.unwrap(),
            m_rendering_layer_mask: m_rendering_layer_mask.unwrap(),
            m_small_mesh_culling,
            m_renderer_priority: m_renderer_priority.unwrap(),
            m_force_mesh_lod,
            m_lightmap_index: m_lightmap_index.unwrap(),
            m_mesh_lod_selection_bias,
            m_lightmap_index_dynamic: m_lightmap_index_dynamic.unwrap(),
            m_lightmap_tiling_offset: m_lightmap_tiling_offset.unwrap(),
            m_lightmap_tiling_offset_dynamic: m_lightmap_tiling_offset_dynamic.unwrap(),
            m_materials: m_materials.unwrap(),
            m_static_batch_info: m_static_batch_info.unwrap(),
            m_static_batch_root: m_static_batch_root.unwrap(),
            m_probe_anchor: m_probe_anchor.unwrap(),
            m_light_probe_volume_override: m_light_probe_volume_override.unwrap(),
            m_sorting_layer_id: m_sorting_layer_id.unwrap(),
            m_sorting_layer: m_sorting_layer.unwrap(),
            m_sorting_order: m_sorting_order.unwrap(),
            m_mask_interaction,
        })
    }
}
