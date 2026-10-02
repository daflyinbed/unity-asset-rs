//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::line_parameters::LineParameters;
use crate::subclasses::static_batch_info::StaticBatchInfo;
use crate::subclasses::vector3f::Vector3f;
use crate::subclasses::vector4f::Vector4f;

/// 版本枚举：RendererFakeMReflectionProbeUsageVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum RendererFakeMReflectionProbeUsageVariant {
    Default(u8),
    V2019_3(u8),
}

impl crate::Parse for RendererFakeMReflectionProbeUsageVariant {
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

impl crate::ToValue for RendererFakeMReflectionProbeUsageVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2019_3(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2019_3(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：RendererFakeMRayTracingModeVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum RendererFakeMRayTracingModeVariant {
    Default(u8),
    V2020_1(u8),
}

impl crate::Parse for RendererFakeMRayTracingModeVariant {
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

impl crate::ToValue for RendererFakeMRayTracingModeVariant {
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
#[derive(Debug, Clone)]
pub struct RendererFake {
    pub m_game_object: PPtr,
    pub m_enabled: bool,
    pub m_cast_shadows: u8,
    pub m_receive_shadows: u8,
    pub m_dynamic_occludee: u8,
    pub m_motion_vectors: u8,
    pub m_light_probe_usage: u8,
    pub m_reflection_probe_usage: RendererFakeMReflectionProbeUsageVariant,
    pub m_rendering_layer_mask: u32,
    pub m_ray_tracing_mode: Option<RendererFakeMRayTracingModeVariant>,
    pub m_renderer_priority: i32,
    pub m_ray_trace_procedural: Option<u8>,
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
    pub m_positions: Vec<Vector3f>,
    pub m_parameters: LineParameters,
    pub m_use_world_space: bool,
    pub m_loop: bool,
}

impl crate::Parse for RendererFake {
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
        let mut m_light_probe_usage = None;
        let mut m_reflection_probe_usage = None;
        let mut m_rendering_layer_mask = None;
        let mut m_ray_tracing_mode = None;
        let mut m_renderer_priority = None;
        let mut m_ray_trace_procedural = None;
        let mut m_lightmap_index = None;
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
        let mut m_positions = None;
        let mut m_parameters = None;
        let mut m_use_world_space = None;
        let mut m_loop = None;
        m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
        m_enabled = Some(rdr.read_bool()?);
        m_cast_shadows = Some(rdr.read_u8()?);
        m_receive_shadows = Some(rdr.read_u8()?);
        m_dynamic_occludee = Some(rdr.read_u8()?);
        m_motion_vectors = Some(rdr.read_u8()?);
        m_light_probe_usage = Some(rdr.read_u8()?);
        m_reflection_probe_usage = Some(RendererFakeMReflectionProbeUsageVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            m_ray_tracing_mode = Some(RendererFakeMRayTracingModeVariant::parse(rdr, ver)?);
            m_ray_trace_procedural = Some(rdr.read_u8()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            m_ray_tracing_mode = Some(RendererFakeMRayTracingModeVariant::parse(rdr, ver)?);
        } else {
        }
        m_rendering_layer_mask = Some(rdr.read_u32()?);
        m_renderer_priority = Some(rdr.read_i32()?);
        m_lightmap_index = Some(rdr.read_u16()?);
        m_lightmap_index_dynamic = Some(rdr.read_u16()?);
        m_lightmap_tiling_offset = Some(Vector4f::parse(rdr, ver)?);
        m_lightmap_tiling_offset_dynamic = Some(Vector4f::parse(rdr, ver)?);
        m_materials = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| crate::PPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
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
        m_positions = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| Vector3f::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        rdr.align(4)?;
        m_parameters = Some(LineParameters::parse(rdr, ver)?);
        m_use_world_space = Some(rdr.read_bool()?);
        m_loop = Some(rdr.read_bool()?);

        Ok(RendererFake {
            m_game_object: m_game_object.unwrap(),
            m_enabled: m_enabled.unwrap(),
            m_cast_shadows: m_cast_shadows.unwrap(),
            m_receive_shadows: m_receive_shadows.unwrap(),
            m_dynamic_occludee: m_dynamic_occludee.unwrap(),
            m_motion_vectors: m_motion_vectors.unwrap(),
            m_light_probe_usage: m_light_probe_usage.unwrap(),
            m_reflection_probe_usage: m_reflection_probe_usage.unwrap(),
            m_rendering_layer_mask: m_rendering_layer_mask.unwrap(),
            m_ray_tracing_mode,
            m_renderer_priority: m_renderer_priority.unwrap(),
            m_ray_trace_procedural,
            m_lightmap_index: m_lightmap_index.unwrap(),
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
            m_positions: m_positions.unwrap(),
            m_parameters: m_parameters.unwrap(),
            m_use_world_space: m_use_world_space.unwrap(),
            m_loop: m_loop.unwrap(),
        })
    }
}

impl crate::ToValue for RendererFake {
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
                "m_DynamicOccludee".to_string(),
                self.m_dynamic_occludee.to_value(),
            )),
            Some((
                "m_MotionVectors".to_string(),
                self.m_motion_vectors.to_value(),
            )),
            Some((
                "m_LightProbeUsage".to_string(),
                self.m_light_probe_usage.to_value(),
            )),
            Some((
                "m_ReflectionProbeUsage".to_string(),
                self.m_reflection_probe_usage.to_value(),
            )),
            Some((
                "m_RenderingLayerMask".to_string(),
                self.m_rendering_layer_mask.to_value(),
            )),
            crate::opt_field("m_RayTracingMode", &self.m_ray_tracing_mode),
            Some((
                "m_RendererPriority".to_string(),
                self.m_renderer_priority.to_value(),
            )),
            crate::opt_field("m_RayTraceProcedural", &self.m_ray_trace_procedural),
            Some((
                "m_LightmapIndex".to_string(),
                self.m_lightmap_index.to_value(),
            )),
            Some((
                "m_LightmapIndexDynamic".to_string(),
                self.m_lightmap_index_dynamic.to_value(),
            )),
            Some((
                "m_LightmapTilingOffset".to_string(),
                self.m_lightmap_tiling_offset.to_value(),
            )),
            Some((
                "m_LightmapTilingOffsetDynamic".to_string(),
                self.m_lightmap_tiling_offset_dynamic.to_value(),
            )),
            Some(("m_Materials".to_string(), self.m_materials.to_value())),
            Some((
                "m_StaticBatchInfo".to_string(),
                self.m_static_batch_info.to_value(),
            )),
            Some((
                "m_StaticBatchRoot".to_string(),
                self.m_static_batch_root.to_value(),
            )),
            Some(("m_ProbeAnchor".to_string(), self.m_probe_anchor.to_value())),
            Some((
                "m_LightProbeVolumeOverride".to_string(),
                self.m_light_probe_volume_override.to_value(),
            )),
            Some((
                "m_SortingLayerID".to_string(),
                self.m_sorting_layer_id.to_value(),
            )),
            Some((
                "m_SortingLayer".to_string(),
                self.m_sorting_layer.to_value(),
            )),
            Some((
                "m_SortingOrder".to_string(),
                self.m_sorting_order.to_value(),
            )),
            Some(("m_Positions".to_string(), self.m_positions.to_value())),
            Some(("m_Parameters".to_string(), self.m_parameters.to_value())),
            Some((
                "m_UseWorldSpace".to_string(),
                self.m_use_world_space.to_value(),
            )),
            Some(("m_Loop".to_string(), self.m_loop.to_value())),
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
                "m_DynamicOccludee" => (self.m_dynamic_occludee).inspect_value(path, offset, limit),
                "m_MotionVectors" => (self.m_motion_vectors).inspect_value(path, offset, limit),
                "m_LightProbeUsage" => {
                    (self.m_light_probe_usage).inspect_value(path, offset, limit)
                }
                "m_ReflectionProbeUsage" => {
                    (self.m_reflection_probe_usage).inspect_value(path, offset, limit)
                }
                "m_RenderingLayerMask" => {
                    (self.m_rendering_layer_mask).inspect_value(path, offset, limit)
                }
                "m_RayTracingMode" => (self.m_ray_tracing_mode).inspect_value(path, offset, limit),
                "m_RendererPriority" => {
                    (self.m_renderer_priority).inspect_value(path, offset, limit)
                }
                "m_RayTraceProcedural" => {
                    (self.m_ray_trace_procedural).inspect_value(path, offset, limit)
                }
                "m_LightmapIndex" => (self.m_lightmap_index).inspect_value(path, offset, limit),
                "m_LightmapIndexDynamic" => {
                    (self.m_lightmap_index_dynamic).inspect_value(path, offset, limit)
                }
                "m_LightmapTilingOffset" => {
                    (self.m_lightmap_tiling_offset).inspect_value(path, offset, limit)
                }
                "m_LightmapTilingOffsetDynamic" => {
                    (self.m_lightmap_tiling_offset_dynamic).inspect_value(path, offset, limit)
                }
                "m_Materials" => (self.m_materials).inspect_value(path, offset, limit),
                "m_StaticBatchInfo" => {
                    (self.m_static_batch_info).inspect_value(path, offset, limit)
                }
                "m_StaticBatchRoot" => {
                    (self.m_static_batch_root).inspect_value(path, offset, limit)
                }
                "m_ProbeAnchor" => (self.m_probe_anchor).inspect_value(path, offset, limit),
                "m_LightProbeVolumeOverride" => {
                    (self.m_light_probe_volume_override).inspect_value(path, offset, limit)
                }
                "m_SortingLayerID" => (self.m_sorting_layer_id).inspect_value(path, offset, limit),
                "m_SortingLayer" => (self.m_sorting_layer).inspect_value(path, offset, limit),
                "m_SortingOrder" => (self.m_sorting_order).inspect_value(path, offset, limit),
                "m_Positions" => (self.m_positions).inspect_value(path, offset, limit),
                "m_Parameters" => (self.m_parameters).inspect_value(path, offset, limit),
                "m_UseWorldSpace" => (self.m_use_world_space).inspect_value(path, offset, limit),
                "m_Loop" => (self.m_loop).inspect_value(path, offset, limit),
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
            ((self.m_dynamic_occludee).inspect_value(path, offset, limit))
                .map(|v| ("m_DynamicOccludee".to_string(), v)),
            ((self.m_motion_vectors).inspect_value(path, offset, limit))
                .map(|v| ("m_MotionVectors".to_string(), v)),
            ((self.m_light_probe_usage).inspect_value(path, offset, limit))
                .map(|v| ("m_LightProbeUsage".to_string(), v)),
            ((self.m_reflection_probe_usage).inspect_value(path, offset, limit))
                .map(|v| ("m_ReflectionProbeUsage".to_string(), v)),
            ((self.m_rendering_layer_mask).inspect_value(path, offset, limit))
                .map(|v| ("m_RenderingLayerMask".to_string(), v)),
            ((self.m_ray_tracing_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_RayTracingMode".to_string(), v)),
            ((self.m_renderer_priority).inspect_value(path, offset, limit))
                .map(|v| ("m_RendererPriority".to_string(), v)),
            ((self.m_ray_trace_procedural).inspect_value(path, offset, limit))
                .map(|v| ("m_RayTraceProcedural".to_string(), v)),
            ((self.m_lightmap_index).inspect_value(path, offset, limit))
                .map(|v| ("m_LightmapIndex".to_string(), v)),
            ((self.m_lightmap_index_dynamic).inspect_value(path, offset, limit))
                .map(|v| ("m_LightmapIndexDynamic".to_string(), v)),
            ((self.m_lightmap_tiling_offset).inspect_value(path, offset, limit))
                .map(|v| ("m_LightmapTilingOffset".to_string(), v)),
            ((self.m_lightmap_tiling_offset_dynamic).inspect_value(path, offset, limit))
                .map(|v| ("m_LightmapTilingOffsetDynamic".to_string(), v)),
            ((self.m_materials).inspect_value(path, offset, limit))
                .map(|v| ("m_Materials".to_string(), v)),
            ((self.m_static_batch_info).inspect_value(path, offset, limit))
                .map(|v| ("m_StaticBatchInfo".to_string(), v)),
            ((self.m_static_batch_root).inspect_value(path, offset, limit))
                .map(|v| ("m_StaticBatchRoot".to_string(), v)),
            ((self.m_probe_anchor).inspect_value(path, offset, limit))
                .map(|v| ("m_ProbeAnchor".to_string(), v)),
            ((self.m_light_probe_volume_override).inspect_value(path, offset, limit))
                .map(|v| ("m_LightProbeVolumeOverride".to_string(), v)),
            ((self.m_sorting_layer_id).inspect_value(path, offset, limit))
                .map(|v| ("m_SortingLayerID".to_string(), v)),
            ((self.m_sorting_layer).inspect_value(path, offset, limit))
                .map(|v| ("m_SortingLayer".to_string(), v)),
            ((self.m_sorting_order).inspect_value(path, offset, limit))
                .map(|v| ("m_SortingOrder".to_string(), v)),
            ((self.m_positions).inspect_value(path, offset, limit))
                .map(|v| ("m_Positions".to_string(), v)),
            ((self.m_parameters).inspect_value(path, offset, limit))
                .map(|v| ("m_Parameters".to_string(), v)),
            ((self.m_use_world_space).inspect_value(path, offset, limit))
                .map(|v| ("m_UseWorldSpace".to_string(), v)),
            ((self.m_loop).inspect_value(path, offset, limit)).map(|v| ("m_Loop".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
