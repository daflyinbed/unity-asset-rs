//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::static_batch_info::StaticBatchInfo;
use crate::subclasses::vector4f::Vector4f;


/// 版本枚举：BillboardRendererMEnabledVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum BillboardRendererMEnabledVariant {
    Default(bool),
    V5_4(bool),
}

impl crate::Parse for BillboardRendererMEnabledVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = r.read_bool()?;
            Ok(Self::V5_4(v))
        } else {
            let v = r.read_bool()?;
            r.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：BillboardRendererMReceiveShadowsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum BillboardRendererMReceiveShadowsVariant {
    Default(bool),
    V5_4(u8),
}

impl crate::Parse for BillboardRendererMReceiveShadowsVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = r.read_u8()?;
            Ok(Self::V5_4(v))
        } else {
            let v = r.read_bool()?;
            r.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：BillboardRendererMReflectionProbeUsageVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum BillboardRendererMReflectionProbeUsageVariant {
    Default(i32),
    V5_4(u8),
    V2019_3(u8),
}

impl crate::Parse for BillboardRendererMReflectionProbeUsageVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            let v = r.read_u8()?;
            Ok(Self::V2019_3(v))
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = r.read_u8()?;
            r.align(4)?;
            Ok(Self::V5_4(v))
        } else {
            let v = r.read_i32()?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：BillboardRendererMMaterialsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum BillboardRendererMMaterialsVariant {
    Default(Vec<PPtr>),
    V2017_1(Vec<PPtr>),
}

impl crate::Parse for BillboardRendererMMaterialsVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：BillboardRendererMRayTracingModeVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum BillboardRendererMRayTracingModeVariant {
    Default(u8),
    V2020_1(u8),
}

impl crate::Parse for BillboardRendererMRayTracingModeVariant {
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

/// 版本枚举：BillboardRendererMRayTraceProceduralVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum BillboardRendererMRayTraceProceduralVariant {
    Default(u8),
    V2023_2(u8),
}

impl crate::Parse for BillboardRendererMRayTraceProceduralVariant {
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

/// 版本枚举：BillboardRendererMRayTracingAccelStructBuildFlagsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum BillboardRendererMRayTracingAccelStructBuildFlagsVariant {
    Default(u8),
    V2023_3(u8),
}

impl crate::Parse for BillboardRendererMRayTracingAccelStructBuildFlagsVariant {
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

/// 版本枚举：BillboardRendererMProbeAnchorVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum BillboardRendererMProbeAnchorVariant {
    Default(PPtr),
    V5_4(PPtr),
}

impl crate::Parse for BillboardRendererMProbeAnchorVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = crate::PPtr::parse(r, ver)?;
            Ok(Self::V5_4(v))
        } else {
            let v = crate::PPtr::parse(r, ver)?;
            r.align(4)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct BillboardRenderer {
    pub m_game_object: PPtr,
    pub m_enabled: BillboardRendererMEnabledVariant,
    pub m_cast_shadows: u8,
    pub m_receive_shadows: BillboardRendererMReceiveShadowsVariant,
    pub m_lightmap_index: u16,
    pub m_motion_vectors: Option<u8>,
    pub m_dynamic_occludee: Option<u8>,
    pub m_lightmap_index_dynamic: u16,
    pub m_light_probe_usage: Option<u8>,
    pub m_static_shadow_caster: Option<u8>,
    pub m_lightmap_tiling_offset: Vector4f,
    pub m_reflection_probe_usage: BillboardRendererMReflectionProbeUsageVariant,
    pub m_lightmap_tiling_offset_dynamic: Vector4f,
    pub m_materials: BillboardRendererMMaterialsVariant,
    pub m_rendering_layer_mask: Option<u32>,
    pub m_ray_tracing_mode: Option<BillboardRendererMRayTracingModeVariant>,
    pub m_subset_indices: Option<Vec<u32>>,
    pub m_renderer_priority: Option<i32>,
    pub m_ray_trace_procedural: Option<BillboardRendererMRayTraceProceduralVariant>,
    pub m_static_batch_root: PPtr,
    pub m_use_light_probes: Option<bool>,
    pub m_ray_tracing_accel_struct_build_flags_override: Option<u8>,
    pub m_static_batch_info: Option<StaticBatchInfo>,
    pub m_ray_tracing_accel_struct_build_flags: Option<BillboardRendererMRayTracingAccelStructBuildFlagsVariant>,
    pub m_probe_anchor: BillboardRendererMProbeAnchorVariant,
    pub m_small_mesh_culling: Option<u8>,
    pub m_sorting_layer_id: i32,
    pub m_force_mesh_lod: Option<i16>,
    pub m_sorting_order: i16,
    pub m_light_probe_volume_override: Option<PPtr>,
    pub m_mesh_lod_selection_bias: Option<f32>,
    pub m_billboard: PPtr,
    pub m_sorting_layer: Option<i16>,
    pub m_mask_interaction: Option<i32>,
}

impl crate::Parse for BillboardRenderer {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut m_enabled = None;
        let mut m_cast_shadows = None;
        let mut m_receive_shadows = None;
        let mut m_lightmap_index = None;
        let mut m_motion_vectors = None;
        let mut m_dynamic_occludee = None;
        let mut m_lightmap_index_dynamic = None;
        let mut m_light_probe_usage = None;
        let mut m_static_shadow_caster = None;
        let mut m_lightmap_tiling_offset = None;
        let mut m_reflection_probe_usage = None;
        let mut m_lightmap_tiling_offset_dynamic = None;
        let mut m_materials = None;
        let mut m_rendering_layer_mask = None;
        let mut m_ray_tracing_mode = None;
        let mut m_subset_indices = None;
        let mut m_renderer_priority = None;
        let mut m_ray_trace_procedural = None;
        let mut m_static_batch_root = None;
        let mut m_use_light_probes = None;
        let mut m_ray_tracing_accel_struct_build_flags_override = None;
        let mut m_static_batch_info = None;
        let mut m_ray_tracing_accel_struct_build_flags = None;
        let mut m_probe_anchor = None;
        let mut m_small_mesh_culling = None;
        let mut m_sorting_layer_id = None;
        let mut m_force_mesh_lod = None;
        let mut m_sorting_order = None;
        let mut m_light_probe_volume_override = None;
        let mut m_mesh_lod_selection_bias = None;
        let mut m_billboard = None;
        let mut m_sorting_layer = None;
        let mut m_mask_interaction = None;
            m_game_object = Some(crate::PPtr::parse(r, ver)?);
            m_enabled = Some(BillboardRendererMEnabledVariant::parse(r, ver)?);
            m_cast_shadows = Some(r.read_u8()?);
            m_receive_shadows = Some(BillboardRendererMReceiveShadowsVariant::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(6000, 2, 0) {
            m_dynamic_occludee = Some(r.read_u8()?);
            m_static_shadow_caster = Some(r.read_u8()?);
            m_motion_vectors = Some(r.read_u8()?);
            m_light_probe_usage = Some(r.read_u8()?);
            m_reflection_probe_usage = Some(BillboardRendererMReflectionProbeUsageVariant::parse(r, ver)?);
            m_ray_tracing_mode = Some(BillboardRendererMRayTracingModeVariant::parse(r, ver)?);
            m_ray_trace_procedural = Some(BillboardRendererMRayTraceProceduralVariant::parse(r, ver)?);
            m_ray_tracing_accel_struct_build_flags_override = Some(r.read_u8()?);
            m_ray_tracing_accel_struct_build_flags = Some(BillboardRendererMRayTracingAccelStructBuildFlagsVariant::parse(r, ver)?);
            m_small_mesh_culling = Some(r.read_u8()?);
            r.align(4)?;
            m_force_mesh_lod = Some(r.read_i16()?);
            r.align(4)?;
            m_mesh_lod_selection_bias = Some(r.read_f32()?);
            m_rendering_layer_mask = Some(r.read_u32()?);
            m_renderer_priority = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            m_dynamic_occludee = Some(r.read_u8()?);
            m_static_shadow_caster = Some(r.read_u8()?);
            m_motion_vectors = Some(r.read_u8()?);
            m_light_probe_usage = Some(r.read_u8()?);
            m_reflection_probe_usage = Some(BillboardRendererMReflectionProbeUsageVariant::parse(r, ver)?);
            m_ray_tracing_mode = Some(BillboardRendererMRayTracingModeVariant::parse(r, ver)?);
            m_ray_trace_procedural = Some(BillboardRendererMRayTraceProceduralVariant::parse(r, ver)?);
            m_ray_tracing_accel_struct_build_flags_override = Some(r.read_u8()?);
            m_ray_tracing_accel_struct_build_flags = Some(BillboardRendererMRayTracingAccelStructBuildFlagsVariant::parse(r, ver)?);
            m_small_mesh_culling = Some(r.read_u8()?);
            r.align(4)?;
            m_rendering_layer_mask = Some(r.read_u32()?);
            m_renderer_priority = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            m_dynamic_occludee = Some(r.read_u8()?);
            m_static_shadow_caster = Some(r.read_u8()?);
            m_motion_vectors = Some(r.read_u8()?);
            m_light_probe_usage = Some(r.read_u8()?);
            m_reflection_probe_usage = Some(BillboardRendererMReflectionProbeUsageVariant::parse(r, ver)?);
            m_ray_tracing_mode = Some(BillboardRendererMRayTracingModeVariant::parse(r, ver)?);
            m_ray_trace_procedural = Some(BillboardRendererMRayTraceProceduralVariant::parse(r, ver)?);
            m_ray_tracing_accel_struct_build_flags_override = Some(r.read_u8()?);
            m_ray_tracing_accel_struct_build_flags = Some(BillboardRendererMRayTracingAccelStructBuildFlagsVariant::parse(r, ver)?);
            m_rendering_layer_mask = Some(r.read_u32()?);
            m_renderer_priority = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            m_dynamic_occludee = Some(r.read_u8()?);
            m_static_shadow_caster = Some(r.read_u8()?);
            m_motion_vectors = Some(r.read_u8()?);
            m_light_probe_usage = Some(r.read_u8()?);
            m_reflection_probe_usage = Some(BillboardRendererMReflectionProbeUsageVariant::parse(r, ver)?);
            m_ray_tracing_mode = Some(BillboardRendererMRayTracingModeVariant::parse(r, ver)?);
            m_ray_trace_procedural = Some(BillboardRendererMRayTraceProceduralVariant::parse(r, ver)?);
            m_rendering_layer_mask = Some(r.read_u32()?);
            m_renderer_priority = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            m_dynamic_occludee = Some(r.read_u8()?);
            m_motion_vectors = Some(r.read_u8()?);
            m_light_probe_usage = Some(r.read_u8()?);
            m_reflection_probe_usage = Some(BillboardRendererMReflectionProbeUsageVariant::parse(r, ver)?);
            m_ray_tracing_mode = Some(BillboardRendererMRayTracingModeVariant::parse(r, ver)?);
            m_ray_trace_procedural = Some(BillboardRendererMRayTraceProceduralVariant::parse(r, ver)?);
            m_rendering_layer_mask = Some(r.read_u32()?);
            m_renderer_priority = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            m_dynamic_occludee = Some(r.read_u8()?);
            m_motion_vectors = Some(r.read_u8()?);
            m_light_probe_usage = Some(r.read_u8()?);
            m_reflection_probe_usage = Some(BillboardRendererMReflectionProbeUsageVariant::parse(r, ver)?);
            m_ray_tracing_mode = Some(BillboardRendererMRayTracingModeVariant::parse(r, ver)?);
            m_rendering_layer_mask = Some(r.read_u32()?);
            m_renderer_priority = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            m_dynamic_occludee = Some(r.read_u8()?);
            m_motion_vectors = Some(r.read_u8()?);
            m_light_probe_usage = Some(r.read_u8()?);
            m_reflection_probe_usage = Some(BillboardRendererMReflectionProbeUsageVariant::parse(r, ver)?);
            m_rendering_layer_mask = Some(r.read_u32()?);
            m_renderer_priority = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            m_dynamic_occludee = Some(r.read_u8()?);
            m_motion_vectors = Some(r.read_u8()?);
            m_light_probe_usage = Some(r.read_u8()?);
            m_reflection_probe_usage = Some(BillboardRendererMReflectionProbeUsageVariant::parse(r, ver)?);
            m_rendering_layer_mask = Some(r.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_dynamic_occludee = Some(r.read_u8()?);
            m_motion_vectors = Some(r.read_u8()?);
            m_light_probe_usage = Some(r.read_u8()?);
            m_reflection_probe_usage = Some(BillboardRendererMReflectionProbeUsageVariant::parse(r, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            m_motion_vectors = Some(r.read_u8()?);
            m_light_probe_usage = Some(r.read_u8()?);
            m_reflection_probe_usage = Some(BillboardRendererMReflectionProbeUsageVariant::parse(r, ver)?);
        } else {
        }
            m_lightmap_index = Some(r.read_u16()?);
            m_lightmap_index_dynamic = Some(r.read_u16()?);
            m_lightmap_tiling_offset = Some(Vector4f::parse(r, ver)?);
            m_lightmap_tiling_offset_dynamic = Some(Vector4f::parse(r, ver)?);
            m_materials = Some(BillboardRendererMMaterialsVariant::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            m_static_batch_info = Some(StaticBatchInfo::parse(r, ver)?);
        } else {
            m_subset_indices = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u32() }).collect::<Result<Vec<_>, _>>() } )?);
        }
            m_static_batch_root = Some(crate::PPtr::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
        } else {
            m_use_light_probes = Some(r.read_bool()?);
            r.align(4)?;
            m_reflection_probe_usage = Some(BillboardRendererMReflectionProbeUsageVariant::parse(r, ver)?);
        }
            m_probe_anchor = Some(BillboardRendererMProbeAnchorVariant::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            m_light_probe_volume_override = Some(crate::PPtr::parse(r, ver)?);
            r.align(4)?;
        } else {
        }
            m_sorting_layer_id = Some(r.read_i32()?);
        if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_sorting_layer = Some(r.read_i16()?);
        } else {
        }
            m_sorting_order = Some(r.read_i16()?);
            r.align(4)?;
        if *ver >= crate::UnityVersion::new(6000, 3, 0) {
            m_mask_interaction = Some(r.read_i32()?);
            r.align(4)?;
        } else {
        }
            m_billboard = Some(crate::PPtr::parse(r, ver)?);

        Ok(BillboardRenderer {
            m_game_object: m_game_object.unwrap(),
            m_enabled: m_enabled.unwrap(),
            m_cast_shadows: m_cast_shadows.unwrap(),
            m_receive_shadows: m_receive_shadows.unwrap(),
            m_lightmap_index: m_lightmap_index.unwrap(),
            m_motion_vectors,
            m_dynamic_occludee,
            m_lightmap_index_dynamic: m_lightmap_index_dynamic.unwrap(),
            m_light_probe_usage,
            m_static_shadow_caster,
            m_lightmap_tiling_offset: m_lightmap_tiling_offset.unwrap(),
            m_reflection_probe_usage: m_reflection_probe_usage.unwrap(),
            m_lightmap_tiling_offset_dynamic: m_lightmap_tiling_offset_dynamic.unwrap(),
            m_materials: m_materials.unwrap(),
            m_rendering_layer_mask,
            m_ray_tracing_mode,
            m_subset_indices,
            m_renderer_priority,
            m_ray_trace_procedural,
            m_static_batch_root: m_static_batch_root.unwrap(),
            m_use_light_probes,
            m_ray_tracing_accel_struct_build_flags_override,
            m_static_batch_info,
            m_ray_tracing_accel_struct_build_flags,
            m_probe_anchor: m_probe_anchor.unwrap(),
            m_small_mesh_culling,
            m_sorting_layer_id: m_sorting_layer_id.unwrap(),
            m_force_mesh_lod,
            m_sorting_order: m_sorting_order.unwrap(),
            m_light_probe_volume_override,
            m_mesh_lod_selection_bias,
            m_billboard: m_billboard.unwrap(),
            m_sorting_layer,
            m_mask_interaction,
        })
    }
}
