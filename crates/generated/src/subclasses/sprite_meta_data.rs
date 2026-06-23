//! 自动生成。请勿手动修改。

use crate::subclasses::rectf::Rectf;
use crate::subclasses::vector2f::Vector2f;

#[derive(Debug, Clone)]
pub struct SpriteMetaData {
    pub m_name: String,
    pub m_rect: Rectf,
    pub m_alignment: i32,
    pub m_pivot: Vector2f,
}

impl crate::Parse for SpriteMetaData {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_rect = Rectf::parse(r, ver)?;
        let m_alignment = r.read_i32()?;
        let m_pivot = Vector2f::parse(r, ver)?;

        Ok(SpriteMetaData {
            m_name,
            m_rect,
            m_alignment,
            m_pivot,
        })
    }
}
