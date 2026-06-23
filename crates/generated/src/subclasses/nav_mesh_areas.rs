//! 自动生成。请勿手动修改。

use crate::subclasses::nav_mesh_area_data::NavMeshAreaData;

#[derive(Debug, Clone)]
pub struct NavMeshAreas {
    pub m_object_hide_flags: u32,
    pub areas: Vec<NavMeshAreaData>,
}

impl crate::Parse for NavMeshAreas {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let areas = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { NavMeshAreaData::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(NavMeshAreas {
            m_object_hide_flags,
            areas,
        })
    }
}
