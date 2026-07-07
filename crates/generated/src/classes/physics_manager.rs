//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::aabb::Aabb;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct PhysicsManager {
    pub m_gravity: Vector3f,
    pub m_default_material: PPtr,
    pub m_bounce_threshold: f32,
    pub m_sleep_velocity: Option<f32>,
    pub m_sleep_threshold: Option<f32>,
    pub m_default_max_depenetration_velocity: Option<f32>,
    pub m_sleep_angular_velocity: Option<f32>,
    pub m_default_contact_offset: Option<f32>,
    pub m_max_angular_velocity: Option<f32>,
    pub m_solver_iteration_count: Option<i32>,
    pub m_default_solver_iterations: Option<i32>,
    pub m_min_penetration_for_penalty: Option<f32>,
    pub m_raycasts_hit_triggers: Option<bool>,
    pub m_queries_hit_triggers: Option<bool>,
    pub m_solver_velocity_iterations: Option<i32>,
    pub m_default_solver_velocity_iterations: Option<i32>,
    pub m_enable_adaptive_force: Option<bool>,
    pub m_queries_hit_backfaces: Option<bool>,
    pub m_layer_collision_matrix: Vec<u32>,
    pub m_enable_pcm: Option<bool>,
    pub m_cloth_inter_collision_distance: Option<f32>,
    pub m_cloth_inter_collision_stiffness: Option<f32>,
    pub m_auto_simulation: Option<bool>,
    pub m_contacts_generation: Option<i32>,
    pub m_auto_sync_transforms: Option<bool>,
    pub m_simulation_mode: Option<i32>,
    pub m_cloth_inter_collision_settings_toggle: Option<bool>,
    pub m_reuse_collision_callbacks: Option<bool>,
    pub m_contact_pairs_mode: Option<i32>,
    pub m_invoke_collision_callbacks: Option<bool>,
    pub m_broadphase_type: Option<i32>,
    pub m_cloth_gravity: Option<Vector3f>,
    pub m_world_bounds: Option<Aabb>,
    pub m_world_subdivisions: Option<i32>,
    pub m_friction_type: Option<i32>,
    pub m_enable_enhanced_determinism: Option<bool>,
    pub m_enable_unified_heightmaps: Option<bool>,
    pub m_improved_patch_friction: Option<bool>,
    pub m_default_max_angluar_speed: Option<f32>,
    pub m_solver_type: Option<i32>,
    pub m_generate_on_trigger_stay_events: Option<bool>,
    pub m_default_max_angular_speed: Option<f32>,
    pub m_scratch_buffer_chunk_count: Option<u32>,
    pub m_current_backend_id: Option<u32>,
    pub m_fast_motion_threshold: Option<f32>,
    pub m_scene_buffers_release_interval: Option<u32>,
    pub m_release_scene_buffers: Option<bool>,
    pub m_log_verbosity: Option<i32>,
    pub m_incremental_static_broadphase: Option<bool>,
}

impl crate::Parse for PhysicsManager {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_gravity = Vector3f::parse(r, ver)?;
        let m_default_material = crate::PPtr::parse(r, ver)?;
        let m_bounce_threshold = r.read_f32()?;
        let m_sleep_velocity = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_sleep_threshold = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_default_max_depenetration_velocity = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_sleep_angular_velocity = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_default_contact_offset = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_max_angular_velocity = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_solver_iteration_count = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_default_solver_iterations = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_min_penetration_for_penalty = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_raycasts_hit_triggers = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_queries_hit_triggers = if *ver >= crate::UnityVersion::new(5, 2, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_solver_velocity_iterations = if *ver >= crate::UnityVersion::new(5, 4, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_default_solver_velocity_iterations = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_enable_adaptive_force = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_queries_hit_backfaces = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_layer_collision_matrix = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u32() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_enable_pcm = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_cloth_inter_collision_distance = if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_cloth_inter_collision_stiffness = if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            let v = r.read_f32()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_auto_simulation = if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_contacts_generation = if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            let v = r.read_i32()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_auto_sync_transforms = if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_simulation_mode = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            let v = r.read_i32()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_cloth_inter_collision_settings_toggle = if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_reuse_collision_callbacks = if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_contact_pairs_mode = if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_invoke_collision_callbacks = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_broadphase_type = if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_cloth_gravity = if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            Some(Vector3f::parse(r, ver)?)
        } else {
            None
        };
        let m_world_bounds = if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            Some(Aabb::parse(r, ver)?)
        } else {
            None
        };
        let m_world_subdivisions = if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            let v = r.read_i32()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_friction_type = if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_enable_enhanced_determinism = if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_enable_unified_heightmaps = if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_improved_patch_friction = if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_default_max_angluar_speed = if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_solver_type = if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            let v = r.read_i32()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_generate_on_trigger_stay_events = if *ver >= crate::UnityVersion::new(6000, 0, 50) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_default_max_angular_speed = if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_scratch_buffer_chunk_count = if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            Some(r.read_u32()?)
        } else {
            None
        };
        let m_current_backend_id = if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            Some(r.read_u32()?)
        } else {
            None
        };
        let m_fast_motion_threshold = if *ver >= crate::UnityVersion::new(2022, 3, 21) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_scene_buffers_release_interval = if *ver >= crate::UnityVersion::new(6000, 0, 65) {
            Some(r.read_u32()?)
        } else {
            None
        };
        let m_release_scene_buffers = if *ver >= crate::UnityVersion::new(6000, 0, 65) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_log_verbosity = if *ver >= crate::UnityVersion::new(6000, 0, 65) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_incremental_static_broadphase = if *ver >= crate::UnityVersion::new(6000, 0, 67) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(PhysicsManager {
            m_gravity,
            m_default_material,
            m_bounce_threshold,
            m_sleep_velocity,
            m_sleep_threshold,
            m_default_max_depenetration_velocity,
            m_sleep_angular_velocity,
            m_default_contact_offset,
            m_max_angular_velocity,
            m_solver_iteration_count,
            m_default_solver_iterations,
            m_min_penetration_for_penalty,
            m_raycasts_hit_triggers,
            m_queries_hit_triggers,
            m_solver_velocity_iterations,
            m_default_solver_velocity_iterations,
            m_enable_adaptive_force,
            m_queries_hit_backfaces,
            m_layer_collision_matrix,
            m_enable_pcm,
            m_cloth_inter_collision_distance,
            m_cloth_inter_collision_stiffness,
            m_auto_simulation,
            m_contacts_generation,
            m_auto_sync_transforms,
            m_simulation_mode,
            m_cloth_inter_collision_settings_toggle,
            m_reuse_collision_callbacks,
            m_contact_pairs_mode,
            m_invoke_collision_callbacks,
            m_broadphase_type,
            m_cloth_gravity,
            m_world_bounds,
            m_world_subdivisions,
            m_friction_type,
            m_enable_enhanced_determinism,
            m_enable_unified_heightmaps,
            m_improved_patch_friction,
            m_default_max_angluar_speed,
            m_solver_type,
            m_generate_on_trigger_stay_events,
            m_default_max_angular_speed,
            m_scratch_buffer_chunk_count,
            m_current_backend_id,
            m_fast_motion_threshold,
            m_scene_buffers_release_interval,
            m_release_scene_buffers,
            m_log_verbosity,
            m_incremental_static_broadphase,
        })
    }
}
