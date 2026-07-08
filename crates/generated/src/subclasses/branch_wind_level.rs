//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct BranchWindLevel {
    pub m_af_bend_0: f32,
    pub m_af_bend_1: f32,
    pub m_af_bend_2: f32,
    pub m_af_bend_3: f32,
    pub m_af_bend_4: f32,
    pub m_af_bend_5: f32,
    pub m_af_bend_6: f32,
    pub m_af_bend_7: f32,
    pub m_af_bend_8: f32,
    pub m_af_bend_9: f32,
    pub m_af_bend_10: f32,
    pub m_af_bend_11: f32,
    pub m_af_bend_12: f32,
    pub m_af_bend_13: f32,
    pub m_af_bend_14: f32,
    pub m_af_bend_15: f32,
    pub m_af_bend_16: f32,
    pub m_af_bend_17: f32,
    pub m_af_bend_18: f32,
    pub m_af_bend_19: f32,
    pub m_af_oscillation_0: f32,
    pub m_af_oscillation_1: f32,
    pub m_af_oscillation_2: f32,
    pub m_af_oscillation_3: f32,
    pub m_af_oscillation_4: f32,
    pub m_af_oscillation_5: f32,
    pub m_af_oscillation_6: f32,
    pub m_af_oscillation_7: f32,
    pub m_af_oscillation_8: f32,
    pub m_af_oscillation_9: f32,
    pub m_af_oscillation_10: f32,
    pub m_af_oscillation_11: f32,
    pub m_af_oscillation_12: f32,
    pub m_af_oscillation_13: f32,
    pub m_af_oscillation_14: f32,
    pub m_af_oscillation_15: f32,
    pub m_af_oscillation_16: f32,
    pub m_af_oscillation_17: f32,
    pub m_af_oscillation_18: f32,
    pub m_af_oscillation_19: f32,
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
    pub m_af_turbulence_0: f32,
    pub m_af_turbulence_1: f32,
    pub m_af_turbulence_2: f32,
    pub m_af_turbulence_3: f32,
    pub m_af_turbulence_4: f32,
    pub m_af_turbulence_5: f32,
    pub m_af_turbulence_6: f32,
    pub m_af_turbulence_7: f32,
    pub m_af_turbulence_8: f32,
    pub m_af_turbulence_9: f32,
    pub m_af_turbulence_10: f32,
    pub m_af_turbulence_11: f32,
    pub m_af_turbulence_12: f32,
    pub m_af_turbulence_13: f32,
    pub m_af_turbulence_14: f32,
    pub m_af_turbulence_15: f32,
    pub m_af_turbulence_16: f32,
    pub m_af_turbulence_17: f32,
    pub m_af_turbulence_18: f32,
    pub m_af_turbulence_19: f32,
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
}

