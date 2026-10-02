//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::color_rgb_af::ColorRgbAf;
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

impl crate::ToValue for SpriteRendererMEnabledVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_0(v) => v.to_value(),
            Self::V5_4(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_0(v) => (v).inspect_value(path, offset, limit),
            Self::V5_4(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for SpriteRendererMCastShadowsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_0(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_0(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for SpriteRendererMReceiveShadowsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_0(v) => v.to_value(),
            Self::V5_4(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_0(v) => (v).inspect_value(path, offset, limit),
            Self::V5_4(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for SpriteRendererMLightmapIndexVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_0(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_0(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for SpriteRendererMMaterialsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for SpriteRendererMReflectionProbeUsageVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_4(v) => v.to_value(),
            Self::V2019_3(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_4(v) => (v).inspect_value(path, offset, limit),
            Self::V2019_3(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for SpriteRendererMRayTracingModeVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2020_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2020_1(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for SpriteRendererMRayTraceProceduralVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2023_2(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2023_2(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for SpriteRendererMSortingLayerIdVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_0(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_0(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for SpriteRendererMSortingOrderVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V4_5(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V4_5(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for SpriteRendererMRayTracingAccelStructBuildFlagsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2023_3(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2023_3(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for SpriteRendererMProbeAnchorVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_4(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_4(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：SpriteRendererMColorVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SpriteRendererMColorVariant {
    Default(Box<ColorRgbAf>),
    V4_5(Box<ColorRgbAf>),
}

impl crate::Parse for SpriteRendererMColorVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(4, 5, 0) {
            let v = ({ Ok(Box::new(ColorRgbAf::parse(rdr, ver)?)) })?;
            rdr.align(4)?;
            Ok(Self::V4_5(v))
        } else {
            let v = ({ Ok(Box::new(ColorRgbAf::parse(rdr, ver)?)) })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for SpriteRendererMColorVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V4_5(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V4_5(v) => (v).inspect_value(path, offset, limit),
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
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V5_4(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for SpriteRendererMFlipYVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_4(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_4(v) => (v).inspect_value(path, offset, limit),
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
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 6, 2) {
            let v = rdr.read_i32()?;
            Ok(Self::V5_6_2(v))
        } else {
            let v = rdr.read_i32()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for SpriteRendererMSpriteTileModeVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_6_2(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_6_2(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for SpriteRendererMMaskInteractionVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V6000_3(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V6000_3(v) => (v).inspect_value(path, offset, limit),
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
    pub m_ray_tracing_accel_struct_build_flags:
        Option<SpriteRendererMRayTracingAccelStructBuildFlagsVariant>,
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
        m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
        m_enabled = Some(SpriteRendererMEnabledVariant::parse(rdr, ver)?);
        m_cast_shadows = Some(SpriteRendererMCastShadowsVariant::parse(rdr, ver)?);
        m_receive_shadows = Some(SpriteRendererMReceiveShadowsVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(6000, 2, 0) {
            m_dynamic_occludee = Some(rdr.read_u8()?);
            m_static_shadow_caster = Some(rdr.read_u8()?);
            m_motion_vectors = Some(rdr.read_u8()?);
            m_light_probe_usage = Some(rdr.read_u8()?);
            m_reflection_probe_usage =
                Some(SpriteRendererMReflectionProbeUsageVariant::parse(rdr, ver)?);
            m_ray_tracing_mode = Some(SpriteRendererMRayTracingModeVariant::parse(rdr, ver)?);
            m_ray_trace_procedural =
                Some(SpriteRendererMRayTraceProceduralVariant::parse(rdr, ver)?);
            m_ray_tracing_accel_struct_build_flags_override = Some(rdr.read_u8()?);
            m_ray_tracing_accel_struct_build_flags =
                Some(SpriteRendererMRayTracingAccelStructBuildFlagsVariant::parse(rdr, ver)?);
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
                Some(SpriteRendererMReflectionProbeUsageVariant::parse(rdr, ver)?);
            m_ray_tracing_mode = Some(SpriteRendererMRayTracingModeVariant::parse(rdr, ver)?);
            m_ray_trace_procedural =
                Some(SpriteRendererMRayTraceProceduralVariant::parse(rdr, ver)?);
            m_ray_tracing_accel_struct_build_flags_override = Some(rdr.read_u8()?);
            m_ray_tracing_accel_struct_build_flags =
                Some(SpriteRendererMRayTracingAccelStructBuildFlagsVariant::parse(rdr, ver)?);
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
                Some(SpriteRendererMReflectionProbeUsageVariant::parse(rdr, ver)?);
            m_ray_tracing_mode = Some(SpriteRendererMRayTracingModeVariant::parse(rdr, ver)?);
            m_ray_trace_procedural =
                Some(SpriteRendererMRayTraceProceduralVariant::parse(rdr, ver)?);
            m_ray_tracing_accel_struct_build_flags_override = Some(rdr.read_u8()?);
            m_ray_tracing_accel_struct_build_flags =
                Some(SpriteRendererMRayTracingAccelStructBuildFlagsVariant::parse(rdr, ver)?);
            m_rendering_layer_mask = Some(rdr.read_u32()?);
            m_renderer_priority = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            m_dynamic_occludee = Some(rdr.read_u8()?);
            m_static_shadow_caster = Some(rdr.read_u8()?);
            m_motion_vectors = Some(rdr.read_u8()?);
            m_light_probe_usage = Some(rdr.read_u8()?);
            m_reflection_probe_usage =
                Some(SpriteRendererMReflectionProbeUsageVariant::parse(rdr, ver)?);
            m_ray_tracing_mode = Some(SpriteRendererMRayTracingModeVariant::parse(rdr, ver)?);
            m_ray_trace_procedural =
                Some(SpriteRendererMRayTraceProceduralVariant::parse(rdr, ver)?);
            m_rendering_layer_mask = Some(rdr.read_u32()?);
            m_renderer_priority = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            m_dynamic_occludee = Some(rdr.read_u8()?);
            m_motion_vectors = Some(rdr.read_u8()?);
            m_light_probe_usage = Some(rdr.read_u8()?);
            m_reflection_probe_usage =
                Some(SpriteRendererMReflectionProbeUsageVariant::parse(rdr, ver)?);
            m_ray_tracing_mode = Some(SpriteRendererMRayTracingModeVariant::parse(rdr, ver)?);
            m_ray_trace_procedural =
                Some(SpriteRendererMRayTraceProceduralVariant::parse(rdr, ver)?);
            m_rendering_layer_mask = Some(rdr.read_u32()?);
            m_renderer_priority = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            m_dynamic_occludee = Some(rdr.read_u8()?);
            m_motion_vectors = Some(rdr.read_u8()?);
            m_light_probe_usage = Some(rdr.read_u8()?);
            m_reflection_probe_usage =
                Some(SpriteRendererMReflectionProbeUsageVariant::parse(rdr, ver)?);
            m_ray_tracing_mode = Some(SpriteRendererMRayTracingModeVariant::parse(rdr, ver)?);
            m_rendering_layer_mask = Some(rdr.read_u32()?);
            m_renderer_priority = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            m_dynamic_occludee = Some(rdr.read_u8()?);
            m_motion_vectors = Some(rdr.read_u8()?);
            m_light_probe_usage = Some(rdr.read_u8()?);
            m_reflection_probe_usage =
                Some(SpriteRendererMReflectionProbeUsageVariant::parse(rdr, ver)?);
            m_rendering_layer_mask = Some(rdr.read_u32()?);
            m_renderer_priority = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            m_dynamic_occludee = Some(rdr.read_u8()?);
            m_motion_vectors = Some(rdr.read_u8()?);
            m_light_probe_usage = Some(rdr.read_u8()?);
            m_reflection_probe_usage =
                Some(SpriteRendererMReflectionProbeUsageVariant::parse(rdr, ver)?);
            m_rendering_layer_mask = Some(rdr.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_dynamic_occludee = Some(rdr.read_u8()?);
            m_motion_vectors = Some(rdr.read_u8()?);
            m_light_probe_usage = Some(rdr.read_u8()?);
            m_reflection_probe_usage =
                Some(SpriteRendererMReflectionProbeUsageVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            m_motion_vectors = Some(rdr.read_u8()?);
            m_light_probe_usage = Some(rdr.read_u8()?);
            m_reflection_probe_usage =
                Some(SpriteRendererMReflectionProbeUsageVariant::parse(rdr, ver)?);
        } else {
        }
        m_lightmap_index = Some(SpriteRendererMLightmapIndexVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_lightmap_index_dynamic = Some(rdr.read_u16()?);
        } else {
        }
        m_lightmap_tiling_offset = Some(Vector4f::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_lightmap_tiling_offset_dynamic = Some(Vector4f::parse(rdr, ver)?);
        } else {
        }
        m_materials = Some(SpriteRendererMMaterialsVariant::parse(rdr, ver)?);
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
        if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_probe_anchor = Some(SpriteRendererMProbeAnchorVariant::parse(rdr, ver)?);
            m_light_probe_volume_override = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
            m_sorting_layer_id = Some(SpriteRendererMSortingLayerIdVariant::parse(rdr, ver)?);
            m_sorting_layer = Some(rdr.read_i16()?);
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            m_probe_anchor = Some(SpriteRendererMProbeAnchorVariant::parse(rdr, ver)?);
            m_light_probe_volume_override = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
            m_sorting_layer_id = Some(SpriteRendererMSortingLayerIdVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_use_light_probes = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_reflection_probe_usage =
                Some(SpriteRendererMReflectionProbeUsageVariant::parse(rdr, ver)?);
            m_probe_anchor = Some(SpriteRendererMProbeAnchorVariant::parse(rdr, ver)?);
            m_sorting_layer_id = Some(SpriteRendererMSortingLayerIdVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 5, 0) {
            m_use_light_probes = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_light_probe_anchor = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
            m_sorting_layer_id = Some(SpriteRendererMSortingLayerIdVariant::parse(rdr, ver)?);
        } else {
            m_use_light_probes = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_light_probe_anchor = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
            m_sorting_layer = Some(rdr.read_i16()?);
        }
        m_sorting_order = Some(SpriteRendererMSortingOrderVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(6000, 3, 0) {
            m_mask_interaction = Some(SpriteRendererMMaskInteractionVariant::parse(rdr, ver)?);
        } else {
        }
        m_sprite = Some(crate::PPtr::parse(rdr, ver)?);
        m_color = Some(SpriteRendererMColorVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            m_flip_x = Some(rdr.read_bool()?);
            m_flip_y = Some(SpriteRendererMFlipYVariant::parse(rdr, ver)?);
            m_draw_mode = Some(rdr.read_i32()?);
            m_size = Some(Vector2f::parse(rdr, ver)?);
            m_adaptive_mode_threshold = Some(rdr.read_f32()?);
            m_sprite_tile_mode = Some(SpriteRendererMSpriteTileModeVariant::parse(rdr, ver)?);
            m_was_sprite_assigned = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_sprite_sort_point = Some(rdr.read_i32()?);
            m_blend_shape_weights = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_f32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(6000, 3, 0) {
            m_flip_x = Some(rdr.read_bool()?);
            m_flip_y = Some(SpriteRendererMFlipYVariant::parse(rdr, ver)?);
            m_draw_mode = Some(rdr.read_i32()?);
            m_size = Some(Vector2f::parse(rdr, ver)?);
            m_adaptive_mode_threshold = Some(rdr.read_f32()?);
            m_sprite_tile_mode = Some(SpriteRendererMSpriteTileModeVariant::parse(rdr, ver)?);
            m_was_sprite_assigned = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_sprite_sort_point = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            m_flip_x = Some(rdr.read_bool()?);
            m_flip_y = Some(SpriteRendererMFlipYVariant::parse(rdr, ver)?);
            m_draw_mode = Some(rdr.read_i32()?);
            m_size = Some(Vector2f::parse(rdr, ver)?);
            m_adaptive_mode_threshold = Some(rdr.read_f32()?);
            m_sprite_tile_mode = Some(SpriteRendererMSpriteTileModeVariant::parse(rdr, ver)?);
            m_was_sprite_assigned = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_mask_interaction = Some(SpriteRendererMMaskInteractionVariant::parse(rdr, ver)?);
            m_sprite_sort_point = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_flip_x = Some(rdr.read_bool()?);
            m_flip_y = Some(SpriteRendererMFlipYVariant::parse(rdr, ver)?);
            m_draw_mode = Some(rdr.read_i32()?);
            m_size = Some(Vector2f::parse(rdr, ver)?);
            m_adaptive_mode_threshold = Some(rdr.read_f32()?);
            m_sprite_tile_mode = Some(SpriteRendererMSpriteTileModeVariant::parse(rdr, ver)?);
            m_was_sprite_assigned = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_mask_interaction = Some(SpriteRendererMMaskInteractionVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 6, 2) {
            m_flip_x = Some(rdr.read_bool()?);
            m_flip_y = Some(SpriteRendererMFlipYVariant::parse(rdr, ver)?);
            m_draw_mode = Some(rdr.read_i32()?);
            m_size = Some(Vector2f::parse(rdr, ver)?);
            m_adaptive_mode_threshold = Some(rdr.read_f32()?);
            m_sprite_tile_mode = Some(SpriteRendererMSpriteTileModeVariant::parse(rdr, ver)?);
            m_was_sprite_assigned = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_flip_x = Some(rdr.read_bool()?);
            m_flip_y = Some(SpriteRendererMFlipYVariant::parse(rdr, ver)?);
            m_draw_mode = Some(rdr.read_i32()?);
            m_size = Some(Vector2f::parse(rdr, ver)?);
            m_adaptive_mode_threshold = Some(rdr.read_f32()?);
            m_sprite_tile_mode = Some(SpriteRendererMSpriteTileModeVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 3, 0) {
            m_flip_x = Some(rdr.read_bool()?);
            m_flip_y = Some(SpriteRendererMFlipYVariant::parse(rdr, ver)?);
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

impl crate::ToValue for SpriteRenderer {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_GameObject".to_string(), self.m_game_object.to_value())),
            Some(("m_Enabled".to_string(), self.m_enabled.to_value())),
            Some(("m_CastShadows".to_string(), self.m_cast_shadows.to_value())),
            Some((
                "m_ReceiveShadows".to_string(),
                self.m_receive_shadows.to_value(),
            )),
            Some((
                "m_LightmapIndex".to_string(),
                self.m_lightmap_index.to_value(),
            )),
            crate::opt_field("m_MotionVectors", &self.m_motion_vectors),
            crate::opt_field("m_DynamicOccludee", &self.m_dynamic_occludee),
            Some((
                "m_LightmapTilingOffset".to_string(),
                self.m_lightmap_tiling_offset.to_value(),
            )),
            crate::opt_field("m_LightmapIndexDynamic", &self.m_lightmap_index_dynamic),
            crate::opt_field("m_LightProbeUsage", &self.m_light_probe_usage),
            crate::opt_field("m_StaticShadowCaster", &self.m_static_shadow_caster),
            Some(("m_Materials".to_string(), self.m_materials.to_value())),
            crate::opt_field("m_ReflectionProbeUsage", &self.m_reflection_probe_usage),
            crate::opt_field("m_SubsetIndices", &self.m_subset_indices),
            crate::opt_field(
                "m_LightmapTilingOffsetDynamic",
                &self.m_lightmap_tiling_offset_dynamic,
            ),
            Some((
                "m_StaticBatchRoot".to_string(),
                self.m_static_batch_root.to_value(),
            )),
            crate::opt_field("m_RenderingLayerMask", &self.m_rendering_layer_mask),
            crate::opt_field("m_RayTracingMode", &self.m_ray_tracing_mode),
            crate::opt_field("m_UseLightProbes", &self.m_use_light_probes),
            crate::opt_field("m_RendererPriority", &self.m_renderer_priority),
            crate::opt_field("m_RayTraceProcedural", &self.m_ray_trace_procedural),
            crate::opt_field("m_LightProbeAnchor", &self.m_light_probe_anchor),
            crate::opt_field("m_SortingLayer", &self.m_sorting_layer),
            crate::opt_field("m_SortingLayerID", &self.m_sorting_layer_id),
            crate::opt_field(
                "m_RayTracingAccelStructBuildFlagsOverride",
                &self.m_ray_tracing_accel_struct_build_flags_override,
            ),
            Some((
                "m_SortingOrder".to_string(),
                self.m_sorting_order.to_value(),
            )),
            crate::opt_field("m_StaticBatchInfo", &self.m_static_batch_info),
            crate::opt_field(
                "m_RayTracingAccelStructBuildFlags",
                &self.m_ray_tracing_accel_struct_build_flags,
            ),
            Some(("m_Sprite".to_string(), self.m_sprite.to_value())),
            crate::opt_field("m_ProbeAnchor", &self.m_probe_anchor),
            crate::opt_field("m_SmallMeshCulling", &self.m_small_mesh_culling),
            Some(("m_Color".to_string(), self.m_color.to_value())),
            crate::opt_field("m_ForceMeshLod", &self.m_force_mesh_lod),
            crate::opt_field(
                "m_LightProbeVolumeOverride",
                &self.m_light_probe_volume_override,
            ),
            crate::opt_field("m_MeshLodSelectionBias", &self.m_mesh_lod_selection_bias),
            crate::opt_field("m_FlipX", &self.m_flip_x),
            crate::opt_field("m_FlipY", &self.m_flip_y),
            crate::opt_field("m_DrawMode", &self.m_draw_mode),
            crate::opt_field("m_Size", &self.m_size),
            crate::opt_field("m_AdaptiveModeThreshold", &self.m_adaptive_mode_threshold),
            crate::opt_field("m_SpriteTileMode", &self.m_sprite_tile_mode),
            crate::opt_field("m_WasSpriteAssigned", &self.m_was_sprite_assigned),
            crate::opt_field("m_MaskInteraction", &self.m_mask_interaction),
            crate::opt_field("m_SpriteSortPoint", &self.m_sprite_sort_point),
            crate::opt_field("m_BlendShapeWeights", &self.m_blend_shape_weights),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_GameObject" => (self.m_game_object).inspect_value(path, offset, limit),
                "m_Enabled" => (self.m_enabled).inspect_value(path, offset, limit),
                "m_CastShadows" => (self.m_cast_shadows).inspect_value(path, offset, limit),
                "m_ReceiveShadows" => (self.m_receive_shadows).inspect_value(path, offset, limit),
                "m_LightmapIndex" => (self.m_lightmap_index).inspect_value(path, offset, limit),
                "m_MotionVectors" => (self.m_motion_vectors).inspect_value(path, offset, limit),
                "m_DynamicOccludee" => (self.m_dynamic_occludee).inspect_value(path, offset, limit),
                "m_LightmapTilingOffset" => {
                    (self.m_lightmap_tiling_offset).inspect_value(path, offset, limit)
                }
                "m_LightmapIndexDynamic" => {
                    (self.m_lightmap_index_dynamic).inspect_value(path, offset, limit)
                }
                "m_LightProbeUsage" => {
                    (self.m_light_probe_usage).inspect_value(path, offset, limit)
                }
                "m_StaticShadowCaster" => {
                    (self.m_static_shadow_caster).inspect_value(path, offset, limit)
                }
                "m_Materials" => (self.m_materials).inspect_value(path, offset, limit),
                "m_ReflectionProbeUsage" => {
                    (self.m_reflection_probe_usage).inspect_value(path, offset, limit)
                }
                "m_SubsetIndices" => (self.m_subset_indices).inspect_value(path, offset, limit),
                "m_LightmapTilingOffsetDynamic" => {
                    (self.m_lightmap_tiling_offset_dynamic).inspect_value(path, offset, limit)
                }
                "m_StaticBatchRoot" => {
                    (self.m_static_batch_root).inspect_value(path, offset, limit)
                }
                "m_RenderingLayerMask" => {
                    (self.m_rendering_layer_mask).inspect_value(path, offset, limit)
                }
                "m_RayTracingMode" => (self.m_ray_tracing_mode).inspect_value(path, offset, limit),
                "m_UseLightProbes" => (self.m_use_light_probes).inspect_value(path, offset, limit),
                "m_RendererPriority" => {
                    (self.m_renderer_priority).inspect_value(path, offset, limit)
                }
                "m_RayTraceProcedural" => {
                    (self.m_ray_trace_procedural).inspect_value(path, offset, limit)
                }
                "m_LightProbeAnchor" => {
                    (self.m_light_probe_anchor).inspect_value(path, offset, limit)
                }
                "m_SortingLayer" => (self.m_sorting_layer).inspect_value(path, offset, limit),
                "m_SortingLayerID" => (self.m_sorting_layer_id).inspect_value(path, offset, limit),
                "m_RayTracingAccelStructBuildFlagsOverride" => (self
                    .m_ray_tracing_accel_struct_build_flags_override)
                    .inspect_value(path, offset, limit),
                "m_SortingOrder" => (self.m_sorting_order).inspect_value(path, offset, limit),
                "m_StaticBatchInfo" => {
                    (self.m_static_batch_info).inspect_value(path, offset, limit)
                }
                "m_RayTracingAccelStructBuildFlags" => {
                    (self.m_ray_tracing_accel_struct_build_flags).inspect_value(path, offset, limit)
                }
                "m_Sprite" => (self.m_sprite).inspect_value(path, offset, limit),
                "m_ProbeAnchor" => (self.m_probe_anchor).inspect_value(path, offset, limit),
                "m_SmallMeshCulling" => {
                    (self.m_small_mesh_culling).inspect_value(path, offset, limit)
                }
                "m_Color" => (self.m_color).inspect_value(path, offset, limit),
                "m_ForceMeshLod" => (self.m_force_mesh_lod).inspect_value(path, offset, limit),
                "m_LightProbeVolumeOverride" => {
                    (self.m_light_probe_volume_override).inspect_value(path, offset, limit)
                }
                "m_MeshLodSelectionBias" => {
                    (self.m_mesh_lod_selection_bias).inspect_value(path, offset, limit)
                }
                "m_FlipX" => (self.m_flip_x).inspect_value(path, offset, limit),
                "m_FlipY" => (self.m_flip_y).inspect_value(path, offset, limit),
                "m_DrawMode" => (self.m_draw_mode).inspect_value(path, offset, limit),
                "m_Size" => (self.m_size).inspect_value(path, offset, limit),
                "m_AdaptiveModeThreshold" => {
                    (self.m_adaptive_mode_threshold).inspect_value(path, offset, limit)
                }
                "m_SpriteTileMode" => (self.m_sprite_tile_mode).inspect_value(path, offset, limit),
                "m_WasSpriteAssigned" => {
                    (self.m_was_sprite_assigned).inspect_value(path, offset, limit)
                }
                "m_MaskInteraction" => (self.m_mask_interaction).inspect_value(path, offset, limit),
                "m_SpriteSortPoint" => {
                    (self.m_sprite_sort_point).inspect_value(path, offset, limit)
                }
                "m_BlendShapeWeights" => {
                    (self.m_blend_shape_weights).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_game_object).inspect_value(path, offset, limit))
                .map(|v| ("m_GameObject".to_string(), v)),
            ((self.m_enabled).inspect_value(path, offset, limit))
                .map(|v| ("m_Enabled".to_string(), v)),
            ((self.m_cast_shadows).inspect_value(path, offset, limit))
                .map(|v| ("m_CastShadows".to_string(), v)),
            ((self.m_receive_shadows).inspect_value(path, offset, limit))
                .map(|v| ("m_ReceiveShadows".to_string(), v)),
            ((self.m_lightmap_index).inspect_value(path, offset, limit))
                .map(|v| ("m_LightmapIndex".to_string(), v)),
            ((self.m_motion_vectors).inspect_value(path, offset, limit))
                .map(|v| ("m_MotionVectors".to_string(), v)),
            ((self.m_dynamic_occludee).inspect_value(path, offset, limit))
                .map(|v| ("m_DynamicOccludee".to_string(), v)),
            ((self.m_lightmap_tiling_offset).inspect_value(path, offset, limit))
                .map(|v| ("m_LightmapTilingOffset".to_string(), v)),
            ((self.m_lightmap_index_dynamic).inspect_value(path, offset, limit))
                .map(|v| ("m_LightmapIndexDynamic".to_string(), v)),
            ((self.m_light_probe_usage).inspect_value(path, offset, limit))
                .map(|v| ("m_LightProbeUsage".to_string(), v)),
            ((self.m_static_shadow_caster).inspect_value(path, offset, limit))
                .map(|v| ("m_StaticShadowCaster".to_string(), v)),
            ((self.m_materials).inspect_value(path, offset, limit))
                .map(|v| ("m_Materials".to_string(), v)),
            ((self.m_reflection_probe_usage).inspect_value(path, offset, limit))
                .map(|v| ("m_ReflectionProbeUsage".to_string(), v)),
            ((self.m_subset_indices).inspect_value(path, offset, limit))
                .map(|v| ("m_SubsetIndices".to_string(), v)),
            ((self.m_lightmap_tiling_offset_dynamic).inspect_value(path, offset, limit))
                .map(|v| ("m_LightmapTilingOffsetDynamic".to_string(), v)),
            ((self.m_static_batch_root).inspect_value(path, offset, limit))
                .map(|v| ("m_StaticBatchRoot".to_string(), v)),
            ((self.m_rendering_layer_mask).inspect_value(path, offset, limit))
                .map(|v| ("m_RenderingLayerMask".to_string(), v)),
            ((self.m_ray_tracing_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_RayTracingMode".to_string(), v)),
            ((self.m_use_light_probes).inspect_value(path, offset, limit))
                .map(|v| ("m_UseLightProbes".to_string(), v)),
            ((self.m_renderer_priority).inspect_value(path, offset, limit))
                .map(|v| ("m_RendererPriority".to_string(), v)),
            ((self.m_ray_trace_procedural).inspect_value(path, offset, limit))
                .map(|v| ("m_RayTraceProcedural".to_string(), v)),
            ((self.m_light_probe_anchor).inspect_value(path, offset, limit))
                .map(|v| ("m_LightProbeAnchor".to_string(), v)),
            ((self.m_sorting_layer).inspect_value(path, offset, limit))
                .map(|v| ("m_SortingLayer".to_string(), v)),
            ((self.m_sorting_layer_id).inspect_value(path, offset, limit))
                .map(|v| ("m_SortingLayerID".to_string(), v)),
            ((self.m_ray_tracing_accel_struct_build_flags_override)
                .inspect_value(path, offset, limit))
            .map(|v| ("m_RayTracingAccelStructBuildFlagsOverride".to_string(), v)),
            ((self.m_sorting_order).inspect_value(path, offset, limit))
                .map(|v| ("m_SortingOrder".to_string(), v)),
            ((self.m_static_batch_info).inspect_value(path, offset, limit))
                .map(|v| ("m_StaticBatchInfo".to_string(), v)),
            ((self.m_ray_tracing_accel_struct_build_flags).inspect_value(path, offset, limit))
                .map(|v| ("m_RayTracingAccelStructBuildFlags".to_string(), v)),
            ((self.m_sprite).inspect_value(path, offset, limit))
                .map(|v| ("m_Sprite".to_string(), v)),
            ((self.m_probe_anchor).inspect_value(path, offset, limit))
                .map(|v| ("m_ProbeAnchor".to_string(), v)),
            ((self.m_small_mesh_culling).inspect_value(path, offset, limit))
                .map(|v| ("m_SmallMeshCulling".to_string(), v)),
            ((self.m_color).inspect_value(path, offset, limit)).map(|v| ("m_Color".to_string(), v)),
            ((self.m_force_mesh_lod).inspect_value(path, offset, limit))
                .map(|v| ("m_ForceMeshLod".to_string(), v)),
            ((self.m_light_probe_volume_override).inspect_value(path, offset, limit))
                .map(|v| ("m_LightProbeVolumeOverride".to_string(), v)),
            ((self.m_mesh_lod_selection_bias).inspect_value(path, offset, limit))
                .map(|v| ("m_MeshLodSelectionBias".to_string(), v)),
            ((self.m_flip_x).inspect_value(path, offset, limit))
                .map(|v| ("m_FlipX".to_string(), v)),
            ((self.m_flip_y).inspect_value(path, offset, limit))
                .map(|v| ("m_FlipY".to_string(), v)),
            ((self.m_draw_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_DrawMode".to_string(), v)),
            ((self.m_size).inspect_value(path, offset, limit)).map(|v| ("m_Size".to_string(), v)),
            ((self.m_adaptive_mode_threshold).inspect_value(path, offset, limit))
                .map(|v| ("m_AdaptiveModeThreshold".to_string(), v)),
            ((self.m_sprite_tile_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_SpriteTileMode".to_string(), v)),
            ((self.m_was_sprite_assigned).inspect_value(path, offset, limit))
                .map(|v| ("m_WasSpriteAssigned".to_string(), v)),
            ((self.m_mask_interaction).inspect_value(path, offset, limit))
                .map(|v| ("m_MaskInteraction".to_string(), v)),
            ((self.m_sprite_sort_point).inspect_value(path, offset, limit))
                .map(|v| ("m_SpriteSortPoint".to_string(), v)),
            ((self.m_blend_shape_weights).inspect_value(path, offset, limit))
                .map(|v| ("m_BlendShapeWeights".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
