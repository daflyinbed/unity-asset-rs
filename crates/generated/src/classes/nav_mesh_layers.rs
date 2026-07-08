//! 自动生成。请勿手动修改。

use crate::subclasses::nav_mesh_area_data::NavMeshAreaData;
use crate::subclasses::nav_mesh_build_settings::NavMeshBuildSettings;
use crate::subclasses::nav_mesh_layer_data::NavMeshLayerData;


/// 版本枚举：NavMeshLayersMSettingsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum NavMeshLayersMSettingsVariant {
    Default(Vec<NavMeshBuildSettings>),
    V2017_1(Vec<NavMeshBuildSettings>),
}

impl crate::Parse for NavMeshLayersMSettingsVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { NavMeshBuildSettings::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { NavMeshBuildSettings::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct NavMeshLayers {
    pub built_in_layer_0: Option<NavMeshLayerData>,
    pub areas: Option<Vec<NavMeshAreaData>>,
    pub built_in_layer_1: Option<NavMeshLayerData>,
    pub m_last_agent_type_id: Option<i32>,
    pub built_in_layer_2: Option<NavMeshLayerData>,
    pub m_settings: Option<NavMeshLayersMSettingsVariant>,
    pub user_layer_0: Option<NavMeshLayerData>,
    pub m_setting_names: Option<Vec<String>>,
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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let built_in_layer_0 = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = NavMeshLayerData::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let areas = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { NavMeshAreaData::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let built_in_layer_1 = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = NavMeshLayerData::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_last_agent_type_id = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let built_in_layer_2 = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = NavMeshLayerData::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_settings = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            Some(NavMeshLayersMSettingsVariant::parse(r, ver)?)
        } else {
            None
        };
        let user_layer_0 = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = NavMeshLayerData::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_setting_names = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let user_layer_1 = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = NavMeshLayerData::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let user_layer_2 = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = NavMeshLayerData::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let user_layer_3 = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = NavMeshLayerData::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let user_layer_4 = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = NavMeshLayerData::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let user_layer_5 = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = NavMeshLayerData::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let user_layer_6 = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = NavMeshLayerData::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let user_layer_7 = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = NavMeshLayerData::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let user_layer_8 = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = NavMeshLayerData::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let user_layer_9 = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = NavMeshLayerData::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let user_layer_10 = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = NavMeshLayerData::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let user_layer_11 = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = NavMeshLayerData::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let user_layer_12 = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = NavMeshLayerData::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let user_layer_13 = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = NavMeshLayerData::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let user_layer_14 = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = NavMeshLayerData::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let user_layer_15 = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = NavMeshLayerData::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let user_layer_16 = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = NavMeshLayerData::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let user_layer_17 = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = NavMeshLayerData::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let user_layer_18 = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = NavMeshLayerData::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let user_layer_19 = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = NavMeshLayerData::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let user_layer_20 = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = NavMeshLayerData::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let user_layer_21 = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = NavMeshLayerData::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let user_layer_22 = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = NavMeshLayerData::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let user_layer_23 = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = NavMeshLayerData::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let user_layer_24 = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = NavMeshLayerData::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let user_layer_25 = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = NavMeshLayerData::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let user_layer_26 = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = NavMeshLayerData::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let user_layer_27 = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = NavMeshLayerData::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let user_layer_28 = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = NavMeshLayerData::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

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
