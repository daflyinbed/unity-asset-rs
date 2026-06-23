//! 自动生成。请勿手动修改。

use crate::PPtr;
use super::global_game_manager::GlobalGameManager;

#[derive(Debug, Clone)]
pub struct PhysicsCoreProjectSettings2D {
    pub parent: GlobalGameManager,
    pub m_physics_core_settings: PPtr,
}

impl crate::Parse for PhysicsCoreProjectSettings2D {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = GlobalGameManager::parse(r, ver)?;
        let m_physics_core_settings = crate::PPtr::parse(r, ver)?;

        Ok(PhysicsCoreProjectSettings2D {
            parent,
            m_physics_core_settings,
        })
    }
}
