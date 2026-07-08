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


/// 版本枚举：NavMeshDataMNavMeshTilesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum NavMeshDataMNavMeshTilesVariant {
    Default(Vec<NavMeshTileData>),
    V2017_1(Vec<NavMeshTileData>),
}

impl crate::Parse for NavMeshDataMNavMeshTilesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { NavMeshTileData::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { NavMeshTileData::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：NavMeshDataMHeightmapsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum NavMeshDataMHeightmapsVariant {
    Default(Vec<HeightmapData>),
    V2017_1(Vec<HeightmapData>),
}

impl crate::Parse for NavMeshDataMHeightmapsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { HeightmapData::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { HeightmapData::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：NavMeshDataMHeightMeshesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum NavMeshDataMHeightMeshesVariant {
    Default(Vec<HeightMeshData>),
    V2017_1(Vec<HeightMeshData>),
}

impl crate::Parse for NavMeshDataMHeightMeshesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { HeightMeshData::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { HeightMeshData::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：NavMeshDataMOffMeshLinksVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum NavMeshDataMOffMeshLinksVariant {
    Default(Vec<AutoOffMeshLinkData>),
    V2017_1(Vec<AutoOffMeshLinkData>),
}

impl crate::Parse for NavMeshDataMOffMeshLinksVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { AutoOffMeshLinkData::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { AutoOffMeshLinkData::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct NavMeshData {
    pub m_name: String,
    pub m_nav_mesh_tiles: NavMeshDataMNavMeshTilesVariant,
    pub m_nav_mesh_params: Option<NavMeshParams>,
    pub m_nav_mesh_build_settings: Option<NavMeshBuildSettings>,
    pub m_heightmaps: NavMeshDataMHeightmapsVariant,
    pub m_height_meshes: NavMeshDataMHeightMeshesVariant,
    pub m_off_mesh_links: NavMeshDataMOffMeshLinksVariant,
    pub m_source_bounds: Option<Aabb>,
    pub m_rotation: Option<Quaternionf>,
    pub m_position: Option<Vector3f>,
    pub m_agent_type_id: Option<i32>,
}

impl crate::Parse for NavMeshData {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut m_nav_mesh_tiles = None;
        let mut m_nav_mesh_params = None;
        let mut m_nav_mesh_build_settings = None;
        let mut m_heightmaps = None;
        let mut m_height_meshes = None;
        let mut m_off_mesh_links = None;
        let mut m_source_bounds = None;
        let mut m_rotation = None;
        let mut m_position = None;
        let mut m_agent_type_id = None;
            m_name = Some(rdr.read_aligned_string()?);
            m_nav_mesh_tiles = Some(NavMeshDataMNavMeshTilesVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_nav_mesh_build_settings = Some(NavMeshBuildSettings::parse(rdr, ver)?);
        } else {
            m_nav_mesh_params = Some(NavMeshParams::parse(rdr, ver)?);
        }
            m_heightmaps = Some(NavMeshDataMHeightmapsVariant::parse(rdr, ver)?);
            m_height_meshes = Some(NavMeshDataMHeightMeshesVariant::parse(rdr, ver)?);
            m_off_mesh_links = Some(NavMeshDataMOffMeshLinksVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 6, 1) {
            m_source_bounds = Some(Aabb::parse(rdr, ver)?);
            m_rotation = Some(Quaternionf::parse(rdr, ver)?);
            m_position = Some(Vector3f::parse(rdr, ver)?);
            m_agent_type_id = Some(rdr.read_i32()?);
        } else {
        }

        Ok(NavMeshData {
            m_name: m_name.unwrap(),
            m_nav_mesh_tiles: m_nav_mesh_tiles.unwrap(),
            m_nav_mesh_params,
            m_nav_mesh_build_settings,
            m_heightmaps: m_heightmaps.unwrap(),
            m_height_meshes: m_height_meshes.unwrap(),
            m_off_mesh_links: m_off_mesh_links.unwrap(),
            m_source_bounds,
            m_rotation,
            m_position,
            m_agent_type_id,
        })
    }
}
