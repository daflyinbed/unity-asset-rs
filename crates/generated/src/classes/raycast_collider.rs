//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct RaycastCollider {
    pub m_game_object: PPtr,
    pub m_material: PPtr,
    pub m_is_trigger: bool,
    pub m_enabled: bool,
    pub m_center: Vector3f,
    pub m_length: f32,
}

impl crate::Parse for RaycastCollider {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_material = crate::PPtr::parse(r, ver)?;
        let m_is_trigger = r.read_bool()?;
        let m_enabled = r.read_bool()?;
        r.align(4)?;
        let m_center = Vector3f::parse(r, ver)?;
        let m_length = r.read_f32()?;

        Ok(RaycastCollider {
            m_game_object,
            m_material,
            m_is_trigger,
            m_enabled,
            m_center,
            m_length,
        })
    }
}