impl crate::Parse for BranchWindLevel {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_af_bend_0 = rdr.read_f32()?;
        let m_af_bend_1 = rdr.read_f32()?;
        let m_af_bend_2 = rdr.read_f32()?;
        let m_af_bend_3 = rdr.read_f32()?;
        let m_af_bend_4 = rdr.read_f32()?;
        let m_af_bend_5 = rdr.read_f32()?;
        let m_af_bend_6 = rdr.read_f32()?;
        let m_af_bend_7 = rdr.read_f32()?;
        let m_af_bend_8 = rdr.read_f32()?;
        let m_af_bend_9 = rdr.read_f32()?;
        let m_af_bend_10 = rdr.read_f32()?;
        let m_af_bend_11 = rdr.read_f32()?;
        let m_af_bend_12 = rdr.read_f32()?;
        let m_af_bend_13 = rdr.read_f32()?;
        let m_af_bend_14 = rdr.read_f32()?;
        let m_af_bend_15 = rdr.read_f32()?;
        let m_af_bend_16 = rdr.read_f32()?;
        let m_af_bend_17 = rdr.read_f32()?;
        let m_af_bend_18 = rdr.read_f32()?;
        let m_af_bend_19 = rdr.read_f32()?;
        let m_af_oscillation_0 = rdr.read_f32()?;
        let m_af_oscillation_1 = rdr.read_f32()?;
        let m_af_oscillation_2 = rdr.read_f32()?;
        let m_af_oscillation_3 = rdr.read_f32()?;
        let m_af_oscillation_4 = rdr.read_f32()?;
        let m_af_oscillation_5 = rdr.read_f32()?;
        let m_af_oscillation_6 = rdr.read_f32()?;
        let m_af_oscillation_7 = rdr.read_f32()?;
        let m_af_oscillation_8 = rdr.read_f32()?;
        let m_af_oscillation_9 = rdr.read_f32()?;
        let m_af_oscillation_10 = rdr.read_f32()?;
        let m_af_oscillation_11 = rdr.read_f32()?;
        let m_af_oscillation_12 = rdr.read_f32()?;
        let m_af_oscillation_13 = rdr.read_f32()?;
        let m_af_oscillation_14 = rdr.read_f32()?;
        let m_af_oscillation_15 = rdr.read_f32()?;
        let m_af_oscillation_16 = rdr.read_f32()?;
        let m_af_oscillation_17 = rdr.read_f32()?;
        let m_af_oscillation_18 = rdr.read_f32()?;
        let m_af_oscillation_19 = rdr.read_f32()?;
        let m_af_speed_0 = rdr.read_f32()?;
        let m_af_speed_1 = rdr.read_f32()?;
        let m_af_speed_2 = rdr.read_f32()?;
        let m_af_speed_3 = rdr.read_f32()?;
        let m_af_speed_4 = rdr.read_f32()?;
        let m_af_speed_5 = rdr.read_f32()?;
        let m_af_speed_6 = rdr.read_f32()?;
        let m_af_speed_7 = rdr.read_f32()?;
        let m_af_speed_8 = rdr.read_f32()?;
        let m_af_speed_9 = rdr.read_f32()?;
        let m_af_speed_10 = rdr.read_f32()?;
        let m_af_speed_11 = rdr.read_f32()?;
        let m_af_speed_12 = rdr.read_f32()?;
        let m_af_speed_13 = rdr.read_f32()?;
        let m_af_speed_14 = rdr.read_f32()?;
        let m_af_speed_15 = rdr.read_f32()?;
        let m_af_speed_16 = rdr.read_f32()?;
        let m_af_speed_17 = rdr.read_f32()?;
        let m_af_speed_18 = rdr.read_f32()?;
        let m_af_speed_19 = rdr.read_f32()?;
        let m_af_turbulence_0 = rdr.read_f32()?;
        let m_af_turbulence_1 = rdr.read_f32()?;
        let m_af_turbulence_2 = rdr.read_f32()?;
        let m_af_turbulence_3 = rdr.read_f32()?;
        let m_af_turbulence_4 = rdr.read_f32()?;
        let m_af_turbulence_5 = rdr.read_f32()?;
        let m_af_turbulence_6 = rdr.read_f32()?;
        let m_af_turbulence_7 = rdr.read_f32()?;
        let m_af_turbulence_8 = rdr.read_f32()?;
        let m_af_turbulence_9 = rdr.read_f32()?;
        let m_af_turbulence_10 = rdr.read_f32()?;
        let m_af_turbulence_11 = rdr.read_f32()?;
        let m_af_turbulence_12 = rdr.read_f32()?;
        let m_af_turbulence_13 = rdr.read_f32()?;
        let m_af_turbulence_14 = rdr.read_f32()?;
        let m_af_turbulence_15 = rdr.read_f32()?;
        let m_af_turbulence_16 = rdr.read_f32()?;
        let m_af_turbulence_17 = rdr.read_f32()?;
        let m_af_turbulence_18 = rdr.read_f32()?;
        let m_af_turbulence_19 = rdr.read_f32()?;
        let m_af_flexibility_0 = rdr.read_f32()?;
        let m_af_flexibility_1 = rdr.read_f32()?;
        let m_af_flexibility_2 = rdr.read_f32()?;
        let m_af_flexibility_3 = rdr.read_f32()?;
        let m_af_flexibility_4 = rdr.read_f32()?;
        let m_af_flexibility_5 = rdr.read_f32()?;
        let m_af_flexibility_6 = rdr.read_f32()?;
        let m_af_flexibility_7 = rdr.read_f32()?;
        let m_af_flexibility_8 = rdr.read_f32()?;
        let m_af_flexibility_9 = rdr.read_f32()?;
        let m_af_flexibility_10 = rdr.read_f32()?;
        let m_af_flexibility_11 = rdr.read_f32()?;
        let m_af_flexibility_12 = rdr.read_f32()?;
        let m_af_flexibility_13 = rdr.read_f32()?;
        let m_af_flexibility_14 = rdr.read_f32()?;
        let m_af_flexibility_15 = rdr.read_f32()?;
        let m_af_flexibility_16 = rdr.read_f32()?;
        let m_af_flexibility_17 = rdr.read_f32()?;
        let m_af_flexibility_18 = rdr.read_f32()?;
        let m_af_flexibility_19 = rdr.read_f32()?;
        let m_f_independence = rdr.read_f32()?;

        Ok(BranchWindLevel {
            m_af_bend_0,
            m_af_bend_1,
            m_af_bend_2,
            m_af_bend_3,
            m_af_bend_4,
            m_af_bend_5,
            m_af_bend_6,
            m_af_bend_7,
            m_af_bend_8,
            m_af_bend_9,
            m_af_bend_10,
            m_af_bend_11,
            m_af_bend_12,
            m_af_bend_13,
            m_af_bend_14,
            m_af_bend_15,
            m_af_bend_16,
            m_af_bend_17,
            m_af_bend_18,
            m_af_bend_19,
            m_af_oscillation_0,
            m_af_oscillation_1,
            m_af_oscillation_2,
            m_af_oscillation_3,
            m_af_oscillation_4,
            m_af_oscillation_5,
            m_af_oscillation_6,
            m_af_oscillation_7,
            m_af_oscillation_8,
            m_af_oscillation_9,
            m_af_oscillation_10,
            m_af_oscillation_11,
            m_af_oscillation_12,
            m_af_oscillation_13,
            m_af_oscillation_14,
            m_af_oscillation_15,
            m_af_oscillation_16,
            m_af_oscillation_17,
            m_af_oscillation_18,
            m_af_oscillation_19,
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
            m_af_turbulence_0,
            m_af_turbulence_1,
            m_af_turbulence_2,
            m_af_turbulence_3,
            m_af_turbulence_4,
            m_af_turbulence_5,
            m_af_turbulence_6,
            m_af_turbulence_7,
            m_af_turbulence_8,
            m_af_turbulence_9,
            m_af_turbulence_10,
            m_af_turbulence_11,
            m_af_turbulence_12,
            m_af_turbulence_13,
            m_af_turbulence_14,
            m_af_turbulence_15,
            m_af_turbulence_16,
            m_af_turbulence_17,
            m_af_turbulence_18,
            m_af_turbulence_19,
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
        })
    }
}
