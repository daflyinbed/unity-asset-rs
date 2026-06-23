//! 自动生成。请勿手动修改。

use crate::PPtr;
use super::component::Component;

#[derive(Debug, Clone)]
pub struct ParticleEmitter {
    pub parent: Component,
    pub m_game_object: PPtr,
}

impl crate::Parse for ParticleEmitter {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = Component::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;

        Ok(ParticleEmitter {
            parent,
            m_game_object,
        })
    }
}
