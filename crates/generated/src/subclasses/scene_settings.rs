//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::occlusion_bake_settings::OcclusionBakeSettings;

#[derive(Debug, Clone)]
pub struct SceneSettings {
    pub m_object_hide_flags: u32,
    pub m_pvs_data: Vec<u8>,
    pub m_query_mode: u32,
    pub m_pvs_objects_array: Vec<PPtr>,
    pub m_pvs_portals_array: Vec<PPtr>,
    pub m_occlusion_bake_settings: OcclusionBakeSettings,
}

impl crate::Parse for SceneSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_pvs_data = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u8() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_query_mode = r.read_u32()?;
        let m_pvs_objects_array = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_pvs_portals_array = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_occlusion_bake_settings = OcclusionBakeSettings::parse(r, ver)?;

        Ok(SceneSettings {
            m_object_hide_flags,
            m_pvs_data,
            m_query_mode,
            m_pvs_objects_array,
            m_pvs_portals_array,
            m_occlusion_bake_settings,
        })
    }
}
