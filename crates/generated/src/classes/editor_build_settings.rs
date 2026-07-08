//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::classes::scene::Scene;

#[derive(Debug, Clone)]
pub struct EditorBuildSettings {
    pub m_scenes: Vec<Scene>,
    pub m_config_objects: Option<Vec<(String, PPtr)>>,
    pub m_use_ucbp_for_asset_bundles: Option<bool>,
}

impl crate::Parse for EditorBuildSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_scenes = None;
        let mut m_config_objects = None;
        let mut m_use_ucbp_for_asset_bundles = None;
            m_scenes = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { Scene::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(6000, 4, 0) {
            m_config_objects = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = rdr.read_aligned_string()?; let b = crate::PPtr::parse(rdr, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            m_config_objects = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = rdr.read_aligned_string()?; let b = crate::PPtr::parse(rdr, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_use_ucbp_for_asset_bundles = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            m_config_objects = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = rdr.read_aligned_string()?; let b = crate::PPtr::parse(rdr, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        } else {
        }

        Ok(EditorBuildSettings {
            m_scenes: m_scenes.unwrap(),
            m_config_objects,
            m_use_ucbp_for_asset_bundles,
        })
    }
}
