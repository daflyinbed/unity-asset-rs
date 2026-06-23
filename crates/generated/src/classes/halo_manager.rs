//! 自动生成。请勿手动修改。

use super::level_game_manager::LevelGameManager;

#[derive(Debug, Clone)]
pub struct HaloManager {
    pub parent: LevelGameManager,
}

impl crate::Parse for HaloManager {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = LevelGameManager::parse(r, ver)?;

        Ok(HaloManager {
            parent,
        })
    }
}
