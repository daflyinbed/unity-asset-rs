//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::prefab_modification::PrefabModification;

#[derive(Debug, Clone)]
pub struct Prefab {
    pub m_object_hide_flags: u32,
    pub m_modification: PrefabModification,
    pub m_parent_prefab: PPtr,
    pub m_root_game_object: PPtr,
    pub m_is_prefab_parent: bool,
    pub m_is_exploded: bool,
}

impl crate::Parse for Prefab {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_modification = PrefabModification::parse(r, ver)?;
        r.align(4)?;
        let m_parent_prefab = crate::PPtr::parse(r, ver)?;
        let m_root_game_object = crate::PPtr::parse(r, ver)?;
        let m_is_prefab_parent = r.read_bool()?;
        let m_is_exploded = r.read_bool()?;
        r.align(4)?;

        Ok(Prefab {
            m_object_hide_flags,
            m_modification,
            m_parent_prefab,
            m_root_game_object,
            m_is_prefab_parent,
            m_is_exploded,
        })
    }
}
