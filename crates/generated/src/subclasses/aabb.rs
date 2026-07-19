//! 自动生成。请勿手动修改。

use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct Aabb {
    pub m_center: Vector3f,
    pub m_extent: Vector3f,
}

impl crate::Parse for Aabb {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_center = Vector3f::parse(rdr, ver)?;
        let m_extent = Vector3f::parse(rdr, ver)?;

        Ok(Aabb { m_center, m_extent })
    }
}
