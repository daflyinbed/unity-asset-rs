//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct Skybox {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_custom_skybox: PPtr,
}

impl crate::Parse for Skybox {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_u8()?;
        r.align(4)?;
        let m_custom_skybox = crate::PPtr::parse(r, ver)?;

        Ok(Skybox {
            m_game_object,
            m_enabled,
            m_custom_skybox,
        })
    }
}
