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
