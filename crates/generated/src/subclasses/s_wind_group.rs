//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct SWindGroup {
    pub m_af_ripple_distance_0: f32,
    pub m_af_ripple_distance_1: f32,
    pub m_af_ripple_distance_2: f32,
    pub m_af_ripple_distance_3: f32,
    pub m_af_ripple_distance_4: f32,
    pub m_af_ripple_distance_5: f32,
    pub m_af_ripple_distance_6: f32,
    pub m_af_ripple_distance_7: f32,
    pub m_af_ripple_distance_8: f32,
    pub m_af_ripple_distance_9: f32,
    pub m_af_tumble_flip_0: f32,
    pub m_af_tumble_flip_1: f32,
    pub m_af_tumble_flip_2: f32,
    pub m_af_tumble_flip_3: f32,
    pub m_af_tumble_flip_4: f32,
    pub m_af_tumble_flip_5: f32,
    pub m_af_tumble_flip_6: f32,
    pub m_af_tumble_flip_7: f32,
    pub m_af_tumble_flip_8: f32,
    pub m_af_tumble_flip_9: f32,
    pub m_af_tumble_twist_0: f32,
    pub m_af_tumble_twist_1: f32,
    pub m_af_tumble_twist_2: f32,
    pub m_af_tumble_twist_3: f32,
    pub m_af_tumble_twist_4: f32,
    pub m_af_tumble_twist_5: f32,
    pub m_af_tumble_twist_6: f32,
    pub m_af_tumble_twist_7: f32,
    pub m_af_tumble_twist_8: f32,
    pub m_af_tumble_twist_9: f32,
    pub m_af_tumble_direction_adherence_0: f32,
    pub m_af_tumble_direction_adherence_1: f32,
    pub m_af_tumble_direction_adherence_2: f32,
    pub m_af_tumble_direction_adherence_3: f32,
    pub m_af_tumble_direction_adherence_4: f32,
    pub m_af_tumble_direction_adherence_5: f32,
    pub m_af_tumble_direction_adherence_6: f32,
    pub m_af_tumble_direction_adherence_7: f32,
    pub m_af_tumble_direction_adherence_8: f32,
    pub m_af_tumble_direction_adherence_9: f32,
    pub m_af_twitch_throw_0: f32,
    pub m_af_twitch_throw_1: f32,
    pub m_af_twitch_throw_2: f32,
    pub m_af_twitch_throw_3: f32,
    pub m_af_twitch_throw_4: f32,
    pub m_af_twitch_throw_5: f32,
    pub m_af_twitch_throw_6: f32,
    pub m_af_twitch_throw_7: f32,
    pub m_af_twitch_throw_8: f32,
    pub m_af_twitch_throw_9: f32,
    pub m_f_twitch_sharpness: f32,
    pub m_f_roll_max_scale: f32,
    pub m_f_roll_min_scale: f32,
    pub m_f_roll_speed: f32,
    pub m_f_roll_separation: f32,
    pub m_f_leeward_scalar: f32,
}

