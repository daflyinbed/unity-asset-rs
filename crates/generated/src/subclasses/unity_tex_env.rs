//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vector2f::Vector2f;

#[derive(Debug, Clone)]
pub struct UnityTexEnv {
    pub m_texture: PPtr,
    pub m_scale: Vector2f,
    pub m_offset: Vector2f,
}

impl crate::Parse for UnityTexEnv {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_texture = crate::PPtr::parse(rdr, ver)?;
        let m_scale = Vector2f::parse(rdr, ver)?;
        let m_offset = Vector2f::parse(rdr, ver)?;

        Ok(UnityTexEnv {
            m_texture,
            m_scale,
            m_offset,
        })
    }
}
