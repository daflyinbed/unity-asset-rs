//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct HaloLayer {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
}

impl crate::Parse for HaloLayer {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(rdr, ver)?;
        let m_enabled = rdr.read_u8()?;
        rdr.align(4)?;

        Ok(HaloLayer {
            m_game_object,
            m_enabled,
        })
    }
}
