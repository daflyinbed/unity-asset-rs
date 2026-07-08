//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::joint_drive::JointDrive;
use crate::subclasses::quaternionf::Quaternionf;
use crate::subclasses::soft_joint_limit::SoftJointLimit;
use crate::subclasses::soft_joint_limit_spring::SoftJointLimitSpring;
use crate::subclasses::vector3f::Vector3f;


/// 版本枚举：ConfigurableJointMConfiguredInWorldSpaceVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ConfigurableJointMConfiguredInWorldSpaceVariant {
    Default(bool),
    V3_5(bool),
}

impl crate::Parse for ConfigurableJointMConfiguredInWorldSpaceVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = r.read_bool()?;
            Ok(Self::V3_5(v))
        } else {
            let v = r.read_bool()?;
            r.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：ConfigurableJointMEnablePreprocessingVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ConfigurableJointMEnablePreprocessingVariant {
    Default(bool),
    V2017_1(bool),
}

impl crate::Parse for ConfigurableJointMEnablePreprocessingVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = r.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct ConfigurableJoint {
    pub m_game_object: PPtr,
    pub m_connected_body: PPtr,
    pub m_anchor: Vector3f,
    pub m_connected_articulation_body: Option<PPtr>,
    pub m_axis: Vector3f,
    pub m_secondary_axis: Vector3f,
    pub m_auto_configure_connected_anchor: Option<bool>,
    pub m_x_motion: i32,
    pub m_connected_anchor: Option<Vector3f>,
    pub m_y_motion: i32,
    pub m_z_motion: i32,
    pub m_angular_x_motion: i32,
    pub m_angular_y_motion: i32,
    pub m_angular_z_motion: i32,
    pub m_linear_limit: SoftJointLimit,
    pub m_low_angular_x_limit: SoftJointLimit,
    pub m_high_angular_x_limit: SoftJointLimit,
    pub m_linear_limit_spring: Option<SoftJointLimitSpring>,
    pub m_angular_y_limit: SoftJointLimit,
    pub m_angular_z_limit: SoftJointLimit,
    pub m_angular_x_limit_spring: Option<SoftJointLimitSpring>,
    pub m_target_position: Vector3f,
    pub m_target_velocity: Vector3f,
    pub m_x_drive: JointDrive,
    pub m_angular_yz_limit_spring: Option<SoftJointLimitSpring>,
    pub m_y_drive: JointDrive,
    pub m_z_drive: JointDrive,
    pub m_target_rotation: Quaternionf,
    pub m_target_angular_velocity: Vector3f,
    pub m_rotation_drive_mode: i32,
    pub m_angular_x_drive: JointDrive,
    pub m_angular_yz_drive: JointDrive,
    pub m_slerp_drive: JointDrive,
    pub m_projection_mode: i32,
    pub m_projection_distance: f32,
    pub m_projection_angle: f32,
    pub m_configured_in_world_space: ConfigurableJointMConfiguredInWorldSpaceVariant,
    pub m_break_force: f32,
    pub m_swap_bodies: Option<bool>,
    pub m_break_torque: f32,
    pub m_enable_collision: Option<bool>,
    pub m_enable_preprocessing: Option<ConfigurableJointMEnablePreprocessingVariant>,
    pub m_mass_scale: Option<f32>,
    pub m_connected_mass_scale: Option<f32>,
}

impl crate::Parse for ConfigurableJoint {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut m_connected_body = None;
        let mut m_anchor = None;
        let mut m_connected_articulation_body = None;
        let mut m_axis = None;
        let mut m_secondary_axis = None;
        let mut m_auto_configure_connected_anchor = None;
        let mut m_x_motion = None;
        let mut m_connected_anchor = None;
        let mut m_y_motion = None;
        let mut m_z_motion = None;
        let mut m_angular_x_motion = None;
        let mut m_angular_y_motion = None;
        let mut m_angular_z_motion = None;
        let mut m_linear_limit = None;
        let mut m_low_angular_x_limit = None;
        let mut m_high_angular_x_limit = None;
        let mut m_linear_limit_spring = None;
        let mut m_angular_y_limit = None;
        let mut m_angular_z_limit = None;
        let mut m_angular_x_limit_spring = None;
        let mut m_target_position = None;
        let mut m_target_velocity = None;
        let mut m_x_drive = None;
        let mut m_angular_yz_limit_spring = None;
        let mut m_y_drive = None;
        let mut m_z_drive = None;
        let mut m_target_rotation = None;
        let mut m_target_angular_velocity = None;
        let mut m_rotation_drive_mode = None;
        let mut m_angular_x_drive = None;
        let mut m_angular_yz_drive = None;
        let mut m_slerp_drive = None;
        let mut m_projection_mode = None;
        let mut m_projection_distance = None;
        let mut m_projection_angle = None;
        let mut m_configured_in_world_space = None;
        let mut m_break_force = None;
        let mut m_swap_bodies = None;
        let mut m_break_torque = None;
        let mut m_enable_collision = None;
        let mut m_enable_preprocessing = None;
        let mut m_mass_scale = None;
        let mut m_connected_mass_scale = None;
            m_game_object = Some(crate::PPtr::parse(r, ver)?);
            m_connected_body = Some(crate::PPtr::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            m_connected_articulation_body = Some(crate::PPtr::parse(r, ver)?);
        } else {
        }
            m_anchor = Some(Vector3f::parse(r, ver)?);
            m_axis = Some(Vector3f::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_auto_configure_connected_anchor = Some(r.read_bool()?);
            r.align(4)?;
            m_connected_anchor = Some(Vector3f::parse(r, ver)?);
        } else {
        }
            m_secondary_axis = Some(Vector3f::parse(r, ver)?);
            m_x_motion = Some(r.read_i32()?);
            m_y_motion = Some(r.read_i32()?);
            m_z_motion = Some(r.read_i32()?);
            m_angular_x_motion = Some(r.read_i32()?);
            m_angular_y_motion = Some(r.read_i32()?);
            m_angular_z_motion = Some(r.read_i32()?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_linear_limit_spring = Some(SoftJointLimitSpring::parse(r, ver)?);
        } else {
        }
            m_linear_limit = Some(SoftJointLimit::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_angular_x_limit_spring = Some(SoftJointLimitSpring::parse(r, ver)?);
        } else {
        }
            m_low_angular_x_limit = Some(SoftJointLimit::parse(r, ver)?);
            m_high_angular_x_limit = Some(SoftJointLimit::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_angular_yz_limit_spring = Some(SoftJointLimitSpring::parse(r, ver)?);
        } else {
        }
            m_angular_y_limit = Some(SoftJointLimit::parse(r, ver)?);
            m_angular_z_limit = Some(SoftJointLimit::parse(r, ver)?);
            m_target_position = Some(Vector3f::parse(r, ver)?);
            m_target_velocity = Some(Vector3f::parse(r, ver)?);
            m_x_drive = Some(JointDrive::parse(r, ver)?);
            m_y_drive = Some(JointDrive::parse(r, ver)?);
            m_z_drive = Some(JointDrive::parse(r, ver)?);
            m_target_rotation = Some(Quaternionf::parse(r, ver)?);
            m_target_angular_velocity = Some(Vector3f::parse(r, ver)?);
            m_rotation_drive_mode = Some(r.read_i32()?);
            m_angular_x_drive = Some(JointDrive::parse(r, ver)?);
            m_angular_yz_drive = Some(JointDrive::parse(r, ver)?);
            m_slerp_drive = Some(JointDrive::parse(r, ver)?);
            m_projection_mode = Some(r.read_i32()?);
            m_projection_distance = Some(r.read_f32()?);
            m_projection_angle = Some(r.read_f32()?);
            m_configured_in_world_space = Some(ConfigurableJointMConfiguredInWorldSpaceVariant::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_swap_bodies = Some(r.read_bool()?);
            r.align(4)?;
        } else {
        }
            m_break_force = Some(r.read_f32()?);
            m_break_torque = Some(r.read_f32()?);
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_enable_collision = Some(r.read_bool()?);
            m_enable_preprocessing = Some(ConfigurableJointMEnablePreprocessingVariant::parse(r, ver)?);
            m_mass_scale = Some(r.read_f32()?);
            m_connected_mass_scale = Some(r.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_enable_collision = Some(r.read_bool()?);
            m_enable_preprocessing = Some(ConfigurableJointMEnablePreprocessingVariant::parse(r, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 5, 0) {
            m_enable_collision = Some(r.read_bool()?);
        } else {
        }

        Ok(ConfigurableJoint {
            m_game_object: m_game_object.unwrap(),
            m_connected_body: m_connected_body.unwrap(),
            m_anchor: m_anchor.unwrap(),
            m_connected_articulation_body,
            m_axis: m_axis.unwrap(),
            m_secondary_axis: m_secondary_axis.unwrap(),
            m_auto_configure_connected_anchor,
            m_x_motion: m_x_motion.unwrap(),
            m_connected_anchor,
            m_y_motion: m_y_motion.unwrap(),
            m_z_motion: m_z_motion.unwrap(),
            m_angular_x_motion: m_angular_x_motion.unwrap(),
            m_angular_y_motion: m_angular_y_motion.unwrap(),
            m_angular_z_motion: m_angular_z_motion.unwrap(),
            m_linear_limit: m_linear_limit.unwrap(),
            m_low_angular_x_limit: m_low_angular_x_limit.unwrap(),
            m_high_angular_x_limit: m_high_angular_x_limit.unwrap(),
            m_linear_limit_spring,
            m_angular_y_limit: m_angular_y_limit.unwrap(),
            m_angular_z_limit: m_angular_z_limit.unwrap(),
            m_angular_x_limit_spring,
            m_target_position: m_target_position.unwrap(),
            m_target_velocity: m_target_velocity.unwrap(),
            m_x_drive: m_x_drive.unwrap(),
            m_angular_yz_limit_spring,
            m_y_drive: m_y_drive.unwrap(),
            m_z_drive: m_z_drive.unwrap(),
            m_target_rotation: m_target_rotation.unwrap(),
            m_target_angular_velocity: m_target_angular_velocity.unwrap(),
            m_rotation_drive_mode: m_rotation_drive_mode.unwrap(),
            m_angular_x_drive: m_angular_x_drive.unwrap(),
            m_angular_yz_drive: m_angular_yz_drive.unwrap(),
            m_slerp_drive: m_slerp_drive.unwrap(),
            m_projection_mode: m_projection_mode.unwrap(),
            m_projection_distance: m_projection_distance.unwrap(),
            m_projection_angle: m_projection_angle.unwrap(),
            m_configured_in_world_space: m_configured_in_world_space.unwrap(),
            m_break_force: m_break_force.unwrap(),
            m_swap_bodies,
            m_break_torque: m_break_torque.unwrap(),
            m_enable_collision,
            m_enable_preprocessing,
            m_mass_scale,
            m_connected_mass_scale,
        })
    }
}
