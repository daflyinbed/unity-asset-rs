//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct DirectorPlayer {
    pub m_game_object: PPtr,
}

impl crate::Parse for DirectorPlayer {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(rdr, ver)?;

        Ok(DirectorPlayer { m_game_object })
    }
}
