//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::object_role_pair::ObjectRolePair;

#[derive(Debug, Clone)]
pub struct MultiplayerRolesData {
    pub m_object_hide_flags: u32,
    pub m_corresponding_source_object: PPtr,
    pub m_prefab_instance: PPtr,
    pub m_prefab_asset: PPtr,
    pub m_game_object: PPtr,
    pub m_game_object_roles_mask: i32,
    pub m_components_roles_masks: Vec<ObjectRolePair>,
}

impl crate::Parse for MultiplayerRolesData {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_corresponding_source_object = crate::PPtr::parse(r, ver)?;
        let m_prefab_instance = crate::PPtr::parse(r, ver)?;
        let m_prefab_asset = crate::PPtr::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_game_object_roles_mask = r.read_i32()?;
        let m_components_roles_masks = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ObjectRolePair::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(MultiplayerRolesData {
            m_object_hide_flags,
            m_corresponding_source_object,
            m_prefab_instance,
            m_prefab_asset,
            m_game_object,
            m_game_object_roles_mask,
            m_components_roles_masks,
        })
    }
}
