//! 自动生成。请勿手动修改。

use crate::subclasses::scene::Scene;

#[derive(Debug, Clone)]
pub struct EditorBuildSettings {
    pub m_object_hide_flags: u32,
    pub m_scenes: Vec<Scene>,
}

impl crate::Parse for EditorBuildSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_scenes = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Scene::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(EditorBuildSettings {
            m_object_hide_flags,
            m_scenes,
        })
    }
}
