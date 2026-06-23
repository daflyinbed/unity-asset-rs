//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::occlusion_scene::OcclusionScene;
use crate::subclasses::scene_object_identifier::SceneObjectIdentifier;

#[derive(Debug, Clone)]
pub struct OcclusionCullingData {
    pub m_object_hide_flags: u32,
    pub m_prefab_parent_object: PPtr,
    pub m_prefab_internal: PPtr,
    pub m_name: String,
    pub m_pvs_data: Vec<u8>,
    pub m_scenes: Vec<OcclusionScene>,
    pub m_static_renderers: Vec<SceneObjectIdentifier>,
    pub m_portals: Vec<SceneObjectIdentifier>,
}

impl crate::Parse for OcclusionCullingData {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_prefab_parent_object = crate::PPtr::parse(r, ver)?;
        let m_prefab_internal = crate::PPtr::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_pvs_data = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u8() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_scenes = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { OcclusionScene::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_static_renderers = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { SceneObjectIdentifier::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_portals = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { SceneObjectIdentifier::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;

        Ok(OcclusionCullingData {
            m_object_hide_flags,
            m_prefab_parent_object,
            m_prefab_internal,
            m_name,
            m_pvs_data,
            m_scenes,
            m_static_renderers,
            m_portals,
        })
    }
}
