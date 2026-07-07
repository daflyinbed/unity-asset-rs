//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::bit_field::BitField;
use crate::subclasses::physics_job_options2_d::PhysicsJobOptions2D;
use crate::subclasses::vector2f::Vector2f;

#[derive(Debug, Clone)]
pub struct Physics2DSettings {
    pub m_gravity: Vector2f,
    pub m_default_material: PPtr,
    pub m_velocity_iterations: i32,
    pub m_position_iterations: i32,
    pub m_raycasts_hit_triggers: Option<bool>,
    pub m_velocity_threshold: Option<f32>,
    pub m_bounce_threshold: Option<f32>,
    pub m_layer_collision_matrix: Vec<u32>,
    pub m_max_linear_correction: Option<f32>,
    pub m_max_angular_correction: Option<f32>,
    pub m_max_translation_speed: Option<f32>,
    pub m_max_rotation_speed: Option<f32>,
    pub m_baumgarte_scale: Option<f32>,
    pub m_min_penetration_for_penalty: Option<f32>,
    pub m_baumgarte_time_of_impact_scale: Option<f32>,
    pub m_time_to_sleep: Option<f32>,
    pub m_linear_sleep_tolerance: Option<f32>,
    pub m_angular_sleep_tolerance: Option<f32>,
    pub m_default_contact_offset: Option<f32>,
    pub m_delete_stops_callbacks: Option<bool>,
    pub m_queries_hit_triggers: Option<bool>,
    pub m_auto_simulation: Option<bool>,
    pub m_job_options: Option<PhysicsJobOptions2D>,
    pub m_contact_threshold: Option<f32>,
    pub m_raycasts_start_in_colliders: Option<bool>,
    pub m_queries_start_in_colliders: Option<bool>,
    pub m_simulation_mode: Option<i32>,
    pub m_change_stops_callbacks: Option<bool>,
    pub m_max_sub_step_count: Option<i32>,
    pub m_simulation_layers: Option<BitField>,
    pub m_callbacks_on_disable: Option<bool>,
    pub m_min_sub_step_fps: Option<f32>,
    pub m_use_sub_stepping: Option<bool>,
    pub m_auto_sync_transforms: Option<bool>,
    pub m_reuse_collision_callbacks: Option<bool>,
    pub m_use_sub_step_contacts: Option<bool>,
    pub m_physics_low_level_settings: Option<PPtr>,
}

impl crate::Parse for Physics2DSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_gravity = Vector2f::parse(r, ver)?;
        let m_default_material = crate::PPtr::parse(r, ver)?;
        let m_velocity_iterations = r.read_i32()?;
        let m_position_iterations = r.read_i32()?;
        let m_raycasts_hit_triggers = if *ver >= crate::UnityVersion::new(4, 3, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_velocity_threshold = if *ver >= crate::UnityVersion::new(4, 5, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_bounce_threshold = if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_layer_collision_matrix = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u32() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_max_linear_correction = if *ver >= crate::UnityVersion::new(4, 5, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_max_angular_correction = if *ver >= crate::UnityVersion::new(4, 5, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_max_translation_speed = if *ver >= crate::UnityVersion::new(4, 5, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_max_rotation_speed = if *ver >= crate::UnityVersion::new(4, 5, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_baumgarte_scale = if *ver >= crate::UnityVersion::new(4, 5, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_min_penetration_for_penalty = if *ver >= crate::UnityVersion::new(4, 6, 1) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_baumgarte_time_of_impact_scale = if *ver >= crate::UnityVersion::new(4, 5, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_time_to_sleep = if *ver >= crate::UnityVersion::new(4, 5, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_linear_sleep_tolerance = if *ver >= crate::UnityVersion::new(4, 5, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_angular_sleep_tolerance = if *ver >= crate::UnityVersion::new(4, 5, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_default_contact_offset = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_delete_stops_callbacks = if *ver >= crate::UnityVersion::new(4, 5, 3) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_queries_hit_triggers = if *ver >= crate::UnityVersion::new(5, 2, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_auto_simulation = if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_job_options = if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            let v = PhysicsJobOptions2D::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_contact_threshold = if *ver >= crate::UnityVersion::new(6000, 0, 1) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_raycasts_start_in_colliders = if *ver >= crate::UnityVersion::new(4, 6, 1) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_queries_start_in_colliders = if *ver >= crate::UnityVersion::new(5, 2, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_simulation_mode = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_change_stops_callbacks = if *ver >= crate::UnityVersion::new(4, 6, 1) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_max_sub_step_count = if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_simulation_layers = if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            Some(BitField::parse(r, ver)?)
        } else {
            None
        };
        let m_callbacks_on_disable = if *ver >= crate::UnityVersion::new(5, 6, 1) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_min_sub_step_fps = if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_use_sub_stepping = if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_auto_sync_transforms = if *ver >= crate::UnityVersion::new(2017, 2, 0) {
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
        let m_use_sub_step_contacts = if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_physics_low_level_settings = if *ver >= crate::UnityVersion::new(6000, 3, 0) {
            Some(crate::PPtr::parse(r, ver)?)
        } else {
            None
        };

        Ok(Physics2DSettings {
            m_gravity,
            m_default_material,
            m_velocity_iterations,
            m_position_iterations,
            m_raycasts_hit_triggers,
            m_velocity_threshold,
            m_bounce_threshold,
            m_layer_collision_matrix,
            m_max_linear_correction,
            m_max_angular_correction,
            m_max_translation_speed,
            m_max_rotation_speed,
            m_baumgarte_scale,
            m_min_penetration_for_penalty,
            m_baumgarte_time_of_impact_scale,
            m_time_to_sleep,
            m_linear_sleep_tolerance,
            m_angular_sleep_tolerance,
            m_default_contact_offset,
            m_delete_stops_callbacks,
            m_queries_hit_triggers,
            m_auto_simulation,
            m_job_options,
            m_contact_threshold,
            m_raycasts_start_in_colliders,
            m_queries_start_in_colliders,
            m_simulation_mode,
            m_change_stops_callbacks,
            m_max_sub_step_count,
            m_simulation_layers,
            m_callbacks_on_disable,
            m_min_sub_step_fps,
            m_use_sub_stepping,
            m_auto_sync_transforms,
            m_reuse_collision_callbacks,
            m_use_sub_step_contacts,
            m_physics_low_level_settings,
        })
    }
}
