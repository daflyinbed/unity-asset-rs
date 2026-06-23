//! 自动生成。请勿手动修改。

use crate::subclasses::scene_identifier::SceneIdentifier;
use crate::subclasses::scene_visibility_data::SceneVisibilityData;

#[derive(Debug, Clone)]
pub struct SceneVisibilityState {
    pub m_object_hide_flags: u32,
    pub m_scene_data: Vec<(SceneIdentifier, SceneVisibilityData)>,
}

impl crate::Parse for SceneVisibilityState {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_scene_data = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = SceneIdentifier::parse(r, ver)?; let b = SceneVisibilityData::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;

        Ok(SceneVisibilityState {
            m_object_hide_flags,
            m_scene_data,
        })
    }
}
