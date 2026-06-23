//! 自动生成。请勿手动修改。

use crate::subclasses::hierarchical_scene_data::HierarchicalSceneData;
use crate::subclasses::scene_identifier::SceneIdentifier;

#[derive(Debug, Clone)]
pub struct SceneDataContainer {
    pub m_scene_data: Vec<(SceneIdentifier, HierarchicalSceneData)>,
}

impl crate::Parse for SceneDataContainer {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_scene_data = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = SceneIdentifier::parse(r, ver)?; let b = HierarchicalSceneData::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;

        Ok(SceneDataContainer {
            m_scene_data,
        })
    }
}
