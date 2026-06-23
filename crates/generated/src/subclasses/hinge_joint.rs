//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::joint_limits::JointLimits;
use crate::subclasses::joint_motor::JointMotor;
use crate::subclasses::joint_spring::JointSpring;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct HingeJoint {
    pub m_object_hide_flags: u32,
    pub m_extension_ptr: PPtr,
    pub m_game_object: PPtr,
    pub m_connected_body: PPtr,
    pub m_anchor: Vector3f,
    pub m_axis: Vector3f,
    pub m_use_spring: bool,
    pub m_spring: JointSpring,
    pub m_use_motor: bool,
    pub m_motor: JointMotor,
    pub m_use_limits: bool,
    pub m_limits: JointLimits,
    pub m_break_force: f32,
    pub m_break_torque: f32,
}

impl crate::Parse for HingeJoint {
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
        let m_use_spring = r.read_bool()?;
        r.align(4)?;
        let m_spring = JointSpring::parse(r, ver)?;
        let m_use_motor = r.read_bool()?;
        r.align(4)?;
        let m_motor = JointMotor::parse(r, ver)?;
        let m_use_limits = r.read_bool()?;
        r.align(4)?;
        let m_limits = JointLimits::parse(r, ver)?;
        let m_break_force = r.read_f32()?;
        let m_break_torque = r.read_f32()?;

        Ok(HingeJoint {
            m_object_hide_flags,
            m_extension_ptr,
            m_game_object,
            m_connected_body,
            m_anchor,
            m_axis,
            m_use_spring,
            m_spring,
            m_use_motor,
            m_motor,
            m_use_limits,
            m_limits,
            m_break_force,
            m_break_torque,
        })
    }
}
