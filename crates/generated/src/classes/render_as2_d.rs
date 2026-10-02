//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::static_batch_info::StaticBatchInfo;
use crate::subclasses::vector4f::Vector4f;

#[derive(Debug, Clone)]
pub struct RenderAs2D {
    pub m_game_object: PPtr,
    pub m_enabled: bool,
    pub m_cast_shadows: u8,
    pub m_receive_shadows: u8,
    pub m_dynamic_occludee: u8,
    pub m_static_shadow_caster: u8,
    pub m_motion_vectors: u8,
    pub m_light_probe_usage: u8,
    pub m_reflection_probe_usage: u8,
    pub m_ray_tracing_mode: u8,
    pub m_ray_trace_procedural: u8,
    pub m_ray_tracing_accel_struct_build_flags_override: u8,
    pub m_ray_tracing_accel_struct_build_flags: u8,
    pub m_small_mesh_culling: u8,
    pub m_force_mesh_lod: i16,
    pub m_mesh_lod_selection_bias: f32,
    pub m_rendering_layer_mask: u32,
    pub m_renderer_priority: i32,
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
    pub m_mask_interaction: i32,
    pub m_owning_sorting_group: PPtr,
}

impl crate::Parse for RenderAs2D {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(rdr, ver)?;
        let m_enabled = rdr.read_bool()?;
        let m_cast_shadows = rdr.read_u8()?;
        let m_receive_shadows = rdr.read_u8()?;
        let m_dynamic_occludee = rdr.read_u8()?;
        let m_static_shadow_caster = rdr.read_u8()?;
        let m_motion_vectors = rdr.read_u8()?;
        let m_light_probe_usage = rdr.read_u8()?;
        let m_reflection_probe_usage = rdr.read_u8()?;
        let m_ray_tracing_mode = rdr.read_u8()?;
        let m_ray_trace_procedural = rdr.read_u8()?;
        let m_ray_tracing_accel_struct_build_flags_override = rdr.read_u8()?;
        let m_ray_tracing_accel_struct_build_flags = rdr.read_u8()?;
        let m_small_mesh_culling = rdr.read_u8()?;
        rdr.align(4)?;
        let m_force_mesh_lod = rdr.read_i16()?;
        rdr.align(4)?;
        let m_mesh_lod_selection_bias = rdr.read_f32()?;
        let m_rendering_layer_mask = rdr.read_u32()?;
        let m_renderer_priority = rdr.read_i32()?;
        let m_lightmap_index = rdr.read_u16()?;
        let m_lightmap_index_dynamic = rdr.read_u16()?;
        let m_lightmap_tiling_offset = Vector4f::parse(rdr, ver)?;
        let m_lightmap_tiling_offset_dynamic = Vector4f::parse(rdr, ver)?;
        let m_materials = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| crate::PPtr::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let m_static_batch_info = StaticBatchInfo::parse(rdr, ver)?;
        let m_static_batch_root = crate::PPtr::parse(rdr, ver)?;
        let m_probe_anchor = crate::PPtr::parse(rdr, ver)?;
        let m_light_probe_volume_override = crate::PPtr::parse(rdr, ver)?;
        rdr.align(4)?;
        let m_sorting_layer_id = rdr.read_i32()?;
        let m_sorting_layer = rdr.read_i16()?;
        let m_sorting_order = rdr.read_i16()?;
        rdr.align(4)?;
        let m_mask_interaction = rdr.read_i32()?;
        rdr.align(4)?;
        let m_owning_sorting_group = crate::PPtr::parse(rdr, ver)?;

        Ok(RenderAs2D {
            m_game_object,
            m_enabled,
            m_cast_shadows,
            m_receive_shadows,
            m_dynamic_occludee,
            m_static_shadow_caster,
            m_motion_vectors,
            m_light_probe_usage,
            m_reflection_probe_usage,
            m_ray_tracing_mode,
            m_ray_trace_procedural,
            m_ray_tracing_accel_struct_build_flags_override,
            m_ray_tracing_accel_struct_build_flags,
            m_small_mesh_culling,
            m_force_mesh_lod,
            m_mesh_lod_selection_bias,
            m_rendering_layer_mask,
            m_renderer_priority,
            m_lightmap_index,
            m_lightmap_index_dynamic,
            m_lightmap_tiling_offset,
            m_lightmap_tiling_offset_dynamic,
            m_materials,
            m_static_batch_info,
            m_static_batch_root,
            m_probe_anchor,
            m_light_probe_volume_override,
            m_sorting_layer_id,
            m_sorting_layer,
            m_sorting_order,
            m_mask_interaction,
            m_owning_sorting_group,
        })
    }
}

