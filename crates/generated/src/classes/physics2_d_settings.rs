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
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(4, 5, 3) {
            let v = rdr.read_bool()?;
            Ok(Self::V4_5_3(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for Physics2DSettingsMRaycastsHitTriggersVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V4_5_3(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V4_5_3(v) => (v).inspect_value(path, offset, limit),
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
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_u32())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_u32())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for Physics2DSettingsMLayerCollisionMatrixVariant {
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

/// 版本枚举：Physics2DSettingsMChangeStopsCallbacksVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum Physics2DSettingsMChangeStopsCallbacksVariant {
    Default(bool),
    V5_6_1(bool),
}

impl crate::Parse for Physics2DSettingsMChangeStopsCallbacksVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 6, 1) {
            let v = rdr.read_bool()?;
            Ok(Self::V5_6_1(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for Physics2DSettingsMChangeStopsCallbacksVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_6_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_6_1(v) => (v).inspect_value(path, offset, limit),
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
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2017_2(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for Physics2DSettingsMCallbacksOnDisableVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_2(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_2(v) => (v).inspect_value(path, offset, limit),
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
        rdr: &mut R,
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
        m_gravity = Some(Vector2f::parse(rdr, ver)?);
        m_default_material = Some(crate::PPtr::parse(rdr, ver)?);
        m_velocity_iterations = Some(rdr.read_i32()?);
        m_position_iterations = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(6000, 0, 1) {
            m_bounce_threshold = Some(rdr.read_f32()?);
            m_max_linear_correction = Some(rdr.read_f32()?);
            m_max_angular_correction = Some(rdr.read_f32()?);
            m_max_translation_speed = Some(rdr.read_f32()?);
            m_max_rotation_speed = Some(rdr.read_f32()?);
            m_baumgarte_scale = Some(rdr.read_f32()?);
            m_baumgarte_time_of_impact_scale = Some(rdr.read_f32()?);
            m_time_to_sleep = Some(rdr.read_f32()?);
            m_linear_sleep_tolerance = Some(rdr.read_f32()?);
            m_angular_sleep_tolerance = Some(rdr.read_f32()?);
            m_default_contact_offset = Some(rdr.read_f32()?);
            m_contact_threshold = Some(rdr.read_f32()?);
            m_job_options = Some(PhysicsJobOptions2D::parse(rdr, ver)?);
            rdr.align(4)?;
            m_simulation_mode = Some(rdr.read_i32()?);
            m_simulation_layers = Some(BitField::parse(rdr, ver)?);
            m_max_sub_step_count = Some(rdr.read_i32()?);
            m_min_sub_step_fps = Some(rdr.read_f32()?);
            m_use_sub_stepping = Some(rdr.read_bool()?);
            m_use_sub_step_contacts = Some(rdr.read_bool()?);
            m_queries_hit_triggers = Some(rdr.read_bool()?);
            m_queries_start_in_colliders = Some(rdr.read_bool()?);
            m_callbacks_on_disable = Some(Physics2DSettingsMCallbacksOnDisableVariant::parse(
                rdr, ver,
            )?);
            m_reuse_collision_callbacks = Some(rdr.read_bool()?);
            m_auto_sync_transforms = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            m_bounce_threshold = Some(rdr.read_f32()?);
            m_max_linear_correction = Some(rdr.read_f32()?);
            m_max_angular_correction = Some(rdr.read_f32()?);
            m_max_translation_speed = Some(rdr.read_f32()?);
            m_max_rotation_speed = Some(rdr.read_f32()?);
            m_baumgarte_scale = Some(rdr.read_f32()?);
            m_baumgarte_time_of_impact_scale = Some(rdr.read_f32()?);
            m_time_to_sleep = Some(rdr.read_f32()?);
            m_linear_sleep_tolerance = Some(rdr.read_f32()?);
            m_angular_sleep_tolerance = Some(rdr.read_f32()?);
            m_default_contact_offset = Some(rdr.read_f32()?);
            m_job_options = Some(PhysicsJobOptions2D::parse(rdr, ver)?);
            rdr.align(4)?;
            m_simulation_mode = Some(rdr.read_i32()?);
            m_simulation_layers = Some(BitField::parse(rdr, ver)?);
            m_max_sub_step_count = Some(rdr.read_i32()?);
            m_min_sub_step_fps = Some(rdr.read_f32()?);
            m_use_sub_stepping = Some(rdr.read_bool()?);
            m_use_sub_step_contacts = Some(rdr.read_bool()?);
            m_queries_hit_triggers = Some(rdr.read_bool()?);
            m_queries_start_in_colliders = Some(rdr.read_bool()?);
            m_callbacks_on_disable = Some(Physics2DSettingsMCallbacksOnDisableVariant::parse(
                rdr, ver,
            )?);
            m_reuse_collision_callbacks = Some(rdr.read_bool()?);
            m_auto_sync_transforms = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            m_velocity_threshold = Some(rdr.read_f32()?);
            m_max_linear_correction = Some(rdr.read_f32()?);
            m_max_angular_correction = Some(rdr.read_f32()?);
            m_max_translation_speed = Some(rdr.read_f32()?);
            m_max_rotation_speed = Some(rdr.read_f32()?);
            m_baumgarte_scale = Some(rdr.read_f32()?);
            m_baumgarte_time_of_impact_scale = Some(rdr.read_f32()?);
            m_time_to_sleep = Some(rdr.read_f32()?);
            m_linear_sleep_tolerance = Some(rdr.read_f32()?);
            m_angular_sleep_tolerance = Some(rdr.read_f32()?);
            m_default_contact_offset = Some(rdr.read_f32()?);
            m_job_options = Some(PhysicsJobOptions2D::parse(rdr, ver)?);
            rdr.align(4)?;
            m_simulation_mode = Some(rdr.read_i32()?);
            m_queries_hit_triggers = Some(rdr.read_bool()?);
            m_queries_start_in_colliders = Some(rdr.read_bool()?);
            m_callbacks_on_disable = Some(Physics2DSettingsMCallbacksOnDisableVariant::parse(
                rdr, ver,
            )?);
            m_reuse_collision_callbacks = Some(rdr.read_bool()?);
            m_auto_sync_transforms = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            m_velocity_threshold = Some(rdr.read_f32()?);
            m_max_linear_correction = Some(rdr.read_f32()?);
            m_max_angular_correction = Some(rdr.read_f32()?);
            m_max_translation_speed = Some(rdr.read_f32()?);
            m_max_rotation_speed = Some(rdr.read_f32()?);
            m_baumgarte_scale = Some(rdr.read_f32()?);
            m_baumgarte_time_of_impact_scale = Some(rdr.read_f32()?);
            m_time_to_sleep = Some(rdr.read_f32()?);
            m_linear_sleep_tolerance = Some(rdr.read_f32()?);
            m_angular_sleep_tolerance = Some(rdr.read_f32()?);
            m_default_contact_offset = Some(rdr.read_f32()?);
            m_job_options = Some(PhysicsJobOptions2D::parse(rdr, ver)?);
            rdr.align(4)?;
            m_auto_simulation = Some(rdr.read_bool()?);
            m_queries_hit_triggers = Some(rdr.read_bool()?);
            m_queries_start_in_colliders = Some(rdr.read_bool()?);
            m_callbacks_on_disable = Some(Physics2DSettingsMCallbacksOnDisableVariant::parse(
                rdr, ver,
            )?);
            m_reuse_collision_callbacks = Some(rdr.read_bool()?);
            m_auto_sync_transforms = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            m_velocity_threshold = Some(rdr.read_f32()?);
            m_max_linear_correction = Some(rdr.read_f32()?);
            m_max_angular_correction = Some(rdr.read_f32()?);
            m_max_translation_speed = Some(rdr.read_f32()?);
            m_max_rotation_speed = Some(rdr.read_f32()?);
            m_baumgarte_scale = Some(rdr.read_f32()?);
            m_baumgarte_time_of_impact_scale = Some(rdr.read_f32()?);
            m_time_to_sleep = Some(rdr.read_f32()?);
            m_linear_sleep_tolerance = Some(rdr.read_f32()?);
            m_angular_sleep_tolerance = Some(rdr.read_f32()?);
            m_default_contact_offset = Some(rdr.read_f32()?);
            m_job_options = Some(PhysicsJobOptions2D::parse(rdr, ver)?);
            rdr.align(4)?;
            m_auto_simulation = Some(rdr.read_bool()?);
            m_queries_hit_triggers = Some(rdr.read_bool()?);
            m_queries_start_in_colliders = Some(rdr.read_bool()?);
            m_callbacks_on_disable = Some(Physics2DSettingsMCallbacksOnDisableVariant::parse(
                rdr, ver,
            )?);
            m_auto_sync_transforms = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_velocity_threshold = Some(rdr.read_f32()?);
            m_max_linear_correction = Some(rdr.read_f32()?);
            m_max_angular_correction = Some(rdr.read_f32()?);
            m_max_translation_speed = Some(rdr.read_f32()?);
            m_max_rotation_speed = Some(rdr.read_f32()?);
            m_baumgarte_scale = Some(rdr.read_f32()?);
            m_baumgarte_time_of_impact_scale = Some(rdr.read_f32()?);
            m_time_to_sleep = Some(rdr.read_f32()?);
            m_linear_sleep_tolerance = Some(rdr.read_f32()?);
            m_angular_sleep_tolerance = Some(rdr.read_f32()?);
            m_default_contact_offset = Some(rdr.read_f32()?);
            m_auto_simulation = Some(rdr.read_bool()?);
            m_queries_hit_triggers = Some(rdr.read_bool()?);
            m_queries_start_in_colliders = Some(rdr.read_bool()?);
            m_change_stops_callbacks = Some(Physics2DSettingsMChangeStopsCallbacksVariant::parse(
                rdr, ver,
            )?);
            m_callbacks_on_disable = Some(Physics2DSettingsMCallbacksOnDisableVariant::parse(
                rdr, ver,
            )?);
            m_auto_sync_transforms = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_velocity_threshold = Some(rdr.read_f32()?);
            m_max_linear_correction = Some(rdr.read_f32()?);
            m_max_angular_correction = Some(rdr.read_f32()?);
            m_max_translation_speed = Some(rdr.read_f32()?);
            m_max_rotation_speed = Some(rdr.read_f32()?);
            m_baumgarte_scale = Some(rdr.read_f32()?);
            m_baumgarte_time_of_impact_scale = Some(rdr.read_f32()?);
            m_time_to_sleep = Some(rdr.read_f32()?);
            m_linear_sleep_tolerance = Some(rdr.read_f32()?);
            m_angular_sleep_tolerance = Some(rdr.read_f32()?);
            m_default_contact_offset = Some(rdr.read_f32()?);
            m_auto_simulation = Some(rdr.read_bool()?);
            m_queries_hit_triggers = Some(rdr.read_bool()?);
            m_queries_start_in_colliders = Some(rdr.read_bool()?);
            m_change_stops_callbacks = Some(Physics2DSettingsMChangeStopsCallbacksVariant::parse(
                rdr, ver,
            )?);
            m_callbacks_on_disable = Some(Physics2DSettingsMCallbacksOnDisableVariant::parse(
                rdr, ver,
            )?);
        } else if *ver >= crate::UnityVersion::new(5, 6, 1) {
            m_velocity_threshold = Some(rdr.read_f32()?);
            m_max_linear_correction = Some(rdr.read_f32()?);
            m_max_angular_correction = Some(rdr.read_f32()?);
            m_max_translation_speed = Some(rdr.read_f32()?);
            m_max_rotation_speed = Some(rdr.read_f32()?);
            m_baumgarte_scale = Some(rdr.read_f32()?);
            m_baumgarte_time_of_impact_scale = Some(rdr.read_f32()?);
            m_time_to_sleep = Some(rdr.read_f32()?);
            m_linear_sleep_tolerance = Some(rdr.read_f32()?);
            m_angular_sleep_tolerance = Some(rdr.read_f32()?);
            m_default_contact_offset = Some(rdr.read_f32()?);
            m_queries_hit_triggers = Some(rdr.read_bool()?);
            m_queries_start_in_colliders = Some(rdr.read_bool()?);
            m_change_stops_callbacks = Some(Physics2DSettingsMChangeStopsCallbacksVariant::parse(
                rdr, ver,
            )?);
            m_callbacks_on_disable = Some(Physics2DSettingsMCallbacksOnDisableVariant::parse(
                rdr, ver,
            )?);
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_velocity_threshold = Some(rdr.read_f32()?);
            m_max_linear_correction = Some(rdr.read_f32()?);
            m_max_angular_correction = Some(rdr.read_f32()?);
            m_max_translation_speed = Some(rdr.read_f32()?);
            m_max_rotation_speed = Some(rdr.read_f32()?);
            m_baumgarte_scale = Some(rdr.read_f32()?);
            m_baumgarte_time_of_impact_scale = Some(rdr.read_f32()?);
            m_time_to_sleep = Some(rdr.read_f32()?);
            m_linear_sleep_tolerance = Some(rdr.read_f32()?);
            m_angular_sleep_tolerance = Some(rdr.read_f32()?);
            m_default_contact_offset = Some(rdr.read_f32()?);
            m_queries_hit_triggers = Some(rdr.read_bool()?);
            m_queries_start_in_colliders = Some(rdr.read_bool()?);
            m_change_stops_callbacks = Some(Physics2DSettingsMChangeStopsCallbacksVariant::parse(
                rdr, ver,
            )?);
        } else if *ver >= crate::UnityVersion::new(5, 2, 0) {
            m_velocity_threshold = Some(rdr.read_f32()?);
            m_max_linear_correction = Some(rdr.read_f32()?);
            m_max_angular_correction = Some(rdr.read_f32()?);
            m_max_translation_speed = Some(rdr.read_f32()?);
            m_max_rotation_speed = Some(rdr.read_f32()?);
            m_min_penetration_for_penalty = Some(rdr.read_f32()?);
            m_baumgarte_scale = Some(rdr.read_f32()?);
            m_baumgarte_time_of_impact_scale = Some(rdr.read_f32()?);
            m_time_to_sleep = Some(rdr.read_f32()?);
            m_linear_sleep_tolerance = Some(rdr.read_f32()?);
            m_angular_sleep_tolerance = Some(rdr.read_f32()?);
            m_queries_hit_triggers = Some(rdr.read_bool()?);
            m_queries_start_in_colliders = Some(rdr.read_bool()?);
            m_change_stops_callbacks = Some(Physics2DSettingsMChangeStopsCallbacksVariant::parse(
                rdr, ver,
            )?);
        } else if *ver >= crate::UnityVersion::new(4, 6, 1) {
            m_velocity_threshold = Some(rdr.read_f32()?);
            m_max_linear_correction = Some(rdr.read_f32()?);
            m_max_angular_correction = Some(rdr.read_f32()?);
            m_max_translation_speed = Some(rdr.read_f32()?);
            m_max_rotation_speed = Some(rdr.read_f32()?);
            m_min_penetration_for_penalty = Some(rdr.read_f32()?);
            m_baumgarte_scale = Some(rdr.read_f32()?);
            m_baumgarte_time_of_impact_scale = Some(rdr.read_f32()?);
            m_time_to_sleep = Some(rdr.read_f32()?);
            m_linear_sleep_tolerance = Some(rdr.read_f32()?);
            m_angular_sleep_tolerance = Some(rdr.read_f32()?);
            m_raycasts_hit_triggers = Some(Physics2DSettingsMRaycastsHitTriggersVariant::parse(
                rdr, ver,
            )?);
            m_raycasts_start_in_colliders = Some(rdr.read_bool()?);
            m_change_stops_callbacks = Some(Physics2DSettingsMChangeStopsCallbacksVariant::parse(
                rdr, ver,
            )?);
        } else if *ver >= crate::UnityVersion::new(4, 5, 3) {
            m_velocity_threshold = Some(rdr.read_f32()?);
            m_max_linear_correction = Some(rdr.read_f32()?);
            m_max_angular_correction = Some(rdr.read_f32()?);
            m_max_translation_speed = Some(rdr.read_f32()?);
            m_max_rotation_speed = Some(rdr.read_f32()?);
            m_baumgarte_scale = Some(rdr.read_f32()?);
            m_baumgarte_time_of_impact_scale = Some(rdr.read_f32()?);
            m_time_to_sleep = Some(rdr.read_f32()?);
            m_linear_sleep_tolerance = Some(rdr.read_f32()?);
            m_angular_sleep_tolerance = Some(rdr.read_f32()?);
            m_raycasts_hit_triggers = Some(Physics2DSettingsMRaycastsHitTriggersVariant::parse(
                rdr, ver,
            )?);
            m_delete_stops_callbacks = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(4, 5, 0) {
            m_velocity_threshold = Some(rdr.read_f32()?);
            m_max_linear_correction = Some(rdr.read_f32()?);
            m_max_angular_correction = Some(rdr.read_f32()?);
            m_max_translation_speed = Some(rdr.read_f32()?);
            m_max_rotation_speed = Some(rdr.read_f32()?);
            m_baumgarte_scale = Some(rdr.read_f32()?);
            m_baumgarte_time_of_impact_scale = Some(rdr.read_f32()?);
            m_time_to_sleep = Some(rdr.read_f32()?);
            m_linear_sleep_tolerance = Some(rdr.read_f32()?);
            m_angular_sleep_tolerance = Some(rdr.read_f32()?);
            m_raycasts_hit_triggers = Some(Physics2DSettingsMRaycastsHitTriggersVariant::parse(
                rdr, ver,
            )?);
        } else {
            m_raycasts_hit_triggers = Some(Physics2DSettingsMRaycastsHitTriggersVariant::parse(
                rdr, ver,
            )?);
        }
        m_layer_collision_matrix = Some(Physics2DSettingsMLayerCollisionMatrixVariant::parse(
            rdr, ver,
        )?);
        if *ver >= crate::UnityVersion::new(6000, 3, 0) {
            m_physics_low_level_settings = Some(crate::PPtr::parse(rdr, ver)?);
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

impl crate::ToValue for Physics2DSettings {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Gravity".to_string(), self.m_gravity.to_value())),
            Some((
                "m_DefaultMaterial".to_string(),
                self.m_default_material.to_value(),
            )),
            Some((
                "m_VelocityIterations".to_string(),
                self.m_velocity_iterations.to_value(),
            )),
            Some((
                "m_PositionIterations".to_string(),
                self.m_position_iterations.to_value(),
            )),
            crate::opt_field("m_RaycastsHitTriggers", &self.m_raycasts_hit_triggers),
            crate::opt_field("m_VelocityThreshold", &self.m_velocity_threshold),
            crate::opt_field("m_BounceThreshold", &self.m_bounce_threshold),
            Some((
                "m_LayerCollisionMatrix".to_string(),
                self.m_layer_collision_matrix.to_value(),
            )),
            crate::opt_field("m_MaxLinearCorrection", &self.m_max_linear_correction),
            crate::opt_field("m_MaxAngularCorrection", &self.m_max_angular_correction),
            crate::opt_field("m_MaxTranslationSpeed", &self.m_max_translation_speed),
            crate::opt_field("m_MaxRotationSpeed", &self.m_max_rotation_speed),
            crate::opt_field("m_BaumgarteScale", &self.m_baumgarte_scale),
            crate::opt_field(
                "m_MinPenetrationForPenalty",
                &self.m_min_penetration_for_penalty,
            ),
            crate::opt_field(
                "m_BaumgarteTimeOfImpactScale",
                &self.m_baumgarte_time_of_impact_scale,
            ),
            crate::opt_field("m_TimeToSleep", &self.m_time_to_sleep),
            crate::opt_field("m_LinearSleepTolerance", &self.m_linear_sleep_tolerance),
            crate::opt_field("m_AngularSleepTolerance", &self.m_angular_sleep_tolerance),
            crate::opt_field("m_DefaultContactOffset", &self.m_default_contact_offset),
            crate::opt_field("m_DeleteStopsCallbacks", &self.m_delete_stops_callbacks),
            crate::opt_field("m_QueriesHitTriggers", &self.m_queries_hit_triggers),
            crate::opt_field("m_AutoSimulation", &self.m_auto_simulation),
            crate::opt_field("m_JobOptions", &self.m_job_options),
            crate::opt_field("m_ContactThreshold", &self.m_contact_threshold),
            crate::opt_field(
                "m_RaycastsStartInColliders",
                &self.m_raycasts_start_in_colliders,
            ),
            crate::opt_field(
                "m_QueriesStartInColliders",
                &self.m_queries_start_in_colliders,
            ),
            crate::opt_field("m_SimulationMode", &self.m_simulation_mode),
            crate::opt_field("m_ChangeStopsCallbacks", &self.m_change_stops_callbacks),
            crate::opt_field("m_SimulationLayers", &self.m_simulation_layers),
            crate::opt_field("m_CallbacksOnDisable", &self.m_callbacks_on_disable),
            crate::opt_field("m_MaxSubStepCount", &self.m_max_sub_step_count),
            crate::opt_field("m_MinSubStepFPS", &self.m_min_sub_step_fps),
            crate::opt_field("m_AutoSyncTransforms", &self.m_auto_sync_transforms),
            crate::opt_field(
                "m_ReuseCollisionCallbacks",
                &self.m_reuse_collision_callbacks,
            ),
            crate::opt_field("m_UseSubStepping", &self.m_use_sub_stepping),
            crate::opt_field("m_UseSubStepContacts", &self.m_use_sub_step_contacts),
            crate::opt_field(
                "m_PhysicsLowLevelSettings",
                &self.m_physics_low_level_settings,
            ),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Gravity" => (self.m_gravity).inspect_value(path, offset, limit),
                "m_DefaultMaterial" => (self.m_default_material).inspect_value(path, offset, limit),
                "m_VelocityIterations" => {
                    (self.m_velocity_iterations).inspect_value(path, offset, limit)
                }
                "m_PositionIterations" => {
                    (self.m_position_iterations).inspect_value(path, offset, limit)
                }
                "m_RaycastsHitTriggers" => {
                    (self.m_raycasts_hit_triggers).inspect_value(path, offset, limit)
                }
                "m_VelocityThreshold" => {
                    (self.m_velocity_threshold).inspect_value(path, offset, limit)
                }
                "m_BounceThreshold" => (self.m_bounce_threshold).inspect_value(path, offset, limit),
                "m_LayerCollisionMatrix" => {
                    (self.m_layer_collision_matrix).inspect_value(path, offset, limit)
                }
                "m_MaxLinearCorrection" => {
                    (self.m_max_linear_correction).inspect_value(path, offset, limit)
                }
                "m_MaxAngularCorrection" => {
                    (self.m_max_angular_correction).inspect_value(path, offset, limit)
                }
                "m_MaxTranslationSpeed" => {
                    (self.m_max_translation_speed).inspect_value(path, offset, limit)
                }
                "m_MaxRotationSpeed" => {
                    (self.m_max_rotation_speed).inspect_value(path, offset, limit)
                }
                "m_BaumgarteScale" => (self.m_baumgarte_scale).inspect_value(path, offset, limit),
                "m_MinPenetrationForPenalty" => {
                    (self.m_min_penetration_for_penalty).inspect_value(path, offset, limit)
                }
                "m_BaumgarteTimeOfImpactScale" => {
                    (self.m_baumgarte_time_of_impact_scale).inspect_value(path, offset, limit)
                }
                "m_TimeToSleep" => (self.m_time_to_sleep).inspect_value(path, offset, limit),
                "m_LinearSleepTolerance" => {
                    (self.m_linear_sleep_tolerance).inspect_value(path, offset, limit)
                }
                "m_AngularSleepTolerance" => {
                    (self.m_angular_sleep_tolerance).inspect_value(path, offset, limit)
                }
                "m_DefaultContactOffset" => {
                    (self.m_default_contact_offset).inspect_value(path, offset, limit)
                }
                "m_DeleteStopsCallbacks" => {
                    (self.m_delete_stops_callbacks).inspect_value(path, offset, limit)
                }
                "m_QueriesHitTriggers" => {
                    (self.m_queries_hit_triggers).inspect_value(path, offset, limit)
                }
                "m_AutoSimulation" => (self.m_auto_simulation).inspect_value(path, offset, limit),
                "m_JobOptions" => (self.m_job_options).inspect_value(path, offset, limit),
                "m_ContactThreshold" => {
                    (self.m_contact_threshold).inspect_value(path, offset, limit)
                }
                "m_RaycastsStartInColliders" => {
                    (self.m_raycasts_start_in_colliders).inspect_value(path, offset, limit)
                }
                "m_QueriesStartInColliders" => {
                    (self.m_queries_start_in_colliders).inspect_value(path, offset, limit)
                }
                "m_SimulationMode" => (self.m_simulation_mode).inspect_value(path, offset, limit),
                "m_ChangeStopsCallbacks" => {
                    (self.m_change_stops_callbacks).inspect_value(path, offset, limit)
                }
                "m_SimulationLayers" => {
                    (self.m_simulation_layers).inspect_value(path, offset, limit)
                }
                "m_CallbacksOnDisable" => {
                    (self.m_callbacks_on_disable).inspect_value(path, offset, limit)
                }
                "m_MaxSubStepCount" => {
                    (self.m_max_sub_step_count).inspect_value(path, offset, limit)
                }
                "m_MinSubStepFPS" => (self.m_min_sub_step_fps).inspect_value(path, offset, limit),
                "m_AutoSyncTransforms" => {
                    (self.m_auto_sync_transforms).inspect_value(path, offset, limit)
                }
                "m_ReuseCollisionCallbacks" => {
                    (self.m_reuse_collision_callbacks).inspect_value(path, offset, limit)
                }
                "m_UseSubStepping" => (self.m_use_sub_stepping).inspect_value(path, offset, limit),
                "m_UseSubStepContacts" => {
                    (self.m_use_sub_step_contacts).inspect_value(path, offset, limit)
                }
                "m_PhysicsLowLevelSettings" => {
                    (self.m_physics_low_level_settings).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_gravity).inspect_value(path, offset, limit))
                .map(|v| ("m_Gravity".to_string(), v)),
            ((self.m_default_material).inspect_value(path, offset, limit))
                .map(|v| ("m_DefaultMaterial".to_string(), v)),
            ((self.m_velocity_iterations).inspect_value(path, offset, limit))
                .map(|v| ("m_VelocityIterations".to_string(), v)),
            ((self.m_position_iterations).inspect_value(path, offset, limit))
                .map(|v| ("m_PositionIterations".to_string(), v)),
            ((self.m_raycasts_hit_triggers).inspect_value(path, offset, limit))
                .map(|v| ("m_RaycastsHitTriggers".to_string(), v)),
            ((self.m_velocity_threshold).inspect_value(path, offset, limit))
                .map(|v| ("m_VelocityThreshold".to_string(), v)),
            ((self.m_bounce_threshold).inspect_value(path, offset, limit))
                .map(|v| ("m_BounceThreshold".to_string(), v)),
            ((self.m_layer_collision_matrix).inspect_value(path, offset, limit))
                .map(|v| ("m_LayerCollisionMatrix".to_string(), v)),
            ((self.m_max_linear_correction).inspect_value(path, offset, limit))
                .map(|v| ("m_MaxLinearCorrection".to_string(), v)),
            ((self.m_max_angular_correction).inspect_value(path, offset, limit))
                .map(|v| ("m_MaxAngularCorrection".to_string(), v)),
            ((self.m_max_translation_speed).inspect_value(path, offset, limit))
                .map(|v| ("m_MaxTranslationSpeed".to_string(), v)),
            ((self.m_max_rotation_speed).inspect_value(path, offset, limit))
                .map(|v| ("m_MaxRotationSpeed".to_string(), v)),
            ((self.m_baumgarte_scale).inspect_value(path, offset, limit))
                .map(|v| ("m_BaumgarteScale".to_string(), v)),
            ((self.m_min_penetration_for_penalty).inspect_value(path, offset, limit))
                .map(|v| ("m_MinPenetrationForPenalty".to_string(), v)),
            ((self.m_baumgarte_time_of_impact_scale).inspect_value(path, offset, limit))
                .map(|v| ("m_BaumgarteTimeOfImpactScale".to_string(), v)),
            ((self.m_time_to_sleep).inspect_value(path, offset, limit))
                .map(|v| ("m_TimeToSleep".to_string(), v)),
            ((self.m_linear_sleep_tolerance).inspect_value(path, offset, limit))
                .map(|v| ("m_LinearSleepTolerance".to_string(), v)),
            ((self.m_angular_sleep_tolerance).inspect_value(path, offset, limit))
                .map(|v| ("m_AngularSleepTolerance".to_string(), v)),
            ((self.m_default_contact_offset).inspect_value(path, offset, limit))
                .map(|v| ("m_DefaultContactOffset".to_string(), v)),
            ((self.m_delete_stops_callbacks).inspect_value(path, offset, limit))
                .map(|v| ("m_DeleteStopsCallbacks".to_string(), v)),
            ((self.m_queries_hit_triggers).inspect_value(path, offset, limit))
                .map(|v| ("m_QueriesHitTriggers".to_string(), v)),
            ((self.m_auto_simulation).inspect_value(path, offset, limit))
                .map(|v| ("m_AutoSimulation".to_string(), v)),
            ((self.m_job_options).inspect_value(path, offset, limit))
                .map(|v| ("m_JobOptions".to_string(), v)),
            ((self.m_contact_threshold).inspect_value(path, offset, limit))
                .map(|v| ("m_ContactThreshold".to_string(), v)),
            ((self.m_raycasts_start_in_colliders).inspect_value(path, offset, limit))
                .map(|v| ("m_RaycastsStartInColliders".to_string(), v)),
            ((self.m_queries_start_in_colliders).inspect_value(path, offset, limit))
                .map(|v| ("m_QueriesStartInColliders".to_string(), v)),
            ((self.m_simulation_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_SimulationMode".to_string(), v)),
            ((self.m_change_stops_callbacks).inspect_value(path, offset, limit))
                .map(|v| ("m_ChangeStopsCallbacks".to_string(), v)),
            ((self.m_simulation_layers).inspect_value(path, offset, limit))
                .map(|v| ("m_SimulationLayers".to_string(), v)),
            ((self.m_callbacks_on_disable).inspect_value(path, offset, limit))
                .map(|v| ("m_CallbacksOnDisable".to_string(), v)),
            ((self.m_max_sub_step_count).inspect_value(path, offset, limit))
                .map(|v| ("m_MaxSubStepCount".to_string(), v)),
            ((self.m_min_sub_step_fps).inspect_value(path, offset, limit))
                .map(|v| ("m_MinSubStepFPS".to_string(), v)),
            ((self.m_auto_sync_transforms).inspect_value(path, offset, limit))
                .map(|v| ("m_AutoSyncTransforms".to_string(), v)),
            ((self.m_reuse_collision_callbacks).inspect_value(path, offset, limit))
                .map(|v| ("m_ReuseCollisionCallbacks".to_string(), v)),
            ((self.m_use_sub_stepping).inspect_value(path, offset, limit))
                .map(|v| ("m_UseSubStepping".to_string(), v)),
            ((self.m_use_sub_step_contacts).inspect_value(path, offset, limit))
                .map(|v| ("m_UseSubStepContacts".to_string(), v)),
            ((self.m_physics_low_level_settings).inspect_value(path, offset, limit))
                .map(|v| ("m_PhysicsLowLevelSettings".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
