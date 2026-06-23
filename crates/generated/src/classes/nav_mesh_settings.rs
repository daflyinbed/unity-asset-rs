//! 自动生成。请勿手动修改。

use crate::PPtr;
use super::level_game_manager::LevelGameManager;

#[derive(Debug, Clone)]
pub struct NavMeshSettings {
    pub parent: LevelGameManager,
    pub m_nav_mesh: Option<PPtr>,
    pub m_nav_mesh_data: Option<PPtr>,
}

impl crate::Parse for NavMeshSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = LevelGameManager::parse(r, ver)?;
        let m_nav_mesh = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(crate::PPtr::parse(r, ver)?)
        } else {
            None
        };
        let m_nav_mesh_data = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(crate::PPtr::parse(r, ver)?)
        } else {
            None
        };

        Ok(NavMeshSettings {
            parent,
            m_nav_mesh,
            m_nav_mesh_data,
        })
    }
}
