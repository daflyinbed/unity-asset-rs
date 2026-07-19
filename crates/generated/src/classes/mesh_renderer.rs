//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::static_batch_info::StaticBatchInfo;
use crate::subclasses::vector4f::Vector4f;

/// 版本枚举：MeshRendererMEnabledVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum MeshRendererMEnabledVariant {
    Default(bool),
    V5_0(bool),
    V5_4(bool),
}

impl crate::Parse for MeshRendererMEnabledVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V5_4(v))
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V5_0(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：MeshRendererMCastShadowsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum MeshRendererMCastShadowsVariant {
    Default(bool),
    V5_0(u8),
}

impl crate::Parse for MeshRendererMCastShadowsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = rdr.read_u8()?;
            Ok(Self::V5_0(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：MeshRendererMReceiveShadowsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum MeshRendererMReceiveShadowsVariant {
    Default(bool),
    V5_0(bool),
    V5_4(u8),
}

impl crate::Parse for MeshRendererMReceiveShadowsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = rdr.read_u8()?;
            Ok(Self::V5_4(v))
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V5_0(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：MeshRendererMLightmapIndexVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum MeshRendererMLightmapIndexVariant {
    Default(u8),
    V5_0(u16),
}

impl crate::Parse for MeshRendererMLightmapIndexVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = rdr.read_u16()?;
            Ok(Self::V5_0(v))
        } else {
            let v = rdr.read_u8()?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：MeshRendererMMaterialsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum MeshRendererMMaterialsVariant {
    Default(Vec<PPtr>),
    V2017_1(Vec<PPtr>),
}

impl crate::Parse for MeshRendererMMaterialsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| crate::PPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| crate::PPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：MeshRendererMReflectionProbeUsageVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum MeshRendererMReflectionProbeUsageVariant {
    Default(i32),
    V5_4(u8),
    V2019_3(u8),
}

impl crate::Parse for MeshRendererMReflectionProbeUsageVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            let v = rdr.read_u8()?;
            Ok(Self::V2019_3(v))
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = rdr.read_u8()?;
            rdr.align(4)?;
            Ok(Self::V5_4(v))
        } else {
            let v = rdr.read_i32()?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：MeshRendererMRayTracingModeVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum MeshRendererMRayTracingModeVariant {
    Default(u8),
    V2020_1(u8),
}

impl crate::Parse for MeshRendererMRayTracingModeVariant {
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

/// 版本枚举：MeshRendererMRayTraceProceduralVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum MeshRendererMRayTraceProceduralVariant {
    Default(u8),
    V2023_2(u8),
}

impl crate::Parse for MeshRendererMRayTraceProceduralVariant {
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

/// 版本枚举：MeshRendererMLightProbeAnchorVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum MeshRendererMLightProbeAnchorVariant {
    Default(PPtr),
    V4_3(PPtr),
}

impl crate::Parse for MeshRendererMLightProbeAnchorVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(4, 3, 0) {
            let v = crate::PPtr::parse(rdr, ver)?;
            rdr.align(4)?;
            Ok(Self::V4_3(v))
        } else {
            let v = crate::PPtr::parse(rdr, ver)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：MeshRendererMSortingLayerIdVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum MeshRendererMSortingLayerIdVariant {
    Default(u32),
    V5_0(i32),
}

impl crate::Parse for MeshRendererMSortingLayerIdVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = rdr.read_i32()?;
            Ok(Self::V5_0(v))
        } else {
            let v = rdr.read_u32()?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：MeshRendererMSortingOrderVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum MeshRendererMSortingOrderVariant {
    Default(i16),
    V4_5(i16),
}

impl crate::Parse for MeshRendererMSortingOrderVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(4, 5, 0) {
            let v = rdr.read_i16()?;
            rdr.align(4)?;
            Ok(Self::V4_5(v))
        } else {
            let v = rdr.read_i16()?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：MeshRendererMRayTracingAccelStructBuildFlagsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum MeshRendererMRayTracingAccelStructBuildFlagsVariant {
    Default(u8),
    V2023_3(u8),
}

impl crate::Parse for MeshRendererMRayTracingAccelStructBuildFlagsVariant {
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

/// 版本枚举：MeshRendererMProbeAnchorVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum MeshRendererMProbeAnchorVariant {
    Default(PPtr),
    V5_4(PPtr),
}

impl crate::Parse for MeshRendererMProbeAnchorVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = crate::PPtr::parse(rdr, ver)?;
            Ok(Self::V5_4(v))
        } else {
            let v = crate::PPtr::parse(rdr, ver)?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct MeshRenderer {
    pub m_game_object: PPtr,
    pub m_enabled: MeshRendererMEnabledVariant,
    pub m_cast_shadows: MeshRendererMCastShadowsVariant,
    pub m_receive_shadows: MeshRendererMReceiveShadowsVariant,
    pub m_lightmap_index: MeshRendererMLightmapIndexVariant,
    pub m_motion_vectors: Option<u8>,
    pub m_dynamic_occludee: Option<u8>,
    pub m_lightmap_tiling_offset: Vector4f,
    pub m_lightmap_index_dynamic: Option<u16>,
    pub m_light_probe_usage: Option<u8>,
    pub m_static_shadow_caster: Option<u8>,
    pub m_materials: MeshRendererMMaterialsVariant,
    pub m_reflection_probe_usage: Option<MeshRendererMReflectionProbeUsageVariant>,
    pub m_subset_indices: Option<Vec<u32>>,
    pub m_lightmap_tiling_offset_dynamic: Option<Vector4f>,
    pub m_static_batch_root: PPtr,
    pub m_rendering_layer_mask: Option<u32>,
    pub m_ray_tracing_mode: Option<MeshRendererMRayTracingModeVariant>,
    pub m_use_light_probes: Option<bool>,
    pub m_renderer_priority: Option<i32>,
    pub m_ray_trace_procedural: Option<MeshRendererMRayTraceProceduralVariant>,
    pub m_light_probe_anchor: Option<MeshRendererMLightProbeAnchorVariant>,
    pub m_sorting_layer: Option<i16>,
    pub m_sorting_layer_id: Option<MeshRendererMSortingLayerIdVariant>,
    pub m_ray_tracing_accel_struct_build_flags_override: Option<u8>,
    pub m_sorting_order: Option<MeshRendererMSortingOrderVariant>,
    pub m_static_batch_info: Option<StaticBatchInfo>,
    pub m_ray_tracing_accel_struct_build_flags:
        Option<MeshRendererMRayTracingAccelStructBuildFlagsVariant>,
    pub m_probe_anchor: Option<MeshRendererMProbeAnchorVariant>,
    pub m_small_mesh_culling: Option<u8>,
    pub m_force_mesh_lod: Option<i16>,
    pub m_light_probe_volume_override: Option<PPtr>,
    pub m_mesh_lod_selection_bias: Option<f32>,
    pub m_additional_vertex_streams: Option<PPtr>,
    pub m_enlighten_vertex_stream: Option<PPtr>,
    pub m_mask_interaction: Option<i32>,
}

impl crate::Parse for MeshRenderer {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut m_enabled = None;
        let mut m_cast_shadows = None;
        let mut m_receive_shadows = None;
        let mut m_lightmap_index = None;
        let mut m_motion_vectors = None;
        let mut m_dynamic_occludee = None;
        let mut m_lightmap_tiling_offset = None;
        let mut m_lightmap_index_dynamic = None;
        let mut m_light_probe_usage = None;
        let mut m_static_shadow_caster = None;
        let mut m_materials = None;
        let mut m_reflection_probe_usage = None;
        let mut m_subset_indices = None;
        let mut m_lightmap_tiling_offset_dynamic = None;
        let mut m_static_batch_root = None;
        let mut m_rendering_layer_mask = None;
        let mut m_ray_tracing_mode = None;
        let mut m_use_light_probes = None;
        let mut m_renderer_priority = None;
        let mut m_ray_trace_procedural = None;
        let mut m_light_probe_anchor = None;
        let mut m_sorting_layer = None;
        let mut m_sorting_layer_id = None;
        let mut m_ray_tracing_accel_struct_build_flags_override = None;
        let mut m_sorting_order = None;
        let mut m_static_batch_info = None;
        let mut m_ray_tracing_accel_struct_build_flags = None;
        let mut m_probe_anchor = None;
        let mut m_small_mesh_culling = None;
        let mut m_force_mesh_lod = None;
        let mut m_light_probe_volume_override = None;
        let mut m_mesh_lod_selection_bias = None;
        let mut m_additional_vertex_streams = None;
        let mut m_enlighten_vertex_stream = None;
        let mut m_mask_interaction = None;
        m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
        m_enabled = Some(MeshRendererMEnabledVariant::parse(rdr, ver)?);
        m_cast_shadows = Some(MeshRendererMCastShadowsVariant::parse(rdr, ver)?);
        m_receive_shadows = Some(MeshRendererMReceiveShadowsVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(6000, 2, 0) {
            m_dynamic_occludee = Some(rdr.read_u8()?);
            m_static_shadow_caster = Some(rdr.read_u8()?);
            m_motion_vectors = Some(rdr.read_u8()?);
            m_light_probe_usage = Some(rdr.read_u8()?);
            m_reflection_probe_usage =
                Some(MeshRendererMReflectionProbeUsageVariant::parse(rdr, ver)?);
            m_ray_tracing_mode = Some(MeshRendererMRayTracingModeVariant::parse(rdr, ver)?);
            m_ray_trace_procedural = Some(MeshRendererMRayTraceProceduralVariant::parse(rdr, ver)?);
            m_ray_tracing_accel_struct_build_flags_override = Some(rdr.read_u8()?);
            m_ray_tracing_accel_struct_build_flags = Some(
                MeshRendererMRayTracingAccelStructBuildFlagsVariant::parse(rdr, ver)?,
            );
            m_small_mesh_culling = Some(rdr.read_u8()?);
            rdr.align(4)?;
            m_force_mesh_lod = Some(rdr.read_i16()?);
            rdr.align(4)?;
            m_mesh_lod_selection_bias = Some(rdr.read_f32()?);
            m_rendering_layer_mask = Some(rdr.read_u32()?);
            m_renderer_priority = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            m_dynamic_occludee = Some(rdr.read_u8()?);
            m_static_shadow_caster = Some(rdr.read_u8()?);
            m_motion_vectors = Some(rdr.read_u8()?);
            m_light_probe_usage = Some(rdr.read_u8()?);
            m_reflection_probe_usage =
                Some(MeshRendererMReflectionProbeUsageVariant::parse(rdr, ver)?);
            m_ray_tracing_mode = Some(MeshRendererMRayTracingModeVariant::parse(rdr, ver)?);
            m_ray_trace_procedural = Some(MeshRendererMRayTraceProceduralVariant::parse(rdr, ver)?);
            m_ray_tracing_accel_struct_build_flags_override = Some(rdr.read_u8()?);
            m_ray_tracing_accel_struct_build_flags = Some(
                MeshRendererMRayTracingAccelStructBuildFlagsVariant::parse(rdr, ver)?,
            );
            m_small_mesh_culling = Some(rdr.read_u8()?);
            rdr.align(4)?;
            m_rendering_layer_mask = Some(rdr.read_u32()?);
            m_renderer_priority = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            m_dynamic_occludee = Some(rdr.read_u8()?);
            m_static_shadow_caster = Some(rdr.read_u8()?);
            m_motion_vectors = Some(rdr.read_u8()?);
            m_light_probe_usage = Some(rdr.read_u8()?);
            m_reflection_probe_usage =
                Some(MeshRendererMReflectionProbeUsageVariant::parse(rdr, ver)?);
            m_ray_tracing_mode = Some(MeshRendererMRayTracingModeVariant::parse(rdr, ver)?);
            m_ray_trace_procedural = Some(MeshRendererMRayTraceProceduralVariant::parse(rdr, ver)?);
            m_ray_tracing_accel_struct_build_flags_override = Some(rdr.read_u8()?);
            m_ray_tracing_accel_struct_build_flags = Some(
                MeshRendererMRayTracingAccelStructBuildFlagsVariant::parse(rdr, ver)?,
            );
            m_rendering_layer_mask = Some(rdr.read_u32()?);
            m_renderer_priority = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            m_dynamic_occludee = Some(rdr.read_u8()?);
            m_static_shadow_caster = Some(rdr.read_u8()?);
            m_motion_vectors = Some(rdr.read_u8()?);
            m_light_probe_usage = Some(rdr.read_u8()?);
            m_reflection_probe_usage =
                Some(MeshRendererMReflectionProbeUsageVariant::parse(rdr, ver)?);
            m_ray_tracing_mode = Some(MeshRendererMRayTracingModeVariant::parse(rdr, ver)?);
            m_ray_trace_procedural = Some(MeshRendererMRayTraceProceduralVariant::parse(rdr, ver)?);
            m_rendering_layer_mask = Some(rdr.read_u32()?);
            m_renderer_priority = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            m_dynamic_occludee = Some(rdr.read_u8()?);
            m_motion_vectors = Some(rdr.read_u8()?);
            m_light_probe_usage = Some(rdr.read_u8()?);
            m_reflection_probe_usage =
                Some(MeshRendererMReflectionProbeUsageVariant::parse(rdr, ver)?);
            m_ray_tracing_mode = Some(MeshRendererMRayTracingModeVariant::parse(rdr, ver)?);
            m_ray_trace_procedural = Some(MeshRendererMRayTraceProceduralVariant::parse(rdr, ver)?);
            m_rendering_layer_mask = Some(rdr.read_u32()?);
            m_renderer_priority = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            m_dynamic_occludee = Some(rdr.read_u8()?);
            m_motion_vectors = Some(rdr.read_u8()?);
            m_light_probe_usage = Some(rdr.read_u8()?);
            m_reflection_probe_usage =
                Some(MeshRendererMReflectionProbeUsageVariant::parse(rdr, ver)?);
            m_ray_tracing_mode = Some(MeshRendererMRayTracingModeVariant::parse(rdr, ver)?);
            m_rendering_layer_mask = Some(rdr.read_u32()?);
            m_renderer_priority = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            m_dynamic_occludee = Some(rdr.read_u8()?);
            m_motion_vectors = Some(rdr.read_u8()?);
            m_light_probe_usage = Some(rdr.read_u8()?);
            m_reflection_probe_usage =
                Some(MeshRendererMReflectionProbeUsageVariant::parse(rdr, ver)?);
            m_rendering_layer_mask = Some(rdr.read_u32()?);
            m_renderer_priority = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            m_dynamic_occludee = Some(rdr.read_u8()?);
            m_motion_vectors = Some(rdr.read_u8()?);
            m_light_probe_usage = Some(rdr.read_u8()?);
            m_reflection_probe_usage =
                Some(MeshRendererMReflectionProbeUsageVariant::parse(rdr, ver)?);
            m_rendering_layer_mask = Some(rdr.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_dynamic_occludee = Some(rdr.read_u8()?);
            m_motion_vectors = Some(rdr.read_u8()?);
            m_light_probe_usage = Some(rdr.read_u8()?);
            m_reflection_probe_usage =
                Some(MeshRendererMReflectionProbeUsageVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            m_motion_vectors = Some(rdr.read_u8()?);
            m_light_probe_usage = Some(rdr.read_u8()?);
            m_reflection_probe_usage =
                Some(MeshRendererMReflectionProbeUsageVariant::parse(rdr, ver)?);
        } else {
        }
        m_lightmap_index = Some(MeshRendererMLightmapIndexVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_lightmap_index_dynamic = Some(rdr.read_u16()?);
        } else {
        }
        m_lightmap_tiling_offset = Some(Vector4f::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_lightmap_tiling_offset_dynamic = Some(Vector4f::parse(rdr, ver)?);
        } else {
        }
        m_materials = Some(MeshRendererMMaterialsVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            m_static_batch_info = Some(StaticBatchInfo::parse(rdr, ver)?);
        } else {
            m_subset_indices = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_u32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
        }
        m_static_batch_root = Some(crate::PPtr::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(6000, 3, 0) {
            m_probe_anchor = Some(MeshRendererMProbeAnchorVariant::parse(rdr, ver)?);
            m_light_probe_volume_override = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
            m_sorting_layer_id = Some(MeshRendererMSortingLayerIdVariant::parse(rdr, ver)?);
            m_sorting_layer = Some(rdr.read_i16()?);
            m_sorting_order = Some(MeshRendererMSortingOrderVariant::parse(rdr, ver)?);
            m_mask_interaction = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_additional_vertex_streams = Some(crate::PPtr::parse(rdr, ver)?);
            m_enlighten_vertex_stream = Some(crate::PPtr::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            m_probe_anchor = Some(MeshRendererMProbeAnchorVariant::parse(rdr, ver)?);
            m_light_probe_volume_override = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
            m_sorting_layer_id = Some(MeshRendererMSortingLayerIdVariant::parse(rdr, ver)?);
            m_sorting_layer = Some(rdr.read_i16()?);
            m_sorting_order = Some(MeshRendererMSortingOrderVariant::parse(rdr, ver)?);
            m_additional_vertex_streams = Some(crate::PPtr::parse(rdr, ver)?);
            m_enlighten_vertex_stream = Some(crate::PPtr::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_probe_anchor = Some(MeshRendererMProbeAnchorVariant::parse(rdr, ver)?);
            m_light_probe_volume_override = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
            m_sorting_layer_id = Some(MeshRendererMSortingLayerIdVariant::parse(rdr, ver)?);
            m_sorting_layer = Some(rdr.read_i16()?);
            m_sorting_order = Some(MeshRendererMSortingOrderVariant::parse(rdr, ver)?);
            m_additional_vertex_streams = Some(crate::PPtr::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            m_probe_anchor = Some(MeshRendererMProbeAnchorVariant::parse(rdr, ver)?);
            m_light_probe_volume_override = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
            m_sorting_layer_id = Some(MeshRendererMSortingLayerIdVariant::parse(rdr, ver)?);
            m_sorting_order = Some(MeshRendererMSortingOrderVariant::parse(rdr, ver)?);
            m_additional_vertex_streams = Some(crate::PPtr::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_use_light_probes = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_reflection_probe_usage =
                Some(MeshRendererMReflectionProbeUsageVariant::parse(rdr, ver)?);
            m_probe_anchor = Some(MeshRendererMProbeAnchorVariant::parse(rdr, ver)?);
            m_sorting_layer_id = Some(MeshRendererMSortingLayerIdVariant::parse(rdr, ver)?);
            m_sorting_order = Some(MeshRendererMSortingOrderVariant::parse(rdr, ver)?);
            m_additional_vertex_streams = Some(crate::PPtr::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 5, 0) {
            m_use_light_probes = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_light_probe_anchor = Some(MeshRendererMLightProbeAnchorVariant::parse(rdr, ver)?);
            m_sorting_layer_id = Some(MeshRendererMSortingLayerIdVariant::parse(rdr, ver)?);
            m_sorting_order = Some(MeshRendererMSortingOrderVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_use_light_probes = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_light_probe_anchor = Some(MeshRendererMLightProbeAnchorVariant::parse(rdr, ver)?);
            m_sorting_layer = Some(rdr.read_i16()?);
            m_sorting_order = Some(MeshRendererMSortingOrderVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_use_light_probes = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_light_probe_anchor = Some(MeshRendererMLightProbeAnchorVariant::parse(rdr, ver)?);
        } else {
        }

        Ok(MeshRenderer {
            m_game_object: m_game_object.unwrap(),
            m_enabled: m_enabled.unwrap(),
            m_cast_shadows: m_cast_shadows.unwrap(),
            m_receive_shadows: m_receive_shadows.unwrap(),
            m_lightmap_index: m_lightmap_index.unwrap(),
            m_motion_vectors,
            m_dynamic_occludee,
            m_lightmap_tiling_offset: m_lightmap_tiling_offset.unwrap(),
            m_lightmap_index_dynamic,
            m_light_probe_usage,
            m_static_shadow_caster,
            m_materials: m_materials.unwrap(),
            m_reflection_probe_usage,
            m_subset_indices,
            m_lightmap_tiling_offset_dynamic,
            m_static_batch_root: m_static_batch_root.unwrap(),
            m_rendering_layer_mask,
            m_ray_tracing_mode,
            m_use_light_probes,
            m_renderer_priority,
            m_ray_trace_procedural,
            m_light_probe_anchor,
            m_sorting_layer,
            m_sorting_layer_id,
            m_ray_tracing_accel_struct_build_flags_override,
            m_sorting_order,
            m_static_batch_info,
            m_ray_tracing_accel_struct_build_flags,
            m_probe_anchor,
            m_small_mesh_culling,
            m_force_mesh_lod,
            m_light_probe_volume_override,
            m_mesh_lod_selection_bias,
            m_additional_vertex_streams,
            m_enlighten_vertex_stream,
            m_mask_interaction,
        })
    }
}
