//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::auto_off_mesh_link_data::AutoOffMeshLinkData;
use crate::subclasses::height_mesh_data::HeightMeshData;
use crate::subclasses::heightmap_data::HeightmapData;
use crate::subclasses::nav_mesh_params::NavMeshParams;
use crate::subclasses::nav_mesh_tile_data::NavMeshTileData;

#[derive(Debug, Clone)]
pub struct NavMeshData {
    pub m_object_hide_flags: u32,
    pub m_prefab_parent_object: PPtr,
    pub m_prefab_internal: PPtr,
    pub m_name: String,
    pub m_nav_mesh_tiles: Vec<NavMeshTileData>,
    pub m_nav_mesh_params: NavMeshParams,
    pub m_heightmaps: Vec<HeightmapData>,
    pub m_height_meshes: Vec<HeightMeshData>,
    pub m_off_mesh_links: Vec<AutoOffMeshLinkData>,
}

impl crate::Parse for NavMeshData {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_prefab_parent_object = crate::PPtr::parse(r, ver)?;
        let m_prefab_internal = crate::PPtr::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_nav_mesh_tiles = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { NavMeshTileData::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_nav_mesh_params = NavMeshParams::parse(r, ver)?;
        let m_heightmaps = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { HeightmapData::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_height_meshes = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { HeightMeshData::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_off_mesh_links = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { AutoOffMeshLinkData::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;

        Ok(NavMeshData {
            m_object_hide_flags,
            m_prefab_parent_object,
            m_prefab_internal,
            m_name,
            m_nav_mesh_tiles,
            m_nav_mesh_params,
            m_heightmaps,
            m_height_meshes,
            m_off_mesh_links,
        })
    }
}
