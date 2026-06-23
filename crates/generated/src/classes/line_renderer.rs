//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::line_parameters::LineParameters;
use crate::subclasses::static_batch_info::StaticBatchInfo;
use crate::subclasses::vector3f::Vector3f;
use crate::subclasses::vector4f::Vector4f;
use super::renderer::Renderer;

#[derive(Debug, Clone)]
pub struct LineRenderer {
    pub parent: Renderer,
    pub m_game_object: PPtr,
    pub m_enabled: bool,
    pub m_cast_shadows: bool,
    pub m_receive_shadows: bool,
    pub m_lightmap_index: u8,
    pub m_motion_vectors: Option<u8>,
    pub m_dynamic_occludee: Option<u8>,
    pub m_lightmap_tiling_offset: Vector4f,
    pub m_lightmap_index_dynamic: Option<u16>,
    pub m_light_probe_usage: Option<u8>,
    pub m_static_shadow_caster: Option<u8>,
    pub m_materials: Vec<PPtr>,
    pub m_reflection_probe_usage: Option<i32>,
    pub m_subset_indices: Option<Vec<u32>>,
    pub m_lightmap_tiling_offset_dynamic: Option<Vector4f>,
    pub m_static_batch_root: PPtr,
    pub m_rendering_layer_mask: Option<u32>,
    pub m_ray_tracing_mode: Option<u8>,
    pub m_positions: Vec<Vector3f>,
    pub m_use_light_probes: Option<bool>,
    pub m_renderer_priority: Option<i32>,
    pub m_ray_trace_procedural: Option<u8>,
    pub m_parameters: LineParameters,
    pub m_light_probe_anchor: Option<PPtr>,
    pub m_use_world_space: bool,
    pub m_sorting_layer: Option<i16>,
    pub m_sorting_layer_id: Option<u32>,
    pub m_ray_tracing_accel_struct_build_flags_override: Option<u8>,
    pub m_sorting_order: Option<i16>,
    pub m_static_batch_info: Option<StaticBatchInfo>,
    pub m_ray_tracing_accel_struct_build_flags: Option<u8>,
    pub m_probe_anchor: Option<PPtr>,
    pub m_small_mesh_culling: Option<u8>,
    pub m_force_mesh_lod: Option<i16>,
    pub m_light_probe_volume_override: Option<PPtr>,
    pub m_mesh_lod_selection_bias: Option<f32>,
    pub m_loop: Option<bool>,
    pub m_mask_interaction: Option<i32>,
    pub m_apply_active_color_space: Option<bool>,
}

impl crate::Parse for LineRenderer {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = Renderer::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_bool()?;
        let m_cast_shadows = r.read_bool()?;
        let m_receive_shadows = r.read_bool()?;
        let m_lightmap_index = r.read_u8()?;
        let m_motion_vectors = if *ver >= crate::UnityVersion::new(5, 4, 0) {
            Some(r.read_u8()?)
        } else {
            None
        };
        let m_dynamic_occludee = if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            Some(r.read_u8()?)
        } else {
            None
        };
        let m_lightmap_tiling_offset = Vector4f::parse(r, ver)?;
        let m_lightmap_index_dynamic = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_u16()?)
        } else {
            None
        };
        let m_light_probe_usage = if *ver >= crate::UnityVersion::new(5, 4, 0) {
            Some(r.read_u8()?)
        } else {
            None
        };
        let m_static_shadow_caster = if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            Some(r.read_u8()?)
        } else {
            None
        };
        let m_materials = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_reflection_probe_usage = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_subset_indices = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u32() }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };
        let m_lightmap_tiling_offset_dynamic = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(Vector4f::parse(r, ver)?)
        } else {
            None
        };
        let m_static_batch_root = crate::PPtr::parse(r, ver)?;
        let m_rendering_layer_mask = if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            Some(r.read_u32()?)
        } else {
            None
        };
        let m_ray_tracing_mode = if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            let v = r.read_u8()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_positions = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Vector3f::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_use_light_probes = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_renderer_priority = if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_ray_trace_procedural = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = r.read_u8()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_parameters = LineParameters::parse(r, ver)?;
        let m_light_probe_anchor = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(crate::PPtr::parse(r, ver)?)
        } else {
            None
        };
        let m_use_world_space = r.read_bool()?;
        let m_sorting_layer = if *ver >= crate::UnityVersion::new(4, 3, 0) {
            Some(r.read_i16()?)
        } else {
            None
        };
        let m_sorting_layer_id = if *ver >= crate::UnityVersion::new(4, 5, 0) {
            Some(r.read_u32()?)
        } else {
            None
        };
        let m_ray_tracing_accel_struct_build_flags_override = if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            Some(r.read_u8()?)
        } else {
            None
        };
        let m_sorting_order = if *ver >= crate::UnityVersion::new(4, 3, 0) {
            Some(r.read_i16()?)
        } else {
            None
        };
        let m_static_batch_info = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            Some(StaticBatchInfo::parse(r, ver)?)
        } else {
            None
        };
        let m_ray_tracing_accel_struct_build_flags = if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            let v = r.read_u8()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_probe_anchor = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = crate::PPtr::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_small_mesh_culling = if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            let v = r.read_u8()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_force_mesh_lod = if *ver >= crate::UnityVersion::new(6000, 2, 0) {
            let v = r.read_i16()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_light_probe_volume_override = if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = crate::PPtr::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_mesh_lod_selection_bias = if *ver >= crate::UnityVersion::new(6000, 2, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_loop = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_mask_interaction = if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_apply_active_color_space = if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };

        Ok(LineRenderer {
            parent,
            m_game_object,
            m_enabled,
            m_cast_shadows,
            m_receive_shadows,
            m_lightmap_index,
            m_motion_vectors,
            m_dynamic_occludee,
            m_lightmap_tiling_offset,
            m_lightmap_index_dynamic,
            m_light_probe_usage,
            m_static_shadow_caster,
            m_materials,
            m_reflection_probe_usage,
            m_subset_indices,
            m_lightmap_tiling_offset_dynamic,
            m_static_batch_root,
            m_rendering_layer_mask,
            m_ray_tracing_mode,
            m_positions,
            m_use_light_probes,
            m_renderer_priority,
            m_ray_trace_procedural,
            m_parameters,
            m_light_probe_anchor,
            m_use_world_space,
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
