//! 自动生成。请勿手动修改。

use crate::PPtr;
use super::behaviour::Behaviour;

#[derive(Debug, Clone)]
pub struct Effector2D {
    pub parent: Behaviour,
    pub m_game_object: PPtr,
}

impl crate::Parse for Effector2D {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = Behaviour::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;

        Ok(Effector2D {
            parent,
            m_game_object,
        })
    }
}
