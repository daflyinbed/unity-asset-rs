//! 自动生成。请勿手动修改。

use crate::subclasses::guid::Guid;

#[derive(Debug, Clone)]
pub struct HierarchicalSceneData {
    pub m_scene_guid: Guid,
}

impl crate::Parse for HierarchicalSceneData {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_scene_guid = Guid::parse(rdr, ver)?;

        Ok(HierarchicalSceneData {
            m_scene_guid,
        })
    }
}
