//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::line_parameters::LineParameters;
use crate::subclasses::static_batch_info::StaticBatchInfo;
use crate::subclasses::vector3f::Vector3f;
use crate::subclasses::vector4f::Vector4f;


/// 版本枚举：LineRendererMEnabledVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum LineRendererMEnabledVariant {
    Default(bool),
    V5_0(bool),
    V5_4(bool),
}

impl crate::Parse for LineRendererMEnabledVariant {
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

/// 版本枚举：LineRendererMCastShadowsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum LineRendererMCastShadowsVariant {
    Default(bool),
    V5_0(u8),
}

impl crate::Parse for LineRendererMCastShadowsVariant {
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

/// 版本枚举：LineRendererMReceiveShadowsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum LineRendererMReceiveShadowsVariant {
    Default(bool),
    V5_0(bool),
    V5_4(u8),
}

impl crate::Parse for LineRendererMReceiveShadowsVariant {
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

/// 版本枚举：LineRendererMLightmapIndexVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum LineRendererMLightmapIndexVariant {
    Default(u8),
    V5_0(u16),
}

impl crate::Parse for LineRendererMLightmapIndexVariant {
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

/// 版本枚举：LineRendererMMaterialsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum LineRendererMMaterialsVariant {
    Default(Vec<PPtr>),
    V2017_1(Vec<PPtr>),
}

impl crate::Parse for LineRendererMMaterialsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：LineRendererMReflectionProbeUsageVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum LineRendererMReflectionProbeUsageVariant {
    Default(i32),
    V5_4(u8),
    V2019_3(u8),
}

impl crate::Parse for LineRendererMReflectionProbeUsageVariant {
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

/// 版本枚举：LineRendererMRayTracingModeVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum LineRendererMRayTracingModeVariant {
    Default(u8),
    V2020_1(u8),
}

impl crate::Parse for LineRendererMRayTracingModeVariant {
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

/// 版本枚举：LineRendererMPositionsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum LineRendererMPositionsVariant {
    Default(Vec<Vector3f>),
    V2017_1(Vec<Vector3f>),
}

