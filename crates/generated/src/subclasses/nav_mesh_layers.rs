//! 自动生成。请勿手动修改。

use crate::subclasses::nav_mesh_layer_data::NavMeshLayerData;

#[derive(Debug, Clone)]
pub struct NavMeshLayers {
    pub m_object_hide_flags: u32,
    pub built_in_layer_0: NavMeshLayerData,
    pub built_in_layer_1: NavMeshLayerData,
    pub built_in_layer_2: NavMeshLayerData,
    pub user_layer_0: NavMeshLayerData,
    pub user_layer_1: NavMeshLayerData,
    pub user_layer_2: NavMeshLayerData,
    pub user_layer_3: NavMeshLayerData,
    pub user_layer_4: NavMeshLayerData,
    pub user_layer_5: NavMeshLayerData,
    pub user_layer_6: NavMeshLayerData,
    pub user_layer_7: NavMeshLayerData,
    pub user_layer_8: NavMeshLayerData,
    pub user_layer_9: NavMeshLayerData,
    pub user_layer_10: NavMeshLayerData,
    pub user_layer_11: NavMeshLayerData,
    pub user_layer_12: NavMeshLayerData,
    pub user_layer_13: NavMeshLayerData,
    pub user_layer_14: NavMeshLayerData,
    pub user_layer_15: NavMeshLayerData,
    pub user_layer_16: NavMeshLayerData,
    pub user_layer_17: NavMeshLayerData,
    pub user_layer_18: NavMeshLayerData,
    pub user_layer_19: NavMeshLayerData,
    pub user_layer_20: NavMeshLayerData,
    pub user_layer_21: NavMeshLayerData,
    pub user_layer_22: NavMeshLayerData,
    pub user_layer_23: NavMeshLayerData,
    pub user_layer_24: NavMeshLayerData,
    pub user_layer_25: NavMeshLayerData,
    pub user_layer_26: NavMeshLayerData,
    pub user_layer_27: NavMeshLayerData,
    pub user_layer_28: NavMeshLayerData,
}

impl crate::Parse for NavMeshLayers {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let built_in_layer_0 = NavMeshLayerData::parse(r, ver)?;
        r.align(4)?;
        let built_in_layer_1 = NavMeshLayerData::parse(r, ver)?;
        r.align(4)?;
        let built_in_layer_2 = NavMeshLayerData::parse(r, ver)?;
        r.align(4)?;
        let user_layer_0 = NavMeshLayerData::parse(r, ver)?;
        r.align(4)?;
        let user_layer_1 = NavMeshLayerData::parse(r, ver)?;
        r.align(4)?;
        let user_layer_2 = NavMeshLayerData::parse(r, ver)?;
        r.align(4)?;
        let user_layer_3 = NavMeshLayerData::parse(r, ver)?;
        r.align(4)?;
        let user_layer_4 = NavMeshLayerData::parse(r, ver)?;
        r.align(4)?;
        let user_layer_5 = NavMeshLayerData::parse(r, ver)?;
        r.align(4)?;
        let user_layer_6 = NavMeshLayerData::parse(r, ver)?;
        r.align(4)?;
        let user_layer_7 = NavMeshLayerData::parse(r, ver)?;
        r.align(4)?;
        let user_layer_8 = NavMeshLayerData::parse(r, ver)?;
        r.align(4)?;
        let user_layer_9 = NavMeshLayerData::parse(r, ver)?;
        r.align(4)?;
        let user_layer_10 = NavMeshLayerData::parse(r, ver)?;
        r.align(4)?;
        let user_layer_11 = NavMeshLayerData::parse(r, ver)?;
        r.align(4)?;
        let user_layer_12 = NavMeshLayerData::parse(r, ver)?;
        r.align(4)?;
        let user_layer_13 = NavMeshLayerData::parse(r, ver)?;
        r.align(4)?;
        let user_layer_14 = NavMeshLayerData::parse(r, ver)?;
        r.align(4)?;
        let user_layer_15 = NavMeshLayerData::parse(r, ver)?;
        r.align(4)?;
        let user_layer_16 = NavMeshLayerData::parse(r, ver)?;
        r.align(4)?;
        let user_layer_17 = NavMeshLayerData::parse(r, ver)?;
        r.align(4)?;
        let user_layer_18 = NavMeshLayerData::parse(r, ver)?;
        r.align(4)?;
        let user_layer_19 = NavMeshLayerData::parse(r, ver)?;
        r.align(4)?;
        let user_layer_20 = NavMeshLayerData::parse(r, ver)?;
        r.align(4)?;
        let user_layer_21 = NavMeshLayerData::parse(r, ver)?;
        r.align(4)?;
        let user_layer_22 = NavMeshLayerData::parse(r, ver)?;
        r.align(4)?;
        let user_layer_23 = NavMeshLayerData::parse(r, ver)?;
        r.align(4)?;
        let user_layer_24 = NavMeshLayerData::parse(r, ver)?;
        r.align(4)?;
        let user_layer_25 = NavMeshLayerData::parse(r, ver)?;
        r.align(4)?;
        let user_layer_26 = NavMeshLayerData::parse(r, ver)?;
        r.align(4)?;
        let user_layer_27 = NavMeshLayerData::parse(r, ver)?;
        r.align(4)?;
        let user_layer_28 = NavMeshLayerData::parse(r, ver)?;
        r.align(4)?;

        Ok(NavMeshLayers {
            m_object_hide_flags,
            built_in_layer_0,
            built_in_layer_1,
            built_in_layer_2,
            user_layer_0,
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
