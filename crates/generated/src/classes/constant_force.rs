//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vector3f::Vector3f;
use super::behaviour::Behaviour;

#[derive(Debug, Clone)]
pub struct ConstantForce {
    pub parent: Behaviour,
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_force: Vector3f,
    pub m_relative_force: Vector3f,
    pub m_torque: Vector3f,
    pub m_relative_torque: Vector3f,
}

impl crate::Parse for ConstantForce {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = Behaviour::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_u8()?;
        r.align(4)?;
        let m_force = Vector3f::parse(r, ver)?;
        let m_relative_force = Vector3f::parse(r, ver)?;
        let m_torque = Vector3f::parse(r, ver)?;
        let m_relative_torque = Vector3f::parse(r, ver)?;

        Ok(ConstantForce {
            parent,
            m_game_object,
            m_enabled,
            m_force,
            m_relative_force,
            m_torque,
            m_relative_torque,
        })
    }
}
