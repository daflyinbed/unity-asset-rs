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


/// 版本枚举：NavMeshDataMNavMeshBuildSettingsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum NavMeshDataMNavMeshBuildSettingsVariant {
    Default(NavMeshBuildSettings),
    V2017_2(NavMeshBuildSettings),
}

impl crate::Parse for NavMeshDataMNavMeshBuildSettingsVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            let v = NavMeshBuildSettings::parse(r, ver)?;
            r.align(4)?;
            Ok(Self::V2017_2(v))
        } else {
            let v = NavMeshBuildSettings::parse(r, ver)?;
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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { HeightmapData::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { HeightmapData::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { HeightMeshData::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { HeightMeshData::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { AutoOffMeshLinkData::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { AutoOffMeshLinkData::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct NavMeshData {
    pub m_name: String,
    pub m_nav_mesh_tiles: Vec<NavMeshTileData>,
    pub m_nav_mesh_params: Option<NavMeshParams>,
    pub m_nav_mesh_build_settings: Option<NavMeshDataMNavMeshBuildSettingsVariant>,
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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
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
            Some(NavMeshDataMNavMeshBuildSettingsVariant::parse(r, ver)?)
        } else {
            None
        };
        let m_heightmaps = NavMeshDataMHeightmapsVariant::parse(r, ver)?;
        let m_height_meshes = NavMeshDataMHeightMeshesVariant::parse(r, ver)?;
        let m_off_mesh_links = NavMeshDataMOffMeshLinksVariant::parse(r, ver)?;
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
