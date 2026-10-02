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

impl crate::ToValue for NavMeshBuildSettingsManualCellSizeVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_6_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_6_1(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for NavMeshBuildSettingsManualTileSizeVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_6_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_6_1(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for NavMeshBuildSettingsAccuratePlacementVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_6_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_6_1(v) => (v).inspect_value(path, offset, limit),
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
            accurate_placement = Some(NavMeshBuildSettingsAccuratePlacementVariant::parse(
                rdr, ver,
            )?);
            max_job_workers = Some(rdr.read_u32()?);
            keep_tiles = Some(rdr.read_i32()?);
            debug = Some(NavMeshBuildDebugSettings::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            accurate_placement = Some(NavMeshBuildSettingsAccuratePlacementVariant::parse(
                rdr, ver,
            )?);
            debug = Some(NavMeshBuildDebugSettings::parse(rdr, ver)?);
        } else {
            accurate_placement = Some(NavMeshBuildSettingsAccuratePlacementVariant::parse(
                rdr, ver,
            )?);
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

impl crate::ToValue for NavMeshBuildSettings {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("agentTypeID".to_string(), self.agent_type_id.to_value())),
            Some(("agentRadius".to_string(), self.agent_radius.to_value())),
            Some(("agentHeight".to_string(), self.agent_height.to_value())),
            Some(("agentSlope".to_string(), self.agent_slope.to_value())),
            Some(("agentClimb".to_string(), self.agent_climb.to_value())),
            Some((
                "ledgeDropHeight".to_string(),
                self.ledge_drop_height.to_value(),
            )),
            Some((
                "maxJumpAcrossDistance".to_string(),
                self.max_jump_across_distance.to_value(),
            )),
            Some(("minRegionArea".to_string(), self.min_region_area.to_value())),
            Some((
                "manualCellSize".to_string(),
                self.manual_cell_size.to_value(),
            )),
            Some(("cellSize".to_string(), self.cell_size.to_value())),
            Some((
                "manualTileSize".to_string(),
                self.manual_tile_size.to_value(),
            )),
            Some(("tileSize".to_string(), self.tile_size.to_value())),
            crate::opt_field("accuratePlacement", &self.accurate_placement),
            crate::opt_field("buildHeightMesh", &self.build_height_mesh),
            crate::opt_field("debug", &self.debug),
            crate::opt_field("maxJobWorkers", &self.max_job_workers),
            crate::opt_field("keepTiles", &self.keep_tiles),
            crate::opt_field(
                "preserveTilesOutsideBounds",
                &self.preserve_tiles_outside_bounds,
            ),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "agentTypeID" => (self.agent_type_id).inspect_value(path, offset, limit),
                "agentRadius" => (self.agent_radius).inspect_value(path, offset, limit),
                "agentHeight" => (self.agent_height).inspect_value(path, offset, limit),
                "agentSlope" => (self.agent_slope).inspect_value(path, offset, limit),
                "agentClimb" => (self.agent_climb).inspect_value(path, offset, limit),
                "ledgeDropHeight" => (self.ledge_drop_height).inspect_value(path, offset, limit),
                "maxJumpAcrossDistance" => {
                    (self.max_jump_across_distance).inspect_value(path, offset, limit)
                }
                "minRegionArea" => (self.min_region_area).inspect_value(path, offset, limit),
                "manualCellSize" => (self.manual_cell_size).inspect_value(path, offset, limit),
                "cellSize" => (self.cell_size).inspect_value(path, offset, limit),
                "manualTileSize" => (self.manual_tile_size).inspect_value(path, offset, limit),
                "tileSize" => (self.tile_size).inspect_value(path, offset, limit),
                "accuratePlacement" => (self.accurate_placement).inspect_value(path, offset, limit),
                "buildHeightMesh" => (self.build_height_mesh).inspect_value(path, offset, limit),
                "debug" => (self.debug).inspect_value(path, offset, limit),
                "maxJobWorkers" => (self.max_job_workers).inspect_value(path, offset, limit),
                "keepTiles" => (self.keep_tiles).inspect_value(path, offset, limit),
                "preserveTilesOutsideBounds" => {
                    (self.preserve_tiles_outside_bounds).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.agent_type_id).inspect_value(path, offset, limit))
                .map(|v| ("agentTypeID".to_string(), v)),
            ((self.agent_radius).inspect_value(path, offset, limit))
                .map(|v| ("agentRadius".to_string(), v)),
            ((self.agent_height).inspect_value(path, offset, limit))
                .map(|v| ("agentHeight".to_string(), v)),
            ((self.agent_slope).inspect_value(path, offset, limit))
                .map(|v| ("agentSlope".to_string(), v)),
            ((self.agent_climb).inspect_value(path, offset, limit))
                .map(|v| ("agentClimb".to_string(), v)),
            ((self.ledge_drop_height).inspect_value(path, offset, limit))
                .map(|v| ("ledgeDropHeight".to_string(), v)),
            ((self.max_jump_across_distance).inspect_value(path, offset, limit))
                .map(|v| ("maxJumpAcrossDistance".to_string(), v)),
            ((self.min_region_area).inspect_value(path, offset, limit))
                .map(|v| ("minRegionArea".to_string(), v)),
            ((self.manual_cell_size).inspect_value(path, offset, limit))
                .map(|v| ("manualCellSize".to_string(), v)),
            ((self.cell_size).inspect_value(path, offset, limit))
                .map(|v| ("cellSize".to_string(), v)),
            ((self.manual_tile_size).inspect_value(path, offset, limit))
                .map(|v| ("manualTileSize".to_string(), v)),
            ((self.tile_size).inspect_value(path, offset, limit))
                .map(|v| ("tileSize".to_string(), v)),
            ((self.accurate_placement).inspect_value(path, offset, limit))
                .map(|v| ("accuratePlacement".to_string(), v)),
            ((self.build_height_mesh).inspect_value(path, offset, limit))
                .map(|v| ("buildHeightMesh".to_string(), v)),
            ((self.debug).inspect_value(path, offset, limit)).map(|v| ("debug".to_string(), v)),
            ((self.max_job_workers).inspect_value(path, offset, limit))
                .map(|v| ("maxJobWorkers".to_string(), v)),
            ((self.keep_tiles).inspect_value(path, offset, limit))
                .map(|v| ("keepTiles".to_string(), v)),
            ((self.preserve_tiles_outside_bounds).inspect_value(path, offset, limit))
                .map(|v| ("preserveTilesOutsideBounds".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
