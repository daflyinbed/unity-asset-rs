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
