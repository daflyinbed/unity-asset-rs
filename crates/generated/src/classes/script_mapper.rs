//! 自动生成。请勿手动修改。

use crate::subclasses::name_to_object_map::NameToObjectMap;
use super::global_game_manager::GlobalGameManager;

#[derive(Debug, Clone)]
pub struct ScriptMapper {
    pub parent: GlobalGameManager,
    pub m_shaders: NameToObjectMap,
    pub m_preload_shaders: Option<bool>,
}

impl crate::Parse for ScriptMapper {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = GlobalGameManager::parse(r, ver)?;
        let m_shaders = NameToObjectMap::parse(r, ver)?;
        r.align(4)?;
        let m_preload_shaders = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };

        Ok(ScriptMapper {
            parent,
            m_shaders,
            m_preload_shaders,
        })
    }
}
