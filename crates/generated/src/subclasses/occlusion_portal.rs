//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct OcclusionPortal {
    pub m_object_hide_flags: u32,
    pub m_prefab_parent_object: PPtr,
    pub m_prefab_internal: PPtr,
    pub m_game_object: PPtr,
    pub m_open: bool,
    pub m_center: Vector3f,
    pub m_size: Vector3f,
}

impl crate::Parse for OcclusionPortal {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_prefab_parent_object = crate::PPtr::parse(r, ver)?;
        let m_prefab_internal = crate::PPtr::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_open = r.read_bool()?;
        r.align(4)?;
        let m_center = Vector3f::parse(r, ver)?;
        let m_size = Vector3f::parse(r, ver)?;

        Ok(OcclusionPortal {
            m_object_hide_flags,
            m_prefab_parent_object,
            m_prefab_internal,
            m_game_object,
            m_open,
            m_center,
            m_size,
        })
    }
}
