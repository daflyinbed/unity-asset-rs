//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::scene::Scene;
use super::object::Object;

#[derive(Debug, Clone)]
pub struct EditorBuildSettings {
    pub parent: Object,
    pub m_scenes: Vec<Scene>,
    pub m_config_objects: Option<Vec<(String, PPtr)>>,
    pub m_use_ucbp_for_asset_bundles: Option<bool>,
}

impl crate::Parse for EditorBuildSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = Object::parse(r, ver)?;
        let m_scenes = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Scene::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_config_objects = if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = crate::PPtr::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_use_ucbp_for_asset_bundles = if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };

        Ok(EditorBuildSettings {
            parent,
            m_scenes,
            m_config_objects,
            m_use_ucbp_for_asset_bundles,
        })
    }
}
