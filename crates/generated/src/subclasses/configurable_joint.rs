//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::joint_drive::JointDrive;
use crate::subclasses::quaternionf::Quaternionf;
use crate::subclasses::soft_joint_limit::SoftJointLimit;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct ConfigurableJoint {
    pub m_object_hide_flags: u32,
    pub m_extension_ptr: PPtr,
    pub m_game_object: PPtr,
    pub m_connected_body: PPtr,
    pub m_anchor: Vector3f,
    pub m_axis: Vector3f,
    pub m_secondary_axis: Vector3f,
    pub m_x_motion: i32,
    pub m_y_motion: i32,
    pub m_z_motion: i32,
    pub m_angular_x_motion: i32,
    pub m_angular_y_motion: i32,
    pub m_angular_z_motion: i32,
    pub m_linear_limit: SoftJointLimit,
    pub m_low_angular_x_limit: SoftJointLimit,
    pub m_high_angular_x_limit: SoftJointLimit,
    pub m_angular_y_limit: SoftJointLimit,
    pub m_angular_z_limit: SoftJointLimit,
    pub m_target_position: Vector3f,
    pub m_target_velocity: Vector3f,
    pub m_x_drive: JointDrive,
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
    pub m_configured_in_world_space: bool,
    pub m_break_force: f32,
    pub m_break_torque: f32,
}

impl crate::Parse for ConfigurableJoint {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_extension_ptr = crate::PPtr::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_connected_body = crate::PPtr::parse(r, ver)?;
        let m_anchor = Vector3f::parse(r, ver)?;
        let m_axis = Vector3f::parse(r, ver)?;
        let m_secondary_axis = Vector3f::parse(r, ver)?;
        let m_x_motion = r.read_i32()?;
        let m_y_motion = r.read_i32()?;
        let m_z_motion = r.read_i32()?;
        let m_angular_x_motion = r.read_i32()?;
        let m_angular_y_motion = r.read_i32()?;
        let m_angular_z_motion = r.read_i32()?;
        let m_linear_limit = SoftJointLimit::parse(r, ver)?;
        let m_low_angular_x_limit = SoftJointLimit::parse(r, ver)?;
        let m_high_angular_x_limit = SoftJointLimit::parse(r, ver)?;
        let m_angular_y_limit = SoftJointLimit::parse(r, ver)?;
        let m_angular_z_limit = SoftJointLimit::parse(r, ver)?;
        let m_target_position = Vector3f::parse(r, ver)?;
        let m_target_velocity = Vector3f::parse(r, ver)?;
        let m_x_drive = JointDrive::parse(r, ver)?;
        let m_y_drive = JointDrive::parse(r, ver)?;
        let m_z_drive = JointDrive::parse(r, ver)?;
        let m_target_rotation = Quaternionf::parse(r, ver)?;
        let m_target_angular_velocity = Vector3f::parse(r, ver)?;
        let m_rotation_drive_mode = r.read_i32()?;
        let m_angular_x_drive = JointDrive::parse(r, ver)?;
        let m_angular_yz_drive = JointDrive::parse(r, ver)?;
        let m_slerp_drive = JointDrive::parse(r, ver)?;
        let m_projection_mode = r.read_i32()?;
        let m_projection_distance = r.read_f32()?;
        let m_projection_angle = r.read_f32()?;
        let m_configured_in_world_space = r.read_bool()?;
        r.align(4)?;
        let m_break_force = r.read_f32()?;
        let m_break_torque = r.read_f32()?;

        Ok(ConfigurableJoint {
            m_object_hide_flags,
            m_extension_ptr,
            m_game_object,
            m_connected_body,
            m_anchor,
            m_axis,
            m_secondary_axis,
            m_x_motion,
            m_y_motion,
            m_z_motion,
            m_angular_x_motion,
            m_angular_y_motion,
            m_angular_z_motion,
            m_linear_limit,
            m_low_angular_x_limit,
            m_high_angular_x_limit,
            m_angular_y_limit,
            m_angular_z_limit,
            m_target_position,
            m_target_velocity,
            m_x_drive,
            m_y_drive,
            m_z_drive,
            m_target_rotation,
            m_target_angular_velocity,
            m_rotation_drive_mode,
            m_angular_x_drive,
            m_angular_yz_drive,
            m_slerp_drive,
            m_projection_mode,
            m_projection_distance,
            m_projection_angle,
            m_configured_in_world_space,
            m_break_force,
            m_break_torque,
        })
    }
}
