//! 自动生成。请勿手动修改。

use crate::classes::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct MinMaxAabb {
    pub m_min: Vector3f,
    pub m_max: Vector3f,
}

impl crate::Parse for MinMaxAabb {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_min = Vector3f::parse(r, ver)?;
        let m_max = Vector3f::parse(r, ver)?;

        Ok(MinMaxAabb {
            m_min,
            m_max,
        })
    }
}
