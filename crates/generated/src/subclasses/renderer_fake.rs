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
