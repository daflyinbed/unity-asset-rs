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
