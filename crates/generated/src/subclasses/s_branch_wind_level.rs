//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct SBranchWindLevel {
    pub m_af_distance_0: f32,
    pub m_af_distance_1: f32,
    pub m_af_distance_2: f32,
    pub m_af_distance_3: f32,
    pub m_af_distance_4: f32,
    pub m_af_distance_5: f32,
    pub m_af_distance_6: f32,
    pub m_af_distance_7: f32,
    pub m_af_distance_8: f32,
    pub m_af_distance_9: f32,
    pub m_af_direction_adherence_0: f32,
    pub m_af_direction_adherence_1: f32,
    pub m_af_direction_adherence_2: f32,
    pub m_af_direction_adherence_3: f32,
    pub m_af_direction_adherence_4: f32,
    pub m_af_direction_adherence_5: f32,
    pub m_af_direction_adherence_6: f32,
    pub m_af_direction_adherence_7: f32,
    pub m_af_direction_adherence_8: f32,
    pub m_af_direction_adherence_9: f32,
    pub m_af_whip_0: f32,
    pub m_af_whip_1: f32,
    pub m_af_whip_2: f32,
    pub m_af_whip_3: f32,
    pub m_af_whip_4: f32,
    pub m_af_whip_5: f32,
    pub m_af_whip_6: f32,
    pub m_af_whip_7: f32,
    pub m_af_whip_8: f32,
    pub m_af_whip_9: f32,
    pub m_f_turbulence: f32,
    pub m_f_twitch: f32,
    pub m_f_twitch_freq_scale: f32,
}

impl crate::Parse for SBranchWindLevel {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_af_distance_0 = rdr.read_f32()?;
        let m_af_distance_1 = rdr.read_f32()?;
        let m_af_distance_2 = rdr.read_f32()?;
        let m_af_distance_3 = rdr.read_f32()?;
        let m_af_distance_4 = rdr.read_f32()?;
        let m_af_distance_5 = rdr.read_f32()?;
        let m_af_distance_6 = rdr.read_f32()?;
        let m_af_distance_7 = rdr.read_f32()?;
        let m_af_distance_8 = rdr.read_f32()?;
        let m_af_distance_9 = rdr.read_f32()?;
        let m_af_direction_adherence_0 = rdr.read_f32()?;
        let m_af_direction_adherence_1 = rdr.read_f32()?;
        let m_af_direction_adherence_2 = rdr.read_f32()?;
        let m_af_direction_adherence_3 = rdr.read_f32()?;
        let m_af_direction_adherence_4 = rdr.read_f32()?;
        let m_af_direction_adherence_5 = rdr.read_f32()?;
        let m_af_direction_adherence_6 = rdr.read_f32()?;
        let m_af_direction_adherence_7 = rdr.read_f32()?;
        let m_af_direction_adherence_8 = rdr.read_f32()?;
        let m_af_direction_adherence_9 = rdr.read_f32()?;
        let m_af_whip_0 = rdr.read_f32()?;
        let m_af_whip_1 = rdr.read_f32()?;
        let m_af_whip_2 = rdr.read_f32()?;
        let m_af_whip_3 = rdr.read_f32()?;
        let m_af_whip_4 = rdr.read_f32()?;
        let m_af_whip_5 = rdr.read_f32()?;
        let m_af_whip_6 = rdr.read_f32()?;
        let m_af_whip_7 = rdr.read_f32()?;
        let m_af_whip_8 = rdr.read_f32()?;
        let m_af_whip_9 = rdr.read_f32()?;
        let m_f_turbulence = rdr.read_f32()?;
        let m_f_twitch = rdr.read_f32()?;
        let m_f_twitch_freq_scale = rdr.read_f32()?;

        Ok(SBranchWindLevel {
            m_af_distance_0,
            m_af_distance_1,
            m_af_distance_2,
            m_af_distance_3,
            m_af_distance_4,
            m_af_distance_5,
            m_af_distance_6,
            m_af_distance_7,
            m_af_distance_8,
            m_af_distance_9,
            m_af_direction_adherence_0,
            m_af_direction_adherence_1,
            m_af_direction_adherence_2,
            m_af_direction_adherence_3,
            m_af_direction_adherence_4,
            m_af_direction_adherence_5,
            m_af_direction_adherence_6,
            m_af_direction_adherence_7,
            m_af_direction_adherence_8,
            m_af_direction_adherence_9,
            m_af_whip_0,
            m_af_whip_1,
            m_af_whip_2,
            m_af_whip_3,
            m_af_whip_4,
            m_af_whip_5,
            m_af_whip_6,
            m_af_whip_7,
            m_af_whip_8,
            m_af_whip_9,
            m_f_turbulence,
            m_f_twitch,
            m_f_twitch_freq_scale,
        })
    }
}

