//! 自动生成。请勿手动修改。

use crate::PPtr;
use super::joint2_d::Joint2D;

#[derive(Debug, Clone)]
pub struct AnchoredJoint2D {
    pub parent: Joint2D,
    pub m_game_object: PPtr,
}

impl crate::Parse for AnchoredJoint2D {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = Joint2D::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;

        Ok(AnchoredJoint2D {
            parent,
            m_game_object,
        })
    }
}
