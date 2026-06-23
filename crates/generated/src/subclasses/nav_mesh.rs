//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::heightmap_data::HeightmapData;

#[derive(Debug, Clone)]
pub struct NavMesh {
    pub m_object_hide_flags: u32,
    pub m_prefab_parent_object: PPtr,
    pub m_prefab_internal: PPtr,
    pub m_name: String,
    pub m_mesh_data: Vec<u8>,
    pub m_heightmaps: Vec<HeightmapData>,
}

impl crate::Parse for NavMesh {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_prefab_parent_object = crate::PPtr::parse(r, ver)?;
        let m_prefab_internal = crate::PPtr::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_mesh_data = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u8() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_heightmaps = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { HeightmapData::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;

        Ok(NavMesh {
            m_object_hide_flags,
            m_prefab_parent_object,
            m_prefab_internal,
            m_name,
            m_mesh_data,
            m_heightmaps,
        })
    }
}
