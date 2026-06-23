//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::joint_motor2_d::JointMotor2D;
use crate::subclasses::joint_translation_limits2_d::JointTranslationLimits2D;
use crate::subclasses::vector2f::Vector2f;

#[derive(Debug, Clone)]
pub struct SliderJoint2D {
    pub m_object_hide_flags: u32,
    pub m_prefab_parent_object: PPtr,
    pub m_prefab_internal: PPtr,
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_collide_connected: bool,
    pub m_connected_rigid_body: PPtr,
    pub m_anchor: Vector2f,
    pub m_connected_anchor: Vector2f,
    pub m_angle: f32,
    pub m_use_motor: bool,
    pub m_motor: JointMotor2D,
    pub m_use_limits: bool,
    pub m_translation_limits: JointTranslationLimits2D,
}

impl crate::Parse for SliderJoint2D {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_prefab_parent_object = crate::PPtr::parse(r, ver)?;
        let m_prefab_internal = crate::PPtr::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_u8()?;
        r.align(4)?;
        let m_collide_connected = r.read_bool()?;
        r.align(4)?;
        let m_connected_rigid_body = crate::PPtr::parse(r, ver)?;
        let m_anchor = Vector2f::parse(r, ver)?;
        let m_connected_anchor = Vector2f::parse(r, ver)?;
        let m_angle = r.read_f32()?;
        let m_use_motor = r.read_bool()?;
        r.align(4)?;
        let m_motor = JointMotor2D::parse(r, ver)?;
        let m_use_limits = r.read_bool()?;
        r.align(4)?;
        let m_translation_limits = JointTranslationLimits2D::parse(r, ver)?;

        Ok(SliderJoint2D {
            m_object_hide_flags,
            m_prefab_parent_object,
            m_prefab_internal,
            m_game_object,
            m_enabled,
            m_collide_connected,
            m_connected_rigid_body,
            m_anchor,
            m_connected_anchor,
            m_angle,
            m_use_motor,
            m_motor,
            m_use_limits,
            m_translation_limits,
        })
    }
}
