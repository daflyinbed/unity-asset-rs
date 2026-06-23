//! 自动生成。请勿手动修改。

use crate::subclasses::occlusion_scene::OcclusionScene;
use super::named_object::NamedObject;

#[derive(Debug, Clone)]
pub struct OcclusionCullingData {
    pub parent: NamedObject,
    pub m_name: String,
    pub m_pvs_data: Vec<u8>,
    pub m_scenes: Vec<OcclusionScene>,
}

impl crate::Parse for OcclusionCullingData {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = NamedObject::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_pvs_data = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u8() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_scenes = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { OcclusionScene::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;

        Ok(OcclusionCullingData {
            parent,
            m_name,
            m_pvs_data,
            m_scenes,
        })
    }
}
