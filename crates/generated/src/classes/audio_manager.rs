//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct AudioManager {
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
        let mut m_volume = None;
        let mut m_speed_of_sound = None;
        let mut rolloff_scale = None;
        let mut doppler_factor = None;
        let mut default_speaker_mode = None;
        let mut m_sample_rate = None;
        let mut m_dsp_buffer_size = None;
        let mut m_disable_audio = None;
        let mut m_virtual_voice_count = None;
        let mut m_real_voice_count = None;
        let mut m_spatializer_plugin = None;
        let mut m_ambisonic_decoder_plugin = None;
        let mut m_virtualize_effects = None;
        let mut m_requested_dsp_buffer_size = None;
        let mut m_audio_foundation = None;
        let mut m_output_channel_layout = None;
        let mut m_output_sampling_rate = None;
            m_volume = Some(r.read_f32()?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            rolloff_scale = Some(r.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            rolloff_scale = Some(r.read_f32()?);
            m_speed_of_sound = Some(r.read_f32()?);
        } else {
            m_speed_of_sound = Some(r.read_f32()?);
        }
            doppler_factor = Some(r.read_f32()?);
            default_speaker_mode = Some(r.read_i32()?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_sample_rate = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
        } else {
            rolloff_scale = Some(r.read_f32()?);
        }
            m_dsp_buffer_size = Some(r.read_i32()?);
        if *ver >= crate::UnityVersion::new(6000, 3, 0) {
            m_virtual_voice_count = Some(r.read_i32()?);
            m_real_voice_count = Some(r.read_i32()?);
            r.align(4)?;
            m_spatializer_plugin = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_ambisonic_decoder_plugin = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_disable_audio = Some(r.read_bool()?);
            m_virtualize_effects = Some(r.read_bool()?);
            r.align(4)?;
            m_requested_dsp_buffer_size = Some(r.read_i32()?);
            m_audio_foundation = Some(r.read_i32()?);
            m_output_channel_layout = Some(r.read_i32()?);
            m_output_sampling_rate = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2019, 2, 0) {
            m_virtual_voice_count = Some(r.read_i32()?);
            m_real_voice_count = Some(r.read_i32()?);
            r.align(4)?;
            m_spatializer_plugin = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_ambisonic_decoder_plugin = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_disable_audio = Some(r.read_bool()?);
            m_virtualize_effects = Some(r.read_bool()?);
            r.align(4)?;
            m_requested_dsp_buffer_size = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2019, 2, 0) {
            m_virtual_voice_count = Some(r.read_i32()?);
            m_real_voice_count = Some(r.read_i32()?);
            r.align(4)?;
            m_spatializer_plugin = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_ambisonic_decoder_plugin = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_disable_audio = Some(r.read_bool()?);
            m_virtualize_effects = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 1, 1) {
            m_virtual_voice_count = Some(r.read_i32()?);
            m_real_voice_count = Some(r.read_i32()?);
            r.align(4)?;
            m_spatializer_plugin = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_ambisonic_decoder_plugin = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_disable_audio = Some(r.read_bool()?);
            m_virtualize_effects = Some(r.read_bool()?);
            r.align(4)?;
            m_requested_dsp_buffer_size = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_virtual_voice_count = Some(r.read_i32()?);
            m_real_voice_count = Some(r.read_i32()?);
            r.align(4)?;
            m_spatializer_plugin = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_ambisonic_decoder_plugin = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_disable_audio = Some(r.read_bool()?);
            m_virtualize_effects = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2018, 3, 14) {
            m_virtual_voice_count = Some(r.read_i32()?);
            m_real_voice_count = Some(r.read_i32()?);
            r.align(4)?;
            m_spatializer_plugin = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_ambisonic_decoder_plugin = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_disable_audio = Some(r.read_bool()?);
            m_virtualize_effects = Some(r.read_bool()?);
            r.align(4)?;
            m_requested_dsp_buffer_size = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_virtual_voice_count = Some(r.read_i32()?);
            m_real_voice_count = Some(r.read_i32()?);
            r.align(4)?;
            m_spatializer_plugin = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_ambisonic_decoder_plugin = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_disable_audio = Some(r.read_bool()?);
            m_virtualize_effects = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 3, 6) {
            m_virtual_voice_count = Some(r.read_i32()?);
            m_real_voice_count = Some(r.read_i32()?);
            r.align(4)?;
            m_spatializer_plugin = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_disable_audio = Some(r.read_bool()?);
            m_virtualize_effects = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 2, 0) {
            m_virtual_voice_count = Some(r.read_i32()?);
            m_real_voice_count = Some(r.read_i32()?);
            r.align(4)?;
            m_spatializer_plugin = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_disable_audio = Some(r.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_virtual_voice_count = Some(r.read_i32()?);
            m_real_voice_count = Some(r.read_i32()?);
            r.align(4)?;
            m_disable_audio = Some(r.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(4, 2, 0) {
            m_disable_audio = Some(r.read_bool()?);
        } else {
        }

        Ok(AudioManager {
            m_volume: m_volume.unwrap(),
            m_speed_of_sound,
            rolloff_scale: rolloff_scale.unwrap(),
            doppler_factor: doppler_factor.unwrap(),
            default_speaker_mode: default_speaker_mode.unwrap(),
            m_sample_rate,
            m_dsp_buffer_size: m_dsp_buffer_size.unwrap(),
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
