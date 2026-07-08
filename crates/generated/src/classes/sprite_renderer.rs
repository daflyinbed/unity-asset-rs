//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::color_rgba::ColorRgba;
use crate::subclasses::static_batch_info::StaticBatchInfo;
use crate::subclasses::vector2f::Vector2f;
use crate::subclasses::vector4f::Vector4f;


/// 版本枚举：SpriteRendererMEnabledVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SpriteRendererMEnabledVariant {
    Default(bool),
    V5_0(bool),
    V5_4(bool),
}

impl crate::Parse for SpriteRendererMEnabledVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = r.read_bool()?;
            Ok(Self::V5_4(v))
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Ok(Self::V5_0(v))
        } else {
            let v = r.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：SpriteRendererMCastShadowsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SpriteRendererMCastShadowsVariant {
    Default(bool),
    V5_0(u8),
}

impl crate::Parse for SpriteRendererMCastShadowsVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = r.read_u8()?;
            Ok(Self::V5_0(v))
        } else {
            let v = r.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：SpriteRendererMReceiveShadowsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SpriteRendererMReceiveShadowsVariant {
    Default(bool),
    V5_0(bool),
    V5_4(u8),
}

impl crate::Parse for SpriteRendererMReceiveShadowsVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = r.read_u8()?;
            Ok(Self::V5_4(v))
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Ok(Self::V5_0(v))
        } else {
            let v = r.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：SpriteRendererMLightmapIndexVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SpriteRendererMLightmapIndexVariant {
    Default(u8),
    V5_0(u16),
}

impl crate::Parse for SpriteRendererMLightmapIndexVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = r.read_u16()?;
            Ok(Self::V5_0(v))
        } else {
            let v = r.read_u8()?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：SpriteRendererMMaterialsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SpriteRendererMMaterialsVariant {
    Default(Vec<PPtr>),
    V2017_1(Vec<PPtr>),
}

impl crate::Parse for SpriteRendererMMaterialsVariant {
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

/// 版本枚举：SpriteRendererMReflectionProbeUsageVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SpriteRendererMReflectionProbeUsageVariant {
    Default(i32),
    V5_4(u8),
    V2019_3(u8),
}

impl crate::Parse for SpriteRendererMReflectionProbeUsageVariant {
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

/// 版本枚举：SpriteRendererMRayTracingModeVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SpriteRendererMRayTracingModeVariant {
    Default(u8),
    V2020_1(u8),
}

impl crate::Parse for SpriteRendererMRayTracingModeVariant {
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

/// 版本枚举：SpriteRendererMRayTraceProceduralVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SpriteRendererMRayTraceProceduralVariant {
    Default(u8),
    V2023_2(u8),
}

impl crate::Parse for SpriteRendererMRayTraceProceduralVariant {
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

/// 版本枚举：SpriteRendererMSortingLayerIdVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SpriteRendererMSortingLayerIdVariant {
    Default(u32),
    V5_0(i32),
}

impl crate::Parse for SpriteRendererMSortingLayerIdVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = r.read_i32()?;
            Ok(Self::V5_0(v))
        } else {
            let v = r.read_u32()?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：SpriteRendererMSortingOrderVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SpriteRendererMSortingOrderVariant {
    Default(i16),
    V4_5(i16),
}

impl crate::Parse for SpriteRendererMSortingOrderVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(4, 5, 0) {
            let v = r.read_i16()?;
            r.align(4)?;
            Ok(Self::V4_5(v))
        } else {
            let v = r.read_i16()?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：SpriteRendererMRayTracingAccelStructBuildFlagsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SpriteRendererMRayTracingAccelStructBuildFlagsVariant {
    Default(u8),
    V2023_3(u8),
}

impl crate::Parse for SpriteRendererMRayTracingAccelStructBuildFlagsVariant {
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

/// 版本枚举：SpriteRendererMProbeAnchorVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SpriteRendererMProbeAnchorVariant {
    Default(PPtr),
    V5_4(PPtr),
}

impl crate::Parse for SpriteRendererMProbeAnchorVariant {
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

/// 版本枚举：SpriteRendererMColorVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SpriteRendererMColorVariant {
    Default(ColorRgba),
    V4_5(ColorRgba),
}

impl crate::Parse for SpriteRendererMColorVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(4, 5, 0) {
            let v = ColorRgba::parse(r, ver)?;
            r.align(4)?;
            Ok(Self::V4_5(v))
        } else {
            let v = ColorRgba::parse(r, ver)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：SpriteRendererMFlipYVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SpriteRendererMFlipYVariant {
    Default(bool),
    V5_4(bool),
}

impl crate::Parse for SpriteRendererMFlipYVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Ok(Self::V5_4(v))
        } else {
            let v = r.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：SpriteRendererMSpriteTileModeVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SpriteRendererMSpriteTileModeVariant {
    Default(i32),
    V5_6_2(i32),
}

impl crate::Parse for SpriteRendererMSpriteTileModeVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 6, 2) {
            let v = r.read_i32()?;
            Ok(Self::V5_6_2(v))
        } else {
            let v = r.read_i32()?;
            r.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：SpriteRendererMMaskInteractionVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SpriteRendererMMaskInteractionVariant {
    Default(i32),
    V6000_3(i32),
}

impl crate::Parse for SpriteRendererMMaskInteractionVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(6000, 3, 0) {
            let v = r.read_i32()?;
            r.align(4)?;
            Ok(Self::V6000_3(v))
        } else {
            let v = r.read_i32()?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct SpriteRenderer {
    pub m_game_object: PPtr,
    pub m_enabled: SpriteRendererMEnabledVariant,
    pub m_cast_shadows: SpriteRendererMCastShadowsVariant,
    pub m_receive_shadows: SpriteRendererMReceiveShadowsVariant,
    pub m_lightmap_index: SpriteRendererMLightmapIndexVariant,
    pub m_motion_vectors: Option<u8>,
    pub m_dynamic_occludee: Option<u8>,
    pub m_lightmap_tiling_offset: Vector4f,
    pub m_lightmap_index_dynamic: Option<u16>,
    pub m_light_probe_usage: Option<u8>,
    pub m_static_shadow_caster: Option<u8>,
    pub m_materials: SpriteRendererMMaterialsVariant,
    pub m_reflection_probe_usage: Option<SpriteRendererMReflectionProbeUsageVariant>,
    pub m_subset_indices: Option<Vec<u32>>,
    pub m_lightmap_tiling_offset_dynamic: Option<Vector4f>,
    pub m_static_batch_root: PPtr,
    pub m_rendering_layer_mask: Option<u32>,
    pub m_ray_tracing_mode: Option<SpriteRendererMRayTracingModeVariant>,
    pub m_use_light_probes: Option<bool>,
    pub m_renderer_priority: Option<i32>,
    pub m_ray_trace_procedural: Option<SpriteRendererMRayTraceProceduralVariant>,
    pub m_light_probe_anchor: Option<PPtr>,
    pub m_sorting_layer: Option<i16>,
    pub m_sorting_layer_id: Option<SpriteRendererMSortingLayerIdVariant>,
    pub m_ray_tracing_accel_struct_build_flags_override: Option<u8>,
    pub m_sorting_order: SpriteRendererMSortingOrderVariant,
    pub m_static_batch_info: Option<StaticBatchInfo>,
    pub m_ray_tracing_accel_struct_build_flags: Option<SpriteRendererMRayTracingAccelStructBuildFlagsVariant>,
    pub m_sprite: PPtr,
    pub m_probe_anchor: Option<SpriteRendererMProbeAnchorVariant>,
    pub m_small_mesh_culling: Option<u8>,
    pub m_color: SpriteRendererMColorVariant,
    pub m_force_mesh_lod: Option<i16>,
    pub m_light_probe_volume_override: Option<PPtr>,
    pub m_mesh_lod_selection_bias: Option<f32>,
    pub m_flip_x: Option<bool>,
    pub m_flip_y: Option<SpriteRendererMFlipYVariant>,
    pub m_draw_mode: Option<i32>,
    pub m_size: Option<Vector2f>,
    pub m_adaptive_mode_threshold: Option<f32>,
    pub m_sprite_tile_mode: Option<SpriteRendererMSpriteTileModeVariant>,
    pub m_was_sprite_assigned: Option<bool>,
    pub m_mask_interaction: Option<SpriteRendererMMaskInteractionVariant>,
    pub m_sprite_sort_point: Option<i32>,
    pub m_blend_shape_weights: Option<Vec<f32>>,
}

impl crate::Parse for SpriteRenderer {
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
        let mut m_sprite = None;
        let mut m_probe_anchor = None;
        let mut m_small_mesh_culling = None;
        let mut m_color = None;
        let mut m_force_mesh_lod = None;
        let mut m_light_probe_volume_override = None;
        let mut m_mesh_lod_selection_bias = None;
        let mut m_flip_x = None;
        let mut m_flip_y = None;
        let mut m_draw_mode = None;
        let mut m_size = None;
        let mut m_adaptive_mode_threshold = None;
        let mut m_sprite_tile_mode = None;
        let mut m_was_sprite_assigned = None;
        let mut m_mask_interaction = None;
        let mut m_sprite_sort_point = None;
        let mut m_blend_shape_weights = None;
            m_game_object = Some(crate::PPtr::parse(r, ver)?);
            m_enabled = Some(SpriteRendererMEnabledVariant::parse(r, ver)?);
            m_cast_shadows = Some(SpriteRendererMCastShadowsVariant::parse(r, ver)?);
            m_receive_shadows = Some(SpriteRendererMReceiveShadowsVariant::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(6000, 2, 0) {
            m_dynamic_occludee = Some(r.read_u8()?);
            m_static_shadow_caster = Some(r.read_u8()?);
            m_motion_vectors = Some(r.read_u8()?);
            m_light_probe_usage = Some(r.read_u8()?);
            m_reflection_probe_usage = Some(SpriteRendererMReflectionProbeUsageVariant::parse(r, ver)?);
            m_ray_tracing_mode = Some(SpriteRendererMRayTracingModeVariant::parse(r, ver)?);
            m_ray_trace_procedural = Some(SpriteRendererMRayTraceProceduralVariant::parse(r, ver)?);
            m_ray_tracing_accel_struct_build_flags_override = Some(r.read_u8()?);
            m_ray_tracing_accel_struct_build_flags = Some(SpriteRendererMRayTracingAccelStructBuildFlagsVariant::parse(r, ver)?);
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
            m_reflection_probe_usage = Some(SpriteRendererMReflectionProbeUsageVariant::parse(r, ver)?);
            m_ray_tracing_mode = Some(SpriteRendererMRayTracingModeVariant::parse(r, ver)?);
            m_ray_trace_procedural = Some(SpriteRendererMRayTraceProceduralVariant::parse(r, ver)?);
            m_ray_tracing_accel_struct_build_flags_override = Some(r.read_u8()?);
            m_ray_tracing_accel_struct_build_flags = Some(SpriteRendererMRayTracingAccelStructBuildFlagsVariant::parse(r, ver)?);
            m_small_mesh_culling = Some(r.read_u8()?);
            r.align(4)?;
            m_rendering_layer_mask = Some(r.read_u32()?);
            m_renderer_priority = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            m_dynamic_occludee = Some(r.read_u8()?);
            m_static_shadow_caster = Some(r.read_u8()?);
            m_motion_vectors = Some(r.read_u8()?);
            m_light_probe_usage = Some(r.read_u8()?);
            m_reflection_probe_usage = Some(SpriteRendererMReflectionProbeUsageVariant::parse(r, ver)?);
            m_ray_tracing_mode = Some(SpriteRendererMRayTracingModeVariant::parse(r, ver)?);
            m_ray_trace_procedural = Some(SpriteRendererMRayTraceProceduralVariant::parse(r, ver)?);
            m_ray_tracing_accel_struct_build_flags_override = Some(r.read_u8()?);
            m_ray_tracing_accel_struct_build_flags = Some(SpriteRendererMRayTracingAccelStructBuildFlagsVariant::parse(r, ver)?);
            m_rendering_layer_mask = Some(r.read_u32()?);
            m_renderer_priority = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            m_dynamic_occludee = Some(r.read_u8()?);
            m_static_shadow_caster = Some(r.read_u8()?);
            m_motion_vectors = Some(r.read_u8()?);
            m_light_probe_usage = Some(r.read_u8()?);
            m_reflection_probe_usage = Some(SpriteRendererMReflectionProbeUsageVariant::parse(r, ver)?);
            m_ray_tracing_mode = Some(SpriteRendererMRayTracingModeVariant::parse(r, ver)?);
            m_ray_trace_procedural = Some(SpriteRendererMRayTraceProceduralVariant::parse(r, ver)?);
            m_rendering_layer_mask = Some(r.read_u32()?);
            m_renderer_priority = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            m_dynamic_occludee = Some(r.read_u8()?);
            m_motion_vectors = Some(r.read_u8()?);
            m_light_probe_usage = Some(r.read_u8()?);
            m_reflection_probe_usage = Some(SpriteRendererMReflectionProbeUsageVariant::parse(r, ver)?);
            m_ray_tracing_mode = Some(SpriteRendererMRayTracingModeVariant::parse(r, ver)?);
            m_ray_trace_procedural = Some(SpriteRendererMRayTraceProceduralVariant::parse(r, ver)?);
            m_rendering_layer_mask = Some(r.read_u32()?);
            m_renderer_priority = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            m_dynamic_occludee = Some(r.read_u8()?);
            m_motion_vectors = Some(r.read_u8()?);
            m_light_probe_usage = Some(r.read_u8()?);
            m_reflection_probe_usage = Some(SpriteRendererMReflectionProbeUsageVariant::parse(r, ver)?);
            m_ray_tracing_mode = Some(SpriteRendererMRayTracingModeVariant::parse(r, ver)?);
            m_rendering_layer_mask = Some(r.read_u32()?);
            m_renderer_priority = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            m_dynamic_occludee = Some(r.read_u8()?);
            m_motion_vectors = Some(r.read_u8()?);
            m_light_probe_usage = Some(r.read_u8()?);
            m_reflection_probe_usage = Some(SpriteRendererMReflectionProbeUsageVariant::parse(r, ver)?);
            m_rendering_layer_mask = Some(r.read_u32()?);
            m_renderer_priority = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            m_dynamic_occludee = Some(r.read_u8()?);
            m_motion_vectors = Some(r.read_u8()?);
            m_light_probe_usage = Some(r.read_u8()?);
            m_reflection_probe_usage = Some(SpriteRendererMReflectionProbeUsageVariant::parse(r, ver)?);
            m_rendering_layer_mask = Some(r.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_dynamic_occludee = Some(r.read_u8()?);
            m_motion_vectors = Some(r.read_u8()?);
            m_light_probe_usage = Some(r.read_u8()?);
            m_reflection_probe_usage = Some(SpriteRendererMReflectionProbeUsageVariant::parse(r, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            m_motion_vectors = Some(r.read_u8()?);
            m_light_probe_usage = Some(r.read_u8()?);
            m_reflection_probe_usage = Some(SpriteRendererMReflectionProbeUsageVariant::parse(r, ver)?);
        } else {
        }
            m_lightmap_index = Some(SpriteRendererMLightmapIndexVariant::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_lightmap_index_dynamic = Some(r.read_u16()?);
        } else {
        }
            m_lightmap_tiling_offset = Some(Vector4f::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_lightmap_tiling_offset_dynamic = Some(Vector4f::parse(r, ver)?);
        } else {
        }
            m_materials = Some(SpriteRendererMMaterialsVariant::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            m_static_batch_info = Some(StaticBatchInfo::parse(r, ver)?);
        } else {
            m_subset_indices = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u32() }).collect::<Result<Vec<_>, _>>() } )?);
        }
            m_static_batch_root = Some(crate::PPtr::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_probe_anchor = Some(SpriteRendererMProbeAnchorVariant::parse(r, ver)?);
            m_light_probe_volume_override = Some(crate::PPtr::parse(r, ver)?);
            r.align(4)?;
            m_sorting_layer_id = Some(SpriteRendererMSortingLayerIdVariant::parse(r, ver)?);
            m_sorting_layer = Some(r.read_i16()?);
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            m_probe_anchor = Some(SpriteRendererMProbeAnchorVariant::parse(r, ver)?);
            m_light_probe_volume_override = Some(crate::PPtr::parse(r, ver)?);
            r.align(4)?;
            m_sorting_layer_id = Some(SpriteRendererMSortingLayerIdVariant::parse(r, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_use_light_probes = Some(r.read_bool()?);
            r.align(4)?;
            m_reflection_probe_usage = Some(SpriteRendererMReflectionProbeUsageVariant::parse(r, ver)?);
            m_probe_anchor = Some(SpriteRendererMProbeAnchorVariant::parse(r, ver)?);
            m_sorting_layer_id = Some(SpriteRendererMSortingLayerIdVariant::parse(r, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 5, 0) {
            m_use_light_probes = Some(r.read_bool()?);
            r.align(4)?;
            m_light_probe_anchor = Some(crate::PPtr::parse(r, ver)?);
            r.align(4)?;
            m_sorting_layer_id = Some(SpriteRendererMSortingLayerIdVariant::parse(r, ver)?);
        } else {
            m_use_light_probes = Some(r.read_bool()?);
            r.align(4)?;
            m_light_probe_anchor = Some(crate::PPtr::parse(r, ver)?);
            r.align(4)?;
            m_sorting_layer = Some(r.read_i16()?);
        }
            m_sorting_order = Some(SpriteRendererMSortingOrderVariant::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(6000, 3, 0) {
            m_mask_interaction = Some(SpriteRendererMMaskInteractionVariant::parse(r, ver)?);
        } else {
        }
            m_sprite = Some(crate::PPtr::parse(r, ver)?);
            m_color = Some(SpriteRendererMColorVariant::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            m_flip_x = Some(r.read_bool()?);
            m_flip_y = Some(SpriteRendererMFlipYVariant::parse(r, ver)?);
            m_draw_mode = Some(r.read_i32()?);
            m_size = Some(Vector2f::parse(r, ver)?);
            m_adaptive_mode_threshold = Some(r.read_f32()?);
            m_sprite_tile_mode = Some(SpriteRendererMSpriteTileModeVariant::parse(r, ver)?);
            m_was_sprite_assigned = Some(r.read_bool()?);
            r.align(4)?;
            m_sprite_sort_point = Some(r.read_i32()?);
            m_blend_shape_weights = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_f32() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(6000, 3, 0) {
            m_flip_x = Some(r.read_bool()?);
            m_flip_y = Some(SpriteRendererMFlipYVariant::parse(r, ver)?);
            m_draw_mode = Some(r.read_i32()?);
            m_size = Some(Vector2f::parse(r, ver)?);
            m_adaptive_mode_threshold = Some(r.read_f32()?);
            m_sprite_tile_mode = Some(SpriteRendererMSpriteTileModeVariant::parse(r, ver)?);
            m_was_sprite_assigned = Some(r.read_bool()?);
            r.align(4)?;
            m_sprite_sort_point = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            m_flip_x = Some(r.read_bool()?);
            m_flip_y = Some(SpriteRendererMFlipYVariant::parse(r, ver)?);
            m_draw_mode = Some(r.read_i32()?);
            m_size = Some(Vector2f::parse(r, ver)?);
            m_adaptive_mode_threshold = Some(r.read_f32()?);
            m_sprite_tile_mode = Some(SpriteRendererMSpriteTileModeVariant::parse(r, ver)?);
            m_was_sprite_assigned = Some(r.read_bool()?);
            r.align(4)?;
            m_mask_interaction = Some(SpriteRendererMMaskInteractionVariant::parse(r, ver)?);
            m_sprite_sort_point = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_flip_x = Some(r.read_bool()?);
            m_flip_y = Some(SpriteRendererMFlipYVariant::parse(r, ver)?);
            m_draw_mode = Some(r.read_i32()?);
            m_size = Some(Vector2f::parse(r, ver)?);
            m_adaptive_mode_threshold = Some(r.read_f32()?);
            m_sprite_tile_mode = Some(SpriteRendererMSpriteTileModeVariant::parse(r, ver)?);
            m_was_sprite_assigned = Some(r.read_bool()?);
            r.align(4)?;
            m_mask_interaction = Some(SpriteRendererMMaskInteractionVariant::parse(r, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 6, 2) {
            m_flip_x = Some(r.read_bool()?);
            m_flip_y = Some(SpriteRendererMFlipYVariant::parse(r, ver)?);
            m_draw_mode = Some(r.read_i32()?);
            m_size = Some(Vector2f::parse(r, ver)?);
            m_adaptive_mode_threshold = Some(r.read_f32()?);
            m_sprite_tile_mode = Some(SpriteRendererMSpriteTileModeVariant::parse(r, ver)?);
            m_was_sprite_assigned = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_flip_x = Some(r.read_bool()?);
            m_flip_y = Some(SpriteRendererMFlipYVariant::parse(r, ver)?);
            m_draw_mode = Some(r.read_i32()?);
            m_size = Some(Vector2f::parse(r, ver)?);
            m_adaptive_mode_threshold = Some(r.read_f32()?);
            m_sprite_tile_mode = Some(SpriteRendererMSpriteTileModeVariant::parse(r, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 3, 0) {
            m_flip_x = Some(r.read_bool()?);
            m_flip_y = Some(SpriteRendererMFlipYVariant::parse(r, ver)?);
        } else {
        }

        Ok(SpriteRenderer {
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
            m_sorting_order: m_sorting_order.unwrap(),
            m_static_batch_info,
            m_ray_tracing_accel_struct_build_flags,
            m_sprite: m_sprite.unwrap(),
            m_probe_anchor,
            m_small_mesh_culling,
            m_color: m_color.unwrap(),
            m_force_mesh_lod,
            m_light_probe_volume_override,
            m_mesh_lod_selection_bias,
            m_flip_x,
            m_flip_y,
            m_draw_mode,
            m_size,
            m_adaptive_mode_threshold,
            m_sprite_tile_mode,
            m_was_sprite_assigned,
            m_mask_interaction,
            m_sprite_sort_point,
            m_blend_shape_weights,
        })
    }
}
