//! 自动生成。请勿手动修改。

use super::game_manager::GameManager;

#[derive(Debug, Clone)]
pub struct GlobalGameManager {
    pub parent: GameManager,
}

impl crate::Parse for GlobalGameManager {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = GameManager::parse(r, ver)?;

        Ok(GlobalGameManager {
            parent,
        })
    }
}
