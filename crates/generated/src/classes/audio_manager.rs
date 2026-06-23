//! 自动生成。请勿手动修改。

use super::global_game_manager::GlobalGameManager;

#[derive(Debug, Clone)]
pub struct AudioManager {
    pub parent: GlobalGameManager,
    pub m_volume: f32,
    pub m_speed_of_sound: Option<f32>,
    pub rolloff_scale: f32,
    pub doppler_factor: f32,
    pub default_speaker_mode: i32,
    pub m_sample_rate: Option<i32>,
    pub m_dsp_buffer_size: i32,
    pub m_disable_audio: Option<bool>,
    pub m_virtual_voice_count: Option<i32>,
    pub m_real_voice_count: Option<i32>,
    pub m_spatializer_plugin: Option<String>,
    pub m_ambisonic_decoder_plugin: Option<String>,
    pub m_virtualize_effects: Option<bool>,
    pub m_requested_dsp_buffer_size: Option<i32>,
    pub m_audio_foundation: Option<i32>,
    pub m_output_channel_layout: Option<i32>,
    pub m_output_sampling_rate: Option<i32>,
}

impl crate::Parse for AudioManager {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = GlobalGameManager::parse(r, ver)?;
        let m_volume = r.read_f32()?;
        let m_speed_of_sound = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let rolloff_scale = r.read_f32()?;
        let doppler_factor = r.read_f32()?;
        let default_speaker_mode = r.read_i32()?;
        let m_sample_rate = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_dsp_buffer_size = r.read_i32()?;
        let m_disable_audio = if *ver >= crate::UnityVersion::new(4, 2, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_virtual_voice_count = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_real_voice_count = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_spatializer_plugin = if *ver >= crate::UnityVersion::new(5, 2, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_ambisonic_decoder_plugin = if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_virtualize_effects = if *ver >= crate::UnityVersion::new(5, 3, 6) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_requested_dsp_buffer_size = if *ver >= crate::UnityVersion::new(2018, 3, 14) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_audio_foundation = if *ver >= crate::UnityVersion::new(6000, 3, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_output_channel_layout = if *ver >= crate::UnityVersion::new(6000, 3, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_output_sampling_rate = if *ver >= crate::UnityVersion::new(6000, 3, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };

        Ok(AudioManager {
            parent,
            m_volume,
            m_speed_of_sound,
            rolloff_scale,
            doppler_factor,
            default_speaker_mode,
            m_sample_rate,
            m_dsp_buffer_size,
            m_disable_audio,
            m_virtual_voice_count,
            m_real_voice_count,
            m_spatializer_plugin,
            m_ambisonic_decoder_plugin,
            m_virtualize_effects,
            m_requested_dsp_buffer_size,
            m_audio_foundation,
            m_output_channel_layout,
            m_output_sampling_rate,
        })
    }
}
