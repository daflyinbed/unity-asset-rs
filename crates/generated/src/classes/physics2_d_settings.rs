//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::bit_field::BitField;
use crate::subclasses::physics_job_options2_d::PhysicsJobOptions2D;
use crate::subclasses::vector2f::Vector2f;


/// 版本枚举：Physics2DSettingsMRaycastsHitTriggersVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum Physics2DSettingsMRaycastsHitTriggersVariant {
    Default(bool),
    V4_5_3(bool),
}

impl crate::Parse for Physics2DSettingsMRaycastsHitTriggersVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(4, 5, 3) {
            let v = r.read_bool()?;
            Ok(Self::V4_5_3(v))
        } else {
            let v = r.read_bool()?;
            r.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：Physics2DSettingsMLayerCollisionMatrixVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum Physics2DSettingsMLayerCollisionMatrixVariant {
    Default(Vec<u32>),
    V2017_1(Vec<u32>),
}

impl crate::Parse for Physics2DSettingsMLayerCollisionMatrixVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u32() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u32() }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：Physics2DSettingsMChangeStopsCallbacksVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum Physics2DSettingsMChangeStopsCallbacksVariant {
    Default(bool),
    V5_6_1(bool),
}

impl crate::Parse for Physics2DSettingsMChangeStopsCallbacksVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 6, 1) {
            let v = r.read_bool()?;
            Ok(Self::V5_6_1(v))
        } else {
            let v = r.read_bool()?;
            r.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：Physics2DSettingsMCallbacksOnDisableVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum Physics2DSettingsMCallbacksOnDisableVariant {
    Default(bool),
    V2017_2(bool),
}

impl crate::Parse for Physics2DSettingsMCallbacksOnDisableVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            let v = r.read_bool()?;
            Ok(Self::V2017_2(v))
        } else {
            let v = r.read_bool()?;
            r.align(4)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct Physics2DSettings {
    pub m_gravity: Vector2f,
    pub m_default_material: PPtr,
    pub m_velocity_iterations: i32,
    pub m_position_iterations: i32,
    pub m_raycasts_hit_triggers: Option<Physics2DSettingsMRaycastsHitTriggersVariant>,
    pub m_velocity_threshold: Option<f32>,
    pub m_bounce_threshold: Option<f32>,
    pub m_layer_collision_matrix: Physics2DSettingsMLayerCollisionMatrixVariant,
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
    pub m_change_stops_callbacks: Option<Physics2DSettingsMChangeStopsCallbacksVariant>,
    pub m_simulation_layers: Option<BitField>,
    pub m_callbacks_on_disable: Option<Physics2DSettingsMCallbacksOnDisableVariant>,
    pub m_max_sub_step_count: Option<i32>,
    pub m_min_sub_step_fps: Option<f32>,
    pub m_auto_sync_transforms: Option<bool>,
    pub m_reuse_collision_callbacks: Option<bool>,
    pub m_use_sub_stepping: Option<bool>,
    pub m_use_sub_step_contacts: Option<bool>,
    pub m_physics_low_level_settings: Option<PPtr>,
}

impl crate::Parse for Physics2DSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_gravity = None;
        let mut m_default_material = None;
        let mut m_velocity_iterations = None;
        let mut m_position_iterations = None;
        let mut m_raycasts_hit_triggers = None;
        let mut m_velocity_threshold = None;
        let mut m_bounce_threshold = None;
        let mut m_layer_collision_matrix = None;
        let mut m_max_linear_correction = None;
        let mut m_max_angular_correction = None;
        let mut m_max_translation_speed = None;
        let mut m_max_rotation_speed = None;
        let mut m_baumgarte_scale = None;
        let mut m_min_penetration_for_penalty = None;
        let mut m_baumgarte_time_of_impact_scale = None;
        let mut m_time_to_sleep = None;
        let mut m_linear_sleep_tolerance = None;
        let mut m_angular_sleep_tolerance = None;
        let mut m_default_contact_offset = None;
        let mut m_delete_stops_callbacks = None;
        let mut m_queries_hit_triggers = None;
        let mut m_auto_simulation = None;
        let mut m_job_options = None;
        let mut m_contact_threshold = None;
        let mut m_raycasts_start_in_colliders = None;
        let mut m_queries_start_in_colliders = None;
        let mut m_simulation_mode = None;
        let mut m_change_stops_callbacks = None;
        let mut m_simulation_layers = None;
        let mut m_callbacks_on_disable = None;
        let mut m_max_sub_step_count = None;
        let mut m_min_sub_step_fps = None;
        let mut m_auto_sync_transforms = None;
        let mut m_reuse_collision_callbacks = None;
        let mut m_use_sub_stepping = None;
        let mut m_use_sub_step_contacts = None;
        let mut m_physics_low_level_settings = None;
            m_gravity = Some(Vector2f::parse(r, ver)?);
            m_default_material = Some(crate::PPtr::parse(r, ver)?);
            m_velocity_iterations = Some(r.read_i32()?);
            m_position_iterations = Some(r.read_i32()?);
        if *ver >= crate::UnityVersion::new(6000, 0, 1) {
            m_bounce_threshold = Some(r.read_f32()?);
            m_max_linear_correction = Some(r.read_f32()?);
            m_max_angular_correction = Some(r.read_f32()?);
            m_max_translation_speed = Some(r.read_f32()?);
            m_max_rotation_speed = Some(r.read_f32()?);
            m_baumgarte_scale = Some(r.read_f32()?);
            m_baumgarte_time_of_impact_scale = Some(r.read_f32()?);
            m_time_to_sleep = Some(r.read_f32()?);
            m_linear_sleep_tolerance = Some(r.read_f32()?);
            m_angular_sleep_tolerance = Some(r.read_f32()?);
            m_default_contact_offset = Some(r.read_f32()?);
            m_contact_threshold = Some(r.read_f32()?);
            m_job_options = Some(PhysicsJobOptions2D::parse(r, ver)?);
            r.align(4)?;
            m_simulation_mode = Some(r.read_i32()?);
            m_simulation_layers = Some(BitField::parse(r, ver)?);
            m_max_sub_step_count = Some(r.read_i32()?);
            m_min_sub_step_fps = Some(r.read_f32()?);
            m_use_sub_stepping = Some(r.read_bool()?);
            m_use_sub_step_contacts = Some(r.read_bool()?);
            m_queries_hit_triggers = Some(r.read_bool()?);
            m_queries_start_in_colliders = Some(r.read_bool()?);
            m_callbacks_on_disable = Some(Physics2DSettingsMCallbacksOnDisableVariant::parse(r, ver)?);
            m_reuse_collision_callbacks = Some(r.read_bool()?);
            m_auto_sync_transforms = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            m_bounce_threshold = Some(r.read_f32()?);
            m_max_linear_correction = Some(r.read_f32()?);
            m_max_angular_correction = Some(r.read_f32()?);
            m_max_translation_speed = Some(r.read_f32()?);
            m_max_rotation_speed = Some(r.read_f32()?);
            m_baumgarte_scale = Some(r.read_f32()?);
            m_baumgarte_time_of_impact_scale = Some(r.read_f32()?);
            m_time_to_sleep = Some(r.read_f32()?);
            m_linear_sleep_tolerance = Some(r.read_f32()?);
            m_angular_sleep_tolerance = Some(r.read_f32()?);
            m_default_contact_offset = Some(r.read_f32()?);
            m_job_options = Some(PhysicsJobOptions2D::parse(r, ver)?);
            r.align(4)?;
            m_simulation_mode = Some(r.read_i32()?);
            m_simulation_layers = Some(BitField::parse(r, ver)?);
            m_max_sub_step_count = Some(r.read_i32()?);
            m_min_sub_step_fps = Some(r.read_f32()?);
            m_use_sub_stepping = Some(r.read_bool()?);
            m_use_sub_step_contacts = Some(r.read_bool()?);
            m_queries_hit_triggers = Some(r.read_bool()?);
            m_queries_start_in_colliders = Some(r.read_bool()?);
            m_callbacks_on_disable = Some(Physics2DSettingsMCallbacksOnDisableVariant::parse(r, ver)?);
            m_reuse_collision_callbacks = Some(r.read_bool()?);
            m_auto_sync_transforms = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            m_velocity_threshold = Some(r.read_f32()?);
            m_max_linear_correction = Some(r.read_f32()?);
            m_max_angular_correction = Some(r.read_f32()?);
            m_max_translation_speed = Some(r.read_f32()?);
            m_max_rotation_speed = Some(r.read_f32()?);
            m_baumgarte_scale = Some(r.read_f32()?);
            m_baumgarte_time_of_impact_scale = Some(r.read_f32()?);
            m_time_to_sleep = Some(r.read_f32()?);
            m_linear_sleep_tolerance = Some(r.read_f32()?);
            m_angular_sleep_tolerance = Some(r.read_f32()?);
            m_default_contact_offset = Some(r.read_f32()?);
            m_job_options = Some(PhysicsJobOptions2D::parse(r, ver)?);
            r.align(4)?;
            m_simulation_mode = Some(r.read_i32()?);
            m_queries_hit_triggers = Some(r.read_bool()?);
            m_queries_start_in_colliders = Some(r.read_bool()?);
            m_callbacks_on_disable = Some(Physics2DSettingsMCallbacksOnDisableVariant::parse(r, ver)?);
            m_reuse_collision_callbacks = Some(r.read_bool()?);
            m_auto_sync_transforms = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            m_velocity_threshold = Some(r.read_f32()?);
            m_max_linear_correction = Some(r.read_f32()?);
            m_max_angular_correction = Some(r.read_f32()?);
            m_max_translation_speed = Some(r.read_f32()?);
            m_max_rotation_speed = Some(r.read_f32()?);
            m_baumgarte_scale = Some(r.read_f32()?);
            m_baumgarte_time_of_impact_scale = Some(r.read_f32()?);
            m_time_to_sleep = Some(r.read_f32()?);
            m_linear_sleep_tolerance = Some(r.read_f32()?);
            m_angular_sleep_tolerance = Some(r.read_f32()?);
            m_default_contact_offset = Some(r.read_f32()?);
            m_job_options = Some(PhysicsJobOptions2D::parse(r, ver)?);
            r.align(4)?;
            m_auto_simulation = Some(r.read_bool()?);
            m_queries_hit_triggers = Some(r.read_bool()?);
            m_queries_start_in_colliders = Some(r.read_bool()?);
            m_callbacks_on_disable = Some(Physics2DSettingsMCallbacksOnDisableVariant::parse(r, ver)?);
            m_reuse_collision_callbacks = Some(r.read_bool()?);
            m_auto_sync_transforms = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            m_velocity_threshold = Some(r.read_f32()?);
            m_max_linear_correction = Some(r.read_f32()?);
            m_max_angular_correction = Some(r.read_f32()?);
            m_max_translation_speed = Some(r.read_f32()?);
            m_max_rotation_speed = Some(r.read_f32()?);
            m_baumgarte_scale = Some(r.read_f32()?);
            m_baumgarte_time_of_impact_scale = Some(r.read_f32()?);
            m_time_to_sleep = Some(r.read_f32()?);
            m_linear_sleep_tolerance = Some(r.read_f32()?);
            m_angular_sleep_tolerance = Some(r.read_f32()?);
            m_default_contact_offset = Some(r.read_f32()?);
            m_job_options = Some(PhysicsJobOptions2D::parse(r, ver)?);
            r.align(4)?;
            m_auto_simulation = Some(r.read_bool()?);
            m_queries_hit_triggers = Some(r.read_bool()?);
            m_queries_start_in_colliders = Some(r.read_bool()?);
            m_callbacks_on_disable = Some(Physics2DSettingsMCallbacksOnDisableVariant::parse(r, ver)?);
            m_auto_sync_transforms = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_velocity_threshold = Some(r.read_f32()?);
            m_max_linear_correction = Some(r.read_f32()?);
            m_max_angular_correction = Some(r.read_f32()?);
            m_max_translation_speed = Some(r.read_f32()?);
            m_max_rotation_speed = Some(r.read_f32()?);
            m_baumgarte_scale = Some(r.read_f32()?);
            m_baumgarte_time_of_impact_scale = Some(r.read_f32()?);
            m_time_to_sleep = Some(r.read_f32()?);
            m_linear_sleep_tolerance = Some(r.read_f32()?);
            m_angular_sleep_tolerance = Some(r.read_f32()?);
            m_default_contact_offset = Some(r.read_f32()?);
            m_auto_simulation = Some(r.read_bool()?);
            m_queries_hit_triggers = Some(r.read_bool()?);
            m_queries_start_in_colliders = Some(r.read_bool()?);
            m_change_stops_callbacks = Some(Physics2DSettingsMChangeStopsCallbacksVariant::parse(r, ver)?);
            m_callbacks_on_disable = Some(Physics2DSettingsMCallbacksOnDisableVariant::parse(r, ver)?);
            m_auto_sync_transforms = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_velocity_threshold = Some(r.read_f32()?);
            m_max_linear_correction = Some(r.read_f32()?);
            m_max_angular_correction = Some(r.read_f32()?);
            m_max_translation_speed = Some(r.read_f32()?);
            m_max_rotation_speed = Some(r.read_f32()?);
            m_baumgarte_scale = Some(r.read_f32()?);
            m_baumgarte_time_of_impact_scale = Some(r.read_f32()?);
            m_time_to_sleep = Some(r.read_f32()?);
            m_linear_sleep_tolerance = Some(r.read_f32()?);
            m_angular_sleep_tolerance = Some(r.read_f32()?);
            m_default_contact_offset = Some(r.read_f32()?);
            m_auto_simulation = Some(r.read_bool()?);
            m_queries_hit_triggers = Some(r.read_bool()?);
            m_queries_start_in_colliders = Some(r.read_bool()?);
            m_change_stops_callbacks = Some(Physics2DSettingsMChangeStopsCallbacksVariant::parse(r, ver)?);
            m_callbacks_on_disable = Some(Physics2DSettingsMCallbacksOnDisableVariant::parse(r, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 6, 1) {
            m_velocity_threshold = Some(r.read_f32()?);
            m_max_linear_correction = Some(r.read_f32()?);
            m_max_angular_correction = Some(r.read_f32()?);
            m_max_translation_speed = Some(r.read_f32()?);
            m_max_rotation_speed = Some(r.read_f32()?);
            m_baumgarte_scale = Some(r.read_f32()?);
            m_baumgarte_time_of_impact_scale = Some(r.read_f32()?);
            m_time_to_sleep = Some(r.read_f32()?);
            m_linear_sleep_tolerance = Some(r.read_f32()?);
            m_angular_sleep_tolerance = Some(r.read_f32()?);
            m_default_contact_offset = Some(r.read_f32()?);
            m_queries_hit_triggers = Some(r.read_bool()?);
            m_queries_start_in_colliders = Some(r.read_bool()?);
            m_change_stops_callbacks = Some(Physics2DSettingsMChangeStopsCallbacksVariant::parse(r, ver)?);
            m_callbacks_on_disable = Some(Physics2DSettingsMCallbacksOnDisableVariant::parse(r, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_velocity_threshold = Some(r.read_f32()?);
            m_max_linear_correction = Some(r.read_f32()?);
            m_max_angular_correction = Some(r.read_f32()?);
            m_max_translation_speed = Some(r.read_f32()?);
            m_max_rotation_speed = Some(r.read_f32()?);
            m_baumgarte_scale = Some(r.read_f32()?);
            m_baumgarte_time_of_impact_scale = Some(r.read_f32()?);
            m_time_to_sleep = Some(r.read_f32()?);
            m_linear_sleep_tolerance = Some(r.read_f32()?);
            m_angular_sleep_tolerance = Some(r.read_f32()?);
            m_default_contact_offset = Some(r.read_f32()?);
            m_queries_hit_triggers = Some(r.read_bool()?);
            m_queries_start_in_colliders = Some(r.read_bool()?);
            m_change_stops_callbacks = Some(Physics2DSettingsMChangeStopsCallbacksVariant::parse(r, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 2, 0) {
            m_velocity_threshold = Some(r.read_f32()?);
            m_max_linear_correction = Some(r.read_f32()?);
            m_max_angular_correction = Some(r.read_f32()?);
            m_max_translation_speed = Some(r.read_f32()?);
            m_max_rotation_speed = Some(r.read_f32()?);
            m_min_penetration_for_penalty = Some(r.read_f32()?);
            m_baumgarte_scale = Some(r.read_f32()?);
            m_baumgarte_time_of_impact_scale = Some(r.read_f32()?);
            m_time_to_sleep = Some(r.read_f32()?);
            m_linear_sleep_tolerance = Some(r.read_f32()?);
            m_angular_sleep_tolerance = Some(r.read_f32()?);
            m_queries_hit_triggers = Some(r.read_bool()?);
            m_queries_start_in_colliders = Some(r.read_bool()?);
            m_change_stops_callbacks = Some(Physics2DSettingsMChangeStopsCallbacksVariant::parse(r, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 6, 1) {
            m_velocity_threshold = Some(r.read_f32()?);
            m_max_linear_correction = Some(r.read_f32()?);
            m_max_angular_correction = Some(r.read_f32()?);
            m_max_translation_speed = Some(r.read_f32()?);
            m_max_rotation_speed = Some(r.read_f32()?);
            m_min_penetration_for_penalty = Some(r.read_f32()?);
            m_baumgarte_scale = Some(r.read_f32()?);
            m_baumgarte_time_of_impact_scale = Some(r.read_f32()?);
            m_time_to_sleep = Some(r.read_f32()?);
            m_linear_sleep_tolerance = Some(r.read_f32()?);
            m_angular_sleep_tolerance = Some(r.read_f32()?);
            m_raycasts_hit_triggers = Some(Physics2DSettingsMRaycastsHitTriggersVariant::parse(r, ver)?);
            m_raycasts_start_in_colliders = Some(r.read_bool()?);
            m_change_stops_callbacks = Some(Physics2DSettingsMChangeStopsCallbacksVariant::parse(r, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 5, 3) {
            m_velocity_threshold = Some(r.read_f32()?);
            m_max_linear_correction = Some(r.read_f32()?);
            m_max_angular_correction = Some(r.read_f32()?);
            m_max_translation_speed = Some(r.read_f32()?);
            m_max_rotation_speed = Some(r.read_f32()?);
            m_baumgarte_scale = Some(r.read_f32()?);
            m_baumgarte_time_of_impact_scale = Some(r.read_f32()?);
            m_time_to_sleep = Some(r.read_f32()?);
            m_linear_sleep_tolerance = Some(r.read_f32()?);
            m_angular_sleep_tolerance = Some(r.read_f32()?);
            m_raycasts_hit_triggers = Some(Physics2DSettingsMRaycastsHitTriggersVariant::parse(r, ver)?);
            m_delete_stops_callbacks = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(4, 5, 0) {
            m_velocity_threshold = Some(r.read_f32()?);
            m_max_linear_correction = Some(r.read_f32()?);
            m_max_angular_correction = Some(r.read_f32()?);
            m_max_translation_speed = Some(r.read_f32()?);
            m_max_rotation_speed = Some(r.read_f32()?);
            m_baumgarte_scale = Some(r.read_f32()?);
            m_baumgarte_time_of_impact_scale = Some(r.read_f32()?);
            m_time_to_sleep = Some(r.read_f32()?);
            m_linear_sleep_tolerance = Some(r.read_f32()?);
            m_angular_sleep_tolerance = Some(r.read_f32()?);
            m_raycasts_hit_triggers = Some(Physics2DSettingsMRaycastsHitTriggersVariant::parse(r, ver)?);
        } else {
            m_raycasts_hit_triggers = Some(Physics2DSettingsMRaycastsHitTriggersVariant::parse(r, ver)?);
        }
            m_layer_collision_matrix = Some(Physics2DSettingsMLayerCollisionMatrixVariant::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(6000, 3, 0) {
            m_physics_low_level_settings = Some(crate::PPtr::parse(r, ver)?);
        } else {
        }

        Ok(Physics2DSettings {
            m_gravity: m_gravity.unwrap(),
            m_default_material: m_default_material.unwrap(),
            m_velocity_iterations: m_velocity_iterations.unwrap(),
            m_position_iterations: m_position_iterations.unwrap(),
            m_raycasts_hit_triggers,
            m_velocity_threshold,
            m_bounce_threshold,
            m_layer_collision_matrix: m_layer_collision_matrix.unwrap(),
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
            m_simulation_layers,
            m_callbacks_on_disable,
            m_max_sub_step_count,
            m_min_sub_step_fps,
            m_auto_sync_transforms,
            m_reuse_collision_callbacks,
            m_use_sub_stepping,
            m_use_sub_step_contacts,
            m_physics_low_level_settings,
        })
    }
}
