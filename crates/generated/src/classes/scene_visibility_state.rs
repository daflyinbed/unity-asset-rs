//! 自动生成。请勿手动修改。

use crate::subclasses::scene_data_container::SceneDataContainer;
use crate::subclasses::scene_identifier::SceneIdentifier;
use crate::subclasses::scene_visibility_data::SceneVisibilityData;

#[derive(Debug, Clone)]
pub struct SceneVisibilityState {
    pub m_scene_data: Option<Vec<(SceneIdentifier, SceneVisibilityData)>>,
    pub m_scene_visibility_data: Option<SceneDataContainer>,
    pub m_main_stage_isolated: Option<bool>,
    pub m_scene_visibility_data_isolated: Option<SceneDataContainer>,
    pub m_prefab_stage_isolated: Option<bool>,
    pub m_scene_picking_data: Option<SceneDataContainer>,
    pub m_isolation_mode: Option<bool>,
}

impl crate::Parse for SceneVisibilityState {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_scene_data = None;
        let mut m_scene_visibility_data = None;
        let mut m_main_stage_isolated = None;
        let mut m_scene_visibility_data_isolated = None;
        let mut m_prefab_stage_isolated = None;
        let mut m_scene_picking_data = None;
        let mut m_isolation_mode = None;
        if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            m_scene_visibility_data = Some(SceneDataContainer::parse(rdr, ver)?);
            m_scene_visibility_data_isolated = Some(SceneDataContainer::parse(rdr, ver)?);
            m_scene_picking_data = Some(SceneDataContainer::parse(rdr, ver)?);
            m_isolation_mode = Some(rdr.read_bool()?);
        } else {
            m_scene_data = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = SceneIdentifier::parse(rdr, ver)?;
                                let b = SceneVisibilityData::parse(rdr, ver)?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_main_stage_isolated = Some(rdr.read_bool()?);
            m_prefab_stage_isolated = Some(rdr.read_bool()?);
        }

        Ok(SceneVisibilityState {
            m_scene_data,
            m_scene_visibility_data,
            m_main_stage_isolated,
            m_scene_visibility_data_isolated,
            m_prefab_stage_isolated,
            m_scene_picking_data,
            m_isolation_mode,
        })
    }
}

impl crate::ToValue for SceneVisibilityState {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            crate::opt_field("m_SceneData", &self.m_scene_data),
            crate::opt_field("m_SceneVisibilityData", &self.m_scene_visibility_data),
            crate::opt_field("m_MainStageIsolated", &self.m_main_stage_isolated),
            crate::opt_field(
                "m_SceneVisibilityDataIsolated",
                &self.m_scene_visibility_data_isolated,
            ),
            crate::opt_field("m_PrefabStageIsolated", &self.m_prefab_stage_isolated),
            crate::opt_field("m_ScenePickingData", &self.m_scene_picking_data),
            crate::opt_field("m_IsolationMode", &self.m_isolation_mode),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_SceneData" => (self.m_scene_data).inspect_value(path, offset, limit),
                "m_SceneVisibilityData" => {
                    (self.m_scene_visibility_data).inspect_value(path, offset, limit)
                }
                "m_MainStageIsolated" => {
                    (self.m_main_stage_isolated).inspect_value(path, offset, limit)
                }
                "m_SceneVisibilityDataIsolated" => {
                    (self.m_scene_visibility_data_isolated).inspect_value(path, offset, limit)
                }
                "m_PrefabStageIsolated" => {
                    (self.m_prefab_stage_isolated).inspect_value(path, offset, limit)
                }
                "m_ScenePickingData" => {
                    (self.m_scene_picking_data).inspect_value(path, offset, limit)
                }
                "m_IsolationMode" => (self.m_isolation_mode).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_scene_data).inspect_value(path, offset, limit))
                .map(|v| ("m_SceneData".to_string(), v)),
            ((self.m_scene_visibility_data).inspect_value(path, offset, limit))
                .map(|v| ("m_SceneVisibilityData".to_string(), v)),
            ((self.m_main_stage_isolated).inspect_value(path, offset, limit))
                .map(|v| ("m_MainStageIsolated".to_string(), v)),
            ((self.m_scene_visibility_data_isolated).inspect_value(path, offset, limit))
                .map(|v| ("m_SceneVisibilityDataIsolated".to_string(), v)),
            ((self.m_prefab_stage_isolated).inspect_value(path, offset, limit))
                .map(|v| ("m_PrefabStageIsolated".to_string(), v)),
            ((self.m_scene_picking_data).inspect_value(path, offset, limit))
                .map(|v| ("m_ScenePickingData".to_string(), v)),
            ((self.m_isolation_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_IsolationMode".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
