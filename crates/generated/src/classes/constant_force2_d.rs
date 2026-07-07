//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vector2f::Vector2f;

#[derive(Debug, Clone)]
pub struct ConstantForce2D {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_force: Vector2f,
    pub m_relative_force: Vector2f,
    pub m_torque: f32,
}

impl crate::Parse for ConstantForce2D {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_u8()?;
        r.align(4)?;
        let m_force = Vector2f::parse(r, ver)?;
        let m_relative_force = Vector2f::parse(r, ver)?;
        let m_torque = r.read_f32()?;

        Ok(ConstantForce2D {
            m_game_object,
            m_enabled,
            m_force,
            m_relative_force,
            m_torque,
        })
    }
}
