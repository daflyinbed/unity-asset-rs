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
        let mut m_lightmap = None;
        let mut sh_0 = None;
        let mut m_indirect_lightmap = None;
        let mut sh_1 = None;
        let mut m_dir_lightmap = None;
        let mut sh_2 = None;
        let mut m_shadow_mask = None;
        let mut sh_3 = None;
        let mut sh_4 = None;
        let mut sh_5 = None;
        let mut sh_6 = None;
        let mut sh_7 = None;
        let mut sh_8 = None;
        let mut sh_9 = None;
        let mut sh_10 = None;
        let mut sh_11 = None;
        let mut sh_12 = None;
        let mut sh_13 = None;
        let mut sh_14 = None;
        let mut sh_15 = None;
        let mut sh_16 = None;
        let mut sh_17 = None;
        let mut sh_18 = None;
        let mut sh_19 = None;
        let mut sh_20 = None;
        let mut sh_21 = None;
        let mut sh_22 = None;
        let mut sh_23 = None;
        let mut sh_24 = None;
        let mut sh_25 = None;
        let mut sh_26 = None;
        if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_lightmap = Some(crate::PPtr::parse(rdr, ver)?);
            m_dir_lightmap = Some(crate::PPtr::parse(rdr, ver)?);
            m_shadow_mask = Some(crate::PPtr::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_lightmap = Some(crate::PPtr::parse(rdr, ver)?);
            m_indirect_lightmap = Some(crate::PPtr::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            sh_0 = Some(rdr.read_f32()?);
            sh_1 = Some(rdr.read_f32()?);
            sh_2 = Some(rdr.read_f32()?);
            sh_3 = Some(rdr.read_f32()?);
            sh_4 = Some(rdr.read_f32()?);
            sh_5 = Some(rdr.read_f32()?);
            sh_6 = Some(rdr.read_f32()?);
            sh_7 = Some(rdr.read_f32()?);
            sh_8 = Some(rdr.read_f32()?);
            sh_9 = Some(rdr.read_f32()?);
            sh_10 = Some(rdr.read_f32()?);
            sh_11 = Some(rdr.read_f32()?);
            sh_12 = Some(rdr.read_f32()?);
            sh_13 = Some(rdr.read_f32()?);
            sh_14 = Some(rdr.read_f32()?);
            sh_15 = Some(rdr.read_f32()?);
            sh_16 = Some(rdr.read_f32()?);
            sh_17 = Some(rdr.read_f32()?);
            sh_18 = Some(rdr.read_f32()?);
            sh_19 = Some(rdr.read_f32()?);
            sh_20 = Some(rdr.read_f32()?);
            sh_21 = Some(rdr.read_f32()?);
            sh_22 = Some(rdr.read_f32()?);
            sh_23 = Some(rdr.read_f32()?);
            sh_24 = Some(rdr.read_f32()?);
            sh_25 = Some(rdr.read_f32()?);
            sh_26 = Some(rdr.read_f32()?);
        } else {
            m_lightmap = Some(crate::PPtr::parse(rdr, ver)?);
            m_indirect_lightmap = Some(crate::PPtr::parse(rdr, ver)?);
        }

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
