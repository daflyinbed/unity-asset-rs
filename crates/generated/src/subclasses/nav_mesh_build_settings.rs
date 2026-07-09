//! 自动生成。请勿手动修改。

use crate::subclasses::nav_mesh_build_debug_settings::NavMeshBuildDebugSettings;


/// 版本枚举：NavMeshBuildSettingsManualCellSizeVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum NavMeshBuildSettingsManualCellSizeVariant {
    Default(bool),
    V5_6_1(i32),
}

impl crate::Parse for NavMeshBuildSettingsManualCellSizeVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 6, 1) {
            let v = rdr.read_i32()?;
            Ok(Self::V5_6_1(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：NavMeshBuildSettingsManualTileSizeVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum NavMeshBuildSettingsManualTileSizeVariant {
    Default(bool),
    V5_6_1(i32),
}

impl crate::Parse for NavMeshBuildSettingsManualTileSizeVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 6, 1) {
            let v = rdr.read_i32()?;
            Ok(Self::V5_6_1(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：NavMeshBuildSettingsAccuratePlacementVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum NavMeshBuildSettingsAccuratePlacementVariant {
    Default(bool),
    V5_6_1(i32),
}

impl crate::Parse for NavMeshBuildSettingsAccuratePlacementVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 6, 1) {
            let v = rdr.read_i32()?;
            Ok(Self::V5_6_1(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct NavMeshBuildSettings {
    pub agent_type_id: i32,
    pub agent_radius: f32,
    pub agent_height: f32,
    pub agent_slope: f32,
    pub agent_climb: f32,
    pub ledge_drop_height: f32,
    pub max_jump_across_distance: f32,
    pub min_region_area: f32,
    pub manual_cell_size: NavMeshBuildSettingsManualCellSizeVariant,
    pub cell_size: f32,
    pub manual_tile_size: NavMeshBuildSettingsManualTileSizeVariant,
    pub tile_size: i32,
    pub accurate_placement: Option<NavMeshBuildSettingsAccuratePlacementVariant>,
    pub build_height_mesh: Option<i32>,
    pub debug: Option<NavMeshBuildDebugSettings>,
    pub max_job_workers: Option<u32>,
    pub keep_tiles: Option<i32>,
    pub preserve_tiles_outside_bounds: Option<i32>,
}

impl crate::Parse for NavMeshBuildSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut agent_type_id = None;
        let mut agent_radius = None;
        let mut agent_height = None;
        let mut agent_slope = None;
        let mut agent_climb = None;
        let mut ledge_drop_height = None;
        let mut max_jump_across_distance = None;
        let mut min_region_area = None;
        let mut manual_cell_size = None;
        let mut cell_size = None;
        let mut manual_tile_size = None;
        let mut tile_size = None;
        let mut accurate_placement = None;
        let mut build_height_mesh = None;
        let mut debug = None;
        let mut max_job_workers = None;
        let mut keep_tiles = None;
        let mut preserve_tiles_outside_bounds = None;
            agent_type_id = Some(rdr.read_i32()?);
            agent_radius = Some(rdr.read_f32()?);
            agent_height = Some(rdr.read_f32()?);
            agent_slope = Some(rdr.read_f32()?);
            agent_climb = Some(rdr.read_f32()?);
            ledge_drop_height = Some(rdr.read_f32()?);
            max_jump_across_distance = Some(rdr.read_f32()?);
            min_region_area = Some(rdr.read_f32()?);
            manual_cell_size = Some(NavMeshBuildSettingsManualCellSizeVariant::parse(rdr, ver)?);
            cell_size = Some(rdr.read_f32()?);
            manual_tile_size = Some(NavMeshBuildSettingsManualTileSizeVariant::parse(rdr, ver)?);
            tile_size = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            build_height_mesh = Some(rdr.read_i32()?);
            max_job_workers = Some(rdr.read_u32()?);
            preserve_tiles_outside_bounds = Some(rdr.read_i32()?);
            debug = Some(NavMeshBuildDebugSettings::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            accurate_placement = Some(NavMeshBuildSettingsAccuratePlacementVariant::parse(rdr, ver)?);
            max_job_workers = Some(rdr.read_u32()?);
            keep_tiles = Some(rdr.read_i32()?);
            debug = Some(NavMeshBuildDebugSettings::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            accurate_placement = Some(NavMeshBuildSettingsAccuratePlacementVariant::parse(rdr, ver)?);
            debug = Some(NavMeshBuildDebugSettings::parse(rdr, ver)?);
        } else {
            accurate_placement = Some(NavMeshBuildSettingsAccuratePlacementVariant::parse(rdr, ver)?);
        }

        Ok(NavMeshBuildSettings {
            agent_type_id: agent_type_id.unwrap(),
            agent_radius: agent_radius.unwrap(),
            agent_height: agent_height.unwrap(),
            agent_slope: agent_slope.unwrap(),
            agent_climb: agent_climb.unwrap(),
            ledge_drop_height: ledge_drop_height.unwrap(),
            max_jump_across_distance: max_jump_across_distance.unwrap(),
            min_region_area: min_region_area.unwrap(),
            manual_cell_size: manual_cell_size.unwrap(),
            cell_size: cell_size.unwrap(),
            manual_tile_size: manual_tile_size.unwrap(),
            tile_size: tile_size.unwrap(),
            accurate_placement,
            build_height_mesh,
            debug,
            max_job_workers,
            keep_tiles,
            preserve_tiles_outside_bounds,
        })
    }
}
