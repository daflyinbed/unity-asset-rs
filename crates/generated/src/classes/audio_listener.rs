//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct AudioListener {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
}

impl crate::Parse for AudioListener {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_u8()?;
        r.align(4)?;

        Ok(AudioListener {
            m_game_object,
            m_enabled,
        })
    }
}