impl crate::Parse for SWindGroup {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_af_ripple_distance_0 = rdr.read_f32()?;
        let m_af_ripple_distance_1 = rdr.read_f32()?;
        let m_af_ripple_distance_2 = rdr.read_f32()?;
        let m_af_ripple_distance_3 = rdr.read_f32()?;
        let m_af_ripple_distance_4 = rdr.read_f32()?;
        let m_af_ripple_distance_5 = rdr.read_f32()?;
        let m_af_ripple_distance_6 = rdr.read_f32()?;
        let m_af_ripple_distance_7 = rdr.read_f32()?;
        let m_af_ripple_distance_8 = rdr.read_f32()?;
        let m_af_ripple_distance_9 = rdr.read_f32()?;
        let m_af_tumble_flip_0 = rdr.read_f32()?;
        let m_af_tumble_flip_1 = rdr.read_f32()?;
        let m_af_tumble_flip_2 = rdr.read_f32()?;
        let m_af_tumble_flip_3 = rdr.read_f32()?;
        let m_af_tumble_flip_4 = rdr.read_f32()?;
        let m_af_tumble_flip_5 = rdr.read_f32()?;
        let m_af_tumble_flip_6 = rdr.read_f32()?;
        let m_af_tumble_flip_7 = rdr.read_f32()?;
        let m_af_tumble_flip_8 = rdr.read_f32()?;
        let m_af_tumble_flip_9 = rdr.read_f32()?;
        let m_af_tumble_twist_0 = rdr.read_f32()?;
        let m_af_tumble_twist_1 = rdr.read_f32()?;
        let m_af_tumble_twist_2 = rdr.read_f32()?;
        let m_af_tumble_twist_3 = rdr.read_f32()?;
        let m_af_tumble_twist_4 = rdr.read_f32()?;
        let m_af_tumble_twist_5 = rdr.read_f32()?;
        let m_af_tumble_twist_6 = rdr.read_f32()?;
        let m_af_tumble_twist_7 = rdr.read_f32()?;
        let m_af_tumble_twist_8 = rdr.read_f32()?;
        let m_af_tumble_twist_9 = rdr.read_f32()?;
        let m_af_tumble_direction_adherence_0 = rdr.read_f32()?;
        let m_af_tumble_direction_adherence_1 = rdr.read_f32()?;
        let m_af_tumble_direction_adherence_2 = rdr.read_f32()?;
        let m_af_tumble_direction_adherence_3 = rdr.read_f32()?;
        let m_af_tumble_direction_adherence_4 = rdr.read_f32()?;
        let m_af_tumble_direction_adherence_5 = rdr.read_f32()?;
        let m_af_tumble_direction_adherence_6 = rdr.read_f32()?;
        let m_af_tumble_direction_adherence_7 = rdr.read_f32()?;
        let m_af_tumble_direction_adherence_8 = rdr.read_f32()?;
        let m_af_tumble_direction_adherence_9 = rdr.read_f32()?;
        let m_af_twitch_throw_0 = rdr.read_f32()?;
        let m_af_twitch_throw_1 = rdr.read_f32()?;
        let m_af_twitch_throw_2 = rdr.read_f32()?;
        let m_af_twitch_throw_3 = rdr.read_f32()?;
        let m_af_twitch_throw_4 = rdr.read_f32()?;
        let m_af_twitch_throw_5 = rdr.read_f32()?;
        let m_af_twitch_throw_6 = rdr.read_f32()?;
        let m_af_twitch_throw_7 = rdr.read_f32()?;
        let m_af_twitch_throw_8 = rdr.read_f32()?;
        let m_af_twitch_throw_9 = rdr.read_f32()?;
        let m_f_twitch_sharpness = rdr.read_f32()?;
        let m_f_roll_max_scale = rdr.read_f32()?;
        let m_f_roll_min_scale = rdr.read_f32()?;
        let m_f_roll_speed = rdr.read_f32()?;
        let m_f_roll_separation = rdr.read_f32()?;
        let m_f_leeward_scalar = rdr.read_f32()?;

        Ok(SWindGroup {
            m_af_ripple_distance_0,
            m_af_ripple_distance_1,
            m_af_ripple_distance_2,
            m_af_ripple_distance_3,
            m_af_ripple_distance_4,
            m_af_ripple_distance_5,
            m_af_ripple_distance_6,
            m_af_ripple_distance_7,
            m_af_ripple_distance_8,
            m_af_ripple_distance_9,
            m_af_tumble_flip_0,
            m_af_tumble_flip_1,
            m_af_tumble_flip_2,
            m_af_tumble_flip_3,
            m_af_tumble_flip_4,
            m_af_tumble_flip_5,
            m_af_tumble_flip_6,
            m_af_tumble_flip_7,
            m_af_tumble_flip_8,
            m_af_tumble_flip_9,
            m_af_tumble_twist_0,
            m_af_tumble_twist_1,
            m_af_tumble_twist_2,
            m_af_tumble_twist_3,
            m_af_tumble_twist_4,
            m_af_tumble_twist_5,
            m_af_tumble_twist_6,
            m_af_tumble_twist_7,
            m_af_tumble_twist_8,
            m_af_tumble_twist_9,
            m_af_tumble_direction_adherence_0,
            m_af_tumble_direction_adherence_1,
            m_af_tumble_direction_adherence_2,
            m_af_tumble_direction_adherence_3,
            m_af_tumble_direction_adherence_4,
            m_af_tumble_direction_adherence_5,
            m_af_tumble_direction_adherence_6,
            m_af_tumble_direction_adherence_7,
            m_af_tumble_direction_adherence_8,
            m_af_tumble_direction_adherence_9,
            m_af_twitch_throw_0,
            m_af_twitch_throw_1,
            m_af_twitch_throw_2,
            m_af_twitch_throw_3,
            m_af_twitch_throw_4,
            m_af_twitch_throw_5,
            m_af_twitch_throw_6,
            m_af_twitch_throw_7,
            m_af_twitch_throw_8,
            m_af_twitch_throw_9,
            m_f_twitch_sharpness,
            m_f_roll_max_scale,
            m_f_roll_min_scale,
            m_f_roll_speed,
            m_f_roll_separation,
            m_f_leeward_scalar,
        })
    }
}

