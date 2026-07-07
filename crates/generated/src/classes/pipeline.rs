//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct Pipeline {
    pub m_game_object: PPtr,
}

impl crate::Parse for Pipeline {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(r, ver)?;

        Ok(Pipeline {
            m_game_object,
        })
    }
}
