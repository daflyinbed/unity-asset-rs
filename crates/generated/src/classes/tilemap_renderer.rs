//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::int3_storage::int3_storage;
use crate::subclasses::static_batch_info::StaticBatchInfo;
use crate::subclasses::vector3f::Vector3f;
use crate::subclasses::vector4f::Vector4f;


/// 版本枚举：TilemapRendererMReflectionProbeUsageVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum TilemapRendererMReflectionProbeUsageVariant {
    Default(u8),
    V2019_3(u8),
}

impl crate::Parse for TilemapRendererMReflectionProbeUsageVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            let v = rdr.read_u8()?;
            Ok(Self::V2019_3(v))
        } else {
            let v = rdr.read_u8()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：TilemapRendererMRayTracingModeVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum TilemapRendererMRayTracingModeVariant {
    Default(u8),
    V2020_1(u8),
}

impl crate::Parse for TilemapRendererMRayTracingModeVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = rdr.read_u8()?;
            Ok(Self::V2020_1(v))
        } else {
            let v = rdr.read_u8()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：TilemapRendererMRayTraceProceduralVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum TilemapRendererMRayTraceProceduralVariant {
    Default(u8),
    V2023_2(u8),
}

impl crate::Parse for TilemapRendererMRayTraceProceduralVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            let v = rdr.read_u8()?;
            Ok(Self::V2023_2(v))
        } else {
            let v = rdr.read_u8()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：TilemapRendererMRayTracingAccelStructBuildFlagsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum TilemapRendererMRayTracingAccelStructBuildFlagsVariant {
    Default(u8),
    V2023_3(u8),
}

impl crate::Parse for TilemapRendererMRayTracingAccelStructBuildFlagsVariant {
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

/// 版本枚举：TilemapRendererMDetectChunkCullingBoundsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum TilemapRendererMDetectChunkCullingBoundsVariant {
    Default(i32),
    V6000_3(i32),
}

impl crate::Parse for TilemapRendererMDetectChunkCullingBoundsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(6000, 3, 0) {
            let v = rdr.read_i32()?;
            rdr.align(4)?;
            Ok(Self::V6000_3(v))
        } else {
            let v = rdr.read_i32()?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct TilemapRenderer {
    pub m_game_object: PPtr,
    pub m_enabled: bool,
    pub m_cast_shadows: u8,
    pub m_receive_shadows: u8,
    pub m_dynamic_occludee: u8,
    pub m_motion_vectors: u8,
    pub m_static_shadow_caster: Option<u8>,
    pub m_light_probe_usage: u8,
    pub m_reflection_probe_usage: TilemapRendererMReflectionProbeUsageVariant,
    pub m_lightmap_index: u16,
    pub m_rendering_layer_mask: Option<u32>,
    pub m_ray_tracing_mode: Option<TilemapRendererMRayTracingModeVariant>,
    pub m_lightmap_index_dynamic: u16,
    pub m_renderer_priority: Option<i32>,
    pub m_ray_trace_procedural: Option<TilemapRendererMRayTraceProceduralVariant>,
    pub m_lightmap_tiling_offset: Vector4f,
    pub m_lightmap_tiling_offset_dynamic: Vector4f,
    pub m_ray_tracing_accel_struct_build_flags_override: Option<u8>,
    pub m_materials: Vec<PPtr>,
    pub m_ray_tracing_accel_struct_build_flags: Option<TilemapRendererMRayTracingAccelStructBuildFlagsVariant>,
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
    pub m_detect_chunk_culling_bounds: Option<TilemapRendererMDetectChunkCullingBoundsVariant>,
    pub m_mode: Option<i32>,
}

impl crate::Parse for TilemapRenderer {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut m_enabled = None;
        let mut m_cast_shadows = None;
        let mut m_receive_shadows = None;
        let mut m_dynamic_occludee = None;
        let mut m_motion_vectors = None;
        let mut m_static_shadow_caster = None;
        let mut m_light_probe_usage = None;
        let mut m_reflection_probe_usage = None;
        let mut m_lightmap_index = None;
        let mut m_rendering_layer_mask = None;
        let mut m_ray_tracing_mode = None;
        let mut m_lightmap_index_dynamic = None;
        let mut m_renderer_priority = None;
        let mut m_ray_trace_procedural = None;
        let mut m_lightmap_tiling_offset = None;
        let mut m_lightmap_tiling_offset_dynamic = None;
        let mut m_ray_tracing_accel_struct_build_flags_override = None;
        let mut m_materials = None;
        let mut m_ray_tracing_accel_struct_build_flags = None;
        let mut m_static_batch_info = None;
        let mut m_small_mesh_culling = None;
        let mut m_static_batch_root = None;
        let mut m_force_mesh_lod = None;
        let mut m_probe_anchor = None;
        let mut m_mesh_lod_selection_bias = None;
        let mut m_light_probe_volume_override = None;
        let mut m_sorting_layer_id = None;
        let mut m_sorting_layer = None;
        let mut m_sorting_order = None;
        let mut m_chunk_size = None;
        let mut m_max_chunk_count = None;
        let mut m_chunk_culling_bounds = None;
        let mut m_max_frame_age = None;
        let mut m_sort_order = None;
        let mut m_mask_interaction = None;
        let mut m_detect_chunk_culling_bounds = None;
        let mut m_mode = None;
            m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
            m_enabled = Some(rdr.read_bool()?);
            m_cast_shadows = Some(rdr.read_u8()?);
            m_receive_shadows = Some(rdr.read_u8()?);
            m_dynamic_occludee = Some(rdr.read_u8()?);
        if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            m_static_shadow_caster = Some(rdr.read_u8()?);
        } else {
        }
            m_motion_vectors = Some(rdr.read_u8()?);
            m_light_probe_usage = Some(rdr.read_u8()?);
            m_reflection_probe_usage = Some(TilemapRendererMReflectionProbeUsageVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(6000, 2, 0) {
            m_ray_tracing_mode = Some(TilemapRendererMRayTracingModeVariant::parse(rdr, ver)?);
            m_ray_trace_procedural = Some(TilemapRendererMRayTraceProceduralVariant::parse(rdr, ver)?);
            m_ray_tracing_accel_struct_build_flags_override = Some(rdr.read_u8()?);
            m_ray_tracing_accel_struct_build_flags = Some(TilemapRendererMRayTracingAccelStructBuildFlagsVariant::parse(rdr, ver)?);
            m_small_mesh_culling = Some(rdr.read_u8()?);
            rdr.align(4)?;
            m_force_mesh_lod = Some(rdr.read_i16()?);
            rdr.align(4)?;
            m_mesh_lod_selection_bias = Some(rdr.read_f32()?);
            m_rendering_layer_mask = Some(rdr.read_u32()?);
            m_renderer_priority = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            m_ray_tracing_mode = Some(TilemapRendererMRayTracingModeVariant::parse(rdr, ver)?);
            m_ray_trace_procedural = Some(TilemapRendererMRayTraceProceduralVariant::parse(rdr, ver)?);
            m_ray_tracing_accel_struct_build_flags_override = Some(rdr.read_u8()?);
            m_ray_tracing_accel_struct_build_flags = Some(TilemapRendererMRayTracingAccelStructBuildFlagsVariant::parse(rdr, ver)?);
            m_small_mesh_culling = Some(rdr.read_u8()?);
            rdr.align(4)?;
            m_rendering_layer_mask = Some(rdr.read_u32()?);
            m_renderer_priority = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            m_ray_tracing_mode = Some(TilemapRendererMRayTracingModeVariant::parse(rdr, ver)?);
            m_ray_trace_procedural = Some(TilemapRendererMRayTraceProceduralVariant::parse(rdr, ver)?);
            m_ray_tracing_accel_struct_build_flags_override = Some(rdr.read_u8()?);
            m_ray_tracing_accel_struct_build_flags = Some(TilemapRendererMRayTracingAccelStructBuildFlagsVariant::parse(rdr, ver)?);
            m_rendering_layer_mask = Some(rdr.read_u32()?);
            m_renderer_priority = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            m_ray_tracing_mode = Some(TilemapRendererMRayTracingModeVariant::parse(rdr, ver)?);
            m_ray_trace_procedural = Some(TilemapRendererMRayTraceProceduralVariant::parse(rdr, ver)?);
            m_rendering_layer_mask = Some(rdr.read_u32()?);
            m_renderer_priority = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            m_ray_tracing_mode = Some(TilemapRendererMRayTracingModeVariant::parse(rdr, ver)?);
            m_rendering_layer_mask = Some(rdr.read_u32()?);
            m_renderer_priority = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            m_rendering_layer_mask = Some(rdr.read_u32()?);
            m_renderer_priority = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            m_rendering_layer_mask = Some(rdr.read_u32()?);
        } else {
        }
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
            m_chunk_size = Some(int3_storage::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2017, 4, 33) {
            m_chunk_culling_bounds = Some(Vector3f::parse(rdr, ver)?);
        } else {
        }
            m_max_chunk_count = Some(rdr.read_u32()?);
            m_max_frame_age = Some(rdr.read_u32()?);
            m_sort_order = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(6000, 3, 0) {
            m_mode = Some(rdr.read_i32()?);
            m_detect_chunk_culling_bounds = Some(TilemapRendererMDetectChunkCullingBoundsVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            m_mode = Some(rdr.read_i32()?);
            m_detect_chunk_culling_bounds = Some(TilemapRendererMDetectChunkCullingBoundsVariant::parse(rdr, ver)?);
            m_mask_interaction = Some(rdr.read_i32()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            m_detect_chunk_culling_bounds = Some(TilemapRendererMDetectChunkCullingBoundsVariant::parse(rdr, ver)?);
            m_mask_interaction = Some(rdr.read_i32()?);
            rdr.align(4)?;
        } else {
            m_mask_interaction = Some(rdr.read_i32()?);
            rdr.align(4)?;
        }

        Ok(TilemapRenderer {
            m_game_object: m_game_object.unwrap(),
            m_enabled: m_enabled.unwrap(),
            m_cast_shadows: m_cast_shadows.unwrap(),
            m_receive_shadows: m_receive_shadows.unwrap(),
            m_dynamic_occludee: m_dynamic_occludee.unwrap(),
            m_motion_vectors: m_motion_vectors.unwrap(),
            m_static_shadow_caster,
            m_light_probe_usage: m_light_probe_usage.unwrap(),
            m_reflection_probe_usage: m_reflection_probe_usage.unwrap(),
            m_lightmap_index: m_lightmap_index.unwrap(),
            m_rendering_layer_mask,
            m_ray_tracing_mode,
            m_lightmap_index_dynamic: m_lightmap_index_dynamic.unwrap(),
            m_renderer_priority,
            m_ray_trace_procedural,
            m_lightmap_tiling_offset: m_lightmap_tiling_offset.unwrap(),
            m_lightmap_tiling_offset_dynamic: m_lightmap_tiling_offset_dynamic.unwrap(),
            m_ray_tracing_accel_struct_build_flags_override,
            m_materials: m_materials.unwrap(),
            m_ray_tracing_accel_struct_build_flags,
            m_static_batch_info: m_static_batch_info.unwrap(),
            m_small_mesh_culling,
            m_static_batch_root: m_static_batch_root.unwrap(),
            m_force_mesh_lod,
            m_probe_anchor: m_probe_anchor.unwrap(),
            m_mesh_lod_selection_bias,
            m_light_probe_volume_override: m_light_probe_volume_override.unwrap(),
            m_sorting_layer_id: m_sorting_layer_id.unwrap(),
            m_sorting_layer: m_sorting_layer.unwrap(),
            m_sorting_order: m_sorting_order.unwrap(),
            m_chunk_size: m_chunk_size.unwrap(),
            m_max_chunk_count: m_max_chunk_count.unwrap(),
            m_chunk_culling_bounds,
            m_max_frame_age: m_max_frame_age.unwrap(),
            m_sort_order: m_sort_order.unwrap(),
            m_mask_interaction: m_mask_interaction.unwrap(),
            m_detect_chunk_culling_bounds,
            m_mode,
        })
    }
}
