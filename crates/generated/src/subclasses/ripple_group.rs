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
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_af_planar_0 = rdr.read_f32()?;
        let m_af_planar_1 = rdr.read_f32()?;
        let m_af_planar_2 = rdr.read_f32()?;
        let m_af_planar_3 = rdr.read_f32()?;
        let m_af_planar_4 = rdr.read_f32()?;
        let m_af_planar_5 = rdr.read_f32()?;
        let m_af_planar_6 = rdr.read_f32()?;
        let m_af_planar_7 = rdr.read_f32()?;
        let m_af_planar_8 = rdr.read_f32()?;
        let m_af_planar_9 = rdr.read_f32()?;
        let m_af_planar_10 = rdr.read_f32()?;
        let m_af_planar_11 = rdr.read_f32()?;
        let m_af_planar_12 = rdr.read_f32()?;
        let m_af_planar_13 = rdr.read_f32()?;
        let m_af_planar_14 = rdr.read_f32()?;
        let m_af_planar_15 = rdr.read_f32()?;
        let m_af_planar_16 = rdr.read_f32()?;
        let m_af_planar_17 = rdr.read_f32()?;
        let m_af_planar_18 = rdr.read_f32()?;
        let m_af_planar_19 = rdr.read_f32()?;
        let m_af_directional_0 = rdr.read_f32()?;
        let m_af_directional_1 = rdr.read_f32()?;
        let m_af_directional_2 = rdr.read_f32()?;
        let m_af_directional_3 = rdr.read_f32()?;
        let m_af_directional_4 = rdr.read_f32()?;
        let m_af_directional_5 = rdr.read_f32()?;
        let m_af_directional_6 = rdr.read_f32()?;
        let m_af_directional_7 = rdr.read_f32()?;
        let m_af_directional_8 = rdr.read_f32()?;
        let m_af_directional_9 = rdr.read_f32()?;
        let m_af_directional_10 = rdr.read_f32()?;
        let m_af_directional_11 = rdr.read_f32()?;
        let m_af_directional_12 = rdr.read_f32()?;
        let m_af_directional_13 = rdr.read_f32()?;
        let m_af_directional_14 = rdr.read_f32()?;
        let m_af_directional_15 = rdr.read_f32()?;
        let m_af_directional_16 = rdr.read_f32()?;
        let m_af_directional_17 = rdr.read_f32()?;
        let m_af_directional_18 = rdr.read_f32()?;
        let m_af_directional_19 = rdr.read_f32()?;
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
        let m_f_shimmer = rdr.read_f32()?;

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

