//! 自动生成。请勿手动修改。

use crate::PPtr;
use super::component::Component;

#[derive(Debug, Clone)]
pub struct Joint {
    pub parent: Component,
    pub m_game_object: PPtr,
}

impl crate::Parse for Joint {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = Component::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;

        Ok(Joint {
            parent,
            m_game_object,
        })
    }
}
