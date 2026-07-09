//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct GridLayout {
    pub m_hide_flags: u32,
    pub m_corresponding_source_object: PPtr,
    pub m_prefab_internal: PPtr,
    pub m_game_object: PPtr,
}

impl crate::Parse for GridLayout {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_hide_flags = rdr.read_u32()?;
        let m_corresponding_source_object = crate::PPtr::parse(rdr, ver)?;
        let m_prefab_internal = crate::PPtr::parse(rdr, ver)?;
        let m_game_object = crate::PPtr::parse(rdr, ver)?;

        Ok(GridLayout {
            m_hide_flags,
            m_corresponding_source_object,
            m_prefab_internal,
            m_game_object,
        })
    }
}