impl crate::Parse for LineRendererMPositionsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { Vector3f::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { Vector3f::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：LineRendererMRayTraceProceduralVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum LineRendererMRayTraceProceduralVariant {
    Default(u8),
    V2023_2(u8),
}

impl crate::Parse for LineRendererMRayTraceProceduralVariant {
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

/// 版本枚举：LineRendererMLightProbeAnchorVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum LineRendererMLightProbeAnchorVariant {
    Default(PPtr),
    V4_3(PPtr),
}

impl crate::Parse for LineRendererMLightProbeAnchorVariant {
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

/// 版本枚举：LineRendererMSortingLayerIdVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum LineRendererMSortingLayerIdVariant {
    Default(u32),
    V5_0(i32),
}

impl crate::Parse for LineRendererMSortingLayerIdVariant {
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

/// 版本枚举：LineRendererMSortingOrderVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum LineRendererMSortingOrderVariant {
    Default(i16),
    V4_5(i16),
}

impl crate::Parse for LineRendererMSortingOrderVariant {
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

/// 版本枚举：LineRendererMRayTracingAccelStructBuildFlagsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum LineRendererMRayTracingAccelStructBuildFlagsVariant {
    Default(u8),
    V2023_3(u8),
}

impl crate::Parse for LineRendererMRayTracingAccelStructBuildFlagsVariant {
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

/// 版本枚举：LineRendererMProbeAnchorVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum LineRendererMProbeAnchorVariant {
    Default(PPtr),
    V5_4(PPtr),
}

impl crate::Parse for LineRendererMProbeAnchorVariant {
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

/// 版本枚举：LineRendererMMaskInteractionVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum LineRendererMMaskInteractionVariant {
    Default(i32),
    V6000_3(i32),
}

impl crate::Parse for LineRendererMMaskInteractionVariant {
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
pub struct LineRenderer {
    pub m_game_object: PPtr,
    pub m_enabled: LineRendererMEnabledVariant,
    pub m_cast_shadows: LineRendererMCastShadowsVariant,
    pub m_receive_shadows: LineRendererMReceiveShadowsVariant,
    pub m_lightmap_index: LineRendererMLightmapIndexVariant,
    pub m_motion_vectors: Option<u8>,
    pub m_dynamic_occludee: Option<u8>,
    pub m_lightmap_tiling_offset: Vector4f,
    pub m_lightmap_index_dynamic: Option<u16>,
    pub m_light_probe_usage: Option<u8>,
    pub m_static_shadow_caster: Option<u8>,
    pub m_materials: LineRendererMMaterialsVariant,
    pub m_reflection_probe_usage: Option<LineRendererMReflectionProbeUsageVariant>,
    pub m_subset_indices: Option<Vec<u32>>,
    pub m_lightmap_tiling_offset_dynamic: Option<Vector4f>,
    pub m_static_batch_root: PPtr,
    pub m_rendering_layer_mask: Option<u32>,
    pub m_ray_tracing_mode: Option<LineRendererMRayTracingModeVariant>,
    pub m_positions: LineRendererMPositionsVariant,
    pub m_use_light_probes: Option<bool>,
    pub m_renderer_priority: Option<i32>,
    pub m_ray_trace_procedural: Option<LineRendererMRayTraceProceduralVariant>,
    pub m_parameters: LineParameters,
    pub m_light_probe_anchor: Option<LineRendererMLightProbeAnchorVariant>,
    pub m_use_world_space: bool,
    pub m_sorting_layer: Option<i16>,
    pub m_sorting_layer_id: Option<LineRendererMSortingLayerIdVariant>,
    pub m_ray_tracing_accel_struct_build_flags_override: Option<u8>,
    pub m_sorting_order: Option<LineRendererMSortingOrderVariant>,
    pub m_static_batch_info: Option<StaticBatchInfo>,
    pub m_ray_tracing_accel_struct_build_flags: Option<LineRendererMRayTracingAccelStructBuildFlagsVariant>,
    pub m_probe_anchor: Option<LineRendererMProbeAnchorVariant>,
    pub m_small_mesh_culling: Option<u8>,
    pub m_force_mesh_lod: Option<i16>,
    pub m_light_probe_volume_override: Option<PPtr>,
    pub m_mesh_lod_selection_bias: Option<f32>,
    pub m_loop: Option<bool>,
    pub m_mask_interaction: Option<LineRendererMMaskInteractionVariant>,
    pub m_apply_active_color_space: Option<bool>,
}

impl crate::Parse for LineRenderer {
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
        let mut m_positions = None;
        let mut m_use_light_probes = None;
        let mut m_renderer_priority = None;
        let mut m_ray_trace_procedural = None;
        let mut m_parameters = None;
        let mut m_light_probe_anchor = None;
        let mut m_use_world_space = None;
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
        let mut m_loop = None;
        let mut m_mask_interaction = None;
        let mut m_apply_active_color_space = None;
            m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
            m_enabled = Some(LineRendererMEnabledVariant::parse(rdr, ver)?);
            m_cast_shadows = Some(LineRendererMCastShadowsVariant::parse(rdr, ver)?);
            m_receive_shadows = Some(LineRendererMReceiveShadowsVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(6000, 2, 0) {
            m_dynamic_occludee = Some(rdr.read_u8()?);
            m_static_shadow_caster = Some(rdr.read_u8()?);
            m_motion_vectors = Some(rdr.read_u8()?);
            m_light_probe_usage = Some(rdr.read_u8()?);
            m_reflection_probe_usage = Some(LineRendererMReflectionProbeUsageVariant::parse(rdr, ver)?);
            m_ray_tracing_mode = Some(LineRendererMRayTracingModeVariant::parse(rdr, ver)?);
            m_ray_trace_procedural = Some(LineRendererMRayTraceProceduralVariant::parse(rdr, ver)?);
            m_ray_tracing_accel_struct_build_flags_override = Some(rdr.read_u8()?);
            m_ray_tracing_accel_struct_build_flags = Some(LineRendererMRayTracingAccelStructBuildFlagsVariant::parse(rdr, ver)?);
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
            m_reflection_probe_usage = Some(LineRendererMReflectionProbeUsageVariant::parse(rdr, ver)?);
            m_ray_tracing_mode = Some(LineRendererMRayTracingModeVariant::parse(rdr, ver)?);
            m_ray_trace_procedural = Some(LineRendererMRayTraceProceduralVariant::parse(rdr, ver)?);
            m_ray_tracing_accel_struct_build_flags_override = Some(rdr.read_u8()?);
            m_ray_tracing_accel_struct_build_flags = Some(LineRendererMRayTracingAccelStructBuildFlagsVariant::parse(rdr, ver)?);
            m_small_mesh_culling = Some(rdr.read_u8()?);
            rdr.align(4)?;
            m_rendering_layer_mask = Some(rdr.read_u32()?);
            m_renderer_priority = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            m_dynamic_occludee = Some(rdr.read_u8()?);
            m_static_shadow_caster = Some(rdr.read_u8()?);
            m_motion_vectors = Some(rdr.read_u8()?);
            m_light_probe_usage = Some(rdr.read_u8()?);
            m_reflection_probe_usage = Some(LineRendererMReflectionProbeUsageVariant::parse(rdr, ver)?);
            m_ray_tracing_mode = Some(LineRendererMRayTracingModeVariant::parse(rdr, ver)?);
            m_ray_trace_procedural = Some(LineRendererMRayTraceProceduralVariant::parse(rdr, ver)?);
            m_ray_tracing_accel_struct_build_flags_override = Some(rdr.read_u8()?);
            m_ray_tracing_accel_struct_build_flags = Some(LineRendererMRayTracingAccelStructBuildFlagsVariant::parse(rdr, ver)?);
            m_rendering_layer_mask = Some(rdr.read_u32()?);
            m_renderer_priority = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            m_dynamic_occludee = Some(rdr.read_u8()?);
            m_static_shadow_caster = Some(rdr.read_u8()?);
            m_motion_vectors = Some(rdr.read_u8()?);
            m_light_probe_usage = Some(rdr.read_u8()?);
            m_reflection_probe_usage = Some(LineRendererMReflectionProbeUsageVariant::parse(rdr, ver)?);
            m_ray_tracing_mode = Some(LineRendererMRayTracingModeVariant::parse(rdr, ver)?);
            m_ray_trace_procedural = Some(LineRendererMRayTraceProceduralVariant::parse(rdr, ver)?);
            m_rendering_layer_mask = Some(rdr.read_u32()?);
            m_renderer_priority = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            m_dynamic_occludee = Some(rdr.read_u8()?);
            m_motion_vectors = Some(rdr.read_u8()?);
            m_light_probe_usage = Some(rdr.read_u8()?);
            m_reflection_probe_usage = Some(LineRendererMReflectionProbeUsageVariant::parse(rdr, ver)?);
            m_ray_tracing_mode = Some(LineRendererMRayTracingModeVariant::parse(rdr, ver)?);
            m_ray_trace_procedural = Some(LineRendererMRayTraceProceduralVariant::parse(rdr, ver)?);
            m_rendering_layer_mask = Some(rdr.read_u32()?);
            m_renderer_priority = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            m_dynamic_occludee = Some(rdr.read_u8()?);
            m_motion_vectors = Some(rdr.read_u8()?);
            m_light_probe_usage = Some(rdr.read_u8()?);
            m_reflection_probe_usage = Some(LineRendererMReflectionProbeUsageVariant::parse(rdr, ver)?);
            m_ray_tracing_mode = Some(LineRendererMRayTracingModeVariant::parse(rdr, ver)?);
            m_rendering_layer_mask = Some(rdr.read_u32()?);
            m_renderer_priority = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            m_dynamic_occludee = Some(rdr.read_u8()?);
            m_motion_vectors = Some(rdr.read_u8()?);
            m_light_probe_usage = Some(rdr.read_u8()?);
            m_reflection_probe_usage = Some(LineRendererMReflectionProbeUsageVariant::parse(rdr, ver)?);
            m_rendering_layer_mask = Some(rdr.read_u32()?);
            m_renderer_priority = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            m_dynamic_occludee = Some(rdr.read_u8()?);
            m_motion_vectors = Some(rdr.read_u8()?);
            m_light_probe_usage = Some(rdr.read_u8()?);
            m_reflection_probe_usage = Some(LineRendererMReflectionProbeUsageVariant::parse(rdr, ver)?);
            m_rendering_layer_mask = Some(rdr.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_dynamic_occludee = Some(rdr.read_u8()?);
            m_motion_vectors = Some(rdr.read_u8()?);
            m_light_probe_usage = Some(rdr.read_u8()?);
            m_reflection_probe_usage = Some(LineRendererMReflectionProbeUsageVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            m_motion_vectors = Some(rdr.read_u8()?);
            m_light_probe_usage = Some(rdr.read_u8()?);
            m_reflection_probe_usage = Some(LineRendererMReflectionProbeUsageVariant::parse(rdr, ver)?);
        } else {
        }
            m_lightmap_index = Some(LineRendererMLightmapIndexVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_lightmap_index_dynamic = Some(rdr.read_u16()?);
        } else {
        }
            m_lightmap_tiling_offset = Some(Vector4f::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_lightmap_tiling_offset_dynamic = Some(Vector4f::parse(rdr, ver)?);
        } else {
        }
            m_materials = Some(LineRendererMMaterialsVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            m_static_batch_info = Some(StaticBatchInfo::parse(rdr, ver)?);
        } else {
            m_subset_indices = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u32() }).collect::<Result<Vec<_>, _>>() } )?);
        }
            m_static_batch_root = Some(crate::PPtr::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(6000, 3, 0) {
            m_probe_anchor = Some(LineRendererMProbeAnchorVariant::parse(rdr, ver)?);
            m_light_probe_volume_override = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
            m_sorting_layer_id = Some(LineRendererMSortingLayerIdVariant::parse(rdr, ver)?);
            m_sorting_layer = Some(rdr.read_i16()?);
            m_sorting_order = Some(LineRendererMSortingOrderVariant::parse(rdr, ver)?);
            m_mask_interaction = Some(LineRendererMMaskInteractionVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_probe_anchor = Some(LineRendererMProbeAnchorVariant::parse(rdr, ver)?);
            m_light_probe_volume_override = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
            m_sorting_layer_id = Some(LineRendererMSortingLayerIdVariant::parse(rdr, ver)?);
            m_sorting_layer = Some(rdr.read_i16()?);
            m_sorting_order = Some(LineRendererMSortingOrderVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            m_probe_anchor = Some(LineRendererMProbeAnchorVariant::parse(rdr, ver)?);
            m_light_probe_volume_override = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
            m_sorting_layer_id = Some(LineRendererMSortingLayerIdVariant::parse(rdr, ver)?);
            m_sorting_order = Some(LineRendererMSortingOrderVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_use_light_probes = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_reflection_probe_usage = Some(LineRendererMReflectionProbeUsageVariant::parse(rdr, ver)?);
            m_probe_anchor = Some(LineRendererMProbeAnchorVariant::parse(rdr, ver)?);
            m_sorting_layer_id = Some(LineRendererMSortingLayerIdVariant::parse(rdr, ver)?);
            m_sorting_order = Some(LineRendererMSortingOrderVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 5, 0) {
            m_use_light_probes = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_light_probe_anchor = Some(LineRendererMLightProbeAnchorVariant::parse(rdr, ver)?);
            m_sorting_layer_id = Some(LineRendererMSortingLayerIdVariant::parse(rdr, ver)?);
            m_sorting_order = Some(LineRendererMSortingOrderVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_use_light_probes = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_light_probe_anchor = Some(LineRendererMLightProbeAnchorVariant::parse(rdr, ver)?);
            m_sorting_layer = Some(rdr.read_i16()?);
            m_sorting_order = Some(LineRendererMSortingOrderVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_use_light_probes = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_light_probe_anchor = Some(LineRendererMLightProbeAnchorVariant::parse(rdr, ver)?);
        } else {
        }
            m_positions = Some(LineRendererMPositionsVariant::parse(rdr, ver)?);
            m_parameters = Some(LineParameters::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(6000, 3, 0) {
        } else if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            m_mask_interaction = Some(LineRendererMMaskInteractionVariant::parse(rdr, ver)?);
        } else {
        }
            m_use_world_space = Some(rdr.read_bool()?);
        if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            m_loop = Some(rdr.read_bool()?);
            m_apply_active_color_space = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_loop = Some(rdr.read_bool()?);
        } else {
        }

        Ok(LineRenderer {
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
            m_positions: m_positions.unwrap(),
            m_use_light_probes,
            m_renderer_priority,
            m_ray_trace_procedural,
            m_parameters: m_parameters.unwrap(),
            m_light_probe_anchor,
            m_use_world_space: m_use_world_space.unwrap(),
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
            m_loop,
            m_mask_interaction,
            m_apply_active_color_space,
        })
    }
}
