//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct LightmapData {
    pub m_lightmap: Option<PPtr>,
    pub sh_0: Option<f32>,
    pub m_indirect_lightmap: Option<PPtr>,
    pub sh_1: Option<f32>,
    pub m_dir_lightmap: Option<PPtr>,
    pub sh_2: Option<f32>,
    pub m_shadow_mask: Option<PPtr>,
    pub sh_3: Option<f32>,
    pub sh_4: Option<f32>,
    pub sh_5: Option<f32>,
    pub sh_6: Option<f32>,
    pub sh_7: Option<f32>,
    pub sh_8: Option<f32>,
    pub sh_9: Option<f32>,
    pub sh_10: Option<f32>,
    pub sh_11: Option<f32>,
    pub sh_12: Option<f32>,
    pub sh_13: Option<f32>,
    pub sh_14: Option<f32>,
    pub sh_15: Option<f32>,
    pub sh_16: Option<f32>,
    pub sh_17: Option<f32>,
    pub sh_18: Option<f32>,
    pub sh_19: Option<f32>,
    pub sh_20: Option<f32>,
    pub sh_21: Option<f32>,
    pub sh_22: Option<f32>,
    pub sh_23: Option<f32>,
    pub sh_24: Option<f32>,
    pub sh_25: Option<f32>,
    pub sh_26: Option<f32>,
}

impl crate::Parse for LightmapData {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_lightmap = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(crate::PPtr::parse(rdr, ver)?)
        } else {
            None
        };
        let sh_0 = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };
        let m_indirect_lightmap = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(crate::PPtr::parse(rdr, ver)?)
        } else {
            None
        };
        let sh_1 = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };
        let m_dir_lightmap = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            Some(crate::PPtr::parse(rdr, ver)?)
        } else {
            None
        };
        let sh_2 = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };
        let m_shadow_mask = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            Some(crate::PPtr::parse(rdr, ver)?)
        } else {
            None
        };
        let sh_3 = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };
        let sh_4 = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };
        let sh_5 = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };
        let sh_6 = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };
        let sh_7 = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };
        let sh_8 = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };
        let sh_9 = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };
        let sh_10 = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };
        let sh_11 = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };
        let sh_12 = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };
        let sh_13 = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };
        let sh_14 = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };
        let sh_15 = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };
        let sh_16 = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };
        let sh_17 = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };
        let sh_18 = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };
        let sh_19 = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };
        let sh_20 = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };
        let sh_21 = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };
        let sh_22 = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };
        let sh_23 = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };
        let sh_24 = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };
        let sh_25 = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };
        let sh_26 = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };

        Ok(LightmapData {
            m_lightmap,
            sh_0,
            m_indirect_lightmap,
            sh_1,
            m_dir_lightmap,
            sh_2,
            m_shadow_mask,
            sh_3,
            sh_4,
            sh_5,
            sh_6,
            sh_7,
            sh_8,
            sh_9,
            sh_10,
            sh_11,
            sh_12,
            sh_13,
            sh_14,
            sh_15,
            sh_16,
            sh_17,
            sh_18,
            sh_19,
            sh_20,
            sh_21,
            sh_22,
            sh_23,
            sh_24,
            sh_25,
            sh_26,
        })
    }
}
