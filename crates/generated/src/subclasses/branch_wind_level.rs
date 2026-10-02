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

impl crate::ToValue for BranchWindLevel {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_afBend_0".to_string(), self.m_af_bend_0.to_value())),
            Some(("m_afBend_1".to_string(), self.m_af_bend_1.to_value())),
            Some(("m_afBend_2".to_string(), self.m_af_bend_2.to_value())),
            Some(("m_afBend_3".to_string(), self.m_af_bend_3.to_value())),
            Some(("m_afBend_4".to_string(), self.m_af_bend_4.to_value())),
            Some(("m_afBend_5".to_string(), self.m_af_bend_5.to_value())),
            Some(("m_afBend_6".to_string(), self.m_af_bend_6.to_value())),
            Some(("m_afBend_7".to_string(), self.m_af_bend_7.to_value())),
            Some(("m_afBend_8".to_string(), self.m_af_bend_8.to_value())),
            Some(("m_afBend_9".to_string(), self.m_af_bend_9.to_value())),
            Some(("m_afBend_10".to_string(), self.m_af_bend_10.to_value())),
            Some(("m_afBend_11".to_string(), self.m_af_bend_11.to_value())),
            Some(("m_afBend_12".to_string(), self.m_af_bend_12.to_value())),
            Some(("m_afBend_13".to_string(), self.m_af_bend_13.to_value())),
            Some(("m_afBend_14".to_string(), self.m_af_bend_14.to_value())),
            Some(("m_afBend_15".to_string(), self.m_af_bend_15.to_value())),
            Some(("m_afBend_16".to_string(), self.m_af_bend_16.to_value())),
            Some(("m_afBend_17".to_string(), self.m_af_bend_17.to_value())),
            Some(("m_afBend_18".to_string(), self.m_af_bend_18.to_value())),
            Some(("m_afBend_19".to_string(), self.m_af_bend_19.to_value())),
            Some((
                "m_afOscillation_0".to_string(),
                self.m_af_oscillation_0.to_value(),
            )),
            Some((
                "m_afOscillation_1".to_string(),
                self.m_af_oscillation_1.to_value(),
            )),
            Some((
                "m_afOscillation_2".to_string(),
                self.m_af_oscillation_2.to_value(),
            )),
            Some((
                "m_afOscillation_3".to_string(),
                self.m_af_oscillation_3.to_value(),
            )),
            Some((
                "m_afOscillation_4".to_string(),
                self.m_af_oscillation_4.to_value(),
            )),
            Some((
                "m_afOscillation_5".to_string(),
                self.m_af_oscillation_5.to_value(),
            )),
            Some((
                "m_afOscillation_6".to_string(),
                self.m_af_oscillation_6.to_value(),
            )),
            Some((
                "m_afOscillation_7".to_string(),
                self.m_af_oscillation_7.to_value(),
            )),
            Some((
                "m_afOscillation_8".to_string(),
                self.m_af_oscillation_8.to_value(),
            )),
            Some((
                "m_afOscillation_9".to_string(),
                self.m_af_oscillation_9.to_value(),
            )),
            Some((
                "m_afOscillation_10".to_string(),
                self.m_af_oscillation_10.to_value(),
            )),
            Some((
                "m_afOscillation_11".to_string(),
                self.m_af_oscillation_11.to_value(),
            )),
            Some((
                "m_afOscillation_12".to_string(),
                self.m_af_oscillation_12.to_value(),
            )),
            Some((
                "m_afOscillation_13".to_string(),
                self.m_af_oscillation_13.to_value(),
            )),
            Some((
                "m_afOscillation_14".to_string(),
                self.m_af_oscillation_14.to_value(),
            )),
            Some((
                "m_afOscillation_15".to_string(),
                self.m_af_oscillation_15.to_value(),
            )),
            Some((
                "m_afOscillation_16".to_string(),
                self.m_af_oscillation_16.to_value(),
            )),
            Some((
                "m_afOscillation_17".to_string(),
                self.m_af_oscillation_17.to_value(),
            )),
            Some((
                "m_afOscillation_18".to_string(),
                self.m_af_oscillation_18.to_value(),
            )),
            Some((
                "m_afOscillation_19".to_string(),
                self.m_af_oscillation_19.to_value(),
            )),
            Some(("m_afSpeed_0".to_string(), self.m_af_speed_0.to_value())),
            Some(("m_afSpeed_1".to_string(), self.m_af_speed_1.to_value())),
            Some(("m_afSpeed_2".to_string(), self.m_af_speed_2.to_value())),
            Some(("m_afSpeed_3".to_string(), self.m_af_speed_3.to_value())),
            Some(("m_afSpeed_4".to_string(), self.m_af_speed_4.to_value())),
            Some(("m_afSpeed_5".to_string(), self.m_af_speed_5.to_value())),
            Some(("m_afSpeed_6".to_string(), self.m_af_speed_6.to_value())),
            Some(("m_afSpeed_7".to_string(), self.m_af_speed_7.to_value())),
            Some(("m_afSpeed_8".to_string(), self.m_af_speed_8.to_value())),
            Some(("m_afSpeed_9".to_string(), self.m_af_speed_9.to_value())),
            Some(("m_afSpeed_10".to_string(), self.m_af_speed_10.to_value())),
            Some(("m_afSpeed_11".to_string(), self.m_af_speed_11.to_value())),
            Some(("m_afSpeed_12".to_string(), self.m_af_speed_12.to_value())),
            Some(("m_afSpeed_13".to_string(), self.m_af_speed_13.to_value())),
            Some(("m_afSpeed_14".to_string(), self.m_af_speed_14.to_value())),
            Some(("m_afSpeed_15".to_string(), self.m_af_speed_15.to_value())),
            Some(("m_afSpeed_16".to_string(), self.m_af_speed_16.to_value())),
            Some(("m_afSpeed_17".to_string(), self.m_af_speed_17.to_value())),
            Some(("m_afSpeed_18".to_string(), self.m_af_speed_18.to_value())),
            Some(("m_afSpeed_19".to_string(), self.m_af_speed_19.to_value())),
            Some((
                "m_afTurbulence_0".to_string(),
                self.m_af_turbulence_0.to_value(),
            )),
            Some((
                "m_afTurbulence_1".to_string(),
                self.m_af_turbulence_1.to_value(),
            )),
            Some((
                "m_afTurbulence_2".to_string(),
                self.m_af_turbulence_2.to_value(),
            )),
            Some((
                "m_afTurbulence_3".to_string(),
                self.m_af_turbulence_3.to_value(),
            )),
            Some((
                "m_afTurbulence_4".to_string(),
                self.m_af_turbulence_4.to_value(),
            )),
            Some((
                "m_afTurbulence_5".to_string(),
                self.m_af_turbulence_5.to_value(),
            )),
            Some((
                "m_afTurbulence_6".to_string(),
                self.m_af_turbulence_6.to_value(),
            )),
            Some((
                "m_afTurbulence_7".to_string(),
                self.m_af_turbulence_7.to_value(),
            )),
            Some((
                "m_afTurbulence_8".to_string(),
                self.m_af_turbulence_8.to_value(),
            )),
            Some((
                "m_afTurbulence_9".to_string(),
                self.m_af_turbulence_9.to_value(),
            )),
            Some((
                "m_afTurbulence_10".to_string(),
                self.m_af_turbulence_10.to_value(),
            )),
            Some((
                "m_afTurbulence_11".to_string(),
                self.m_af_turbulence_11.to_value(),
            )),
            Some((
                "m_afTurbulence_12".to_string(),
                self.m_af_turbulence_12.to_value(),
            )),
            Some((
                "m_afTurbulence_13".to_string(),
                self.m_af_turbulence_13.to_value(),
            )),
            Some((
                "m_afTurbulence_14".to_string(),
                self.m_af_turbulence_14.to_value(),
            )),
            Some((
                "m_afTurbulence_15".to_string(),
                self.m_af_turbulence_15.to_value(),
            )),
            Some((
                "m_afTurbulence_16".to_string(),
                self.m_af_turbulence_16.to_value(),
            )),
            Some((
                "m_afTurbulence_17".to_string(),
                self.m_af_turbulence_17.to_value(),
            )),
            Some((
                "m_afTurbulence_18".to_string(),
                self.m_af_turbulence_18.to_value(),
            )),
            Some((
                "m_afTurbulence_19".to_string(),
                self.m_af_turbulence_19.to_value(),
            )),
            Some((
                "m_afFlexibility_0".to_string(),
                self.m_af_flexibility_0.to_value(),
            )),
            Some((
                "m_afFlexibility_1".to_string(),
                self.m_af_flexibility_1.to_value(),
            )),
            Some((
                "m_afFlexibility_2".to_string(),
                self.m_af_flexibility_2.to_value(),
            )),
            Some((
                "m_afFlexibility_3".to_string(),
                self.m_af_flexibility_3.to_value(),
            )),
            Some((
                "m_afFlexibility_4".to_string(),
                self.m_af_flexibility_4.to_value(),
            )),
            Some((
                "m_afFlexibility_5".to_string(),
                self.m_af_flexibility_5.to_value(),
            )),
            Some((
                "m_afFlexibility_6".to_string(),
                self.m_af_flexibility_6.to_value(),
            )),
            Some((
                "m_afFlexibility_7".to_string(),
                self.m_af_flexibility_7.to_value(),
            )),
            Some((
                "m_afFlexibility_8".to_string(),
                self.m_af_flexibility_8.to_value(),
            )),
            Some((
                "m_afFlexibility_9".to_string(),
                self.m_af_flexibility_9.to_value(),
            )),
            Some((
                "m_afFlexibility_10".to_string(),
                self.m_af_flexibility_10.to_value(),
            )),
            Some((
                "m_afFlexibility_11".to_string(),
                self.m_af_flexibility_11.to_value(),
            )),
            Some((
                "m_afFlexibility_12".to_string(),
                self.m_af_flexibility_12.to_value(),
            )),
            Some((
                "m_afFlexibility_13".to_string(),
                self.m_af_flexibility_13.to_value(),
            )),
            Some((
                "m_afFlexibility_14".to_string(),
                self.m_af_flexibility_14.to_value(),
            )),
            Some((
                "m_afFlexibility_15".to_string(),
                self.m_af_flexibility_15.to_value(),
            )),
            Some((
                "m_afFlexibility_16".to_string(),
                self.m_af_flexibility_16.to_value(),
            )),
            Some((
                "m_afFlexibility_17".to_string(),
                self.m_af_flexibility_17.to_value(),
            )),
            Some((
                "m_afFlexibility_18".to_string(),
                self.m_af_flexibility_18.to_value(),
            )),
            Some((
                "m_afFlexibility_19".to_string(),
                self.m_af_flexibility_19.to_value(),
            )),
            Some((
                "m_fIndependence".to_string(),
                self.m_f_independence.to_value(),
            )),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_afBend_0" => (self.m_af_bend_0).inspect_value(path, offset, limit),
                "m_afBend_1" => (self.m_af_bend_1).inspect_value(path, offset, limit),
                "m_afBend_2" => (self.m_af_bend_2).inspect_value(path, offset, limit),
                "m_afBend_3" => (self.m_af_bend_3).inspect_value(path, offset, limit),
                "m_afBend_4" => (self.m_af_bend_4).inspect_value(path, offset, limit),
                "m_afBend_5" => (self.m_af_bend_5).inspect_value(path, offset, limit),
                "m_afBend_6" => (self.m_af_bend_6).inspect_value(path, offset, limit),
                "m_afBend_7" => (self.m_af_bend_7).inspect_value(path, offset, limit),
                "m_afBend_8" => (self.m_af_bend_8).inspect_value(path, offset, limit),
                "m_afBend_9" => (self.m_af_bend_9).inspect_value(path, offset, limit),
                "m_afBend_10" => (self.m_af_bend_10).inspect_value(path, offset, limit),
                "m_afBend_11" => (self.m_af_bend_11).inspect_value(path, offset, limit),
                "m_afBend_12" => (self.m_af_bend_12).inspect_value(path, offset, limit),
                "m_afBend_13" => (self.m_af_bend_13).inspect_value(path, offset, limit),
                "m_afBend_14" => (self.m_af_bend_14).inspect_value(path, offset, limit),
                "m_afBend_15" => (self.m_af_bend_15).inspect_value(path, offset, limit),
                "m_afBend_16" => (self.m_af_bend_16).inspect_value(path, offset, limit),
                "m_afBend_17" => (self.m_af_bend_17).inspect_value(path, offset, limit),
                "m_afBend_18" => (self.m_af_bend_18).inspect_value(path, offset, limit),
                "m_afBend_19" => (self.m_af_bend_19).inspect_value(path, offset, limit),
                "m_afOscillation_0" => (self.m_af_oscillation_0).inspect_value(path, offset, limit),
                "m_afOscillation_1" => (self.m_af_oscillation_1).inspect_value(path, offset, limit),
                "m_afOscillation_2" => (self.m_af_oscillation_2).inspect_value(path, offset, limit),
                "m_afOscillation_3" => (self.m_af_oscillation_3).inspect_value(path, offset, limit),
                "m_afOscillation_4" => (self.m_af_oscillation_4).inspect_value(path, offset, limit),
                "m_afOscillation_5" => (self.m_af_oscillation_5).inspect_value(path, offset, limit),
                "m_afOscillation_6" => (self.m_af_oscillation_6).inspect_value(path, offset, limit),
                "m_afOscillation_7" => (self.m_af_oscillation_7).inspect_value(path, offset, limit),
                "m_afOscillation_8" => (self.m_af_oscillation_8).inspect_value(path, offset, limit),
                "m_afOscillation_9" => (self.m_af_oscillation_9).inspect_value(path, offset, limit),
                "m_afOscillation_10" => {
                    (self.m_af_oscillation_10).inspect_value(path, offset, limit)
                }
                "m_afOscillation_11" => {
                    (self.m_af_oscillation_11).inspect_value(path, offset, limit)
                }
                "m_afOscillation_12" => {
                    (self.m_af_oscillation_12).inspect_value(path, offset, limit)
                }
                "m_afOscillation_13" => {
                    (self.m_af_oscillation_13).inspect_value(path, offset, limit)
                }
                "m_afOscillation_14" => {
                    (self.m_af_oscillation_14).inspect_value(path, offset, limit)
                }
                "m_afOscillation_15" => {
                    (self.m_af_oscillation_15).inspect_value(path, offset, limit)
                }
                "m_afOscillation_16" => {
                    (self.m_af_oscillation_16).inspect_value(path, offset, limit)
                }
                "m_afOscillation_17" => {
                    (self.m_af_oscillation_17).inspect_value(path, offset, limit)
                }
                "m_afOscillation_18" => {
                    (self.m_af_oscillation_18).inspect_value(path, offset, limit)
                }
                "m_afOscillation_19" => {
                    (self.m_af_oscillation_19).inspect_value(path, offset, limit)
                }
                "m_afSpeed_0" => (self.m_af_speed_0).inspect_value(path, offset, limit),
                "m_afSpeed_1" => (self.m_af_speed_1).inspect_value(path, offset, limit),
                "m_afSpeed_2" => (self.m_af_speed_2).inspect_value(path, offset, limit),
                "m_afSpeed_3" => (self.m_af_speed_3).inspect_value(path, offset, limit),
                "m_afSpeed_4" => (self.m_af_speed_4).inspect_value(path, offset, limit),
                "m_afSpeed_5" => (self.m_af_speed_5).inspect_value(path, offset, limit),
                "m_afSpeed_6" => (self.m_af_speed_6).inspect_value(path, offset, limit),
                "m_afSpeed_7" => (self.m_af_speed_7).inspect_value(path, offset, limit),
                "m_afSpeed_8" => (self.m_af_speed_8).inspect_value(path, offset, limit),
                "m_afSpeed_9" => (self.m_af_speed_9).inspect_value(path, offset, limit),
                "m_afSpeed_10" => (self.m_af_speed_10).inspect_value(path, offset, limit),
                "m_afSpeed_11" => (self.m_af_speed_11).inspect_value(path, offset, limit),
                "m_afSpeed_12" => (self.m_af_speed_12).inspect_value(path, offset, limit),
                "m_afSpeed_13" => (self.m_af_speed_13).inspect_value(path, offset, limit),
                "m_afSpeed_14" => (self.m_af_speed_14).inspect_value(path, offset, limit),
                "m_afSpeed_15" => (self.m_af_speed_15).inspect_value(path, offset, limit),
                "m_afSpeed_16" => (self.m_af_speed_16).inspect_value(path, offset, limit),
                "m_afSpeed_17" => (self.m_af_speed_17).inspect_value(path, offset, limit),
                "m_afSpeed_18" => (self.m_af_speed_18).inspect_value(path, offset, limit),
                "m_afSpeed_19" => (self.m_af_speed_19).inspect_value(path, offset, limit),
                "m_afTurbulence_0" => (self.m_af_turbulence_0).inspect_value(path, offset, limit),
                "m_afTurbulence_1" => (self.m_af_turbulence_1).inspect_value(path, offset, limit),
                "m_afTurbulence_2" => (self.m_af_turbulence_2).inspect_value(path, offset, limit),
                "m_afTurbulence_3" => (self.m_af_turbulence_3).inspect_value(path, offset, limit),
                "m_afTurbulence_4" => (self.m_af_turbulence_4).inspect_value(path, offset, limit),
                "m_afTurbulence_5" => (self.m_af_turbulence_5).inspect_value(path, offset, limit),
                "m_afTurbulence_6" => (self.m_af_turbulence_6).inspect_value(path, offset, limit),
                "m_afTurbulence_7" => (self.m_af_turbulence_7).inspect_value(path, offset, limit),
                "m_afTurbulence_8" => (self.m_af_turbulence_8).inspect_value(path, offset, limit),
                "m_afTurbulence_9" => (self.m_af_turbulence_9).inspect_value(path, offset, limit),
                "m_afTurbulence_10" => (self.m_af_turbulence_10).inspect_value(path, offset, limit),
                "m_afTurbulence_11" => (self.m_af_turbulence_11).inspect_value(path, offset, limit),
                "m_afTurbulence_12" => (self.m_af_turbulence_12).inspect_value(path, offset, limit),
                "m_afTurbulence_13" => (self.m_af_turbulence_13).inspect_value(path, offset, limit),
                "m_afTurbulence_14" => (self.m_af_turbulence_14).inspect_value(path, offset, limit),
                "m_afTurbulence_15" => (self.m_af_turbulence_15).inspect_value(path, offset, limit),
                "m_afTurbulence_16" => (self.m_af_turbulence_16).inspect_value(path, offset, limit),
                "m_afTurbulence_17" => (self.m_af_turbulence_17).inspect_value(path, offset, limit),
                "m_afTurbulence_18" => (self.m_af_turbulence_18).inspect_value(path, offset, limit),
                "m_afTurbulence_19" => (self.m_af_turbulence_19).inspect_value(path, offset, limit),
                "m_afFlexibility_0" => (self.m_af_flexibility_0).inspect_value(path, offset, limit),
                "m_afFlexibility_1" => (self.m_af_flexibility_1).inspect_value(path, offset, limit),
                "m_afFlexibility_2" => (self.m_af_flexibility_2).inspect_value(path, offset, limit),
                "m_afFlexibility_3" => (self.m_af_flexibility_3).inspect_value(path, offset, limit),
                "m_afFlexibility_4" => (self.m_af_flexibility_4).inspect_value(path, offset, limit),
                "m_afFlexibility_5" => (self.m_af_flexibility_5).inspect_value(path, offset, limit),
                "m_afFlexibility_6" => (self.m_af_flexibility_6).inspect_value(path, offset, limit),
                "m_afFlexibility_7" => (self.m_af_flexibility_7).inspect_value(path, offset, limit),
                "m_afFlexibility_8" => (self.m_af_flexibility_8).inspect_value(path, offset, limit),
                "m_afFlexibility_9" => (self.m_af_flexibility_9).inspect_value(path, offset, limit),
                "m_afFlexibility_10" => {
                    (self.m_af_flexibility_10).inspect_value(path, offset, limit)
                }
                "m_afFlexibility_11" => {
                    (self.m_af_flexibility_11).inspect_value(path, offset, limit)
                }
                "m_afFlexibility_12" => {
                    (self.m_af_flexibility_12).inspect_value(path, offset, limit)
                }
                "m_afFlexibility_13" => {
                    (self.m_af_flexibility_13).inspect_value(path, offset, limit)
                }
                "m_afFlexibility_14" => {
                    (self.m_af_flexibility_14).inspect_value(path, offset, limit)
                }
                "m_afFlexibility_15" => {
                    (self.m_af_flexibility_15).inspect_value(path, offset, limit)
                }
                "m_afFlexibility_16" => {
                    (self.m_af_flexibility_16).inspect_value(path, offset, limit)
                }
                "m_afFlexibility_17" => {
                    (self.m_af_flexibility_17).inspect_value(path, offset, limit)
                }
                "m_afFlexibility_18" => {
                    (self.m_af_flexibility_18).inspect_value(path, offset, limit)
                }
                "m_afFlexibility_19" => {
                    (self.m_af_flexibility_19).inspect_value(path, offset, limit)
                }
                "m_fIndependence" => (self.m_f_independence).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_af_bend_0).inspect_value(path, offset, limit))
                .map(|v| ("m_afBend_0".to_string(), v)),
            ((self.m_af_bend_1).inspect_value(path, offset, limit))
                .map(|v| ("m_afBend_1".to_string(), v)),
            ((self.m_af_bend_2).inspect_value(path, offset, limit))
                .map(|v| ("m_afBend_2".to_string(), v)),
            ((self.m_af_bend_3).inspect_value(path, offset, limit))
                .map(|v| ("m_afBend_3".to_string(), v)),
            ((self.m_af_bend_4).inspect_value(path, offset, limit))
                .map(|v| ("m_afBend_4".to_string(), v)),
            ((self.m_af_bend_5).inspect_value(path, offset, limit))
                .map(|v| ("m_afBend_5".to_string(), v)),
            ((self.m_af_bend_6).inspect_value(path, offset, limit))
                .map(|v| ("m_afBend_6".to_string(), v)),
            ((self.m_af_bend_7).inspect_value(path, offset, limit))
                .map(|v| ("m_afBend_7".to_string(), v)),
            ((self.m_af_bend_8).inspect_value(path, offset, limit))
                .map(|v| ("m_afBend_8".to_string(), v)),
            ((self.m_af_bend_9).inspect_value(path, offset, limit))
                .map(|v| ("m_afBend_9".to_string(), v)),
            ((self.m_af_bend_10).inspect_value(path, offset, limit))
                .map(|v| ("m_afBend_10".to_string(), v)),
            ((self.m_af_bend_11).inspect_value(path, offset, limit))
                .map(|v| ("m_afBend_11".to_string(), v)),
            ((self.m_af_bend_12).inspect_value(path, offset, limit))
                .map(|v| ("m_afBend_12".to_string(), v)),
            ((self.m_af_bend_13).inspect_value(path, offset, limit))
                .map(|v| ("m_afBend_13".to_string(), v)),
            ((self.m_af_bend_14).inspect_value(path, offset, limit))
                .map(|v| ("m_afBend_14".to_string(), v)),
            ((self.m_af_bend_15).inspect_value(path, offset, limit))
                .map(|v| ("m_afBend_15".to_string(), v)),
            ((self.m_af_bend_16).inspect_value(path, offset, limit))
                .map(|v| ("m_afBend_16".to_string(), v)),
            ((self.m_af_bend_17).inspect_value(path, offset, limit))
                .map(|v| ("m_afBend_17".to_string(), v)),
            ((self.m_af_bend_18).inspect_value(path, offset, limit))
                .map(|v| ("m_afBend_18".to_string(), v)),
            ((self.m_af_bend_19).inspect_value(path, offset, limit))
                .map(|v| ("m_afBend_19".to_string(), v)),
            ((self.m_af_oscillation_0).inspect_value(path, offset, limit))
                .map(|v| ("m_afOscillation_0".to_string(), v)),
            ((self.m_af_oscillation_1).inspect_value(path, offset, limit))
                .map(|v| ("m_afOscillation_1".to_string(), v)),
            ((self.m_af_oscillation_2).inspect_value(path, offset, limit))
                .map(|v| ("m_afOscillation_2".to_string(), v)),
            ((self.m_af_oscillation_3).inspect_value(path, offset, limit))
                .map(|v| ("m_afOscillation_3".to_string(), v)),
            ((self.m_af_oscillation_4).inspect_value(path, offset, limit))
                .map(|v| ("m_afOscillation_4".to_string(), v)),
            ((self.m_af_oscillation_5).inspect_value(path, offset, limit))
                .map(|v| ("m_afOscillation_5".to_string(), v)),
            ((self.m_af_oscillation_6).inspect_value(path, offset, limit))
                .map(|v| ("m_afOscillation_6".to_string(), v)),
            ((self.m_af_oscillation_7).inspect_value(path, offset, limit))
                .map(|v| ("m_afOscillation_7".to_string(), v)),
            ((self.m_af_oscillation_8).inspect_value(path, offset, limit))
                .map(|v| ("m_afOscillation_8".to_string(), v)),
            ((self.m_af_oscillation_9).inspect_value(path, offset, limit))
                .map(|v| ("m_afOscillation_9".to_string(), v)),
            ((self.m_af_oscillation_10).inspect_value(path, offset, limit))
                .map(|v| ("m_afOscillation_10".to_string(), v)),
            ((self.m_af_oscillation_11).inspect_value(path, offset, limit))
                .map(|v| ("m_afOscillation_11".to_string(), v)),
            ((self.m_af_oscillation_12).inspect_value(path, offset, limit))
                .map(|v| ("m_afOscillation_12".to_string(), v)),
            ((self.m_af_oscillation_13).inspect_value(path, offset, limit))
                .map(|v| ("m_afOscillation_13".to_string(), v)),
            ((self.m_af_oscillation_14).inspect_value(path, offset, limit))
                .map(|v| ("m_afOscillation_14".to_string(), v)),
            ((self.m_af_oscillation_15).inspect_value(path, offset, limit))
                .map(|v| ("m_afOscillation_15".to_string(), v)),
            ((self.m_af_oscillation_16).inspect_value(path, offset, limit))
                .map(|v| ("m_afOscillation_16".to_string(), v)),
            ((self.m_af_oscillation_17).inspect_value(path, offset, limit))
                .map(|v| ("m_afOscillation_17".to_string(), v)),
            ((self.m_af_oscillation_18).inspect_value(path, offset, limit))
                .map(|v| ("m_afOscillation_18".to_string(), v)),
            ((self.m_af_oscillation_19).inspect_value(path, offset, limit))
                .map(|v| ("m_afOscillation_19".to_string(), v)),
            ((self.m_af_speed_0).inspect_value(path, offset, limit))
                .map(|v| ("m_afSpeed_0".to_string(), v)),
            ((self.m_af_speed_1).inspect_value(path, offset, limit))
                .map(|v| ("m_afSpeed_1".to_string(), v)),
            ((self.m_af_speed_2).inspect_value(path, offset, limit))
                .map(|v| ("m_afSpeed_2".to_string(), v)),
            ((self.m_af_speed_3).inspect_value(path, offset, limit))
                .map(|v| ("m_afSpeed_3".to_string(), v)),
            ((self.m_af_speed_4).inspect_value(path, offset, limit))
                .map(|v| ("m_afSpeed_4".to_string(), v)),
            ((self.m_af_speed_5).inspect_value(path, offset, limit))
                .map(|v| ("m_afSpeed_5".to_string(), v)),
            ((self.m_af_speed_6).inspect_value(path, offset, limit))
                .map(|v| ("m_afSpeed_6".to_string(), v)),
            ((self.m_af_speed_7).inspect_value(path, offset, limit))
                .map(|v| ("m_afSpeed_7".to_string(), v)),
            ((self.m_af_speed_8).inspect_value(path, offset, limit))
                .map(|v| ("m_afSpeed_8".to_string(), v)),
            ((self.m_af_speed_9).inspect_value(path, offset, limit))
                .map(|v| ("m_afSpeed_9".to_string(), v)),
            ((self.m_af_speed_10).inspect_value(path, offset, limit))
                .map(|v| ("m_afSpeed_10".to_string(), v)),
            ((self.m_af_speed_11).inspect_value(path, offset, limit))
                .map(|v| ("m_afSpeed_11".to_string(), v)),
            ((self.m_af_speed_12).inspect_value(path, offset, limit))
                .map(|v| ("m_afSpeed_12".to_string(), v)),
            ((self.m_af_speed_13).inspect_value(path, offset, limit))
                .map(|v| ("m_afSpeed_13".to_string(), v)),
            ((self.m_af_speed_14).inspect_value(path, offset, limit))
                .map(|v| ("m_afSpeed_14".to_string(), v)),
            ((self.m_af_speed_15).inspect_value(path, offset, limit))
                .map(|v| ("m_afSpeed_15".to_string(), v)),
            ((self.m_af_speed_16).inspect_value(path, offset, limit))
                .map(|v| ("m_afSpeed_16".to_string(), v)),
            ((self.m_af_speed_17).inspect_value(path, offset, limit))
                .map(|v| ("m_afSpeed_17".to_string(), v)),
            ((self.m_af_speed_18).inspect_value(path, offset, limit))
                .map(|v| ("m_afSpeed_18".to_string(), v)),
            ((self.m_af_speed_19).inspect_value(path, offset, limit))
                .map(|v| ("m_afSpeed_19".to_string(), v)),
            ((self.m_af_turbulence_0).inspect_value(path, offset, limit))
                .map(|v| ("m_afTurbulence_0".to_string(), v)),
            ((self.m_af_turbulence_1).inspect_value(path, offset, limit))
                .map(|v| ("m_afTurbulence_1".to_string(), v)),
            ((self.m_af_turbulence_2).inspect_value(path, offset, limit))
                .map(|v| ("m_afTurbulence_2".to_string(), v)),
            ((self.m_af_turbulence_3).inspect_value(path, offset, limit))
                .map(|v| ("m_afTurbulence_3".to_string(), v)),
            ((self.m_af_turbulence_4).inspect_value(path, offset, limit))
                .map(|v| ("m_afTurbulence_4".to_string(), v)),
            ((self.m_af_turbulence_5).inspect_value(path, offset, limit))
                .map(|v| ("m_afTurbulence_5".to_string(), v)),
            ((self.m_af_turbulence_6).inspect_value(path, offset, limit))
                .map(|v| ("m_afTurbulence_6".to_string(), v)),
            ((self.m_af_turbulence_7).inspect_value(path, offset, limit))
                .map(|v| ("m_afTurbulence_7".to_string(), v)),
            ((self.m_af_turbulence_8).inspect_value(path, offset, limit))
                .map(|v| ("m_afTurbulence_8".to_string(), v)),
            ((self.m_af_turbulence_9).inspect_value(path, offset, limit))
                .map(|v| ("m_afTurbulence_9".to_string(), v)),
            ((self.m_af_turbulence_10).inspect_value(path, offset, limit))
                .map(|v| ("m_afTurbulence_10".to_string(), v)),
            ((self.m_af_turbulence_11).inspect_value(path, offset, limit))
                .map(|v| ("m_afTurbulence_11".to_string(), v)),
            ((self.m_af_turbulence_12).inspect_value(path, offset, limit))
                .map(|v| ("m_afTurbulence_12".to_string(), v)),
            ((self.m_af_turbulence_13).inspect_value(path, offset, limit))
                .map(|v| ("m_afTurbulence_13".to_string(), v)),
            ((self.m_af_turbulence_14).inspect_value(path, offset, limit))
                .map(|v| ("m_afTurbulence_14".to_string(), v)),
            ((self.m_af_turbulence_15).inspect_value(path, offset, limit))
                .map(|v| ("m_afTurbulence_15".to_string(), v)),
            ((self.m_af_turbulence_16).inspect_value(path, offset, limit))
                .map(|v| ("m_afTurbulence_16".to_string(), v)),
            ((self.m_af_turbulence_17).inspect_value(path, offset, limit))
                .map(|v| ("m_afTurbulence_17".to_string(), v)),
            ((self.m_af_turbulence_18).inspect_value(path, offset, limit))
                .map(|v| ("m_afTurbulence_18".to_string(), v)),
            ((self.m_af_turbulence_19).inspect_value(path, offset, limit))
                .map(|v| ("m_afTurbulence_19".to_string(), v)),
            ((self.m_af_flexibility_0).inspect_value(path, offset, limit))
                .map(|v| ("m_afFlexibility_0".to_string(), v)),
            ((self.m_af_flexibility_1).inspect_value(path, offset, limit))
                .map(|v| ("m_afFlexibility_1".to_string(), v)),
            ((self.m_af_flexibility_2).inspect_value(path, offset, limit))
                .map(|v| ("m_afFlexibility_2".to_string(), v)),
            ((self.m_af_flexibility_3).inspect_value(path, offset, limit))
                .map(|v| ("m_afFlexibility_3".to_string(), v)),
            ((self.m_af_flexibility_4).inspect_value(path, offset, limit))
                .map(|v| ("m_afFlexibility_4".to_string(), v)),
            ((self.m_af_flexibility_5).inspect_value(path, offset, limit))
                .map(|v| ("m_afFlexibility_5".to_string(), v)),
            ((self.m_af_flexibility_6).inspect_value(path, offset, limit))
                .map(|v| ("m_afFlexibility_6".to_string(), v)),
            ((self.m_af_flexibility_7).inspect_value(path, offset, limit))
                .map(|v| ("m_afFlexibility_7".to_string(), v)),
            ((self.m_af_flexibility_8).inspect_value(path, offset, limit))
                .map(|v| ("m_afFlexibility_8".to_string(), v)),
            ((self.m_af_flexibility_9).inspect_value(path, offset, limit))
                .map(|v| ("m_afFlexibility_9".to_string(), v)),
            ((self.m_af_flexibility_10).inspect_value(path, offset, limit))
                .map(|v| ("m_afFlexibility_10".to_string(), v)),
            ((self.m_af_flexibility_11).inspect_value(path, offset, limit))
                .map(|v| ("m_afFlexibility_11".to_string(), v)),
            ((self.m_af_flexibility_12).inspect_value(path, offset, limit))
                .map(|v| ("m_afFlexibility_12".to_string(), v)),
            ((self.m_af_flexibility_13).inspect_value(path, offset, limit))
                .map(|v| ("m_afFlexibility_13".to_string(), v)),
            ((self.m_af_flexibility_14).inspect_value(path, offset, limit))
                .map(|v| ("m_afFlexibility_14".to_string(), v)),
            ((self.m_af_flexibility_15).inspect_value(path, offset, limit))
                .map(|v| ("m_afFlexibility_15".to_string(), v)),
            ((self.m_af_flexibility_16).inspect_value(path, offset, limit))
                .map(|v| ("m_afFlexibility_16".to_string(), v)),
            ((self.m_af_flexibility_17).inspect_value(path, offset, limit))
                .map(|v| ("m_afFlexibility_17".to_string(), v)),
            ((self.m_af_flexibility_18).inspect_value(path, offset, limit))
                .map(|v| ("m_afFlexibility_18".to_string(), v)),
            ((self.m_af_flexibility_19).inspect_value(path, offset, limit))
                .map(|v| ("m_afFlexibility_19".to_string(), v)),
            ((self.m_f_independence).inspect_value(path, offset, limit))
                .map(|v| ("m_fIndependence".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