impl crate::ToValue for SBranchWindLevel {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "m_afDistance_0".to_string(),
                self.m_af_distance_0.to_value(),
            )),
            Some((
                "m_afDistance_1".to_string(),
                self.m_af_distance_1.to_value(),
            )),
            Some((
                "m_afDistance_2".to_string(),
                self.m_af_distance_2.to_value(),
            )),
            Some((
                "m_afDistance_3".to_string(),
                self.m_af_distance_3.to_value(),
            )),
            Some((
                "m_afDistance_4".to_string(),
                self.m_af_distance_4.to_value(),
            )),
            Some((
                "m_afDistance_5".to_string(),
                self.m_af_distance_5.to_value(),
            )),
            Some((
                "m_afDistance_6".to_string(),
                self.m_af_distance_6.to_value(),
            )),
            Some((
                "m_afDistance_7".to_string(),
                self.m_af_distance_7.to_value(),
            )),
            Some((
                "m_afDistance_8".to_string(),
                self.m_af_distance_8.to_value(),
            )),
            Some((
                "m_afDistance_9".to_string(),
                self.m_af_distance_9.to_value(),
            )),
            Some((
                "m_afDirectionAdherence_0".to_string(),
                self.m_af_direction_adherence_0.to_value(),
            )),
            Some((
                "m_afDirectionAdherence_1".to_string(),
                self.m_af_direction_adherence_1.to_value(),
            )),
            Some((
                "m_afDirectionAdherence_2".to_string(),
                self.m_af_direction_adherence_2.to_value(),
            )),
            Some((
                "m_afDirectionAdherence_3".to_string(),
                self.m_af_direction_adherence_3.to_value(),
            )),
            Some((
                "m_afDirectionAdherence_4".to_string(),
                self.m_af_direction_adherence_4.to_value(),
            )),
            Some((
                "m_afDirectionAdherence_5".to_string(),
                self.m_af_direction_adherence_5.to_value(),
            )),
            Some((
                "m_afDirectionAdherence_6".to_string(),
                self.m_af_direction_adherence_6.to_value(),
            )),
            Some((
                "m_afDirectionAdherence_7".to_string(),
                self.m_af_direction_adherence_7.to_value(),
            )),
            Some((
                "m_afDirectionAdherence_8".to_string(),
                self.m_af_direction_adherence_8.to_value(),
            )),
            Some((
                "m_afDirectionAdherence_9".to_string(),
                self.m_af_direction_adherence_9.to_value(),
            )),
            Some(("m_afWhip_0".to_string(), self.m_af_whip_0.to_value())),
            Some(("m_afWhip_1".to_string(), self.m_af_whip_1.to_value())),
            Some(("m_afWhip_2".to_string(), self.m_af_whip_2.to_value())),
            Some(("m_afWhip_3".to_string(), self.m_af_whip_3.to_value())),
            Some(("m_afWhip_4".to_string(), self.m_af_whip_4.to_value())),
            Some(("m_afWhip_5".to_string(), self.m_af_whip_5.to_value())),
            Some(("m_afWhip_6".to_string(), self.m_af_whip_6.to_value())),
            Some(("m_afWhip_7".to_string(), self.m_af_whip_7.to_value())),
            Some(("m_afWhip_8".to_string(), self.m_af_whip_8.to_value())),
            Some(("m_afWhip_9".to_string(), self.m_af_whip_9.to_value())),
            Some(("m_fTurbulence".to_string(), self.m_f_turbulence.to_value())),
            Some(("m_fTwitch".to_string(), self.m_f_twitch.to_value())),
            Some((
                "m_fTwitchFreqScale".to_string(),
                self.m_f_twitch_freq_scale.to_value(),
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
                "m_afDistance_0" => (self.m_af_distance_0).inspect_value(path, offset, limit),
                "m_afDistance_1" => (self.m_af_distance_1).inspect_value(path, offset, limit),
                "m_afDistance_2" => (self.m_af_distance_2).inspect_value(path, offset, limit),
                "m_afDistance_3" => (self.m_af_distance_3).inspect_value(path, offset, limit),
                "m_afDistance_4" => (self.m_af_distance_4).inspect_value(path, offset, limit),
                "m_afDistance_5" => (self.m_af_distance_5).inspect_value(path, offset, limit),
                "m_afDistance_6" => (self.m_af_distance_6).inspect_value(path, offset, limit),
                "m_afDistance_7" => (self.m_af_distance_7).inspect_value(path, offset, limit),
                "m_afDistance_8" => (self.m_af_distance_8).inspect_value(path, offset, limit),
                "m_afDistance_9" => (self.m_af_distance_9).inspect_value(path, offset, limit),
                "m_afDirectionAdherence_0" => {
                    (self.m_af_direction_adherence_0).inspect_value(path, offset, limit)
                }
                "m_afDirectionAdherence_1" => {
                    (self.m_af_direction_adherence_1).inspect_value(path, offset, limit)
                }
                "m_afDirectionAdherence_2" => {
                    (self.m_af_direction_adherence_2).inspect_value(path, offset, limit)
                }
                "m_afDirectionAdherence_3" => {
                    (self.m_af_direction_adherence_3).inspect_value(path, offset, limit)
                }
                "m_afDirectionAdherence_4" => {
                    (self.m_af_direction_adherence_4).inspect_value(path, offset, limit)
                }
                "m_afDirectionAdherence_5" => {
                    (self.m_af_direction_adherence_5).inspect_value(path, offset, limit)
                }
                "m_afDirectionAdherence_6" => {
                    (self.m_af_direction_adherence_6).inspect_value(path, offset, limit)
                }
                "m_afDirectionAdherence_7" => {
                    (self.m_af_direction_adherence_7).inspect_value(path, offset, limit)
                }
                "m_afDirectionAdherence_8" => {
                    (self.m_af_direction_adherence_8).inspect_value(path, offset, limit)
                }
                "m_afDirectionAdherence_9" => {
                    (self.m_af_direction_adherence_9).inspect_value(path, offset, limit)
                }
                "m_afWhip_0" => (self.m_af_whip_0).inspect_value(path, offset, limit),
                "m_afWhip_1" => (self.m_af_whip_1).inspect_value(path, offset, limit),
                "m_afWhip_2" => (self.m_af_whip_2).inspect_value(path, offset, limit),
                "m_afWhip_3" => (self.m_af_whip_3).inspect_value(path, offset, limit),
                "m_afWhip_4" => (self.m_af_whip_4).inspect_value(path, offset, limit),
                "m_afWhip_5" => (self.m_af_whip_5).inspect_value(path, offset, limit),
                "m_afWhip_6" => (self.m_af_whip_6).inspect_value(path, offset, limit),
                "m_afWhip_7" => (self.m_af_whip_7).inspect_value(path, offset, limit),
                "m_afWhip_8" => (self.m_af_whip_8).inspect_value(path, offset, limit),
                "m_afWhip_9" => (self.m_af_whip_9).inspect_value(path, offset, limit),
                "m_fTurbulence" => (self.m_f_turbulence).inspect_value(path, offset, limit),
                "m_fTwitch" => (self.m_f_twitch).inspect_value(path, offset, limit),
                "m_fTwitchFreqScale" => {
                    (self.m_f_twitch_freq_scale).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_af_distance_0).inspect_value(path, offset, limit))
                .map(|v| ("m_afDistance_0".to_string(), v)),
            ((self.m_af_distance_1).inspect_value(path, offset, limit))
                .map(|v| ("m_afDistance_1".to_string(), v)),
            ((self.m_af_distance_2).inspect_value(path, offset, limit))
                .map(|v| ("m_afDistance_2".to_string(), v)),
            ((self.m_af_distance_3).inspect_value(path, offset, limit))
                .map(|v| ("m_afDistance_3".to_string(), v)),
            ((self.m_af_distance_4).inspect_value(path, offset, limit))
                .map(|v| ("m_afDistance_4".to_string(), v)),
            ((self.m_af_distance_5).inspect_value(path, offset, limit))
                .map(|v| ("m_afDistance_5".to_string(), v)),
            ((self.m_af_distance_6).inspect_value(path, offset, limit))
                .map(|v| ("m_afDistance_6".to_string(), v)),
            ((self.m_af_distance_7).inspect_value(path, offset, limit))
                .map(|v| ("m_afDistance_7".to_string(), v)),
            ((self.m_af_distance_8).inspect_value(path, offset, limit))
                .map(|v| ("m_afDistance_8".to_string(), v)),
            ((self.m_af_distance_9).inspect_value(path, offset, limit))
                .map(|v| ("m_afDistance_9".to_string(), v)),
            ((self.m_af_direction_adherence_0).inspect_value(path, offset, limit))
                .map(|v| ("m_afDirectionAdherence_0".to_string(), v)),
            ((self.m_af_direction_adherence_1).inspect_value(path, offset, limit))
                .map(|v| ("m_afDirectionAdherence_1".to_string(), v)),
            ((self.m_af_direction_adherence_2).inspect_value(path, offset, limit))
                .map(|v| ("m_afDirectionAdherence_2".to_string(), v)),
            ((self.m_af_direction_adherence_3).inspect_value(path, offset, limit))
                .map(|v| ("m_afDirectionAdherence_3".to_string(), v)),
            ((self.m_af_direction_adherence_4).inspect_value(path, offset, limit))
                .map(|v| ("m_afDirectionAdherence_4".to_string(), v)),
            ((self.m_af_direction_adherence_5).inspect_value(path, offset, limit))
                .map(|v| ("m_afDirectionAdherence_5".to_string(), v)),
            ((self.m_af_direction_adherence_6).inspect_value(path, offset, limit))
                .map(|v| ("m_afDirectionAdherence_6".to_string(), v)),
            ((self.m_af_direction_adherence_7).inspect_value(path, offset, limit))
                .map(|v| ("m_afDirectionAdherence_7".to_string(), v)),
            ((self.m_af_direction_adherence_8).inspect_value(path, offset, limit))
                .map(|v| ("m_afDirectionAdherence_8".to_string(), v)),
            ((self.m_af_direction_adherence_9).inspect_value(path, offset, limit))
                .map(|v| ("m_afDirectionAdherence_9".to_string(), v)),
            ((self.m_af_whip_0).inspect_value(path, offset, limit))
                .map(|v| ("m_afWhip_0".to_string(), v)),
            ((self.m_af_whip_1).inspect_value(path, offset, limit))
                .map(|v| ("m_afWhip_1".to_string(), v)),
            ((self.m_af_whip_2).inspect_value(path, offset, limit))
                .map(|v| ("m_afWhip_2".to_string(), v)),
            ((self.m_af_whip_3).inspect_value(path, offset, limit))
                .map(|v| ("m_afWhip_3".to_string(), v)),
            ((self.m_af_whip_4).inspect_value(path, offset, limit))
                .map(|v| ("m_afWhip_4".to_string(), v)),
            ((self.m_af_whip_5).inspect_value(path, offset, limit))
                .map(|v| ("m_afWhip_5".to_string(), v)),
            ((self.m_af_whip_6).inspect_value(path, offset, limit))
                .map(|v| ("m_afWhip_6".to_string(), v)),
            ((self.m_af_whip_7).inspect_value(path, offset, limit))
                .map(|v| ("m_afWhip_7".to_string(), v)),
            ((self.m_af_whip_8).inspect_value(path, offset, limit))
                .map(|v| ("m_afWhip_8".to_string(), v)),
            ((self.m_af_whip_9).inspect_value(path, offset, limit))
                .map(|v| ("m_afWhip_9".to_string(), v)),
            ((self.m_f_turbulence).inspect_value(path, offset, limit))
                .map(|v| ("m_fTurbulence".to_string(), v)),
            ((self.m_f_twitch).inspect_value(path, offset, limit))
                .map(|v| ("m_fTwitch".to_string(), v)),
            ((self.m_f_twitch_freq_scale).inspect_value(path, offset, limit))
                .map(|v| ("m_fTwitchFreqScale".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