impl crate::ToValue for SWindGroup {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "m_afRippleDistance_0".to_string(),
                self.m_af_ripple_distance_0.to_value(),
            )),
            Some((
                "m_afRippleDistance_1".to_string(),
                self.m_af_ripple_distance_1.to_value(),
            )),
            Some((
                "m_afRippleDistance_2".to_string(),
                self.m_af_ripple_distance_2.to_value(),
            )),
            Some((
                "m_afRippleDistance_3".to_string(),
                self.m_af_ripple_distance_3.to_value(),
            )),
            Some((
                "m_afRippleDistance_4".to_string(),
                self.m_af_ripple_distance_4.to_value(),
            )),
            Some((
                "m_afRippleDistance_5".to_string(),
                self.m_af_ripple_distance_5.to_value(),
            )),
            Some((
                "m_afRippleDistance_6".to_string(),
                self.m_af_ripple_distance_6.to_value(),
            )),
            Some((
                "m_afRippleDistance_7".to_string(),
                self.m_af_ripple_distance_7.to_value(),
            )),
            Some((
                "m_afRippleDistance_8".to_string(),
                self.m_af_ripple_distance_8.to_value(),
            )),
            Some((
                "m_afRippleDistance_9".to_string(),
                self.m_af_ripple_distance_9.to_value(),
            )),
            Some((
                "m_afTumbleFlip_0".to_string(),
                self.m_af_tumble_flip_0.to_value(),
            )),
            Some((
                "m_afTumbleFlip_1".to_string(),
                self.m_af_tumble_flip_1.to_value(),
            )),
            Some((
                "m_afTumbleFlip_2".to_string(),
                self.m_af_tumble_flip_2.to_value(),
            )),
            Some((
                "m_afTumbleFlip_3".to_string(),
                self.m_af_tumble_flip_3.to_value(),
            )),
            Some((
                "m_afTumbleFlip_4".to_string(),
                self.m_af_tumble_flip_4.to_value(),
            )),
            Some((
                "m_afTumbleFlip_5".to_string(),
                self.m_af_tumble_flip_5.to_value(),
            )),
            Some((
                "m_afTumbleFlip_6".to_string(),
                self.m_af_tumble_flip_6.to_value(),
            )),
            Some((
                "m_afTumbleFlip_7".to_string(),
                self.m_af_tumble_flip_7.to_value(),
            )),
            Some((
                "m_afTumbleFlip_8".to_string(),
                self.m_af_tumble_flip_8.to_value(),
            )),
            Some((
                "m_afTumbleFlip_9".to_string(),
                self.m_af_tumble_flip_9.to_value(),
            )),
            Some((
                "m_afTumbleTwist_0".to_string(),
                self.m_af_tumble_twist_0.to_value(),
            )),
            Some((
                "m_afTumbleTwist_1".to_string(),
                self.m_af_tumble_twist_1.to_value(),
            )),
            Some((
                "m_afTumbleTwist_2".to_string(),
                self.m_af_tumble_twist_2.to_value(),
            )),
            Some((
                "m_afTumbleTwist_3".to_string(),
                self.m_af_tumble_twist_3.to_value(),
            )),
            Some((
                "m_afTumbleTwist_4".to_string(),
                self.m_af_tumble_twist_4.to_value(),
            )),
            Some((
                "m_afTumbleTwist_5".to_string(),
                self.m_af_tumble_twist_5.to_value(),
            )),
            Some((
                "m_afTumbleTwist_6".to_string(),
                self.m_af_tumble_twist_6.to_value(),
            )),
            Some((
                "m_afTumbleTwist_7".to_string(),
                self.m_af_tumble_twist_7.to_value(),
            )),
            Some((
                "m_afTumbleTwist_8".to_string(),
                self.m_af_tumble_twist_8.to_value(),
            )),
            Some((
                "m_afTumbleTwist_9".to_string(),
                self.m_af_tumble_twist_9.to_value(),
            )),
            Some((
                "m_afTumbleDirectionAdherence_0".to_string(),
                self.m_af_tumble_direction_adherence_0.to_value(),
            )),
            Some((
                "m_afTumbleDirectionAdherence_1".to_string(),
                self.m_af_tumble_direction_adherence_1.to_value(),
            )),
            Some((
                "m_afTumbleDirectionAdherence_2".to_string(),
                self.m_af_tumble_direction_adherence_2.to_value(),
            )),
            Some((
                "m_afTumbleDirectionAdherence_3".to_string(),
                self.m_af_tumble_direction_adherence_3.to_value(),
            )),
            Some((
                "m_afTumbleDirectionAdherence_4".to_string(),
                self.m_af_tumble_direction_adherence_4.to_value(),
            )),
            Some((
                "m_afTumbleDirectionAdherence_5".to_string(),
                self.m_af_tumble_direction_adherence_5.to_value(),
            )),
            Some((
                "m_afTumbleDirectionAdherence_6".to_string(),
                self.m_af_tumble_direction_adherence_6.to_value(),
            )),
            Some((
                "m_afTumbleDirectionAdherence_7".to_string(),
                self.m_af_tumble_direction_adherence_7.to_value(),
            )),
            Some((
                "m_afTumbleDirectionAdherence_8".to_string(),
                self.m_af_tumble_direction_adherence_8.to_value(),
            )),
            Some((
                "m_afTumbleDirectionAdherence_9".to_string(),
                self.m_af_tumble_direction_adherence_9.to_value(),
            )),
            Some((
                "m_afTwitchThrow_0".to_string(),
                self.m_af_twitch_throw_0.to_value(),
            )),
            Some((
                "m_afTwitchThrow_1".to_string(),
                self.m_af_twitch_throw_1.to_value(),
            )),
            Some((
                "m_afTwitchThrow_2".to_string(),
                self.m_af_twitch_throw_2.to_value(),
            )),
            Some((
                "m_afTwitchThrow_3".to_string(),
                self.m_af_twitch_throw_3.to_value(),
            )),
            Some((
                "m_afTwitchThrow_4".to_string(),
                self.m_af_twitch_throw_4.to_value(),
            )),
            Some((
                "m_afTwitchThrow_5".to_string(),
                self.m_af_twitch_throw_5.to_value(),
            )),
            Some((
                "m_afTwitchThrow_6".to_string(),
                self.m_af_twitch_throw_6.to_value(),
            )),
            Some((
                "m_afTwitchThrow_7".to_string(),
                self.m_af_twitch_throw_7.to_value(),
            )),
            Some((
                "m_afTwitchThrow_8".to_string(),
                self.m_af_twitch_throw_8.to_value(),
            )),
            Some((
                "m_afTwitchThrow_9".to_string(),
                self.m_af_twitch_throw_9.to_value(),
            )),
            Some((
                "m_fTwitchSharpness".to_string(),
                self.m_f_twitch_sharpness.to_value(),
            )),
            Some((
                "m_fRollMaxScale".to_string(),
                self.m_f_roll_max_scale.to_value(),
            )),
            Some((
                "m_fRollMinScale".to_string(),
                self.m_f_roll_min_scale.to_value(),
            )),
            Some(("m_fRollSpeed".to_string(), self.m_f_roll_speed.to_value())),
            Some((
                "m_fRollSeparation".to_string(),
                self.m_f_roll_separation.to_value(),
            )),
            Some((
                "m_fLeewardScalar".to_string(),
                self.m_f_leeward_scalar.to_value(),
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
                "m_afRippleDistance_0" => {
                    (self.m_af_ripple_distance_0).inspect_value(path, offset, limit)
                }
                "m_afRippleDistance_1" => {
                    (self.m_af_ripple_distance_1).inspect_value(path, offset, limit)
                }
                "m_afRippleDistance_2" => {
                    (self.m_af_ripple_distance_2).inspect_value(path, offset, limit)
                }
                "m_afRippleDistance_3" => {
                    (self.m_af_ripple_distance_3).inspect_value(path, offset, limit)
                }
                "m_afRippleDistance_4" => {
                    (self.m_af_ripple_distance_4).inspect_value(path, offset, limit)
                }
                "m_afRippleDistance_5" => {
                    (self.m_af_ripple_distance_5).inspect_value(path, offset, limit)
                }
                "m_afRippleDistance_6" => {
                    (self.m_af_ripple_distance_6).inspect_value(path, offset, limit)
                }
                "m_afRippleDistance_7" => {
                    (self.m_af_ripple_distance_7).inspect_value(path, offset, limit)
                }
                "m_afRippleDistance_8" => {
                    (self.m_af_ripple_distance_8).inspect_value(path, offset, limit)
                }
                "m_afRippleDistance_9" => {
                    (self.m_af_ripple_distance_9).inspect_value(path, offset, limit)
                }
                "m_afTumbleFlip_0" => (self.m_af_tumble_flip_0).inspect_value(path, offset, limit),
                "m_afTumbleFlip_1" => (self.m_af_tumble_flip_1).inspect_value(path, offset, limit),
                "m_afTumbleFlip_2" => (self.m_af_tumble_flip_2).inspect_value(path, offset, limit),
                "m_afTumbleFlip_3" => (self.m_af_tumble_flip_3).inspect_value(path, offset, limit),
                "m_afTumbleFlip_4" => (self.m_af_tumble_flip_4).inspect_value(path, offset, limit),
                "m_afTumbleFlip_5" => (self.m_af_tumble_flip_5).inspect_value(path, offset, limit),
                "m_afTumbleFlip_6" => (self.m_af_tumble_flip_6).inspect_value(path, offset, limit),
                "m_afTumbleFlip_7" => (self.m_af_tumble_flip_7).inspect_value(path, offset, limit),
                "m_afTumbleFlip_8" => (self.m_af_tumble_flip_8).inspect_value(path, offset, limit),
                "m_afTumbleFlip_9" => (self.m_af_tumble_flip_9).inspect_value(path, offset, limit),
                "m_afTumbleTwist_0" => {
                    (self.m_af_tumble_twist_0).inspect_value(path, offset, limit)
                }
                "m_afTumbleTwist_1" => {
                    (self.m_af_tumble_twist_1).inspect_value(path, offset, limit)
                }
                "m_afTumbleTwist_2" => {
                    (self.m_af_tumble_twist_2).inspect_value(path, offset, limit)
                }
                "m_afTumbleTwist_3" => {
                    (self.m_af_tumble_twist_3).inspect_value(path, offset, limit)
                }
                "m_afTumbleTwist_4" => {
                    (self.m_af_tumble_twist_4).inspect_value(path, offset, limit)
                }
                "m_afTumbleTwist_5" => {
                    (self.m_af_tumble_twist_5).inspect_value(path, offset, limit)
                }
                "m_afTumbleTwist_6" => {
                    (self.m_af_tumble_twist_6).inspect_value(path, offset, limit)
                }
                "m_afTumbleTwist_7" => {
                    (self.m_af_tumble_twist_7).inspect_value(path, offset, limit)
                }
                "m_afTumbleTwist_8" => {
                    (self.m_af_tumble_twist_8).inspect_value(path, offset, limit)
                }
                "m_afTumbleTwist_9" => {
                    (self.m_af_tumble_twist_9).inspect_value(path, offset, limit)
                }
                "m_afTumbleDirectionAdherence_0" => {
                    (self.m_af_tumble_direction_adherence_0).inspect_value(path, offset, limit)
                }
                "m_afTumbleDirectionAdherence_1" => {
                    (self.m_af_tumble_direction_adherence_1).inspect_value(path, offset, limit)
                }
                "m_afTumbleDirectionAdherence_2" => {
                    (self.m_af_tumble_direction_adherence_2).inspect_value(path, offset, limit)
                }
                "m_afTumbleDirectionAdherence_3" => {
                    (self.m_af_tumble_direction_adherence_3).inspect_value(path, offset, limit)
                }
                "m_afTumbleDirectionAdherence_4" => {
                    (self.m_af_tumble_direction_adherence_4).inspect_value(path, offset, limit)
                }
                "m_afTumbleDirectionAdherence_5" => {
                    (self.m_af_tumble_direction_adherence_5).inspect_value(path, offset, limit)
                }
                "m_afTumbleDirectionAdherence_6" => {
                    (self.m_af_tumble_direction_adherence_6).inspect_value(path, offset, limit)
                }
                "m_afTumbleDirectionAdherence_7" => {
                    (self.m_af_tumble_direction_adherence_7).inspect_value(path, offset, limit)
                }
                "m_afTumbleDirectionAdherence_8" => {
                    (self.m_af_tumble_direction_adherence_8).inspect_value(path, offset, limit)
                }
                "m_afTumbleDirectionAdherence_9" => {
                    (self.m_af_tumble_direction_adherence_9).inspect_value(path, offset, limit)
                }
                "m_afTwitchThrow_0" => {
                    (self.m_af_twitch_throw_0).inspect_value(path, offset, limit)
                }
                "m_afTwitchThrow_1" => {
                    (self.m_af_twitch_throw_1).inspect_value(path, offset, limit)
                }
                "m_afTwitchThrow_2" => {
                    (self.m_af_twitch_throw_2).inspect_value(path, offset, limit)
                }
                "m_afTwitchThrow_3" => {
                    (self.m_af_twitch_throw_3).inspect_value(path, offset, limit)
                }
                "m_afTwitchThrow_4" => {
                    (self.m_af_twitch_throw_4).inspect_value(path, offset, limit)
                }
                "m_afTwitchThrow_5" => {
                    (self.m_af_twitch_throw_5).inspect_value(path, offset, limit)
                }
                "m_afTwitchThrow_6" => {
                    (self.m_af_twitch_throw_6).inspect_value(path, offset, limit)
                }
                "m_afTwitchThrow_7" => {
                    (self.m_af_twitch_throw_7).inspect_value(path, offset, limit)
                }
                "m_afTwitchThrow_8" => {
                    (self.m_af_twitch_throw_8).inspect_value(path, offset, limit)
                }
                "m_afTwitchThrow_9" => {
                    (self.m_af_twitch_throw_9).inspect_value(path, offset, limit)
                }
                "m_fTwitchSharpness" => {
                    (self.m_f_twitch_sharpness).inspect_value(path, offset, limit)
                }
                "m_fRollMaxScale" => (self.m_f_roll_max_scale).inspect_value(path, offset, limit),
                "m_fRollMinScale" => (self.m_f_roll_min_scale).inspect_value(path, offset, limit),
                "m_fRollSpeed" => (self.m_f_roll_speed).inspect_value(path, offset, limit),
                "m_fRollSeparation" => {
                    (self.m_f_roll_separation).inspect_value(path, offset, limit)
                }
                "m_fLeewardScalar" => (self.m_f_leeward_scalar).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_af_ripple_distance_0).inspect_value(path, offset, limit))
                .map(|v| ("m_afRippleDistance_0".to_string(), v)),
            ((self.m_af_ripple_distance_1).inspect_value(path, offset, limit))
                .map(|v| ("m_afRippleDistance_1".to_string(), v)),
            ((self.m_af_ripple_distance_2).inspect_value(path, offset, limit))
                .map(|v| ("m_afRippleDistance_2".to_string(), v)),
            ((self.m_af_ripple_distance_3).inspect_value(path, offset, limit))
                .map(|v| ("m_afRippleDistance_3".to_string(), v)),
            ((self.m_af_ripple_distance_4).inspect_value(path, offset, limit))
                .map(|v| ("m_afRippleDistance_4".to_string(), v)),
            ((self.m_af_ripple_distance_5).inspect_value(path, offset, limit))
                .map(|v| ("m_afRippleDistance_5".to_string(), v)),
            ((self.m_af_ripple_distance_6).inspect_value(path, offset, limit))
                .map(|v| ("m_afRippleDistance_6".to_string(), v)),
            ((self.m_af_ripple_distance_7).inspect_value(path, offset, limit))
                .map(|v| ("m_afRippleDistance_7".to_string(), v)),
            ((self.m_af_ripple_distance_8).inspect_value(path, offset, limit))
                .map(|v| ("m_afRippleDistance_8".to_string(), v)),
            ((self.m_af_ripple_distance_9).inspect_value(path, offset, limit))
                .map(|v| ("m_afRippleDistance_9".to_string(), v)),
            ((self.m_af_tumble_flip_0).inspect_value(path, offset, limit))
                .map(|v| ("m_afTumbleFlip_0".to_string(), v)),
            ((self.m_af_tumble_flip_1).inspect_value(path, offset, limit))
                .map(|v| ("m_afTumbleFlip_1".to_string(), v)),
            ((self.m_af_tumble_flip_2).inspect_value(path, offset, limit))
                .map(|v| ("m_afTumbleFlip_2".to_string(), v)),
            ((self.m_af_tumble_flip_3).inspect_value(path, offset, limit))
                .map(|v| ("m_afTumbleFlip_3".to_string(), v)),
            ((self.m_af_tumble_flip_4).inspect_value(path, offset, limit))
                .map(|v| ("m_afTumbleFlip_4".to_string(), v)),
            ((self.m_af_tumble_flip_5).inspect_value(path, offset, limit))
                .map(|v| ("m_afTumbleFlip_5".to_string(), v)),
            ((self.m_af_tumble_flip_6).inspect_value(path, offset, limit))
                .map(|v| ("m_afTumbleFlip_6".to_string(), v)),
            ((self.m_af_tumble_flip_7).inspect_value(path, offset, limit))
                .map(|v| ("m_afTumbleFlip_7".to_string(), v)),
            ((self.m_af_tumble_flip_8).inspect_value(path, offset, limit))
                .map(|v| ("m_afTumbleFlip_8".to_string(), v)),
            ((self.m_af_tumble_flip_9).inspect_value(path, offset, limit))
                .map(|v| ("m_afTumbleFlip_9".to_string(), v)),
            ((self.m_af_tumble_twist_0).inspect_value(path, offset, limit))
                .map(|v| ("m_afTumbleTwist_0".to_string(), v)),
            ((self.m_af_tumble_twist_1).inspect_value(path, offset, limit))
                .map(|v| ("m_afTumbleTwist_1".to_string(), v)),
            ((self.m_af_tumble_twist_2).inspect_value(path, offset, limit))
                .map(|v| ("m_afTumbleTwist_2".to_string(), v)),
            ((self.m_af_tumble_twist_3).inspect_value(path, offset, limit))
                .map(|v| ("m_afTumbleTwist_3".to_string(), v)),
            ((self.m_af_tumble_twist_4).inspect_value(path, offset, limit))
                .map(|v| ("m_afTumbleTwist_4".to_string(), v)),
            ((self.m_af_tumble_twist_5).inspect_value(path, offset, limit))
                .map(|v| ("m_afTumbleTwist_5".to_string(), v)),
            ((self.m_af_tumble_twist_6).inspect_value(path, offset, limit))
                .map(|v| ("m_afTumbleTwist_6".to_string(), v)),
            ((self.m_af_tumble_twist_7).inspect_value(path, offset, limit))
                .map(|v| ("m_afTumbleTwist_7".to_string(), v)),
            ((self.m_af_tumble_twist_8).inspect_value(path, offset, limit))
                .map(|v| ("m_afTumbleTwist_8".to_string(), v)),
            ((self.m_af_tumble_twist_9).inspect_value(path, offset, limit))
                .map(|v| ("m_afTumbleTwist_9".to_string(), v)),
            ((self.m_af_tumble_direction_adherence_0).inspect_value(path, offset, limit))
                .map(|v| ("m_afTumbleDirectionAdherence_0".to_string(), v)),
            ((self.m_af_tumble_direction_adherence_1).inspect_value(path, offset, limit))
                .map(|v| ("m_afTumbleDirectionAdherence_1".to_string(), v)),
            ((self.m_af_tumble_direction_adherence_2).inspect_value(path, offset, limit))
                .map(|v| ("m_afTumbleDirectionAdherence_2".to_string(), v)),
            ((self.m_af_tumble_direction_adherence_3).inspect_value(path, offset, limit))
                .map(|v| ("m_afTumbleDirectionAdherence_3".to_string(), v)),
            ((self.m_af_tumble_direction_adherence_4).inspect_value(path, offset, limit))
                .map(|v| ("m_afTumbleDirectionAdherence_4".to_string(), v)),
            ((self.m_af_tumble_direction_adherence_5).inspect_value(path, offset, limit))
                .map(|v| ("m_afTumbleDirectionAdherence_5".to_string(), v)),
            ((self.m_af_tumble_direction_adherence_6).inspect_value(path, offset, limit))
                .map(|v| ("m_afTumbleDirectionAdherence_6".to_string(), v)),
            ((self.m_af_tumble_direction_adherence_7).inspect_value(path, offset, limit))
                .map(|v| ("m_afTumbleDirectionAdherence_7".to_string(), v)),
            ((self.m_af_tumble_direction_adherence_8).inspect_value(path, offset, limit))
                .map(|v| ("m_afTumbleDirectionAdherence_8".to_string(), v)),
            ((self.m_af_tumble_direction_adherence_9).inspect_value(path, offset, limit))
                .map(|v| ("m_afTumbleDirectionAdherence_9".to_string(), v)),
            ((self.m_af_twitch_throw_0).inspect_value(path, offset, limit))
                .map(|v| ("m_afTwitchThrow_0".to_string(), v)),
            ((self.m_af_twitch_throw_1).inspect_value(path, offset, limit))
                .map(|v| ("m_afTwitchThrow_1".to_string(), v)),
            ((self.m_af_twitch_throw_2).inspect_value(path, offset, limit))
                .map(|v| ("m_afTwitchThrow_2".to_string(), v)),
            ((self.m_af_twitch_throw_3).inspect_value(path, offset, limit))
                .map(|v| ("m_afTwitchThrow_3".to_string(), v)),
            ((self.m_af_twitch_throw_4).inspect_value(path, offset, limit))
                .map(|v| ("m_afTwitchThrow_4".to_string(), v)),
            ((self.m_af_twitch_throw_5).inspect_value(path, offset, limit))
                .map(|v| ("m_afTwitchThrow_5".to_string(), v)),
            ((self.m_af_twitch_throw_6).inspect_value(path, offset, limit))
                .map(|v| ("m_afTwitchThrow_6".to_string(), v)),
            ((self.m_af_twitch_throw_7).inspect_value(path, offset, limit))
                .map(|v| ("m_afTwitchThrow_7".to_string(), v)),
            ((self.m_af_twitch_throw_8).inspect_value(path, offset, limit))
                .map(|v| ("m_afTwitchThrow_8".to_string(), v)),
            ((self.m_af_twitch_throw_9).inspect_value(path, offset, limit))
                .map(|v| ("m_afTwitchThrow_9".to_string(), v)),
            ((self.m_f_twitch_sharpness).inspect_value(path, offset, limit))
                .map(|v| ("m_fTwitchSharpness".to_string(), v)),
            ((self.m_f_roll_max_scale).inspect_value(path, offset, limit))
                .map(|v| ("m_fRollMaxScale".to_string(), v)),
            ((self.m_f_roll_min_scale).inspect_value(path, offset, limit))
                .map(|v| ("m_fRollMinScale".to_string(), v)),
            ((self.m_f_roll_speed).inspect_value(path, offset, limit))
                .map(|v| ("m_fRollSpeed".to_string(), v)),
            ((self.m_f_roll_separation).inspect_value(path, offset, limit))
                .map(|v| ("m_fRollSeparation".to_string(), v)),
            ((self.m_f_leeward_scalar).inspect_value(path, offset, limit))
                .map(|v| ("m_fLeewardScalar".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
