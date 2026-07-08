//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::static_batch_info::StaticBatchInfo;
use crate::subclasses::vector4f::Vector4f;


/// 版本枚举：VfxRendererMReflectionProbeUsageVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum VfxRendererMReflectionProbeUsageVariant {
    Default(u8),
    V2019_3(u8),
}

impl crate::Parse for VfxRendererMReflectionProbeUsageVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            let v = r.read_u8()?;
            Ok(Self::V2019_3(v))
        } else {
            let v = r.read_u8()?;
            r.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：VfxRendererMRayTracingModeVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum VfxRendererMRayTracingModeVariant {
    Default(u8),
    V2020_1(u8),
}

impl crate::Parse for VfxRendererMRayTracingModeVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = r.read_u8()?;
            Ok(Self::V2020_1(v))
        } else {
            let v = r.read_u8()?;
            r.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：VfxRendererMRayTraceProceduralVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum VfxRendererMRayTraceProceduralVariant {
    Default(u8),
    V2023_2(u8),
}

impl crate::Parse for VfxRendererMRayTraceProceduralVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            let v = r.read_u8()?;
            Ok(Self::V2023_2(v))
        } else {
            let v = r.read_u8()?;
            r.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：VfxRendererMRayTracingAccelStructBuildFlagsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum VfxRendererMRayTracingAccelStructBuildFlagsVariant {
    Default(u8),
    V2023_3(u8),
}

impl crate::Parse for VfxRendererMRayTracingAccelStructBuildFlagsVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            let v = r.read_u8()?;
            Ok(Self::V2023_3(v))
        } else {
            let v = r.read_u8()?;
            r.align(4)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct VfxRenderer {
    pub m_game_object: PPtr,
    pub m_enabled: bool,
    pub m_cast_shadows: u8,
    pub m_receive_shadows: u8,
    pub m_dynamic_occludee: u8,
    pub m_motion_vectors: u8,
    pub m_static_shadow_caster: Option<u8>,
    pub m_light_probe_usage: u8,
    pub m_reflection_probe_usage: VfxRendererMReflectionProbeUsageVariant,
    pub m_rendering_layer_mask: u32,
    pub m_ray_tracing_mode: Option<VfxRendererMRayTracingModeVariant>,
    pub m_renderer_priority: i32,
    pub m_ray_trace_procedural: Option<VfxRendererMRayTraceProceduralVariant>,
    pub m_lightmap_index: u16,
    pub m_lightmap_index_dynamic: u16,
    pub m_ray_tracing_accel_struct_build_flags_override: Option<u8>,
    pub m_lightmap_tiling_offset: Vector4f,
    pub m_ray_tracing_accel_struct_build_flags: Option<VfxRendererMRayTracingAccelStructBuildFlagsVariant>,
    pub m_lightmap_tiling_offset_dynamic: Vector4f,
    pub m_small_mesh_culling: Option<u8>,
    pub m_materials: Option<Vec<PPtr>>,
    pub m_force_mesh_lod: Option<i16>,
    pub m_static_batch_info: StaticBatchInfo,
    pub m_mesh_lod_selection_bias: Option<f32>,
    pub m_static_batch_root: PPtr,
    pub m_probe_anchor: PPtr,
    pub m_light_probe_volume_override: PPtr,
    pub m_sorting_layer_id: i32,
    pub m_sorting_layer: i16,
    pub m_sorting_order: i16,
    pub m_mask_interaction: Option<i32>,
}

impl crate::Parse for VfxRenderer {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
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
        let mut m_rendering_layer_mask = None;
        let mut m_ray_tracing_mode = None;
        let mut m_renderer_priority = None;
        let mut m_ray_trace_procedural = None;
        let mut m_lightmap_index = None;
        let mut m_lightmap_index_dynamic = None;
        let mut m_ray_tracing_accel_struct_build_flags_override = None;
        let mut m_lightmap_tiling_offset = None;
        let mut m_ray_tracing_accel_struct_build_flags = None;
        let mut m_lightmap_tiling_offset_dynamic = None;
        let mut m_small_mesh_culling = None;
        let mut m_materials = None;
        let mut m_force_mesh_lod = None;
        let mut m_static_batch_info = None;
        let mut m_mesh_lod_selection_bias = None;
        let mut m_static_batch_root = None;
        let mut m_probe_anchor = None;
        let mut m_light_probe_volume_override = None;
        let mut m_sorting_layer_id = None;
        let mut m_sorting_layer = None;
        let mut m_sorting_order = None;
        let mut m_mask_interaction = None;
            m_game_object = Some(crate::PPtr::parse(r, ver)?);
            m_enabled = Some(r.read_bool()?);
            m_cast_shadows = Some(r.read_u8()?);
            m_receive_shadows = Some(r.read_u8()?);
            m_dynamic_occludee = Some(r.read_u8()?);
        if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            m_static_shadow_caster = Some(r.read_u8()?);
        } else {
        }
            m_motion_vectors = Some(r.read_u8()?);
            m_light_probe_usage = Some(r.read_u8()?);
            m_reflection_probe_usage = Some(VfxRendererMReflectionProbeUsageVariant::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(6000, 2, 0) {
            m_ray_tracing_mode = Some(VfxRendererMRayTracingModeVariant::parse(r, ver)?);
            m_ray_trace_procedural = Some(VfxRendererMRayTraceProceduralVariant::parse(r, ver)?);
            m_ray_tracing_accel_struct_build_flags_override = Some(r.read_u8()?);
            m_ray_tracing_accel_struct_build_flags = Some(VfxRendererMRayTracingAccelStructBuildFlagsVariant::parse(r, ver)?);
            m_small_mesh_culling = Some(r.read_u8()?);
            r.align(4)?;
            m_force_mesh_lod = Some(r.read_i16()?);
            r.align(4)?;
            m_mesh_lod_selection_bias = Some(r.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            m_ray_tracing_mode = Some(VfxRendererMRayTracingModeVariant::parse(r, ver)?);
            m_ray_trace_procedural = Some(VfxRendererMRayTraceProceduralVariant::parse(r, ver)?);
            m_ray_tracing_accel_struct_build_flags_override = Some(r.read_u8()?);
            m_ray_tracing_accel_struct_build_flags = Some(VfxRendererMRayTracingAccelStructBuildFlagsVariant::parse(r, ver)?);
            m_small_mesh_culling = Some(r.read_u8()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            m_ray_tracing_mode = Some(VfxRendererMRayTracingModeVariant::parse(r, ver)?);
            m_ray_trace_procedural = Some(VfxRendererMRayTraceProceduralVariant::parse(r, ver)?);
            m_ray_tracing_accel_struct_build_flags_override = Some(r.read_u8()?);
            m_ray_tracing_accel_struct_build_flags = Some(VfxRendererMRayTracingAccelStructBuildFlagsVariant::parse(r, ver)?);
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            m_ray_tracing_mode = Some(VfxRendererMRayTracingModeVariant::parse(r, ver)?);
            m_ray_trace_procedural = Some(VfxRendererMRayTraceProceduralVariant::parse(r, ver)?);
        } else if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            m_ray_tracing_mode = Some(VfxRendererMRayTracingModeVariant::parse(r, ver)?);
        } else {
        }
            m_rendering_layer_mask = Some(r.read_u32()?);
            m_renderer_priority = Some(r.read_i32()?);
            m_lightmap_index = Some(r.read_u16()?);
            m_lightmap_index_dynamic = Some(r.read_u16()?);
            m_lightmap_tiling_offset = Some(Vector4f::parse(r, ver)?);
            m_lightmap_tiling_offset_dynamic = Some(Vector4f::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(2021, 2, 0) {
        } else if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            m_materials = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2020, 3, 26) {
        } else {
            m_materials = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        }
            m_static_batch_info = Some(StaticBatchInfo::parse(r, ver)?);
            m_static_batch_root = Some(crate::PPtr::parse(r, ver)?);
            m_probe_anchor = Some(crate::PPtr::parse(r, ver)?);
            m_light_probe_volume_override = Some(crate::PPtr::parse(r, ver)?);
            r.align(4)?;
            m_sorting_layer_id = Some(r.read_i32()?);
            m_sorting_layer = Some(r.read_i16()?);
            m_sorting_order = Some(r.read_i16()?);
            r.align(4)?;
        if *ver >= crate::UnityVersion::new(6000, 3, 0) {
            m_mask_interaction = Some(r.read_i32()?);
            r.align(4)?;
        } else {
        }

        Ok(VfxRenderer {
            m_game_object: m_game_object.unwrap(),
            m_enabled: m_enabled.unwrap(),
            m_cast_shadows: m_cast_shadows.unwrap(),
            m_receive_shadows: m_receive_shadows.unwrap(),
            m_dynamic_occludee: m_dynamic_occludee.unwrap(),
            m_motion_vectors: m_motion_vectors.unwrap(),
            m_static_shadow_caster,
            m_light_probe_usage: m_light_probe_usage.unwrap(),
            m_reflection_probe_usage: m_reflection_probe_usage.unwrap(),
            m_rendering_layer_mask: m_rendering_layer_mask.unwrap(),
            m_ray_tracing_mode,
            m_renderer_priority: m_renderer_priority.unwrap(),
            m_ray_trace_procedural,
            m_lightmap_index: m_lightmap_index.unwrap(),
            m_lightmap_index_dynamic: m_lightmap_index_dynamic.unwrap(),
            m_ray_tracing_accel_struct_build_flags_override,
            m_lightmap_tiling_offset: m_lightmap_tiling_offset.unwrap(),
            m_ray_tracing_accel_struct_build_flags,
            m_lightmap_tiling_offset_dynamic: m_lightmap_tiling_offset_dynamic.unwrap(),
            m_small_mesh_culling,
            m_materials,
            m_force_mesh_lod,
            m_static_batch_info: m_static_batch_info.unwrap(),
            m_mesh_lod_selection_bias,
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
