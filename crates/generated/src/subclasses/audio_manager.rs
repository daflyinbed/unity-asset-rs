//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct AudioManager {
    pub m_object_hide_flags: u32,
    pub m_volume: f32,
    pub m_speed_of_sound: f32,
    pub doppler_factor: f32,
    pub default_speaker_mode: i32,
    pub rolloff_scale: f32,
    pub m_dsp_buffer_size: i32,
}

impl crate::Parse for AudioManager {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_volume = r.read_f32()?;
        let m_speed_of_sound = r.read_f32()?;
        let doppler_factor = r.read_f32()?;
        let default_speaker_mode = r.read_i32()?;
        let rolloff_scale = r.read_f32()?;
        let m_dsp_buffer_size = r.read_i32()?;

        Ok(AudioManager {
            m_object_hide_flags,
            m_volume,
            m_speed_of_sound,
            doppler_factor,
            default_speaker_mode,
            rolloff_scale,
            m_dsp_buffer_size,
        })
    }
}
