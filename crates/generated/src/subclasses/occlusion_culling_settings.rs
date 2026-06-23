//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::guid::Guid;
use crate::subclasses::occlusion_bake_settings::OcclusionBakeSettings;

#[derive(Debug, Clone)]
pub struct OcclusionCullingSettings {
    pub m_object_hide_flags: u32,
    pub m_occlusion_bake_settings: OcclusionBakeSettings,
    pub m_scene_guid: Guid,
    pub m_occlusion_culling_data: PPtr,
}

impl crate::Parse for OcclusionCullingSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_occlusion_bake_settings = OcclusionBakeSettings::parse(r, ver)?;
        let m_scene_guid = Guid::parse(r, ver)?;
        let m_occlusion_culling_data = crate::PPtr::parse(r, ver)?;

        Ok(OcclusionCullingSettings {
            m_object_hide_flags,
            m_occlusion_bake_settings,
            m_scene_guid,
            m_occlusion_culling_data,
        })
    }
}
