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
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(rdr, ver)?;
        let m_enabled = rdr.read_u8()?;
        rdr.align(4)?;
        let m_custom_skybox = crate::PPtr::parse(rdr, ver)?;

        Ok(Skybox {
            m_game_object,
            m_enabled,
            m_custom_skybox,
        })
    }
}
