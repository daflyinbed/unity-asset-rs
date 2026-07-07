//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::classes::vector3f::Vector3f;
use crate::subclasses::static_batch_info::StaticBatchInfo;
use crate::subclasses::vector4f::Vector4f;

#[derive(Debug, Clone)]
pub struct ParticleSystemRenderer {
    pub m_game_object: PPtr,
    pub m_enabled: bool,
    pub m_cast_shadows: u8,
    pub m_receive_shadows: u8,
    pub m_lightmap_index: u16,
    pub m_motion_vectors: Option<u8>,
    pub m_dynamic_occludee: Option<u8>,
    pub m_lightmap_tiling_offset: Vector4f,
    pub m_lightmap_index_dynamic: Option<u16>,
    pub m_light_probe_usage: Option<u8>,
    pub m_static_shadow_caster: Option<u8>,
    pub m_materials: Vec<PPtr>,
    pub m_reflection_probe_usage: Option<u8>,
    pub m_subset_indices: Option<Vec<u32>>,
    pub m_lightmap_tiling_offset_dynamic: Option<Vector4f>,
    pub m_static_batch_root: PPtr,
    pub m_rendering_layer_mask: Option<u32>,
    pub m_ray_tracing_mode: Option<u8>,
    pub m_use_light_probes: Option<bool>,
    pub m_renderer_priority: Option<i32>,
    pub m_ray_trace_procedural: Option<u8>,
    pub m_light_probe_anchor: Option<PPtr>,
    pub m_render_mode: u16,
    pub m_sorting_layer: Option<i16>,
    pub m_sorting_layer_id: Option<i32>,
    pub m_ray_tracing_accel_struct_build_flags_override: Option<u8>,
    pub m_max_particle_size: f32,
    pub m_sorting_order: Option<i16>,
    pub m_static_batch_info: Option<StaticBatchInfo>,
    pub m_ray_tracing_accel_struct_build_flags: Option<u8>,
    pub m_camera_velocity_scale: f32,
    pub m_probe_anchor: Option<PPtr>,
    pub m_small_mesh_culling: Option<u8>,
    pub m_velocity_scale: f32,
    pub m_force_mesh_lod: Option<i16>,
    pub m_length_scale: f32,
    pub m_light_probe_volume_override: Option<PPtr>,
    pub m_mesh_lod_selection_bias: Option<f32>,
    pub m_sorting_fudge: f32,
    pub m_sort_mode: u8,
    pub m_normal_direction: Option<f32>,
    pub m_mesh: PPtr,
    pub m_min_particle_size: Option<f32>,
    pub m_mesh1: Option<PPtr>,
    pub m_mesh2: Option<PPtr>,
    pub m_mesh3: Option<PPtr>,
    pub m_render_alignment: Option<i32>,
    pub m_pivot: Option<Vector3f>,
    pub m_mesh_distribution: Option<u8>,
    pub m_use_custom_vertex_streams: Option<bool>,
    pub m_vertex_stream_mask: Option<i32>,
    pub m_vertex_streams: Option<Vec<u8>>,
    pub m_mask_interaction: Option<i32>,
    pub m_shadow_bias: Option<f32>,
    pub m_enable_gpu_instancing: Option<bool>,
    pub m_apply_active_color_space: Option<bool>,
    pub m_flip: Option<Vector3f>,
    pub m_allow_roll: Option<bool>,
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
        let mut m_vertex_streams = None;
        let mut m_mask_interaction = None;
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
            m_game_object = Some(crate::PPtr::parse(r, ver)?);
            m_enabled = Some(r.read_bool()?);
            m_cast_shadows = Some(r.read_u8()?);
            m_receive_shadows = Some(r.read_u8()?);
        if *ver >= crate::UnityVersion::new(6000, 2, 0) {
            m_dynamic_occludee = Some(r.read_u8()?);
            m_static_shadow_caster = Some(r.read_u8()?);
            m_motion_vectors = Some(r.read_u8()?);
            m_light_probe_usage = Some(r.read_u8()?);
            m_reflection_probe_usage = Some(r.read_u8()?);
            m_ray_tracing_mode = Some(r.read_u8()?);
            m_ray_trace_procedural = Some(r.read_u8()?);
            m_ray_tracing_accel_struct_build_flags_override = Some(r.read_u8()?);
            m_ray_tracing_accel_struct_build_flags = Some(r.read_u8()?);
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
            m_reflection_probe_usage = Some(r.read_u8()?);
            m_ray_tracing_mode = Some(r.read_u8()?);
            m_ray_trace_procedural = Some(r.read_u8()?);
            m_ray_tracing_accel_struct_build_flags_override = Some(r.read_u8()?);
            m_ray_tracing_accel_struct_build_flags = Some(r.read_u8()?);
            m_small_mesh_culling = Some(r.read_u8()?);
            r.align(4)?;
            m_rendering_layer_mask = Some(r.read_u32()?);
            m_renderer_priority = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            m_dynamic_occludee = Some(r.read_u8()?);
            m_static_shadow_caster = Some(r.read_u8()?);
            m_motion_vectors = Some(r.read_u8()?);
            m_light_probe_usage = Some(r.read_u8()?);
            m_reflection_probe_usage = Some(r.read_u8()?);
            m_ray_tracing_mode = Some(r.read_u8()?);
            m_ray_trace_procedural = Some(r.read_u8()?);
            m_ray_tracing_accel_struct_build_flags_override = Some(r.read_u8()?);
            m_ray_tracing_accel_struct_build_flags = Some(r.read_u8()?);
            m_rendering_layer_mask = Some(r.read_u32()?);
            m_renderer_priority = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            m_dynamic_occludee = Some(r.read_u8()?);
            m_static_shadow_caster = Some(r.read_u8()?);
            m_motion_vectors = Some(r.read_u8()?);
            m_light_probe_usage = Some(r.read_u8()?);
            m_reflection_probe_usage = Some(r.read_u8()?);
            m_ray_tracing_mode = Some(r.read_u8()?);
            m_ray_trace_procedural = Some(r.read_u8()?);
            m_rendering_layer_mask = Some(r.read_u32()?);
            m_renderer_priority = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            m_dynamic_occludee = Some(r.read_u8()?);
            m_motion_vectors = Some(r.read_u8()?);
            m_light_probe_usage = Some(r.read_u8()?);
            m_reflection_probe_usage = Some(r.read_u8()?);
            m_ray_tracing_mode = Some(r.read_u8()?);
            m_ray_trace_procedural = Some(r.read_u8()?);
            m_rendering_layer_mask = Some(r.read_u32()?);
            m_renderer_priority = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            m_dynamic_occludee = Some(r.read_u8()?);
            m_motion_vectors = Some(r.read_u8()?);
            m_light_probe_usage = Some(r.read_u8()?);
            m_reflection_probe_usage = Some(r.read_u8()?);
            m_ray_tracing_mode = Some(r.read_u8()?);
            m_rendering_layer_mask = Some(r.read_u32()?);
            m_renderer_priority = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            m_dynamic_occludee = Some(r.read_u8()?);
            m_motion_vectors = Some(r.read_u8()?);
            m_light_probe_usage = Some(r.read_u8()?);
            m_reflection_probe_usage = Some(r.read_u8()?);
            m_rendering_layer_mask = Some(r.read_u32()?);
            m_renderer_priority = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            m_dynamic_occludee = Some(r.read_u8()?);
            m_motion_vectors = Some(r.read_u8()?);
            m_light_probe_usage = Some(r.read_u8()?);
            m_reflection_probe_usage = Some(r.read_u8()?);
            m_rendering_layer_mask = Some(r.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_dynamic_occludee = Some(r.read_u8()?);
            m_motion_vectors = Some(r.read_u8()?);
            m_light_probe_usage = Some(r.read_u8()?);
            m_reflection_probe_usage = Some(r.read_u8()?);
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            m_motion_vectors = Some(r.read_u8()?);
            m_light_probe_usage = Some(r.read_u8()?);
            m_reflection_probe_usage = Some(r.read_u8()?);
        } else {
        }
            m_lightmap_index = Some(r.read_u16()?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_lightmap_index_dynamic = Some(r.read_u16()?);
        } else {
        }
            m_lightmap_tiling_offset = Some(Vector4f::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_lightmap_tiling_offset_dynamic = Some(Vector4f::parse(r, ver)?);
        } else {
        }
            m_materials = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            m_static_batch_info = Some(StaticBatchInfo::parse(r, ver)?);
        } else {
            m_subset_indices = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u32() }).collect::<Result<Vec<_>, _>>() } )?);
        }
            m_static_batch_root = Some(crate::PPtr::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(6000, 3, 0) {
            m_probe_anchor = Some(crate::PPtr::parse(r, ver)?);
            m_light_probe_volume_override = Some(crate::PPtr::parse(r, ver)?);
            r.align(4)?;
            m_sorting_layer_id = Some(r.read_i32()?);
            m_sorting_layer = Some(r.read_i16()?);
            m_sorting_order = Some(r.read_i16()?);
            r.align(4)?;
            m_mask_interaction = Some(r.read_i32()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_probe_anchor = Some(crate::PPtr::parse(r, ver)?);
            m_light_probe_volume_override = Some(crate::PPtr::parse(r, ver)?);
            r.align(4)?;
            m_sorting_layer_id = Some(r.read_i32()?);
            m_sorting_layer = Some(r.read_i16()?);
            m_sorting_order = Some(r.read_i16()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_probe_anchor = Some(crate::PPtr::parse(r, ver)?);
            m_light_probe_volume_override = Some(crate::PPtr::parse(r, ver)?);
            r.align(4)?;
            m_sorting_layer = Some(r.read_i16()?);
            m_sorting_order = Some(r.read_i16()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            m_probe_anchor = Some(crate::PPtr::parse(r, ver)?);
            m_light_probe_volume_override = Some(crate::PPtr::parse(r, ver)?);
            r.align(4)?;
            m_sorting_layer_id = Some(r.read_i32()?);
            m_sorting_order = Some(r.read_i16()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_use_light_probes = Some(r.read_bool()?);
            r.align(4)?;
            m_reflection_probe_usage = Some(r.read_u8()?);
            m_probe_anchor = Some(crate::PPtr::parse(r, ver)?);
            m_sorting_layer_id = Some(r.read_i32()?);
            m_sorting_order = Some(r.read_i16()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(4, 5, 0) {
            m_use_light_probes = Some(r.read_bool()?);
            r.align(4)?;
            m_light_probe_anchor = Some(crate::PPtr::parse(r, ver)?);
            r.align(4)?;
            m_sorting_layer_id = Some(r.read_i32()?);
            m_sorting_order = Some(r.read_i16()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_use_light_probes = Some(r.read_bool()?);
            r.align(4)?;
            m_light_probe_anchor = Some(crate::PPtr::parse(r, ver)?);
            r.align(4)?;
            m_sorting_layer = Some(r.read_i16()?);
            m_sorting_order = Some(r.read_i16()?);
            r.align(4)?;
        } else {
            m_use_light_probes = Some(r.read_bool()?);
            r.align(4)?;
            m_light_probe_anchor = Some(crate::PPtr::parse(r, ver)?);
            r.align(4)?;
        }
            m_render_mode = Some(r.read_u16()?);
        if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            m_mesh_distribution = Some(r.read_u8()?);
            m_sort_mode = Some(r.read_u8()?);
            m_min_particle_size = Some(r.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(5, 3, 0) {
            m_sort_mode = Some(r.read_u8()?);
            m_min_particle_size = Some(r.read_f32()?);
        } else {
        }
            m_max_particle_size = Some(r.read_f32()?);
            m_camera_velocity_scale = Some(r.read_f32()?);
            m_velocity_scale = Some(r.read_f32()?);
            m_length_scale = Some(r.read_f32()?);
            m_sorting_fudge = Some(r.read_f32()?);
        if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            m_normal_direction = Some(r.read_f32()?);
            m_shadow_bias = Some(r.read_f32()?);
            m_render_alignment = Some(r.read_i32()?);
            m_pivot = Some(Vector3f::parse(r, ver)?);
            m_flip = Some(Vector3f::parse(r, ver)?);
            m_enable_gpu_instancing = Some(r.read_bool()?);
            m_apply_active_color_space = Some(r.read_bool()?);
            m_allow_roll = Some(r.read_bool()?);
            m_freeform_stretching = Some(r.read_bool()?);
            m_rotate_with_stretch_direction = Some(r.read_bool()?);
            r.align(4)?;
            m_use_custom_vertex_streams = Some(r.read_bool()?);
            r.align(4)?;
            m_vertex_streams = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u8() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_use_custom_trail_vertex_streams = Some(r.read_bool()?);
            r.align(4)?;
            m_trail_vertex_streams = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u8() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            m_normal_direction = Some(r.read_f32()?);
            m_shadow_bias = Some(r.read_f32()?);
            m_render_alignment = Some(r.read_i32()?);
            m_pivot = Some(Vector3f::parse(r, ver)?);
            m_flip = Some(Vector3f::parse(r, ver)?);
            m_use_custom_vertex_streams = Some(r.read_bool()?);
            r.align(4)?;
            m_enable_gpu_instancing = Some(r.read_bool()?);
            m_apply_active_color_space = Some(r.read_bool()?);
            m_allow_roll = Some(r.read_bool()?);
            m_freeform_stretching = Some(r.read_bool()?);
            m_rotate_with_stretch_direction = Some(r.read_bool()?);
            r.align(4)?;
            m_vertex_streams = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u8() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            m_normal_direction = Some(r.read_f32()?);
            m_shadow_bias = Some(r.read_f32()?);
            m_render_alignment = Some(r.read_i32()?);
            m_pivot = Some(Vector3f::parse(r, ver)?);
            m_flip = Some(Vector3f::parse(r, ver)?);
            m_enable_gpu_instancing = Some(r.read_bool()?);
            m_apply_active_color_space = Some(r.read_bool()?);
            m_allow_roll = Some(r.read_bool()?);
            m_freeform_stretching = Some(r.read_bool()?);
            m_rotate_with_stretch_direction = Some(r.read_bool()?);
            r.align(4)?;
            m_use_custom_vertex_streams = Some(r.read_bool()?);
            r.align(4)?;
            m_vertex_streams = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u8() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_use_custom_trail_vertex_streams = Some(r.read_bool()?);
            r.align(4)?;
            m_trail_vertex_streams = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u8() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            m_normal_direction = Some(r.read_f32()?);
            m_shadow_bias = Some(r.read_f32()?);
            m_render_alignment = Some(r.read_i32()?);
            m_pivot = Some(Vector3f::parse(r, ver)?);
            m_flip = Some(Vector3f::parse(r, ver)?);
            m_use_custom_vertex_streams = Some(r.read_bool()?);
            r.align(4)?;
            m_enable_gpu_instancing = Some(r.read_bool()?);
            m_apply_active_color_space = Some(r.read_bool()?);
            m_allow_roll = Some(r.read_bool()?);
            m_freeform_stretching = Some(r.read_bool()?);
            m_rotate_with_stretch_direction = Some(r.read_bool()?);
            r.align(4)?;
            m_vertex_streams = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u8() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2023, 1, 16) {
            m_normal_direction = Some(r.read_f32()?);
            m_shadow_bias = Some(r.read_f32()?);
            m_render_alignment = Some(r.read_i32()?);
            m_pivot = Some(Vector3f::parse(r, ver)?);
            m_flip = Some(Vector3f::parse(r, ver)?);
            m_enable_gpu_instancing = Some(r.read_bool()?);
            m_apply_active_color_space = Some(r.read_bool()?);
            m_allow_roll = Some(r.read_bool()?);
            m_freeform_stretching = Some(r.read_bool()?);
            m_rotate_with_stretch_direction = Some(r.read_bool()?);
            r.align(4)?;
            m_use_custom_vertex_streams = Some(r.read_bool()?);
            r.align(4)?;
            m_vertex_streams = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u8() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_use_custom_trail_vertex_streams = Some(r.read_bool()?);
            r.align(4)?;
            m_trail_vertex_streams = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u8() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            m_normal_direction = Some(r.read_f32()?);
            m_shadow_bias = Some(r.read_f32()?);
            m_render_alignment = Some(r.read_i32()?);
            m_pivot = Some(Vector3f::parse(r, ver)?);
            m_flip = Some(Vector3f::parse(r, ver)?);
            m_use_custom_vertex_streams = Some(r.read_bool()?);
            r.align(4)?;
            m_enable_gpu_instancing = Some(r.read_bool()?);
            m_apply_active_color_space = Some(r.read_bool()?);
            m_allow_roll = Some(r.read_bool()?);
            m_freeform_stretching = Some(r.read_bool()?);
            m_rotate_with_stretch_direction = Some(r.read_bool()?);
            r.align(4)?;
            m_vertex_streams = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u8() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2022, 3, 11) {
            m_normal_direction = Some(r.read_f32()?);
            m_shadow_bias = Some(r.read_f32()?);
            m_render_alignment = Some(r.read_i32()?);
            m_pivot = Some(Vector3f::parse(r, ver)?);
            m_flip = Some(Vector3f::parse(r, ver)?);
            m_enable_gpu_instancing = Some(r.read_bool()?);
            m_apply_active_color_space = Some(r.read_bool()?);
            m_allow_roll = Some(r.read_bool()?);
            m_freeform_stretching = Some(r.read_bool()?);
            m_rotate_with_stretch_direction = Some(r.read_bool()?);
            r.align(4)?;
            m_use_custom_vertex_streams = Some(r.read_bool()?);
            r.align(4)?;
            m_vertex_streams = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u8() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_use_custom_trail_vertex_streams = Some(r.read_bool()?);
            r.align(4)?;
            m_trail_vertex_streams = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u8() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            m_normal_direction = Some(r.read_f32()?);
            m_shadow_bias = Some(r.read_f32()?);
            m_render_alignment = Some(r.read_i32()?);
            m_pivot = Some(Vector3f::parse(r, ver)?);
            m_flip = Some(Vector3f::parse(r, ver)?);
            m_use_custom_vertex_streams = Some(r.read_bool()?);
            r.align(4)?;
            m_enable_gpu_instancing = Some(r.read_bool()?);
            m_apply_active_color_space = Some(r.read_bool()?);
            m_allow_roll = Some(r.read_bool()?);
            m_freeform_stretching = Some(r.read_bool()?);
            m_rotate_with_stretch_direction = Some(r.read_bool()?);
            r.align(4)?;
            m_vertex_streams = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u8() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            m_normal_direction = Some(r.read_f32()?);
            m_shadow_bias = Some(r.read_f32()?);
            m_render_alignment = Some(r.read_i32()?);
            m_pivot = Some(Vector3f::parse(r, ver)?);
            m_flip = Some(Vector3f::parse(r, ver)?);
            m_use_custom_vertex_streams = Some(r.read_bool()?);
            r.align(4)?;
            m_enable_gpu_instancing = Some(r.read_bool()?);
            m_apply_active_color_space = Some(r.read_bool()?);
            m_allow_roll = Some(r.read_bool()?);
            m_vertex_streams = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u8() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            m_normal_direction = Some(r.read_f32()?);
            m_render_alignment = Some(r.read_i32()?);
            m_pivot = Some(Vector3f::parse(r, ver)?);
            m_use_custom_vertex_streams = Some(r.read_bool()?);
            r.align(4)?;
            m_enable_gpu_instancing = Some(r.read_bool()?);
            m_apply_active_color_space = Some(r.read_bool()?);
            m_vertex_streams = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u8() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            m_normal_direction = Some(r.read_f32()?);
            m_render_alignment = Some(r.read_i32()?);
            m_pivot = Some(Vector3f::parse(r, ver)?);
            m_use_custom_vertex_streams = Some(r.read_bool()?);
            r.align(4)?;
            m_enable_gpu_instancing = Some(r.read_bool()?);
            m_vertex_streams = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u8() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_normal_direction = Some(r.read_f32()?);
            m_render_alignment = Some(r.read_i32()?);
            m_pivot = Some(Vector3f::parse(r, ver)?);
            m_use_custom_vertex_streams = Some(r.read_bool()?);
            r.align(4)?;
            m_vertex_streams = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u8() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 5, 0) {
            m_normal_direction = Some(r.read_f32()?);
            m_render_alignment = Some(r.read_i32()?);
            m_pivot = Some(Vector3f::parse(r, ver)?);
            m_use_custom_vertex_streams = Some(r.read_bool()?);
            r.align(4)?;
            m_vertex_stream_mask = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(5, 3, 0) {
            m_normal_direction = Some(r.read_f32()?);
            m_render_alignment = Some(r.read_i32()?);
            m_pivot = Some(Vector3f::parse(r, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_normal_direction = Some(r.read_f32()?);
            m_sort_mode = Some(r.read_u8()?);
        } else {
            m_sort_mode = Some(r.read_u8()?);
        }
            m_mesh = Some(crate::PPtr::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(6000, 3, 0) {
            m_mesh1 = Some(crate::PPtr::parse(r, ver)?);
            m_mesh2 = Some(crate::PPtr::parse(r, ver)?);
            m_mesh3 = Some(crate::PPtr::parse(r, ver)?);
            m_mesh_weighting = Some(r.read_f32()?);
            m_mesh_weighting1 = Some(r.read_f32()?);
            m_mesh_weighting2 = Some(r.read_f32()?);
            m_mesh_weighting3 = Some(r.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            m_mesh1 = Some(crate::PPtr::parse(r, ver)?);
            m_mesh2 = Some(crate::PPtr::parse(r, ver)?);
            m_mesh3 = Some(crate::PPtr::parse(r, ver)?);
            m_mesh_weighting = Some(r.read_f32()?);
            m_mesh_weighting1 = Some(r.read_f32()?);
            m_mesh_weighting2 = Some(r.read_f32()?);
            m_mesh_weighting3 = Some(r.read_f32()?);
            m_mask_interaction = Some(r.read_i32()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_mesh1 = Some(crate::PPtr::parse(r, ver)?);
            m_mesh2 = Some(crate::PPtr::parse(r, ver)?);
            m_mesh3 = Some(crate::PPtr::parse(r, ver)?);
            m_mask_interaction = Some(r.read_i32()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_mesh1 = Some(crate::PPtr::parse(r, ver)?);
            m_mesh2 = Some(crate::PPtr::parse(r, ver)?);
            m_mesh3 = Some(crate::PPtr::parse(r, ver)?);
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
            m_vertex_streams,
            m_mask_interaction,
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
