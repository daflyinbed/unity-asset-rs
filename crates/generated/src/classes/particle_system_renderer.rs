//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::static_batch_info::StaticBatchInfo;
use crate::subclasses::vector3f::Vector3f;
use crate::subclasses::vector4f::Vector4f;


/// 版本枚举：ParticleSystemRendererMEnabledVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ParticleSystemRendererMEnabledVariant {
    Default(bool),
    V5_0(bool),
    V5_4(bool),
}

impl crate::Parse for ParticleSystemRendererMEnabledVariant {
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

/// 版本枚举：ParticleSystemRendererMCastShadowsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ParticleSystemRendererMCastShadowsVariant {
    Default(bool),
    V5_0(u8),
}

impl crate::Parse for ParticleSystemRendererMCastShadowsVariant {
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

/// 版本枚举：ParticleSystemRendererMReceiveShadowsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ParticleSystemRendererMReceiveShadowsVariant {
    Default(bool),
    V5_0(bool),
    V5_4(u8),
}

impl crate::Parse for ParticleSystemRendererMReceiveShadowsVariant {
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

/// 版本枚举：ParticleSystemRendererMLightmapIndexVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ParticleSystemRendererMLightmapIndexVariant {
    Default(u8),
    V5_0(u16),
}

impl crate::Parse for ParticleSystemRendererMLightmapIndexVariant {
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

/// 版本枚举：ParticleSystemRendererMMaterialsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ParticleSystemRendererMMaterialsVariant {
    Default(Vec<PPtr>),
    V2017_1(Vec<PPtr>),
}

impl crate::Parse for ParticleSystemRendererMMaterialsVariant {
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

/// 版本枚举：ParticleSystemRendererMReflectionProbeUsageVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ParticleSystemRendererMReflectionProbeUsageVariant {
    Default(i32),
    V5_4(u8),
    V2019_3(u8),
}

impl crate::Parse for ParticleSystemRendererMReflectionProbeUsageVariant {
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

/// 版本枚举：ParticleSystemRendererMRayTracingModeVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ParticleSystemRendererMRayTracingModeVariant {
    Default(u8),
    V2020_1(u8),
}

impl crate::Parse for ParticleSystemRendererMRayTracingModeVariant {
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

/// 版本枚举：ParticleSystemRendererMRayTraceProceduralVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ParticleSystemRendererMRayTraceProceduralVariant {
    Default(u8),
    V2023_2(u8),
}

impl crate::Parse for ParticleSystemRendererMRayTraceProceduralVariant {
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

/// 版本枚举：ParticleSystemRendererMLightProbeAnchorVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ParticleSystemRendererMLightProbeAnchorVariant {
    Default(PPtr),
    V4_3(PPtr),
}

impl crate::Parse for ParticleSystemRendererMLightProbeAnchorVariant {
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

/// 版本枚举：ParticleSystemRendererMRenderModeVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ParticleSystemRendererMRenderModeVariant {
    Default(i32),
    V5_3(u16),
}