impl crate::ToValue for RenderAs2D {
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
                "m_StaticShadowCaster".to_string(),
                self.m_static_shadow_caster.to_value(),
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
                "m_RayTracingMode".to_string(),
                self.m_ray_tracing_mode.to_value(),
            )),
            Some((
                "m_RayTraceProcedural".to_string(),
                self.m_ray_trace_procedural.to_value(),
            )),
            Some((
                "m_RayTracingAccelStructBuildFlagsOverride".to_string(),
                self.m_ray_tracing_accel_struct_build_flags_override
                    .to_value(),
            )),
            Some((
                "m_RayTracingAccelStructBuildFlags".to_string(),
                self.m_ray_tracing_accel_struct_build_flags.to_value(),
            )),
            Some((
                "m_SmallMeshCulling".to_string(),
                self.m_small_mesh_culling.to_value(),
            )),
            Some((
                "m_ForceMeshLod".to_string(),
                self.m_force_mesh_lod.to_value(),
            )),
            Some((
                "m_MeshLodSelectionBias".to_string(),
                self.m_mesh_lod_selection_bias.to_value(),
            )),
            Some((
                "m_RenderingLayerMask".to_string(),
                self.m_rendering_layer_mask.to_value(),
            )),
            Some((
                "m_RendererPriority".to_string(),
                self.m_renderer_priority.to_value(),
            )),
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
            Some((
                "m_MaskInteraction".to_string(),
                self.m_mask_interaction.to_value(),
            )),
            Some((
                "m_OwningSortingGroup".to_string(),
                self.m_owning_sorting_group.to_value(),
            )),
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
                "m_StaticShadowCaster" => {
                    (self.m_static_shadow_caster).inspect_value(path, offset, limit)
                }
                "m_MotionVectors" => (self.m_motion_vectors).inspect_value(path, offset, limit),
                "m_LightProbeUsage" => {
                    (self.m_light_probe_usage).inspect_value(path, offset, limit)
                }
                "m_ReflectionProbeUsage" => {
                    (self.m_reflection_probe_usage).inspect_value(path, offset, limit)
                }
                "m_RayTracingMode" => (self.m_ray_tracing_mode).inspect_value(path, offset, limit),
                "m_RayTraceProcedural" => {
                    (self.m_ray_trace_procedural).inspect_value(path, offset, limit)
                }
                "m_RayTracingAccelStructBuildFlagsOverride" => (self
                    .m_ray_tracing_accel_struct_build_flags_override)
                    .inspect_value(path, offset, limit),
                "m_RayTracingAccelStructBuildFlags" => {
                    (self.m_ray_tracing_accel_struct_build_flags).inspect_value(path, offset, limit)
                }
                "m_SmallMeshCulling" => {
                    (self.m_small_mesh_culling).inspect_value(path, offset, limit)
                }
                "m_ForceMeshLod" => (self.m_force_mesh_lod).inspect_value(path, offset, limit),
                "m_MeshLodSelectionBias" => {
                    (self.m_mesh_lod_selection_bias).inspect_value(path, offset, limit)
                }
                "m_RenderingLayerMask" => {
                    (self.m_rendering_layer_mask).inspect_value(path, offset, limit)
                }
                "m_RendererPriority" => {
                    (self.m_renderer_priority).inspect_value(path, offset, limit)
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
                "m_MaskInteraction" => (self.m_mask_interaction).inspect_value(path, offset, limit),
                "m_OwningSortingGroup" => {
                    (self.m_owning_sorting_group).inspect_value(path, offset, limit)
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
            ((self.m_dynamic_occludee).inspect_value(path, offset, limit))
                .map(|v| ("m_DynamicOccludee".to_string(), v)),
            ((self.m_static_shadow_caster).inspect_value(path, offset, limit))
                .map(|v| ("m_StaticShadowCaster".to_string(), v)),
            ((self.m_motion_vectors).inspect_value(path, offset, limit))
                .map(|v| ("m_MotionVectors".to_string(), v)),
            ((self.m_light_probe_usage).inspect_value(path, offset, limit))
                .map(|v| ("m_LightProbeUsage".to_string(), v)),
            ((self.m_reflection_probe_usage).inspect_value(path, offset, limit))
                .map(|v| ("m_ReflectionProbeUsage".to_string(), v)),
            ((self.m_ray_tracing_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_RayTracingMode".to_string(), v)),
            ((self.m_ray_trace_procedural).inspect_value(path, offset, limit))
                .map(|v| ("m_RayTraceProcedural".to_string(), v)),
            ((self.m_ray_tracing_accel_struct_build_flags_override)
                .inspect_value(path, offset, limit))
            .map(|v| ("m_RayTracingAccelStructBuildFlagsOverride".to_string(), v)),
            ((self.m_ray_tracing_accel_struct_build_flags).inspect_value(path, offset, limit))
                .map(|v| ("m_RayTracingAccelStructBuildFlags".to_string(), v)),
            ((self.m_small_mesh_culling).inspect_value(path, offset, limit))
                .map(|v| ("m_SmallMeshCulling".to_string(), v)),
            ((self.m_force_mesh_lod).inspect_value(path, offset, limit))
                .map(|v| ("m_ForceMeshLod".to_string(), v)),
            ((self.m_mesh_lod_selection_bias).inspect_value(path, offset, limit))
                .map(|v| ("m_MeshLodSelectionBias".to_string(), v)),
            ((self.m_rendering_layer_mask).inspect_value(path, offset, limit))
                .map(|v| ("m_RenderingLayerMask".to_string(), v)),
            ((self.m_renderer_priority).inspect_value(path, offset, limit))
                .map(|v| ("m_RendererPriority".to_string(), v)),
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
            ((self.m_mask_interaction).inspect_value(path, offset, limit))
                .map(|v| ("m_MaskInteraction".to_string(), v)),
            ((self.m_owning_sorting_group).inspect_value(path, offset, limit))
                .map(|v| ("m_OwningSortingGroup".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
