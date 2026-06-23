//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct RippleGroup {
    pub m_af_planar_0: f32,
    pub m_af_planar_1: f32,
    pub m_af_planar_2: f32,
    pub m_af_planar_3: f32,
    pub m_af_planar_4: f32,
    pub m_af_planar_5: f32,
    pub m_af_planar_6: f32,
    pub m_af_planar_7: f32,
    pub m_af_planar_8: f32,
    pub m_af_planar_9: f32,
    pub m_af_planar_10: f32,
    pub m_af_planar_11: f32,
    pub m_af_planar_12: f32,
    pub m_af_planar_13: f32,
    pub m_af_planar_14: f32,
    pub m_af_planar_15: f32,
    pub m_af_planar_16: f32,
    pub m_af_planar_17: f32,
    pub m_af_planar_18: f32,
    pub m_af_planar_19: f32,
    pub m_af_directional_0: f32,
    pub m_af_directional_1: f32,
    pub m_af_directional_2: f32,
    pub m_af_directional_3: f32,
    pub m_af_directional_4: f32,
    pub m_af_directional_5: f32,
    pub m_af_directional_6: f32,
    pub m_af_directional_7: f32,
    pub m_af_directional_8: f32,
    pub m_af_directional_9: f32,
    pub m_af_directional_10: f32,
    pub m_af_directional_11: f32,
    pub m_af_directional_12: f32,
    pub m_af_directional_13: f32,
    pub m_af_directional_14: f32,
    pub m_af_directional_15: f32,
    pub m_af_directional_16: f32,
    pub m_af_directional_17: f32,
    pub m_af_directional_18: f32,
    pub m_af_directional_19: f32,
    pub m_af_speed_0: f32,
    pub m_af_speed_1: f32,
    pub m_af_speed_2: f32,
    pub m_af_speed_3: f32,
    pub m_af_speed_4: f32,
    pub m_af_speed_5: f32,
    pub m_af_speed_6: f32,
    pub m_af_speed_7: f32,
    pub m_af_speed_8: f32,
    pub m_af_speed_9: f32,
    pub m_af_speed_10: f32,
    pub m_af_speed_11: f32,
    pub m_af_speed_12: f32,
    pub m_af_speed_13: f32,
    pub m_af_speed_14: f32,
    pub m_af_speed_15: f32,
    pub m_af_speed_16: f32,
    pub m_af_speed_17: f32,
    pub m_af_speed_18: f32,
    pub m_af_speed_19: f32,
    pub m_af_flexibility_0: f32,
    pub m_af_flexibility_1: f32,
    pub m_af_flexibility_2: f32,
    pub m_af_flexibility_3: f32,
    pub m_af_flexibility_4: f32,
    pub m_af_flexibility_5: f32,
    pub m_af_flexibility_6: f32,
    pub m_af_flexibility_7: f32,
    pub m_af_flexibility_8: f32,
    pub m_af_flexibility_9: f32,
    pub m_af_flexibility_10: f32,
    pub m_af_flexibility_11: f32,
    pub m_af_flexibility_12: f32,
    pub m_af_flexibility_13: f32,
    pub m_af_flexibility_14: f32,
    pub m_af_flexibility_15: f32,
    pub m_af_flexibility_16: f32,
    pub m_af_flexibility_17: f32,
    pub m_af_flexibility_18: f32,
    pub m_af_flexibility_19: f32,
    pub m_f_independence: f32,
    pub m_f_shimmer: f32,
}

