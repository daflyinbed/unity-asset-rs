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
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| NavMeshTileData::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| NavMeshTileData::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for NavMeshDataMNavMeshTilesVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
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
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| HeightmapData::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| HeightmapData::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for NavMeshDataMHeightmapsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
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
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| HeightMeshData::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| HeightMeshData::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for NavMeshDataMHeightMeshesVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
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
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| AutoOffMeshLinkData::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| AutoOffMeshLinkData::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for NavMeshDataMOffMeshLinksVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct NavMeshData {
    pub m_name: String,
    pub m_mesh_data: Option<Vec<u8>>,
    pub m_nav_mesh_tiles: Option<NavMeshDataMNavMeshTilesVariant>,
    pub m_heightmaps: NavMeshDataMHeightmapsVariant,
    pub m_nav_mesh_params: Option<NavMeshParams>,
    pub m_nav_mesh_build_settings: Option<NavMeshBuildSettings>,
    pub m_height_meshes: Option<NavMeshDataMHeightMeshesVariant>,
    pub m_off_mesh_links: Option<NavMeshDataMOffMeshLinksVariant>,
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
        let mut m_mesh_data = None;
        let mut m_nav_mesh_tiles = None;
        let mut m_heightmaps = None;
        let mut m_nav_mesh_params = None;
        let mut m_nav_mesh_build_settings = None;
        let mut m_height_meshes = None;
        let mut m_off_mesh_links = None;
        let mut m_source_bounds = None;
        let mut m_rotation = None;
        let mut m_position = None;
        let mut m_agent_type_id = None;
        m_name = Some(rdr.read_aligned_string()?);
        if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_nav_mesh_tiles = Some(NavMeshDataMNavMeshTilesVariant::parse(rdr, ver)?);
            m_nav_mesh_build_settings = Some(NavMeshBuildSettings::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_nav_mesh_tiles = Some(NavMeshDataMNavMeshTilesVariant::parse(rdr, ver)?);
            m_nav_mesh_params = Some(NavMeshParams::parse(rdr, ver)?);
        } else {
            m_mesh_data = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
        }
        m_heightmaps = Some(NavMeshDataMHeightmapsVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 6, 1) {
            m_height_meshes = Some(NavMeshDataMHeightMeshesVariant::parse(rdr, ver)?);
            m_off_mesh_links = Some(NavMeshDataMOffMeshLinksVariant::parse(rdr, ver)?);
            m_source_bounds = Some(Aabb::parse(rdr, ver)?);
            m_rotation = Some(Quaternionf::parse(rdr, ver)?);
            m_position = Some(Vector3f::parse(rdr, ver)?);
            m_agent_type_id = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_height_meshes = Some(NavMeshDataMHeightMeshesVariant::parse(rdr, ver)?);
            m_off_mesh_links = Some(NavMeshDataMOffMeshLinksVariant::parse(rdr, ver)?);
        } else {
        }

        Ok(NavMeshData {
            m_name: m_name.unwrap(),
            m_mesh_data,
            m_nav_mesh_tiles,
            m_heightmaps: m_heightmaps.unwrap(),
            m_nav_mesh_params,
            m_nav_mesh_build_settings,
            m_height_meshes,
            m_off_mesh_links,
            m_source_bounds,
            m_rotation,
            m_position,
            m_agent_type_id,
        })
    }
}

impl crate::ToValue for NavMeshData {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            crate::opt_field("m_MeshData", &self.m_mesh_data),
            crate::opt_field("m_NavMeshTiles", &self.m_nav_mesh_tiles),
            Some(("m_Heightmaps".to_string(), self.m_heightmaps.to_value())),
            crate::opt_field("m_NavMeshParams", &self.m_nav_mesh_params),
            crate::opt_field("m_NavMeshBuildSettings", &self.m_nav_mesh_build_settings),
            crate::opt_field("m_HeightMeshes", &self.m_height_meshes),
            crate::opt_field("m_OffMeshLinks", &self.m_off_mesh_links),
            crate::opt_field("m_SourceBounds", &self.m_source_bounds),
            crate::opt_field("m_Rotation", &self.m_rotation),
            crate::opt_field("m_Position", &self.m_position),
            crate::opt_field("m_AgentTypeID", &self.m_agent_type_id),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Name" => (self.m_name).inspect_value(path, offset, limit),
                "m_MeshData" => (self.m_mesh_data).inspect_value(path, offset, limit),
                "m_NavMeshTiles" => (self.m_nav_mesh_tiles).inspect_value(path, offset, limit),
                "m_Heightmaps" => (self.m_heightmaps).inspect_value(path, offset, limit),
                "m_NavMeshParams" => (self.m_nav_mesh_params).inspect_value(path, offset, limit),
                "m_NavMeshBuildSettings" => {
                    (self.m_nav_mesh_build_settings).inspect_value(path, offset, limit)
                }
                "m_HeightMeshes" => (self.m_height_meshes).inspect_value(path, offset, limit),
                "m_OffMeshLinks" => (self.m_off_mesh_links).inspect_value(path, offset, limit),
                "m_SourceBounds" => (self.m_source_bounds).inspect_value(path, offset, limit),
                "m_Rotation" => (self.m_rotation).inspect_value(path, offset, limit),
                "m_Position" => (self.m_position).inspect_value(path, offset, limit),
                "m_AgentTypeID" => (self.m_agent_type_id).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_mesh_data).inspect_value(path, offset, limit))
                .map(|v| ("m_MeshData".to_string(), v)),
            ((self.m_nav_mesh_tiles).inspect_value(path, offset, limit))
                .map(|v| ("m_NavMeshTiles".to_string(), v)),
            ((self.m_heightmaps).inspect_value(path, offset, limit))
                .map(|v| ("m_Heightmaps".to_string(), v)),
            ((self.m_nav_mesh_params).inspect_value(path, offset, limit))
                .map(|v| ("m_NavMeshParams".to_string(), v)),
            ((self.m_nav_mesh_build_settings).inspect_value(path, offset, limit))
                .map(|v| ("m_NavMeshBuildSettings".to_string(), v)),
            ((self.m_height_meshes).inspect_value(path, offset, limit))
                .map(|v| ("m_HeightMeshes".to_string(), v)),
            ((self.m_off_mesh_links).inspect_value(path, offset, limit))
                .map(|v| ("m_OffMeshLinks".to_string(), v)),
            ((self.m_source_bounds).inspect_value(path, offset, limit))
                .map(|v| ("m_SourceBounds".to_string(), v)),
            ((self.m_rotation).inspect_value(path, offset, limit))
                .map(|v| ("m_Rotation".to_string(), v)),
            ((self.m_position).inspect_value(path, offset, limit))
                .map(|v| ("m_Position".to_string(), v)),
            ((self.m_agent_type_id).inspect_value(path, offset, limit))
                .map(|v| ("m_AgentTypeID".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
