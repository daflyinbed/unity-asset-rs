//! 自动生成。请勿手动修改。

use crate::subclasses::nav_mesh_area_data::NavMeshAreaData;
use crate::subclasses::nav_mesh_build_settings::NavMeshBuildSettings;
use crate::subclasses::nav_mesh_layer_data::NavMeshLayerData;

/// 版本枚举：NavMeshLayersAreasVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum NavMeshLayersAreasVariant {
    Default(Vec<NavMeshAreaData>),
    V2017_1(Vec<NavMeshAreaData>),
}

impl crate::Parse for NavMeshLayersAreasVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| NavMeshAreaData::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| NavMeshAreaData::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for NavMeshLayersAreasVariant {
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

/// 版本枚举：NavMeshLayersMSettingsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum NavMeshLayersMSettingsVariant {
    Default(Vec<NavMeshBuildSettings>),
    V2017_1(Vec<NavMeshBuildSettings>),
}

impl crate::Parse for NavMeshLayersMSettingsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| NavMeshBuildSettings::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| NavMeshBuildSettings::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for NavMeshLayersMSettingsVariant {
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

/// 版本枚举：NavMeshLayersMSettingNamesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum NavMeshLayersMSettingNamesVariant {
    Default(Vec<String>),
    V2017_1(Vec<String>),
}

