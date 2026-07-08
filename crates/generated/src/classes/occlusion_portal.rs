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
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(rdr, ver)?;
        let m_open = rdr.read_bool()?;
        rdr.align(4)?;
        let m_center = Vector3f::parse(rdr, ver)?;
        let m_size = Vector3f::parse(rdr, ver)?;

        Ok(OcclusionPortal {
            m_game_object,
            m_open,
            m_center,
            m_size,
        })
    }
}
