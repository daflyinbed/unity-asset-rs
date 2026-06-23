//! 自动生成。请勿手动修改。

use crate::PPtr;
use super::collider2_d::Collider2D;

#[derive(Debug, Clone)]
pub struct PolygonColliderBase2D {
    pub parent: Collider2D,
    pub m_game_object: PPtr,
}

impl crate::Parse for PolygonColliderBase2D {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = Collider2D::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;

        Ok(PolygonColliderBase2D {
            parent,
            m_game_object,
        })
    }
}
