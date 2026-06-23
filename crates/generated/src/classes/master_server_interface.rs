//! 自动生成。请勿手动修改。

use super::global_game_manager::GlobalGameManager;

#[derive(Debug, Clone)]
pub struct MasterServerInterface {
    pub parent: GlobalGameManager,
}

impl crate::Parse for MasterServerInterface {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = GlobalGameManager::parse(r, ver)?;

        Ok(MasterServerInterface {
            parent,
        })
    }
}
