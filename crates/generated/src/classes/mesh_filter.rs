//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct MeshFilter {
    pub m_game_object: PPtr,
    pub m_mesh: PPtr,
}

impl crate::Parse for MeshFilter {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(rdr, ver)?;
        let m_mesh = crate::PPtr::parse(rdr, ver)?;

        Ok(MeshFilter {
            m_game_object,
            m_mesh,
        })
    }
}