impl crate::Parse for ParticleSystemRendererMRenderModeVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 3, 0) {
            let v = rdr.read_u16()?;
            Ok(Self::V5_3(v))
        } else {
            let v = rdr.read_i32()?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：ParticleSystemRendererMSortingLayerIdVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ParticleSystemRendererMSortingLayerIdVariant {
    Default(u32),
    V5_0(i32),
}

impl crate::Parse for ParticleSystemRendererMSortingLayerIdVariant {
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

/// 版本枚举：ParticleSystemRendererMSortingOrderVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ParticleSystemRendererMSortingOrderVariant {
    Default(i16),
    V4_5(i16),
}

impl crate::Parse for ParticleSystemRendererMSortingOrderVariant {
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

/// 版本枚举：ParticleSystemRendererMRayTracingAccelStructBuildFlagsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ParticleSystemRendererMRayTracingAccelStructBuildFlagsVariant {
    Default(u8),
    V2023_3(u8),
}

impl crate::Parse for ParticleSystemRendererMRayTracingAccelStructBuildFlagsVariant {
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

/// 版本枚举：ParticleSystemRendererMProbeAnchorVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ParticleSystemRendererMProbeAnchorVariant {
    Default(PPtr),
    V5_4(PPtr),
}

impl crate::Parse for ParticleSystemRendererMProbeAnchorVariant {
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

/// 版本枚举：ParticleSystemRendererMSortModeVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ParticleSystemRendererMSortModeVariant {
    Default(i32),
    V5_3(u16),
    V2021_2(u8),
}

impl crate::Parse for ParticleSystemRendererMSortModeVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            let v = rdr.read_u8()?;
            Ok(Self::V2021_2(v))
        } else if *ver >= crate::UnityVersion::new(5, 3, 0) {
            let v = rdr.read_u16()?;
            Ok(Self::V5_3(v))
        } else {
            let v = rdr.read_i32()?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：ParticleSystemRendererMUseCustomVertexStreamsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ParticleSystemRendererMUseCustomVertexStreamsVariant {
    Default(bool),
    V2018_1(bool),
    V2022_3_11(bool),
    V2023_1(bool),
    V2023_1_16(bool),
}

impl crate::Parse for ParticleSystemRendererMUseCustomVertexStreamsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2023, 1, 16) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V2023_1_16(v))
        } else if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2023_1(v))
        } else if *ver >= crate::UnityVersion::new(2022, 3, 11) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V2022_3_11(v))
        } else if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2018_1(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：ParticleSystemRendererMMaskInteractionVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ParticleSystemRendererMMaskInteractionVariant {
    Default(i32),
    V6000_3(i32),
}

impl crate::Parse for ParticleSystemRendererMMaskInteractionVariant {
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

/// 版本枚举：ParticleSystemRendererMEnableGpuInstancingVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ParticleSystemRendererMEnableGpuInstancingVariant {
    Default(bool),
    V2018_2(bool),
}

impl crate::Parse for ParticleSystemRendererMEnableGpuInstancingVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2018_2(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：ParticleSystemRendererMApplyActiveColorSpaceVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ParticleSystemRendererMApplyActiveColorSpaceVariant {
    Default(bool),
    V2018_3(bool),
}

impl crate::Parse for ParticleSystemRendererMApplyActiveColorSpaceVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2018_3(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：ParticleSystemRendererMAllowRollVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ParticleSystemRendererMAllowRollVariant {
    Default(bool),
    V2020_1(bool),
}

impl crate::Parse for ParticleSystemRendererMAllowRollVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2020_1(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct ParticleSystemRenderer {
    pub m_game_object: PPtr,
    pub m_enabled: ParticleSystemRendererMEnabledVariant,
    pub m_cast_shadows: ParticleSystemRendererMCastShadowsVariant,
    pub m_receive_shadows: ParticleSystemRendererMReceiveShadowsVariant,
    pub m_lightmap_index: ParticleSystemRendererMLightmapIndexVariant,
    pub m_motion_vectors: Option<u8>,
    pub m_dynamic_occludee: Option<u8>,
    pub m_lightmap_tiling_offset: Vector4f,
    pub m_lightmap_index_dynamic: Option<u16>,
    pub m_light_probe_usage: Option<u8>,
    pub m_static_shadow_caster: Option<u8>,
    pub m_materials: ParticleSystemRendererMMaterialsVariant,
    pub m_reflection_probe_usage: Option<ParticleSystemRendererMReflectionProbeUsageVariant>,
    pub m_subset_indices: Option<Vec<u32>>,
    pub m_lightmap_tiling_offset_dynamic: Option<Vector4f>,
    pub m_static_batch_root: PPtr,
    pub m_rendering_layer_mask: Option<u32>,
    pub m_ray_tracing_mode: Option<ParticleSystemRendererMRayTracingModeVariant>,
    pub m_use_light_probes: Option<bool>,
    pub m_renderer_priority: Option<i32>,
    pub m_ray_trace_procedural: Option<ParticleSystemRendererMRayTraceProceduralVariant>,
    pub m_light_probe_anchor: Option<ParticleSystemRendererMLightProbeAnchorVariant>,
    pub m_render_mode: ParticleSystemRendererMRenderModeVariant,
    pub m_sorting_layer: Option<i16>,
    pub m_sorting_layer_id: Option<ParticleSystemRendererMSortingLayerIdVariant>,
    pub m_ray_tracing_accel_struct_build_flags_override: Option<u8>,
    pub m_max_particle_size: f32,
    pub m_sorting_order: Option<ParticleSystemRendererMSortingOrderVariant>,
    pub m_static_batch_info: Option<StaticBatchInfo>,
    pub m_ray_tracing_accel_struct_build_flags: Option<ParticleSystemRendererMRayTracingAccelStructBuildFlagsVariant>,
    pub m_camera_velocity_scale: f32,
    pub m_probe_anchor: Option<ParticleSystemRendererMProbeAnchorVariant>,
    pub m_small_mesh_culling: Option<u8>,
    pub m_velocity_scale: f32,
    pub m_force_mesh_lod: Option<i16>,
    pub m_length_scale: f32,
    pub m_light_probe_volume_override: Option<PPtr>,
    pub m_mesh_lod_selection_bias: Option<f32>,
    pub m_sorting_fudge: f32,
    pub m_sort_mode: ParticleSystemRendererMSortModeVariant,
    pub m_normal_direction: Option<f32>,
    pub m_mesh: PPtr,
    pub m_min_particle_size: Option<f32>,
    pub m_mesh1: Option<PPtr>,
    pub m_mesh2: Option<PPtr>,
    pub m_mesh3: Option<PPtr>,
    pub m_render_alignment: Option<i32>,
    pub m_pivot: Option<Vector3f>,
    pub m_mesh_distribution: Option<u8>,
    pub m_use_custom_vertex_streams: Option<ParticleSystemRendererMUseCustomVertexStreamsVariant>,
    pub m_vertex_stream_mask: Option<i32>,
    pub m_mask_interaction: Option<ParticleSystemRendererMMaskInteractionVariant>,
    pub m_vertex_streams: Option<Vec<u8>>,
    pub m_shadow_bias: Option<f32>,
    pub m_enable_gpu_instancing: Option<ParticleSystemRendererMEnableGpuInstancingVariant>,
    pub m_apply_active_color_space: Option<ParticleSystemRendererMApplyActiveColorSpaceVariant>,
    pub m_flip: Option<Vector3f>,
    pub m_allow_roll: Option<ParticleSystemRendererMAllowRollVariant>,
    pub m_freeform_stretching: Option<bool>,
    pub m_rotate_with_stretch_direction: Option<bool>,
    pub m_use_custom_trail_vertex_streams: Option<bool>,
    pub m_trail_vertex_streams: Option<Vec<u8>>,
    pub m_mesh_weighting: Option<f32>,
    pub m_mesh_weighting1: Option<f32>,
    pub m_mesh_weighting2: Option<f32>,
    pub m_mesh_weighting3: Option<f32>,
}

impl crate::Parse for ParticleSystemRenderer {
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
        let mut m_render_mode = None;
        let mut m_sorting_layer = None;
        let mut m_sorting_layer_id = None;
        let mut m_ray_tracing_accel_struct_build_flags_override = None;
        let mut m_max_particle_size = None;
        let mut m_sorting_order = None;
        let mut m_static_batch_info = None;
        let mut m_ray_tracing_accel_struct_build_flags = None;
        let mut m_camera_velocity_scale = None;
        let mut m_probe_anchor = None;
        let mut m_small_mesh_culling = None;
        let mut m_velocity_scale = None;
        let mut m_force_mesh_lod = None;
        let mut m_length_scale = None;
        let mut m_light_probe_volume_override = None;
        let mut m_mesh_lod_selection_bias = None;
        let mut m_sorting_fudge = None;
        let mut m_sort_mode = None;
        let mut m_normal_direction = None;
        let mut m_mesh = None;
        let mut m_min_particle_size = None;
        let mut m_mesh1 = None;
        let mut m_mesh2 = None;
        let mut m_mesh3 = None;
        let mut m_render_alignment = None;
        let mut m_pivot = None;
        let mut m_mesh_distribution = None;
        let mut m_use_custom_vertex_streams = None;
        let mut m_vertex_stream_mask = None;
        let mut m_mask_interaction = None;
        let mut m_vertex_streams = None;
        let mut m_shadow_bias = None;
        let mut m_enable_gpu_instancing = None;
        let mut m_apply_active_color_space = None;
        let mut m_flip = None;
        let mut m_allow_roll = None;
        let mut m_freeform_stretching = None;
        let mut m_rotate_with_stretch_direction = None;
        let mut m_use_custom_trail_vertex_streams = None;
        let mut m_trail_vertex_streams = None;
        let mut m_mesh_weighting = None;
        let mut m_mesh_weighting1 = None;
        let mut m_mesh_weighting2 = None;
        let mut m_mesh_weighting3 = None;
            m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
            m_enabled = Some(ParticleSystemRendererMEnabledVariant::parse(rdr, ver)?);
            m_cast_shadows = Some(ParticleSystemRendererMCastShadowsVariant::parse(rdr, ver)?);
            m_receive_shadows = Some(ParticleSystemRendererMReceiveShadowsVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(6000, 2, 0) {
            m_dynamic_occludee = Some(rdr.read_u8()?);
            m_static_shadow_caster = Some(rdr.read_u8()?);
            m_motion_vectors = Some(rdr.read_u8()?);
            m_light_probe_usage = Some(rdr.read_u8()?);
            m_reflection_probe_usage = Some(ParticleSystemRendererMReflectionProbeUsageVariant::parse(rdr, ver)?);
            m_ray_tracing_mode = Some(ParticleSystemRendererMRayTracingModeVariant::parse(rdr, ver)?);
            m_ray_trace_procedural = Some(ParticleSystemRendererMRayTraceProceduralVariant::parse(rdr, ver)?);
            m_ray_tracing_accel_struct_build_flags_override = Some(rdr.read_u8()?);
            m_ray_tracing_accel_struct_build_flags = Some(ParticleSystemRendererMRayTracingAccelStructBuildFlagsVariant::parse(rdr, ver)?);
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
            m_reflection_probe_usage = Some(ParticleSystemRendererMReflectionProbeUsageVariant::parse(rdr, ver)?);
            m_ray_tracing_mode = Some(ParticleSystemRendererMRayTracingModeVariant::parse(rdr, ver)?);
            m_ray_trace_procedural = Some(ParticleSystemRendererMRayTraceProceduralVariant::parse(rdr, ver)?);
            m_ray_tracing_accel_struct_build_flags_override = Some(rdr.read_u8()?);
            m_ray_tracing_accel_struct_build_flags = Some(ParticleSystemRendererMRayTracingAccelStructBuildFlagsVariant::parse(rdr, ver)?);
            m_small_mesh_culling = Some(rdr.read_u8()?);
            rdr.align(4)?;
            m_rendering_layer_mask = Some(rdr.read_u32()?);
            m_renderer_priority = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            m_dynamic_occludee = Some(rdr.read_u8()?);
            m_static_shadow_caster = Some(rdr.read_u8()?);
            m_motion_vectors = Some(rdr.read_u8()?);
            m_light_probe_usage = Some(rdr.read_u8()?);
            m_reflection_probe_usage = Some(ParticleSystemRendererMReflectionProbeUsageVariant::parse(rdr, ver)?);
            m_ray_tracing_mode = Some(ParticleSystemRendererMRayTracingModeVariant::parse(rdr, ver)?);
            m_ray_trace_procedural = Some(ParticleSystemRendererMRayTraceProceduralVariant::parse(rdr, ver)?);
            m_ray_tracing_accel_struct_build_flags_override = Some(rdr.read_u8()?);
            m_ray_tracing_accel_struct_build_flags = Some(ParticleSystemRendererMRayTracingAccelStructBuildFlagsVariant::parse(rdr, ver)?);
            m_rendering_layer_mask = Some(rdr.read_u32()?);
            m_renderer_priority = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            m_dynamic_occludee = Some(rdr.read_u8()?);
            m_static_shadow_caster = Some(rdr.read_u8()?);
            m_motion_vectors = Some(rdr.read_u8()?);
            m_light_probe_usage = Some(rdr.read_u8()?);
            m_reflection_probe_usage = Some(ParticleSystemRendererMReflectionProbeUsageVariant::parse(rdr, ver)?);
            m_ray_tracing_mode = Some(ParticleSystemRendererMRayTracingModeVariant::parse(rdr, ver)?);
            m_ray_trace_procedural = Some(ParticleSystemRendererMRayTraceProceduralVariant::parse(rdr, ver)?);
            m_rendering_layer_mask = Some(rdr.read_u32()?);
            m_renderer_priority = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            m_dynamic_occludee = Some(rdr.read_u8()?);
            m_motion_vectors = Some(rdr.read_u8()?);
            m_light_probe_usage = Some(rdr.read_u8()?);
            m_reflection_probe_usage = Some(ParticleSystemRendererMReflectionProbeUsageVariant::parse(rdr, ver)?);
            m_ray_tracing_mode = Some(ParticleSystemRendererMRayTracingModeVariant::parse(rdr, ver)?);
            m_ray_trace_procedural = Some(ParticleSystemRendererMRayTraceProceduralVariant::parse(rdr, ver)?);
            m_rendering_layer_mask = Some(rdr.read_u32()?);
            m_renderer_priority = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            m_dynamic_occludee = Some(rdr.read_u8()?);
            m_motion_vectors = Some(rdr.read_u8()?);
            m_light_probe_usage = Some(rdr.read_u8()?);
            m_reflection_probe_usage = Some(ParticleSystemRendererMReflectionProbeUsageVariant::parse(rdr, ver)?);
            m_ray_tracing_mode = Some(ParticleSystemRendererMRayTracingModeVariant::parse(rdr, ver)?);
            m_rendering_layer_mask = Some(rdr.read_u32()?);
            m_renderer_priority = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            m_dynamic_occludee = Some(rdr.read_u8()?);
            m_motion_vectors = Some(rdr.read_u8()?);
            m_light_probe_usage = Some(rdr.read_u8()?);
            m_reflection_probe_usage = Some(ParticleSystemRendererMReflectionProbeUsageVariant::parse(rdr, ver)?);
            m_rendering_layer_mask = Some(rdr.read_u32()?);
            m_renderer_priority = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            m_dynamic_occludee = Some(rdr.read_u8()?);
            m_motion_vectors = Some(rdr.read_u8()?);
            m_light_probe_usage = Some(rdr.read_u8()?);
            m_reflection_probe_usage = Some(ParticleSystemRendererMReflectionProbeUsageVariant::parse(rdr, ver)?);
            m_rendering_layer_mask = Some(rdr.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_dynamic_occludee = Some(rdr.read_u8()?);
            m_motion_vectors = Some(rdr.read_u8()?);
            m_light_probe_usage = Some(rdr.read_u8()?);
            m_reflection_probe_usage = Some(ParticleSystemRendererMReflectionProbeUsageVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            m_motion_vectors = Some(rdr.read_u8()?);
            m_light_probe_usage = Some(rdr.read_u8()?);
            m_reflection_probe_usage = Some(ParticleSystemRendererMReflectionProbeUsageVariant::parse(rdr, ver)?);
        } else {
        }
            m_lightmap_index = Some(ParticleSystemRendererMLightmapIndexVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_lightmap_index_dynamic = Some(rdr.read_u16()?);
        } else {
        }
            m_lightmap_tiling_offset = Some(Vector4f::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_lightmap_tiling_offset_dynamic = Some(Vector4f::parse(rdr, ver)?);
        } else {
        }
            m_materials = Some(ParticleSystemRendererMMaterialsVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            m_static_batch_info = Some(StaticBatchInfo::parse(rdr, ver)?);
        } else {
            m_subset_indices = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u32() }).collect::<Result<Vec<_>, _>>() } )?);
        }
            m_static_batch_root = Some(crate::PPtr::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(6000, 3, 0) {
            m_probe_anchor = Some(ParticleSystemRendererMProbeAnchorVariant::parse(rdr, ver)?);
            m_light_probe_volume_override = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
            m_sorting_layer_id = Some(ParticleSystemRendererMSortingLayerIdVariant::parse(rdr, ver)?);
            m_sorting_layer = Some(rdr.read_i16()?);
            m_sorting_order = Some(ParticleSystemRendererMSortingOrderVariant::parse(rdr, ver)?);
            m_mask_interaction = Some(ParticleSystemRendererMMaskInteractionVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_probe_anchor = Some(ParticleSystemRendererMProbeAnchorVariant::parse(rdr, ver)?);
            m_light_probe_volume_override = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
            m_sorting_layer_id = Some(ParticleSystemRendererMSortingLayerIdVariant::parse(rdr, ver)?);
            m_sorting_layer = Some(rdr.read_i16()?);
            m_sorting_order = Some(ParticleSystemRendererMSortingOrderVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            m_probe_anchor = Some(ParticleSystemRendererMProbeAnchorVariant::parse(rdr, ver)?);
            m_light_probe_volume_override = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
            m_sorting_layer_id = Some(ParticleSystemRendererMSortingLayerIdVariant::parse(rdr, ver)?);
            m_sorting_order = Some(ParticleSystemRendererMSortingOrderVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_use_light_probes = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_reflection_probe_usage = Some(ParticleSystemRendererMReflectionProbeUsageVariant::parse(rdr, ver)?);
            m_probe_anchor = Some(ParticleSystemRendererMProbeAnchorVariant::parse(rdr, ver)?);
            m_sorting_layer_id = Some(ParticleSystemRendererMSortingLayerIdVariant::parse(rdr, ver)?);
            m_sorting_order = Some(ParticleSystemRendererMSortingOrderVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 5, 0) {
            m_use_light_probes = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_light_probe_anchor = Some(ParticleSystemRendererMLightProbeAnchorVariant::parse(rdr, ver)?);
            m_sorting_layer_id = Some(ParticleSystemRendererMSortingLayerIdVariant::parse(rdr, ver)?);
            m_sorting_order = Some(ParticleSystemRendererMSortingOrderVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_use_light_probes = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_light_probe_anchor = Some(ParticleSystemRendererMLightProbeAnchorVariant::parse(rdr, ver)?);
            m_sorting_layer = Some(rdr.read_i16()?);
            m_sorting_order = Some(ParticleSystemRendererMSortingOrderVariant::parse(rdr, ver)?);
        } else {
            m_use_light_probes = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_light_probe_anchor = Some(ParticleSystemRendererMLightProbeAnchorVariant::parse(rdr, ver)?);
        }
            m_render_mode = Some(ParticleSystemRendererMRenderModeVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            m_mesh_distribution = Some(rdr.read_u8()?);
            m_sort_mode = Some(ParticleSystemRendererMSortModeVariant::parse(rdr, ver)?);
            m_min_particle_size = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(5, 3, 0) {
            m_sort_mode = Some(ParticleSystemRendererMSortModeVariant::parse(rdr, ver)?);
            m_min_particle_size = Some(rdr.read_f32()?);
        } else {
        }
            m_max_particle_size = Some(rdr.read_f32()?);
            m_camera_velocity_scale = Some(rdr.read_f32()?);
            m_velocity_scale = Some(rdr.read_f32()?);
            m_length_scale = Some(rdr.read_f32()?);
            m_sorting_fudge = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(2023, 1, 16) {
            m_normal_direction = Some(rdr.read_f32()?);
            m_shadow_bias = Some(rdr.read_f32()?);
            m_render_alignment = Some(rdr.read_i32()?);
            m_pivot = Some(Vector3f::parse(rdr, ver)?);
            m_flip = Some(Vector3f::parse(rdr, ver)?);
            m_enable_gpu_instancing = Some(ParticleSystemRendererMEnableGpuInstancingVariant::parse(rdr, ver)?);
            m_apply_active_color_space = Some(ParticleSystemRendererMApplyActiveColorSpaceVariant::parse(rdr, ver)?);
            m_allow_roll = Some(ParticleSystemRendererMAllowRollVariant::parse(rdr, ver)?);
            m_freeform_stretching = Some(rdr.read_bool()?);
            m_rotate_with_stretch_direction = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_use_custom_vertex_streams = Some(ParticleSystemRendererMUseCustomVertexStreamsVariant::parse(rdr, ver)?);
            m_vertex_streams = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u8() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_use_custom_trail_vertex_streams = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_trail_vertex_streams = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u8() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            m_normal_direction = Some(rdr.read_f32()?);
            m_shadow_bias = Some(rdr.read_f32()?);
            m_render_alignment = Some(rdr.read_i32()?);
            m_pivot = Some(Vector3f::parse(rdr, ver)?);
            m_flip = Some(Vector3f::parse(rdr, ver)?);
            m_use_custom_vertex_streams = Some(ParticleSystemRendererMUseCustomVertexStreamsVariant::parse(rdr, ver)?);
            m_enable_gpu_instancing = Some(ParticleSystemRendererMEnableGpuInstancingVariant::parse(rdr, ver)?);
            m_apply_active_color_space = Some(ParticleSystemRendererMApplyActiveColorSpaceVariant::parse(rdr, ver)?);
            m_allow_roll = Some(ParticleSystemRendererMAllowRollVariant::parse(rdr, ver)?);
            m_freeform_stretching = Some(rdr.read_bool()?);
            m_rotate_with_stretch_direction = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_vertex_streams = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u8() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2022, 3, 11) {
            m_normal_direction = Some(rdr.read_f32()?);
            m_shadow_bias = Some(rdr.read_f32()?);
            m_render_alignment = Some(rdr.read_i32()?);
            m_pivot = Some(Vector3f::parse(rdr, ver)?);
            m_flip = Some(Vector3f::parse(rdr, ver)?);
            m_enable_gpu_instancing = Some(ParticleSystemRendererMEnableGpuInstancingVariant::parse(rdr, ver)?);
            m_apply_active_color_space = Some(ParticleSystemRendererMApplyActiveColorSpaceVariant::parse(rdr, ver)?);
            m_allow_roll = Some(ParticleSystemRendererMAllowRollVariant::parse(rdr, ver)?);
            m_freeform_stretching = Some(rdr.read_bool()?);
            m_rotate_with_stretch_direction = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_use_custom_vertex_streams = Some(ParticleSystemRendererMUseCustomVertexStreamsVariant::parse(rdr, ver)?);
            m_vertex_streams = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u8() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_use_custom_trail_vertex_streams = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_trail_vertex_streams = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u8() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            m_normal_direction = Some(rdr.read_f32()?);
            m_shadow_bias = Some(rdr.read_f32()?);
            m_render_alignment = Some(rdr.read_i32()?);
            m_pivot = Some(Vector3f::parse(rdr, ver)?);
            m_flip = Some(Vector3f::parse(rdr, ver)?);
            m_use_custom_vertex_streams = Some(ParticleSystemRendererMUseCustomVertexStreamsVariant::parse(rdr, ver)?);
            m_enable_gpu_instancing = Some(ParticleSystemRendererMEnableGpuInstancingVariant::parse(rdr, ver)?);
            m_apply_active_color_space = Some(ParticleSystemRendererMApplyActiveColorSpaceVariant::parse(rdr, ver)?);
            m_allow_roll = Some(ParticleSystemRendererMAllowRollVariant::parse(rdr, ver)?);
            m_freeform_stretching = Some(rdr.read_bool()?);
            m_rotate_with_stretch_direction = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_vertex_streams = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u8() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            m_normal_direction = Some(rdr.read_f32()?);
            m_shadow_bias = Some(rdr.read_f32()?);
            m_render_alignment = Some(rdr.read_i32()?);
            m_pivot = Some(Vector3f::parse(rdr, ver)?);
            m_flip = Some(Vector3f::parse(rdr, ver)?);
            m_use_custom_vertex_streams = Some(ParticleSystemRendererMUseCustomVertexStreamsVariant::parse(rdr, ver)?);
            m_enable_gpu_instancing = Some(ParticleSystemRendererMEnableGpuInstancingVariant::parse(rdr, ver)?);
            m_apply_active_color_space = Some(ParticleSystemRendererMApplyActiveColorSpaceVariant::parse(rdr, ver)?);
            m_allow_roll = Some(ParticleSystemRendererMAllowRollVariant::parse(rdr, ver)?);
            m_vertex_streams = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u8() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            m_normal_direction = Some(rdr.read_f32()?);
            m_render_alignment = Some(rdr.read_i32()?);
            m_pivot = Some(Vector3f::parse(rdr, ver)?);
            m_use_custom_vertex_streams = Some(ParticleSystemRendererMUseCustomVertexStreamsVariant::parse(rdr, ver)?);
            m_enable_gpu_instancing = Some(ParticleSystemRendererMEnableGpuInstancingVariant::parse(rdr, ver)?);
            m_apply_active_color_space = Some(ParticleSystemRendererMApplyActiveColorSpaceVariant::parse(rdr, ver)?);
            m_vertex_streams = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u8() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            m_normal_direction = Some(rdr.read_f32()?);
            m_render_alignment = Some(rdr.read_i32()?);
            m_pivot = Some(Vector3f::parse(rdr, ver)?);
            m_use_custom_vertex_streams = Some(ParticleSystemRendererMUseCustomVertexStreamsVariant::parse(rdr, ver)?);
            m_enable_gpu_instancing = Some(ParticleSystemRendererMEnableGpuInstancingVariant::parse(rdr, ver)?);
            m_vertex_streams = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u8() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_normal_direction = Some(rdr.read_f32()?);
            m_render_alignment = Some(rdr.read_i32()?);
            m_pivot = Some(Vector3f::parse(rdr, ver)?);
            m_use_custom_vertex_streams = Some(ParticleSystemRendererMUseCustomVertexStreamsVariant::parse(rdr, ver)?);
            m_vertex_streams = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u8() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 5, 0) {
            m_normal_direction = Some(rdr.read_f32()?);
            m_render_alignment = Some(rdr.read_i32()?);
            m_pivot = Some(Vector3f::parse(rdr, ver)?);
            m_use_custom_vertex_streams = Some(ParticleSystemRendererMUseCustomVertexStreamsVariant::parse(rdr, ver)?);
            m_vertex_stream_mask = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(5, 3, 0) {
            m_normal_direction = Some(rdr.read_f32()?);
            m_render_alignment = Some(rdr.read_i32()?);
            m_pivot = Some(Vector3f::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_normal_direction = Some(rdr.read_f32()?);
            m_sort_mode = Some(ParticleSystemRendererMSortModeVariant::parse(rdr, ver)?);
        } else {
            m_sort_mode = Some(ParticleSystemRendererMSortModeVariant::parse(rdr, ver)?);
        }
            m_mesh = Some(crate::PPtr::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(6000, 3, 0) {
            m_mesh1 = Some(crate::PPtr::parse(rdr, ver)?);
            m_mesh2 = Some(crate::PPtr::parse(rdr, ver)?);
            m_mesh3 = Some(crate::PPtr::parse(rdr, ver)?);
            m_mesh_weighting = Some(rdr.read_f32()?);
            m_mesh_weighting1 = Some(rdr.read_f32()?);
            m_mesh_weighting2 = Some(rdr.read_f32()?);
            m_mesh_weighting3 = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            m_mesh1 = Some(crate::PPtr::parse(rdr, ver)?);
            m_mesh2 = Some(crate::PPtr::parse(rdr, ver)?);
            m_mesh3 = Some(crate::PPtr::parse(rdr, ver)?);
            m_mesh_weighting = Some(rdr.read_f32()?);
            m_mesh_weighting1 = Some(rdr.read_f32()?);
            m_mesh_weighting2 = Some(rdr.read_f32()?);
            m_mesh_weighting3 = Some(rdr.read_f32()?);
            m_mask_interaction = Some(ParticleSystemRendererMMaskInteractionVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_mesh1 = Some(crate::PPtr::parse(rdr, ver)?);
            m_mesh2 = Some(crate::PPtr::parse(rdr, ver)?);
            m_mesh3 = Some(crate::PPtr::parse(rdr, ver)?);
            m_mask_interaction = Some(ParticleSystemRendererMMaskInteractionVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_mesh1 = Some(crate::PPtr::parse(rdr, ver)?);
            m_mesh2 = Some(crate::PPtr::parse(rdr, ver)?);
            m_mesh3 = Some(crate::PPtr::parse(rdr, ver)?);
        } else {
        }

        Ok(ParticleSystemRenderer {
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
            m_render_mode: m_render_mode.unwrap(),
            m_sorting_layer,
            m_sorting_layer_id,
            m_ray_tracing_accel_struct_build_flags_override,
            m_max_particle_size: m_max_particle_size.unwrap(),
            m_sorting_order,
            m_static_batch_info,
            m_ray_tracing_accel_struct_build_flags,
            m_camera_velocity_scale: m_camera_velocity_scale.unwrap(),
            m_probe_anchor,
            m_small_mesh_culling,
            m_velocity_scale: m_velocity_scale.unwrap(),
            m_force_mesh_lod,
            m_length_scale: m_length_scale.unwrap(),
            m_light_probe_volume_override,
            m_mesh_lod_selection_bias,
            m_sorting_fudge: m_sorting_fudge.unwrap(),
            m_sort_mode: m_sort_mode.unwrap(),
            m_normal_direction,
            m_mesh: m_mesh.unwrap(),
            m_min_particle_size,
            m_mesh1,
            m_mesh2,
            m_mesh3,
            m_render_alignment,
            m_pivot,
            m_mesh_distribution,
            m_use_custom_vertex_streams,
            m_vertex_stream_mask,
            m_mask_interaction,
            m_vertex_streams,
            m_shadow_bias,
            m_enable_gpu_instancing,
            m_apply_active_color_space,
            m_flip,
            m_allow_roll,
            m_freeform_stretching,
            m_rotate_with_stretch_direction,
            m_use_custom_trail_vertex_streams,
            m_trail_vertex_streams,
            m_mesh_weighting,
            m_mesh_weighting1,
            m_mesh_weighting2,
            m_mesh_weighting3,
        })
    }
}