impl crate::Parse for NavMeshLayersMSettingNamesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_aligned_string())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_aligned_string())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for NavMeshLayersMSettingNamesVariant {
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
pub struct NavMeshLayers {
    pub built_in_layer_0: Option<NavMeshLayerData>,
    pub areas: Option<NavMeshLayersAreasVariant>,
    pub built_in_layer_1: Option<NavMeshLayerData>,
    pub m_last_agent_type_id: Option<i32>,
    pub built_in_layer_2: Option<NavMeshLayerData>,
    pub m_settings: Option<NavMeshLayersMSettingsVariant>,
    pub user_layer_0: Option<NavMeshLayerData>,
    pub m_setting_names: Option<NavMeshLayersMSettingNamesVariant>,
    pub user_layer_1: Option<NavMeshLayerData>,
    pub user_layer_2: Option<NavMeshLayerData>,
    pub user_layer_3: Option<NavMeshLayerData>,
    pub user_layer_4: Option<NavMeshLayerData>,
    pub user_layer_5: Option<NavMeshLayerData>,
    pub user_layer_6: Option<NavMeshLayerData>,
    pub user_layer_7: Option<NavMeshLayerData>,
    pub user_layer_8: Option<NavMeshLayerData>,
    pub user_layer_9: Option<NavMeshLayerData>,
    pub user_layer_10: Option<NavMeshLayerData>,
    pub user_layer_11: Option<NavMeshLayerData>,
    pub user_layer_12: Option<NavMeshLayerData>,
    pub user_layer_13: Option<NavMeshLayerData>,
    pub user_layer_14: Option<NavMeshLayerData>,
    pub user_layer_15: Option<NavMeshLayerData>,
    pub user_layer_16: Option<NavMeshLayerData>,
    pub user_layer_17: Option<NavMeshLayerData>,
    pub user_layer_18: Option<NavMeshLayerData>,
    pub user_layer_19: Option<NavMeshLayerData>,
    pub user_layer_20: Option<NavMeshLayerData>,
    pub user_layer_21: Option<NavMeshLayerData>,
    pub user_layer_22: Option<NavMeshLayerData>,
    pub user_layer_23: Option<NavMeshLayerData>,
    pub user_layer_24: Option<NavMeshLayerData>,
    pub user_layer_25: Option<NavMeshLayerData>,
    pub user_layer_26: Option<NavMeshLayerData>,
    pub user_layer_27: Option<NavMeshLayerData>,
    pub user_layer_28: Option<NavMeshLayerData>,
}

impl crate::Parse for NavMeshLayers {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut built_in_layer_0 = None;
        let mut areas = None;
        let mut built_in_layer_1 = None;
        let mut m_last_agent_type_id = None;
        let mut built_in_layer_2 = None;
        let mut m_settings = None;
        let mut user_layer_0 = None;
        let mut m_setting_names = None;
        let mut user_layer_1 = None;
        let mut user_layer_2 = None;
        let mut user_layer_3 = None;
        let mut user_layer_4 = None;
        let mut user_layer_5 = None;
        let mut user_layer_6 = None;
        let mut user_layer_7 = None;
        let mut user_layer_8 = None;
        let mut user_layer_9 = None;
        let mut user_layer_10 = None;
        let mut user_layer_11 = None;
        let mut user_layer_12 = None;
        let mut user_layer_13 = None;
        let mut user_layer_14 = None;
        let mut user_layer_15 = None;
        let mut user_layer_16 = None;
        let mut user_layer_17 = None;
        let mut user_layer_18 = None;
        let mut user_layer_19 = None;
        let mut user_layer_20 = None;
        let mut user_layer_21 = None;
        let mut user_layer_22 = None;
        let mut user_layer_23 = None;
        let mut user_layer_24 = None;
        let mut user_layer_25 = None;
        let mut user_layer_26 = None;
        let mut user_layer_27 = None;
        let mut user_layer_28 = None;
        if *ver >= crate::UnityVersion::new(5, 6, 0) {
            areas = Some(NavMeshLayersAreasVariant::parse(rdr, ver)?);
            m_last_agent_type_id = Some(rdr.read_i32()?);
            m_settings = Some(NavMeshLayersMSettingsVariant::parse(rdr, ver)?);
            m_setting_names = Some(NavMeshLayersMSettingNamesVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            areas = Some(NavMeshLayersAreasVariant::parse(rdr, ver)?);
        } else {
            built_in_layer_0 = Some(NavMeshLayerData::parse(rdr, ver)?);
            built_in_layer_1 = Some(NavMeshLayerData::parse(rdr, ver)?);
            built_in_layer_2 = Some(NavMeshLayerData::parse(rdr, ver)?);
            user_layer_0 = Some(NavMeshLayerData::parse(rdr, ver)?);
            user_layer_1 = Some(NavMeshLayerData::parse(rdr, ver)?);
            user_layer_2 = Some(NavMeshLayerData::parse(rdr, ver)?);
            user_layer_3 = Some(NavMeshLayerData::parse(rdr, ver)?);
            user_layer_4 = Some(NavMeshLayerData::parse(rdr, ver)?);
            user_layer_5 = Some(NavMeshLayerData::parse(rdr, ver)?);
            user_layer_6 = Some(NavMeshLayerData::parse(rdr, ver)?);
            user_layer_7 = Some(NavMeshLayerData::parse(rdr, ver)?);
            user_layer_8 = Some(NavMeshLayerData::parse(rdr, ver)?);
            user_layer_9 = Some(NavMeshLayerData::parse(rdr, ver)?);
            user_layer_10 = Some(NavMeshLayerData::parse(rdr, ver)?);
            user_layer_11 = Some(NavMeshLayerData::parse(rdr, ver)?);
            user_layer_12 = Some(NavMeshLayerData::parse(rdr, ver)?);
            user_layer_13 = Some(NavMeshLayerData::parse(rdr, ver)?);
            user_layer_14 = Some(NavMeshLayerData::parse(rdr, ver)?);
            user_layer_15 = Some(NavMeshLayerData::parse(rdr, ver)?);
            user_layer_16 = Some(NavMeshLayerData::parse(rdr, ver)?);
            user_layer_17 = Some(NavMeshLayerData::parse(rdr, ver)?);
            user_layer_18 = Some(NavMeshLayerData::parse(rdr, ver)?);
            user_layer_19 = Some(NavMeshLayerData::parse(rdr, ver)?);
            user_layer_20 = Some(NavMeshLayerData::parse(rdr, ver)?);
            user_layer_21 = Some(NavMeshLayerData::parse(rdr, ver)?);
            user_layer_22 = Some(NavMeshLayerData::parse(rdr, ver)?);
            user_layer_23 = Some(NavMeshLayerData::parse(rdr, ver)?);
            user_layer_24 = Some(NavMeshLayerData::parse(rdr, ver)?);
            user_layer_25 = Some(NavMeshLayerData::parse(rdr, ver)?);
            user_layer_26 = Some(NavMeshLayerData::parse(rdr, ver)?);
            user_layer_27 = Some(NavMeshLayerData::parse(rdr, ver)?);
            user_layer_28 = Some(NavMeshLayerData::parse(rdr, ver)?);
        }

        Ok(NavMeshLayers {
            built_in_layer_0,
            areas,
            built_in_layer_1,
            m_last_agent_type_id,
            built_in_layer_2,
            m_settings,
            user_layer_0,
            m_setting_names,
            user_layer_1,
            user_layer_2,
            user_layer_3,
            user_layer_4,
            user_layer_5,
            user_layer_6,
            user_layer_7,
            user_layer_8,
            user_layer_9,
            user_layer_10,
            user_layer_11,
            user_layer_12,
            user_layer_13,
            user_layer_14,
            user_layer_15,
            user_layer_16,
            user_layer_17,
            user_layer_18,
            user_layer_19,
            user_layer_20,
            user_layer_21,
            user_layer_22,
            user_layer_23,
            user_layer_24,
            user_layer_25,
            user_layer_26,
            user_layer_27,
            user_layer_28,
        })
    }
}

impl crate::ToValue for NavMeshLayers {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            crate::opt_field("Built-in Layer 0", &self.built_in_layer_0),
            crate::opt_field("areas", &self.areas),
            crate::opt_field("Built-in Layer 1", &self.built_in_layer_1),
            crate::opt_field("m_LastAgentTypeID", &self.m_last_agent_type_id),
            crate::opt_field("Built-in Layer 2", &self.built_in_layer_2),
            crate::opt_field("m_Settings", &self.m_settings),
            crate::opt_field("User Layer 0", &self.user_layer_0),
            crate::opt_field("m_SettingNames", &self.m_setting_names),
            crate::opt_field("User Layer 1", &self.user_layer_1),
            crate::opt_field("User Layer 2", &self.user_layer_2),
            crate::opt_field("User Layer 3", &self.user_layer_3),
            crate::opt_field("User Layer 4", &self.user_layer_4),
            crate::opt_field("User Layer 5", &self.user_layer_5),
            crate::opt_field("User Layer 6", &self.user_layer_6),
            crate::opt_field("User Layer 7", &self.user_layer_7),
            crate::opt_field("User Layer 8", &self.user_layer_8),
            crate::opt_field("User Layer 9", &self.user_layer_9),
            crate::opt_field("User Layer 10", &self.user_layer_10),
            crate::opt_field("User Layer 11", &self.user_layer_11),
            crate::opt_field("User Layer 12", &self.user_layer_12),
            crate::opt_field("User Layer 13", &self.user_layer_13),
            crate::opt_field("User Layer 14", &self.user_layer_14),
            crate::opt_field("User Layer 15", &self.user_layer_15),
            crate::opt_field("User Layer 16", &self.user_layer_16),
            crate::opt_field("User Layer 17", &self.user_layer_17),
            crate::opt_field("User Layer 18", &self.user_layer_18),
            crate::opt_field("User Layer 19", &self.user_layer_19),
            crate::opt_field("User Layer 20", &self.user_layer_20),
            crate::opt_field("User Layer 21", &self.user_layer_21),
            crate::opt_field("User Layer 22", &self.user_layer_22),
            crate::opt_field("User Layer 23", &self.user_layer_23),
            crate::opt_field("User Layer 24", &self.user_layer_24),
            crate::opt_field("User Layer 25", &self.user_layer_25),
            crate::opt_field("User Layer 26", &self.user_layer_26),
            crate::opt_field("User Layer 27", &self.user_layer_27),
            crate::opt_field("User Layer 28", &self.user_layer_28),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "Built-in Layer 0" => (self.built_in_layer_0).inspect_value(path, offset, limit),
                "areas" => (self.areas).inspect_value(path, offset, limit),
                "Built-in Layer 1" => (self.built_in_layer_1).inspect_value(path, offset, limit),
                "m_LastAgentTypeID" => {
                    (self.m_last_agent_type_id).inspect_value(path, offset, limit)
                }
                "Built-in Layer 2" => (self.built_in_layer_2).inspect_value(path, offset, limit),
                "m_Settings" => (self.m_settings).inspect_value(path, offset, limit),
                "User Layer 0" => (self.user_layer_0).inspect_value(path, offset, limit),
                "m_SettingNames" => (self.m_setting_names).inspect_value(path, offset, limit),
                "User Layer 1" => (self.user_layer_1).inspect_value(path, offset, limit),
                "User Layer 2" => (self.user_layer_2).inspect_value(path, offset, limit),
                "User Layer 3" => (self.user_layer_3).inspect_value(path, offset, limit),
                "User Layer 4" => (self.user_layer_4).inspect_value(path, offset, limit),
                "User Layer 5" => (self.user_layer_5).inspect_value(path, offset, limit),
                "User Layer 6" => (self.user_layer_6).inspect_value(path, offset, limit),
                "User Layer 7" => (self.user_layer_7).inspect_value(path, offset, limit),
                "User Layer 8" => (self.user_layer_8).inspect_value(path, offset, limit),
                "User Layer 9" => (self.user_layer_9).inspect_value(path, offset, limit),
                "User Layer 10" => (self.user_layer_10).inspect_value(path, offset, limit),
                "User Layer 11" => (self.user_layer_11).inspect_value(path, offset, limit),
                "User Layer 12" => (self.user_layer_12).inspect_value(path, offset, limit),
                "User Layer 13" => (self.user_layer_13).inspect_value(path, offset, limit),
                "User Layer 14" => (self.user_layer_14).inspect_value(path, offset, limit),
                "User Layer 15" => (self.user_layer_15).inspect_value(path, offset, limit),
                "User Layer 16" => (self.user_layer_16).inspect_value(path, offset, limit),
                "User Layer 17" => (self.user_layer_17).inspect_value(path, offset, limit),
                "User Layer 18" => (self.user_layer_18).inspect_value(path, offset, limit),
                "User Layer 19" => (self.user_layer_19).inspect_value(path, offset, limit),
                "User Layer 20" => (self.user_layer_20).inspect_value(path, offset, limit),
                "User Layer 21" => (self.user_layer_21).inspect_value(path, offset, limit),
                "User Layer 22" => (self.user_layer_22).inspect_value(path, offset, limit),
                "User Layer 23" => (self.user_layer_23).inspect_value(path, offset, limit),
                "User Layer 24" => (self.user_layer_24).inspect_value(path, offset, limit),
                "User Layer 25" => (self.user_layer_25).inspect_value(path, offset, limit),
                "User Layer 26" => (self.user_layer_26).inspect_value(path, offset, limit),
                "User Layer 27" => (self.user_layer_27).inspect_value(path, offset, limit),
                "User Layer 28" => (self.user_layer_28).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.built_in_layer_0).inspect_value(path, offset, limit))
                .map(|v| ("Built-in Layer 0".to_string(), v)),
            ((self.areas).inspect_value(path, offset, limit)).map(|v| ("areas".to_string(), v)),
            ((self.built_in_layer_1).inspect_value(path, offset, limit))
                .map(|v| ("Built-in Layer 1".to_string(), v)),
            ((self.m_last_agent_type_id).inspect_value(path, offset, limit))
                .map(|v| ("m_LastAgentTypeID".to_string(), v)),
            ((self.built_in_layer_2).inspect_value(path, offset, limit))
                .map(|v| ("Built-in Layer 2".to_string(), v)),
            ((self.m_settings).inspect_value(path, offset, limit))
                .map(|v| ("m_Settings".to_string(), v)),
            ((self.user_layer_0).inspect_value(path, offset, limit))
                .map(|v| ("User Layer 0".to_string(), v)),
            ((self.m_setting_names).inspect_value(path, offset, limit))
                .map(|v| ("m_SettingNames".to_string(), v)),
            ((self.user_layer_1).inspect_value(path, offset, limit))
                .map(|v| ("User Layer 1".to_string(), v)),
            ((self.user_layer_2).inspect_value(path, offset, limit))
                .map(|v| ("User Layer 2".to_string(), v)),
            ((self.user_layer_3).inspect_value(path, offset, limit))
                .map(|v| ("User Layer 3".to_string(), v)),
            ((self.user_layer_4).inspect_value(path, offset, limit))
                .map(|v| ("User Layer 4".to_string(), v)),
            ((self.user_layer_5).inspect_value(path, offset, limit))
                .map(|v| ("User Layer 5".to_string(), v)),
            ((self.user_layer_6).inspect_value(path, offset, limit))
                .map(|v| ("User Layer 6".to_string(), v)),
            ((self.user_layer_7).inspect_value(path, offset, limit))
                .map(|v| ("User Layer 7".to_string(), v)),
            ((self.user_layer_8).inspect_value(path, offset, limit))
                .map(|v| ("User Layer 8".to_string(), v)),
            ((self.user_layer_9).inspect_value(path, offset, limit))
                .map(|v| ("User Layer 9".to_string(), v)),
            ((self.user_layer_10).inspect_value(path, offset, limit))
                .map(|v| ("User Layer 10".to_string(), v)),
            ((self.user_layer_11).inspect_value(path, offset, limit))
                .map(|v| ("User Layer 11".to_string(), v)),
            ((self.user_layer_12).inspect_value(path, offset, limit))
                .map(|v| ("User Layer 12".to_string(), v)),
            ((self.user_layer_13).inspect_value(path, offset, limit))
                .map(|v| ("User Layer 13".to_string(), v)),
            ((self.user_layer_14).inspect_value(path, offset, limit))
                .map(|v| ("User Layer 14".to_string(), v)),
            ((self.user_layer_15).inspect_value(path, offset, limit))
                .map(|v| ("User Layer 15".to_string(), v)),
            ((self.user_layer_16).inspect_value(path, offset, limit))
                .map(|v| ("User Layer 16".to_string(), v)),
            ((self.user_layer_17).inspect_value(path, offset, limit))
                .map(|v| ("User Layer 17".to_string(), v)),
            ((self.user_layer_18).inspect_value(path, offset, limit))
                .map(|v| ("User Layer 18".to_string(), v)),
            ((self.user_layer_19).inspect_value(path, offset, limit))
                .map(|v| ("User Layer 19".to_string(), v)),
            ((self.user_layer_20).inspect_value(path, offset, limit))
                .map(|v| ("User Layer 20".to_string(), v)),
            ((self.user_layer_21).inspect_value(path, offset, limit))
                .map(|v| ("User Layer 21".to_string(), v)),
            ((self.user_layer_22).inspect_value(path, offset, limit))
                .map(|v| ("User Layer 22".to_string(), v)),
            ((self.user_layer_23).inspect_value(path, offset, limit))
                .map(|v| ("User Layer 23".to_string(), v)),
            ((self.user_layer_24).inspect_value(path, offset, limit))
                .map(|v| ("User Layer 24".to_string(), v)),
            ((self.user_layer_25).inspect_value(path, offset, limit))
                .map(|v| ("User Layer 25".to_string(), v)),
            ((self.user_layer_26).inspect_value(path, offset, limit))
                .map(|v| ("User Layer 26".to_string(), v)),
            ((self.user_layer_27).inspect_value(path, offset, limit))
                .map(|v| ("User Layer 27".to_string(), v)),
            ((self.user_layer_28).inspect_value(path, offset, limit))
                .map(|v| ("User Layer 28".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
