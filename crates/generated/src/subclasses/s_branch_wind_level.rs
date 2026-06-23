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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_af_distance_0 = r.read_f32()?;
        let m_af_distance_1 = r.read_f32()?;
        let m_af_distance_2 = r.read_f32()?;
        let m_af_distance_3 = r.read_f32()?;
        let m_af_distance_4 = r.read_f32()?;
        let m_af_distance_5 = r.read_f32()?;
        let m_af_distance_6 = r.read_f32()?;
        let m_af_distance_7 = r.read_f32()?;
        let m_af_distance_8 = r.read_f32()?;
        let m_af_distance_9 = r.read_f32()?;
        let m_af_direction_adherence_0 = r.read_f32()?;
        let m_af_direction_adherence_1 = r.read_f32()?;
        let m_af_direction_adherence_2 = r.read_f32()?;
        let m_af_direction_adherence_3 = r.read_f32()?;
        let m_af_direction_adherence_4 = r.read_f32()?;
        let m_af_direction_adherence_5 = r.read_f32()?;
        let m_af_direction_adherence_6 = r.read_f32()?;
        let m_af_direction_adherence_7 = r.read_f32()?;
        let m_af_direction_adherence_8 = r.read_f32()?;
        let m_af_direction_adherence_9 = r.read_f32()?;
        let m_af_whip_0 = r.read_f32()?;
        let m_af_whip_1 = r.read_f32()?;
        let m_af_whip_2 = r.read_f32()?;
        let m_af_whip_3 = r.read_f32()?;
        let m_af_whip_4 = r.read_f32()?;
        let m_af_whip_5 = r.read_f32()?;
        let m_af_whip_6 = r.read_f32()?;
        let m_af_whip_7 = r.read_f32()?;
        let m_af_whip_8 = r.read_f32()?;
        let m_af_whip_9 = r.read_f32()?;
        let m_f_turbulence = r.read_f32()?;
        let m_f_twitch = r.read_f32()?;
        let m_f_twitch_freq_scale = r.read_f32()?;

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