impl crate::ToValue for RippleGroup {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_afPlanar_0".to_string(), self.m_af_planar_0.to_value())),
            Some(("m_afPlanar_1".to_string(), self.m_af_planar_1.to_value())),
            Some(("m_afPlanar_2".to_string(), self.m_af_planar_2.to_value())),
            Some(("m_afPlanar_3".to_string(), self.m_af_planar_3.to_value())),
            Some(("m_afPlanar_4".to_string(), self.m_af_planar_4.to_value())),
            Some(("m_afPlanar_5".to_string(), self.m_af_planar_5.to_value())),
            Some(("m_afPlanar_6".to_string(), self.m_af_planar_6.to_value())),
            Some(("m_afPlanar_7".to_string(), self.m_af_planar_7.to_value())),
            Some(("m_afPlanar_8".to_string(), self.m_af_planar_8.to_value())),
            Some(("m_afPlanar_9".to_string(), self.m_af_planar_9.to_value())),
            Some(("m_afPlanar_10".to_string(), self.m_af_planar_10.to_value())),
            Some(("m_afPlanar_11".to_string(), self.m_af_planar_11.to_value())),
            Some(("m_afPlanar_12".to_string(), self.m_af_planar_12.to_value())),
            Some(("m_afPlanar_13".to_string(), self.m_af_planar_13.to_value())),
            Some(("m_afPlanar_14".to_string(), self.m_af_planar_14.to_value())),
            Some(("m_afPlanar_15".to_string(), self.m_af_planar_15.to_value())),
            Some(("m_afPlanar_16".to_string(), self.m_af_planar_16.to_value())),
            Some(("m_afPlanar_17".to_string(), self.m_af_planar_17.to_value())),
            Some(("m_afPlanar_18".to_string(), self.m_af_planar_18.to_value())),
            Some(("m_afPlanar_19".to_string(), self.m_af_planar_19.to_value())),
            Some((
                "m_afDirectional_0".to_string(),
                self.m_af_directional_0.to_value(),
            )),
            Some((
                "m_afDirectional_1".to_string(),
                self.m_af_directional_1.to_value(),
            )),
            Some((
                "m_afDirectional_2".to_string(),
                self.m_af_directional_2.to_value(),
            )),
            Some((
                "m_afDirectional_3".to_string(),
                self.m_af_directional_3.to_value(),
            )),
            Some((
                "m_afDirectional_4".to_string(),
                self.m_af_directional_4.to_value(),
            )),
            Some((
                "m_afDirectional_5".to_string(),
                self.m_af_directional_5.to_value(),
            )),
            Some((
                "m_afDirectional_6".to_string(),
                self.m_af_directional_6.to_value(),
            )),
            Some((
                "m_afDirectional_7".to_string(),
                self.m_af_directional_7.to_value(),
            )),
            Some((
                "m_afDirectional_8".to_string(),
                self.m_af_directional_8.to_value(),
            )),
            Some((
                "m_afDirectional_9".to_string(),
                self.m_af_directional_9.to_value(),
            )),
            Some((
                "m_afDirectional_10".to_string(),
                self.m_af_directional_10.to_value(),
            )),
            Some((
                "m_afDirectional_11".to_string(),
                self.m_af_directional_11.to_value(),
            )),
            Some((
                "m_afDirectional_12".to_string(),
                self.m_af_directional_12.to_value(),
            )),
            Some((
                "m_afDirectional_13".to_string(),
                self.m_af_directional_13.to_value(),
            )),
            Some((
                "m_afDirectional_14".to_string(),
                self.m_af_directional_14.to_value(),
            )),
            Some((
                "m_afDirectional_15".to_string(),
                self.m_af_directional_15.to_value(),
            )),
            Some((
                "m_afDirectional_16".to_string(),
                self.m_af_directional_16.to_value(),
            )),
            Some((
                "m_afDirectional_17".to_string(),
                self.m_af_directional_17.to_value(),
            )),
            Some((
                "m_afDirectional_18".to_string(),
                self.m_af_directional_18.to_value(),
            )),
            Some((
                "m_afDirectional_19".to_string(),
                self.m_af_directional_19.to_value(),
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
            Some(("m_fShimmer".to_string(), self.m_f_shimmer.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_afPlanar_0" => (self.m_af_planar_0).inspect_value(path, offset, limit),
                "m_afPlanar_1" => (self.m_af_planar_1).inspect_value(path, offset, limit),
                "m_afPlanar_2" => (self.m_af_planar_2).inspect_value(path, offset, limit),
                "m_afPlanar_3" => (self.m_af_planar_3).inspect_value(path, offset, limit),
                "m_afPlanar_4" => (self.m_af_planar_4).inspect_value(path, offset, limit),
                "m_afPlanar_5" => (self.m_af_planar_5).inspect_value(path, offset, limit),
                "m_afPlanar_6" => (self.m_af_planar_6).inspect_value(path, offset, limit),
                "m_afPlanar_7" => (self.m_af_planar_7).inspect_value(path, offset, limit),
                "m_afPlanar_8" => (self.m_af_planar_8).inspect_value(path, offset, limit),
                "m_afPlanar_9" => (self.m_af_planar_9).inspect_value(path, offset, limit),
                "m_afPlanar_10" => (self.m_af_planar_10).inspect_value(path, offset, limit),
                "m_afPlanar_11" => (self.m_af_planar_11).inspect_value(path, offset, limit),
                "m_afPlanar_12" => (self.m_af_planar_12).inspect_value(path, offset, limit),
                "m_afPlanar_13" => (self.m_af_planar_13).inspect_value(path, offset, limit),
                "m_afPlanar_14" => (self.m_af_planar_14).inspect_value(path, offset, limit),
                "m_afPlanar_15" => (self.m_af_planar_15).inspect_value(path, offset, limit),
                "m_afPlanar_16" => (self.m_af_planar_16).inspect_value(path, offset, limit),
                "m_afPlanar_17" => (self.m_af_planar_17).inspect_value(path, offset, limit),
                "m_afPlanar_18" => (self.m_af_planar_18).inspect_value(path, offset, limit),
                "m_afPlanar_19" => (self.m_af_planar_19).inspect_value(path, offset, limit),
                "m_afDirectional_0" => (self.m_af_directional_0).inspect_value(path, offset, limit),
                "m_afDirectional_1" => (self.m_af_directional_1).inspect_value(path, offset, limit),
                "m_afDirectional_2" => (self.m_af_directional_2).inspect_value(path, offset, limit),
                "m_afDirectional_3" => (self.m_af_directional_3).inspect_value(path, offset, limit),
                "m_afDirectional_4" => (self.m_af_directional_4).inspect_value(path, offset, limit),
                "m_afDirectional_5" => (self.m_af_directional_5).inspect_value(path, offset, limit),
                "m_afDirectional_6" => (self.m_af_directional_6).inspect_value(path, offset, limit),
                "m_afDirectional_7" => (self.m_af_directional_7).inspect_value(path, offset, limit),
                "m_afDirectional_8" => (self.m_af_directional_8).inspect_value(path, offset, limit),
                "m_afDirectional_9" => (self.m_af_directional_9).inspect_value(path, offset, limit),
                "m_afDirectional_10" => {
                    (self.m_af_directional_10).inspect_value(path, offset, limit)
                }
                "m_afDirectional_11" => {
                    (self.m_af_directional_11).inspect_value(path, offset, limit)
                }
                "m_afDirectional_12" => {
                    (self.m_af_directional_12).inspect_value(path, offset, limit)
                }
                "m_afDirectional_13" => {
                    (self.m_af_directional_13).inspect_value(path, offset, limit)
                }
                "m_afDirectional_14" => {
                    (self.m_af_directional_14).inspect_value(path, offset, limit)
                }
                "m_afDirectional_15" => {
                    (self.m_af_directional_15).inspect_value(path, offset, limit)
                }
                "m_afDirectional_16" => {
                    (self.m_af_directional_16).inspect_value(path, offset, limit)
                }
                "m_afDirectional_17" => {
                    (self.m_af_directional_17).inspect_value(path, offset, limit)
                }
                "m_afDirectional_18" => {
                    (self.m_af_directional_18).inspect_value(path, offset, limit)
                }
                "m_afDirectional_19" => {
                    (self.m_af_directional_19).inspect_value(path, offset, limit)
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
                "m_fShimmer" => (self.m_f_shimmer).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_af_planar_0).inspect_value(path, offset, limit))
                .map(|v| ("m_afPlanar_0".to_string(), v)),
            ((self.m_af_planar_1).inspect_value(path, offset, limit))
                .map(|v| ("m_afPlanar_1".to_string(), v)),
            ((self.m_af_planar_2).inspect_value(path, offset, limit))
                .map(|v| ("m_afPlanar_2".to_string(), v)),
            ((self.m_af_planar_3).inspect_value(path, offset, limit))
                .map(|v| ("m_afPlanar_3".to_string(), v)),
            ((self.m_af_planar_4).inspect_value(path, offset, limit))
                .map(|v| ("m_afPlanar_4".to_string(), v)),
            ((self.m_af_planar_5).inspect_value(path, offset, limit))
                .map(|v| ("m_afPlanar_5".to_string(), v)),
            ((self.m_af_planar_6).inspect_value(path, offset, limit))
                .map(|v| ("m_afPlanar_6".to_string(), v)),
            ((self.m_af_planar_7).inspect_value(path, offset, limit))
                .map(|v| ("m_afPlanar_7".to_string(), v)),
            ((self.m_af_planar_8).inspect_value(path, offset, limit))
                .map(|v| ("m_afPlanar_8".to_string(), v)),
            ((self.m_af_planar_9).inspect_value(path, offset, limit))
                .map(|v| ("m_afPlanar_9".to_string(), v)),
            ((self.m_af_planar_10).inspect_value(path, offset, limit))
                .map(|v| ("m_afPlanar_10".to_string(), v)),
            ((self.m_af_planar_11).inspect_value(path, offset, limit))
                .map(|v| ("m_afPlanar_11".to_string(), v)),
            ((self.m_af_planar_12).inspect_value(path, offset, limit))
                .map(|v| ("m_afPlanar_12".to_string(), v)),
            ((self.m_af_planar_13).inspect_value(path, offset, limit))
                .map(|v| ("m_afPlanar_13".to_string(), v)),
            ((self.m_af_planar_14).inspect_value(path, offset, limit))
                .map(|v| ("m_afPlanar_14".to_string(), v)),
            ((self.m_af_planar_15).inspect_value(path, offset, limit))
                .map(|v| ("m_afPlanar_15".to_string(), v)),
            ((self.m_af_planar_16).inspect_value(path, offset, limit))
                .map(|v| ("m_afPlanar_16".to_string(), v)),
            ((self.m_af_planar_17).inspect_value(path, offset, limit))
                .map(|v| ("m_afPlanar_17".to_string(), v)),
            ((self.m_af_planar_18).inspect_value(path, offset, limit))
                .map(|v| ("m_afPlanar_18".to_string(), v)),
            ((self.m_af_planar_19).inspect_value(path, offset, limit))
                .map(|v| ("m_afPlanar_19".to_string(), v)),
            ((self.m_af_directional_0).inspect_value(path, offset, limit))
                .map(|v| ("m_afDirectional_0".to_string(), v)),
            ((self.m_af_directional_1).inspect_value(path, offset, limit))
                .map(|v| ("m_afDirectional_1".to_string(), v)),
            ((self.m_af_directional_2).inspect_value(path, offset, limit))
                .map(|v| ("m_afDirectional_2".to_string(), v)),
            ((self.m_af_directional_3).inspect_value(path, offset, limit))
                .map(|v| ("m_afDirectional_3".to_string(), v)),
            ((self.m_af_directional_4).inspect_value(path, offset, limit))
                .map(|v| ("m_afDirectional_4".to_string(), v)),
            ((self.m_af_directional_5).inspect_value(path, offset, limit))
                .map(|v| ("m_afDirectional_5".to_string(), v)),
            ((self.m_af_directional_6).inspect_value(path, offset, limit))
                .map(|v| ("m_afDirectional_6".to_string(), v)),
            ((self.m_af_directional_7).inspect_value(path, offset, limit))
                .map(|v| ("m_afDirectional_7".to_string(), v)),
            ((self.m_af_directional_8).inspect_value(path, offset, limit))
                .map(|v| ("m_afDirectional_8".to_string(), v)),
            ((self.m_af_directional_9).inspect_value(path, offset, limit))
                .map(|v| ("m_afDirectional_9".to_string(), v)),
            ((self.m_af_directional_10).inspect_value(path, offset, limit))
                .map(|v| ("m_afDirectional_10".to_string(), v)),
            ((self.m_af_directional_11).inspect_value(path, offset, limit))
                .map(|v| ("m_afDirectional_11".to_string(), v)),
            ((self.m_af_directional_12).inspect_value(path, offset, limit))
                .map(|v| ("m_afDirectional_12".to_string(), v)),
            ((self.m_af_directional_13).inspect_value(path, offset, limit))
                .map(|v| ("m_afDirectional_13".to_string(), v)),
            ((self.m_af_directional_14).inspect_value(path, offset, limit))
                .map(|v| ("m_afDirectional_14".to_string(), v)),
            ((self.m_af_directional_15).inspect_value(path, offset, limit))
                .map(|v| ("m_afDirectional_15".to_string(), v)),
            ((self.m_af_directional_16).inspect_value(path, offset, limit))
                .map(|v| ("m_afDirectional_16".to_string(), v)),
            ((self.m_af_directional_17).inspect_value(path, offset, limit))
                .map(|v| ("m_afDirectional_17".to_string(), v)),
            ((self.m_af_directional_18).inspect_value(path, offset, limit))
                .map(|v| ("m_afDirectional_18".to_string(), v)),
            ((self.m_af_directional_19).inspect_value(path, offset, limit))
                .map(|v| ("m_afDirectional_19".to_string(), v)),
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
            ((self.m_f_shimmer).inspect_value(path, offset, limit))
                .map(|v| ("m_fShimmer".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
