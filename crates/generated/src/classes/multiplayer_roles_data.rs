//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::object_role_pair::ObjectRolePair;

#[derive(Debug, Clone)]
pub struct MultiplayerRolesData {
    pub m_game_object: PPtr,
    pub m_game_object_roles_mask: i32,
    pub m_components_roles_masks: Vec<ObjectRolePair>,
}

impl crate::Parse for MultiplayerRolesData {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(rdr, ver)?;
        let m_game_object_roles_mask = rdr.read_i32()?;
        let m_components_roles_masks = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| ObjectRolePair::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;

        Ok(MultiplayerRolesData {
            m_game_object,
            m_game_object_roles_mask,
            m_components_roles_masks,
        })
    }
}
