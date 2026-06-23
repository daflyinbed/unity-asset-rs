//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::prefab_modification::PrefabModification;

#[derive(Debug, Clone)]
pub struct PrefabInstance {
    pub m_object_hide_flags: u32,
    pub m_root_game_object: PPtr,
    pub m_modification: PrefabModification,
    pub m_source_prefab: PPtr,
}

impl crate::Parse for PrefabInstance {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_root_game_object = crate::PPtr::parse(r, ver)?;
        let m_modification = PrefabModification::parse(r, ver)?;
        r.align(4)?;
        let m_source_prefab = crate::PPtr::parse(r, ver)?;

        Ok(PrefabInstance {
            m_object_hide_flags,
            m_root_game_object,
            m_modification,
            m_source_prefab,
        })
    }
}
