//! 自动生成。请勿手动修改。

use crate::subclasses::aabb::Aabb;
use crate::subclasses::auto_off_mesh_link_data::AutoOffMeshLinkData;
use crate::subclasses::height_mesh_data::HeightMeshData;
use crate::subclasses::heightmap_data::HeightmapData;
use crate::subclasses::nav_mesh_build_settings::NavMeshBuildSettings;
use crate::subclasses::nav_mesh_params::NavMeshParams;
use crate::subclasses::nav_mesh_tile_data::NavMeshTileData;
use crate::subclasses::quaternionf::Quaternionf;
use crate::subclasses::vector3f::Vector3f;
use super::named_object::NamedObject;

#[derive(Debug, Clone)]
pub struct NavMeshData {
    pub parent: NamedObject,
    pub m_name: String,
    pub m_nav_mesh_tiles: Vec<NavMeshTileData>,
    pub m_nav_mesh_params: Option<NavMeshParams>,
    pub m_nav_mesh_build_settings: Option<NavMeshBuildSettings>,
    pub m_heightmaps: Vec<HeightmapData>,
    pub m_height_meshes: Vec<HeightMeshData>,
    pub m_off_mesh_links: Vec<AutoOffMeshLinkData>,
    pub m_source_bounds: Option<Aabb>,
    pub m_rotation: Option<Quaternionf>,
    pub m_position: Option<Vector3f>,
    pub m_agent_type_id: Option<i32>,
}

impl crate::Parse for NavMeshData {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = NamedObject::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_nav_mesh_tiles = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { NavMeshTileData::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_nav_mesh_params = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(NavMeshParams::parse(r, ver)?)
        } else {
            None
        };
        let m_nav_mesh_build_settings = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = NavMeshBuildSettings::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_heightmaps = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { HeightmapData::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_height_meshes = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { HeightMeshData::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_off_mesh_links = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { AutoOffMeshLinkData::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_source_bounds = if *ver >= crate::UnityVersion::new(5, 6, 1) {
            Some(Aabb::parse(r, ver)?)
        } else {
            None
        };
        let m_rotation = if *ver >= crate::UnityVersion::new(5, 6, 1) {
            Some(Quaternionf::parse(r, ver)?)
        } else {
            None
        };
        let m_position = if *ver >= crate::UnityVersion::new(5, 6, 1) {
            Some(Vector3f::parse(r, ver)?)
        } else {
            None
        };
        let m_agent_type_id = if *ver >= crate::UnityVersion::new(5, 6, 1) {
            Some(r.read_i32()?)
        } else {
            None
        };

        Ok(NavMeshData {
            parent,
            m_name,
            m_nav_mesh_tiles,
            m_nav_mesh_params,
            m_nav_mesh_build_settings,
            m_heightmaps,
            m_height_meshes,
            m_off_mesh_links,
            m_source_bounds,
            m_rotation,
            m_position,
            m_agent_type_id,
        })
    }
}
