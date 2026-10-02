//! 自动生成。请勿手动修改。

use crate::subclasses::hierarchical_scene_data::HierarchicalSceneData;
use crate::subclasses::scene_identifier::SceneIdentifier;

#[derive(Debug, Clone)]
pub struct SceneDataContainer {
    pub m_scene_data: Vec<(SceneIdentifier, HierarchicalSceneData)>,
}

impl crate::Parse for SceneDataContainer {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_scene_data = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| {
                    ({
                        let a = SceneIdentifier::parse(rdr, ver)?;
                        let b = HierarchicalSceneData::parse(rdr, ver)?;
                        Ok((a, b))
                    })
                })
                .collect::<Result<Vec<_>, _>>()
        })?;

        Ok(SceneDataContainer { m_scene_data })
    }
}

impl crate::ToValue for SceneDataContainer {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [Some((
            "m_SceneData".to_string(),
            self.m_scene_data.to_value(),
        ))]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_SceneData" => (self.m_scene_data).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_scene_data).inspect_value(path, offset, limit))
                .map(|v| ("m_SceneData".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
