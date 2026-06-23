//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::soft_joint_limit::SoftJointLimit;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct CharacterJoint {
    pub m_object_hide_flags: u32,
    pub m_extension_ptr: PPtr,
    pub m_game_object: PPtr,
    pub m_connected_body: PPtr,
    pub m_anchor: Vector3f,
    pub m_axis: Vector3f,
    pub m_swing_axis: Vector3f,
    pub m_low_twist_limit: SoftJointLimit,
    pub m_high_twist_limit: SoftJointLimit,
    pub m_swing1_limit: SoftJointLimit,
    pub m_swing2_limit: SoftJointLimit,
    pub m_break_force: f32,
    pub m_break_torque: f32,
}

impl crate::Parse for CharacterJoint {
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
        let m_swing_axis = Vector3f::parse(r, ver)?;
        let m_low_twist_limit = SoftJointLimit::parse(r, ver)?;
        let m_high_twist_limit = SoftJointLimit::parse(r, ver)?;
        let m_swing1_limit = SoftJointLimit::parse(r, ver)?;
        let m_swing2_limit = SoftJointLimit::parse(r, ver)?;
        let m_break_force = r.read_f32()?;
        let m_break_torque = r.read_f32()?;

        Ok(CharacterJoint {
            m_object_hide_flags,
            m_extension_ptr,
            m_game_object,
            m_connected_body,
            m_anchor,
            m_axis,
            m_swing_axis,
            m_low_twist_limit,
            m_high_twist_limit,
            m_swing1_limit,
            m_swing2_limit,
            m_break_force,
            m_break_torque,
        })
    }
}
