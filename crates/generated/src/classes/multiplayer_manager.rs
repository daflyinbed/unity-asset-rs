//! 自动生成。请勿手动修改。

use super::global_game_manager::GlobalGameManager;

#[derive(Debug, Clone)]
pub struct MultiplayerManager {
    pub parent: GlobalGameManager,
    pub m_active_multiplayer_role: Option<i32>,
    pub m_active_multiplayer_roles: Option<i32>,
}

impl crate::Parse for MultiplayerManager {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = GlobalGameManager::parse(r, ver)?;
        let m_active_multiplayer_role = if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_active_multiplayer_roles = if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };

        Ok(MultiplayerManager {
            parent,
            m_active_multiplayer_role,
            m_active_multiplayer_roles,
        })
    }
}
