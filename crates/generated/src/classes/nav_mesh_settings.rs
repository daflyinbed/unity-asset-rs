//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct NavMeshSettings {
    pub m_nav_mesh: Option<PPtr>,
    pub m_nav_mesh_data: Option<PPtr>,
}

impl crate::Parse for NavMeshSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_nav_mesh = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(crate::PPtr::parse(rdr, ver)?)
        } else {
            None
        };
        let m_nav_mesh_data = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(crate::PPtr::parse(rdr, ver)?)
        } else {
            None
        };

        Ok(NavMeshSettings {
            m_nav_mesh,
            m_nav_mesh_data,
        })
    }
}
