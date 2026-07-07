//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct OcclusionPortal {
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
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_open = r.read_bool()?;
        r.align(4)?;
        let m_center = Vector3f::parse(r, ver)?;
        let m_size = Vector3f::parse(r, ver)?;

        Ok(OcclusionPortal {
            m_game_object,
            m_open,
            m_center,
            m_size,
        })
    }
}
