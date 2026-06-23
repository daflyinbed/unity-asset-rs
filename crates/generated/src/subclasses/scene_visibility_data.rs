//! 自动生成。请勿手动修改。

use crate::subclasses::guid::Guid;

#[derive(Debug, Clone)]
pub struct SceneVisibilityData {
    pub m_scene_guid: Guid,
}

impl crate::Parse for SceneVisibilityData {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_scene_guid = Guid::parse(r, ver)?;

        Ok(SceneVisibilityData {
            m_scene_guid,
        })
    }
}