impl crate::Parse for RippleGroup {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_af_planar_0 = r.read_f32()?;
        let m_af_planar_1 = r.read_f32()?;
        let m_af_planar_2 = r.read_f32()?;
        let m_af_planar_3 = r.read_f32()?;
        let m_af_planar_4 = r.read_f32()?;
        let m_af_planar_5 = r.read_f32()?;
        let m_af_planar_6 = r.read_f32()?;
        let m_af_planar_7 = r.read_f32()?;
        let m_af_planar_8 = r.read_f32()?;
        let m_af_planar_9 = r.read_f32()?;
        let m_af_planar_10 = r.read_f32()?;
        let m_af_planar_11 = r.read_f32()?;
        let m_af_planar_12 = r.read_f32()?;
        let m_af_planar_13 = r.read_f32()?;
        let m_af_planar_14 = r.read_f32()?;
        let m_af_planar_15 = r.read_f32()?;
        let m_af_planar_16 = r.read_f32()?;
        let m_af_planar_17 = r.read_f32()?;
        let m_af_planar_18 = r.read_f32()?;
        let m_af_planar_19 = r.read_f32()?;
        let m_af_directional_0 = r.read_f32()?;
        let m_af_directional_1 = r.read_f32()?;
        let m_af_directional_2 = r.read_f32()?;
        let m_af_directional_3 = r.read_f32()?;
        let m_af_directional_4 = r.read_f32()?;
        let m_af_directional_5 = r.read_f32()?;
        let m_af_directional_6 = r.read_f32()?;
        let m_af_directional_7 = r.read_f32()?;
        let m_af_directional_8 = r.read_f32()?;
        let m_af_directional_9 = r.read_f32()?;
        let m_af_directional_10 = r.read_f32()?;
        let m_af_directional_11 = r.read_f32()?;
        let m_af_directional_12 = r.read_f32()?;
        let m_af_directional_13 = r.read_f32()?;
        let m_af_directional_14 = r.read_f32()?;
        let m_af_directional_15 = r.read_f32()?;
        let m_af_directional_16 = r.read_f32()?;
        let m_af_directional_17 = r.read_f32()?;
        let m_af_directional_18 = r.read_f32()?;
        let m_af_directional_19 = r.read_f32()?;
        let m_af_speed_0 = r.read_f32()?;
        let m_af_speed_1 = r.read_f32()?;
        let m_af_speed_2 = r.read_f32()?;
        let m_af_speed_3 = r.read_f32()?;
        let m_af_speed_4 = r.read_f32()?;
        let m_af_speed_5 = r.read_f32()?;
        let m_af_speed_6 = r.read_f32()?;
        let m_af_speed_7 = r.read_f32()?;
        let m_af_speed_8 = r.read_f32()?;
        let m_af_speed_9 = r.read_f32()?;
        let m_af_speed_10 = r.read_f32()?;
        let m_af_speed_11 = r.read_f32()?;
        let m_af_speed_12 = r.read_f32()?;
        let m_af_speed_13 = r.read_f32()?;
        let m_af_speed_14 = r.read_f32()?;
        let m_af_speed_15 = r.read_f32()?;
        let m_af_speed_16 = r.read_f32()?;
        let m_af_speed_17 = r.read_f32()?;
        let m_af_speed_18 = r.read_f32()?;
        let m_af_speed_19 = r.read_f32()?;
        let m_af_flexibility_0 = r.read_f32()?;
        let m_af_flexibility_1 = r.read_f32()?;
        let m_af_flexibility_2 = r.read_f32()?;
        let m_af_flexibility_3 = r.read_f32()?;
        let m_af_flexibility_4 = r.read_f32()?;
        let m_af_flexibility_5 = r.read_f32()?;
        let m_af_flexibility_6 = r.read_f32()?;
        let m_af_flexibility_7 = r.read_f32()?;
        let m_af_flexibility_8 = r.read_f32()?;
        let m_af_flexibility_9 = r.read_f32()?;
        let m_af_flexibility_10 = r.read_f32()?;
        let m_af_flexibility_11 = r.read_f32()?;
        let m_af_flexibility_12 = r.read_f32()?;
        let m_af_flexibility_13 = r.read_f32()?;
        let m_af_flexibility_14 = r.read_f32()?;
        let m_af_flexibility_15 = r.read_f32()?;
        let m_af_flexibility_16 = r.read_f32()?;
        let m_af_flexibility_17 = r.read_f32()?;
        let m_af_flexibility_18 = r.read_f32()?;
        let m_af_flexibility_19 = r.read_f32()?;
        let m_f_independence = r.read_f32()?;
        let m_f_shimmer = r.read_f32()?;

        Ok(RippleGroup {
            m_af_planar_0,
            m_af_planar_1,
            m_af_planar_2,
            m_af_planar_3,
            m_af_planar_4,
            m_af_planar_5,
            m_af_planar_6,
            m_af_planar_7,
            m_af_planar_8,
            m_af_planar_9,
            m_af_planar_10,
            m_af_planar_11,
            m_af_planar_12,
            m_af_planar_13,
            m_af_planar_14,
            m_af_planar_15,
            m_af_planar_16,
            m_af_planar_17,
            m_af_planar_18,
            m_af_planar_19,
            m_af_directional_0,
            m_af_directional_1,
            m_af_directional_2,
            m_af_directional_3,
            m_af_directional_4,
            m_af_directional_5,
            m_af_directional_6,
            m_af_directional_7,
            m_af_directional_8,
            m_af_directional_9,
            m_af_directional_10,
            m_af_directional_11,
            m_af_directional_12,
            m_af_directional_13,
            m_af_directional_14,
            m_af_directional_15,
            m_af_directional_16,
            m_af_directional_17,
            m_af_directional_18,
            m_af_directional_19,
            m_af_speed_0,
            m_af_speed_1,
            m_af_speed_2,
            m_af_speed_3,
            m_af_speed_4,
            m_af_speed_5,
            m_af_speed_6,
            m_af_speed_7,
            m_af_speed_8,
            m_af_speed_9,
            m_af_speed_10,
            m_af_speed_11,
            m_af_speed_12,
            m_af_speed_13,
            m_af_speed_14,
            m_af_speed_15,
            m_af_speed_16,
            m_af_speed_17,
            m_af_speed_18,
            m_af_speed_19,
            m_af_flexibility_0,
            m_af_flexibility_1,
            m_af_flexibility_2,
            m_af_flexibility_3,
            m_af_flexibility_4,
            m_af_flexibility_5,
            m_af_flexibility_6,
            m_af_flexibility_7,
            m_af_flexibility_8,
            m_af_flexibility_9,
            m_af_flexibility_10,
            m_af_flexibility_11,
            m_af_flexibility_12,
            m_af_flexibility_13,
            m_af_flexibility_14,
            m_af_flexibility_15,
            m_af_flexibility_16,
            m_af_flexibility_17,
            m_af_flexibility_18,
            m_af_flexibility_19,
            m_f_independence,
            m_f_shimmer,
        })
    }
}
