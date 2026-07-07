//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::static_batch_info::StaticBatchInfo;
use crate::subclasses::uv_animation::UvAnimation;
use crate::subclasses::vector4f::Vector4f;

#[derive(Debug, Clone)]
pub struct ParticleRenderer {
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
    pub m_materials: Vec<PPtr>,
    pub m_reflection_probe_usage: Option<u8>,
    pub m_subset_indices: Option<Vec<u32>>,
    pub m_lightmap_tiling_offset_dynamic: Option<Vector4f>,
    pub m_static_batch_root: PPtr,
    pub m_rendering_layer_mask: Option<u32>,
    pub m_camera_velocity_scale: f32,
    pub m_use_light_probes: Option<bool>,
    pub m_stretch_particles: i32,
    pub m_light_probe_anchor: Option<PPtr>,
    pub m_length_scale: f32,
    pub m_sorting_layer: Option<i16>,
    pub m_sorting_layer_id: Option<i32>,
    pub m_velocity_scale: f32,
    pub m_sorting_order: Option<i16>,
    pub m_static_batch_info: Option<StaticBatchInfo>,
    pub m_max_particle_size: f32,
    pub m_probe_anchor: Option<PPtr>,
    pub uv_animation: UvAnimation,
    pub m_light_probe_volume_override: Option<PPtr>,
}

impl crate::Parse for ParticleRenderer {
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
        let mut m_materials = None;
        let mut m_reflection_probe_usage = None;
        let mut m_subset_indices = None;
        let mut m_lightmap_tiling_offset_dynamic = None;
        let mut m_static_batch_root = None;
        let mut m_rendering_layer_mask = None;
        let mut m_camera_velocity_scale = None;
        let mut m_use_light_probes = None;
        let mut m_stretch_particles = None;
        let mut m_light_probe_anchor = None;
        let mut m_length_scale = None;
        let mut m_sorting_layer = None;
        let mut m_sorting_layer_id = None;
        let mut m_velocity_scale = None;
        let mut m_sorting_order = None;
        let mut m_static_batch_info = None;
        let mut m_max_particle_size = None;
        let mut m_probe_anchor = None;
        let mut uv_animation = None;
        let mut m_light_probe_volume_override = None;
            m_game_object = Some(crate::PPtr::parse(r, ver)?);
            m_enabled = Some(r.read_bool()?);
            m_cast_shadows = Some(r.read_u8()?);
            m_receive_shadows = Some(r.read_u8()?);
        if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            m_dynamic_occludee = Some(r.read_u8()?);
            m_motion_vectors = Some(r.read_u8()?);
            m_light_probe_usage = Some(r.read_u8()?);
            m_reflection_probe_usage = Some(r.read_u8()?);
            r.align(4)?;
            m_rendering_layer_mask = Some(r.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_dynamic_occludee = Some(r.read_u8()?);
            m_motion_vectors = Some(r.read_u8()?);
            m_light_probe_usage = Some(r.read_u8()?);
            m_reflection_probe_usage = Some(r.read_u8()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            m_motion_vectors = Some(r.read_u8()?);
            m_light_probe_usage = Some(r.read_u8()?);
            m_reflection_probe_usage = Some(r.read_u8()?);
            r.align(4)?;
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
        if *ver >= crate::UnityVersion::new(5, 6, 0) {
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
            r.align(4)?;
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
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_use_light_probes = Some(r.read_bool()?);
            r.align(4)?;
            m_light_probe_anchor = Some(crate::PPtr::parse(r, ver)?);
            r.align(4)?;
        } else {
        }
            m_camera_velocity_scale = Some(r.read_f32()?);
            m_stretch_particles = Some(r.read_i32()?);
            m_length_scale = Some(r.read_f32()?);
            m_velocity_scale = Some(r.read_f32()?);
            m_max_particle_size = Some(r.read_f32()?);
            uv_animation = Some(UvAnimation::parse(r, ver)?);

        Ok(ParticleRenderer {
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
            m_materials: m_materials.unwrap(),
            m_reflection_probe_usage,
            m_subset_indices,
            m_lightmap_tiling_offset_dynamic,
            m_static_batch_root: m_static_batch_root.unwrap(),
            m_rendering_layer_mask,
            m_camera_velocity_scale: m_camera_velocity_scale.unwrap(),
            m_use_light_probes,
            m_stretch_particles: m_stretch_particles.unwrap(),
            m_light_probe_anchor,
            m_length_scale: m_length_scale.unwrap(),
            m_sorting_layer,
            m_sorting_layer_id,
            m_velocity_scale: m_velocity_scale.unwrap(),
            m_sorting_order,
            m_static_batch_info,
            m_max_particle_size: m_max_particle_size.unwrap(),
            m_probe_anchor,
            uv_animation: uv_animation.unwrap(),
            m_light_probe_volume_override,
        })
    }
}
