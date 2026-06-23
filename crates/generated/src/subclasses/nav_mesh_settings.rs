//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::nav_mesh_build_settings::NavMeshBuildSettings;

#[derive(Debug, Clone)]
pub struct NavMeshSettings {
    pub m_object_hide_flags: u32,
    pub m_build_settings: NavMeshBuildSettings,
    pub m_nav_mesh: PPtr,
}

impl crate::Parse for NavMeshSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_build_settings = NavMeshBuildSettings::parse(r, ver)?;
        r.align(4)?;
        let m_nav_mesh = crate::PPtr::parse(r, ver)?;

        Ok(NavMeshSettings {
            m_object_hide_flags,
            m_build_settings,
            m_nav_mesh,
        })
    }
}
