//! 自动生成。请勿手动修改。

/// 版本枚举：AudioManagerMDisableAudioVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AudioManagerMDisableAudioVariant {
    Default(bool),
    V5_0(bool),
    V5_3_6(bool),
}

impl crate::Parse for AudioManagerMDisableAudioVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 3, 6) {
            let v = rdr.read_bool()?;
            Ok(Self::V5_3_6(v))
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V5_0(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for AudioManagerMDisableAudioVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_0(v) => v.to_value(),
            Self::V5_3_6(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_0(v) => (v).inspect_value(path, offset, limit),
            Self::V5_3_6(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：AudioManagerMRealVoiceCountVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AudioManagerMRealVoiceCountVariant {
    Default(i32),
    V2019_4_24(i32),
    V2020_1(i32),
    V2020_2_2(i32),
}

impl crate::Parse for AudioManagerMRealVoiceCountVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2020, 2, 2) {
            let v = rdr.read_i32()?;
            rdr.align(4)?;
            Ok(Self::V2020_2_2(v))
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = rdr.read_i32()?;
            Ok(Self::V2020_1(v))
        } else if *ver >= crate::UnityVersion::new(2019, 4, 24) {
            let v = rdr.read_i32()?;
            rdr.align(4)?;
            Ok(Self::V2019_4_24(v))
        } else {
            let v = rdr.read_i32()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for AudioManagerMRealVoiceCountVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2019_4_24(v) => v.to_value(),
            Self::V2020_1(v) => v.to_value(),
            Self::V2020_2_2(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2019_4_24(v) => (v).inspect_value(path, offset, limit),
            Self::V2020_1(v) => (v).inspect_value(path, offset, limit),
            Self::V2020_2_2(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct AudioManager {
    pub m_volume: f32,
    pub m_speed_of_sound: Option<f32>,
    pub rolloff_scale: f32,
    pub doppler_factor: f32,
    pub default_speaker_mode: i32,
    pub m_sample_rate: Option<i32>,
    pub m_dsp_buffer_size: i32,
    pub m_disable_audio: Option<AudioManagerMDisableAudioVariant>,
    pub m_virtual_voice_count: Option<i32>,
    pub m_real_voice_count: Option<AudioManagerMRealVoiceCountVariant>,
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
        rdr: &mut R,
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
        m_volume = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            rolloff_scale = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            rolloff_scale = Some(rdr.read_f32()?);
            m_speed_of_sound = Some(rdr.read_f32()?);
        } else {
            m_speed_of_sound = Some(rdr.read_f32()?);
        }
        doppler_factor = Some(rdr.read_f32()?);
        default_speaker_mode = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_sample_rate = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
        } else {
            rolloff_scale = Some(rdr.read_f32()?);
        }
        m_dsp_buffer_size = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(6000, 3, 0) {
            m_virtual_voice_count = Some(rdr.read_i32()?);
            m_real_voice_count = Some(AudioManagerMRealVoiceCountVariant::parse(rdr, ver)?);
            m_spatializer_plugin = Some(rdr.read_aligned_string()?);
            m_ambisonic_decoder_plugin = Some(rdr.read_aligned_string()?);
            m_disable_audio = Some(AudioManagerMDisableAudioVariant::parse(rdr, ver)?);
            m_virtualize_effects = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_requested_dsp_buffer_size = Some(rdr.read_i32()?);
            m_audio_foundation = Some(rdr.read_i32()?);
            m_output_channel_layout = Some(rdr.read_i32()?);
            m_output_sampling_rate = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2019, 1, 1) {
            m_virtual_voice_count = Some(rdr.read_i32()?);
            m_real_voice_count = Some(AudioManagerMRealVoiceCountVariant::parse(rdr, ver)?);
            m_spatializer_plugin = Some(rdr.read_aligned_string()?);
            m_ambisonic_decoder_plugin = Some(rdr.read_aligned_string()?);
            m_disable_audio = Some(AudioManagerMDisableAudioVariant::parse(rdr, ver)?);
            m_virtualize_effects = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_requested_dsp_buffer_size = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_virtual_voice_count = Some(rdr.read_i32()?);
            m_real_voice_count = Some(AudioManagerMRealVoiceCountVariant::parse(rdr, ver)?);
            m_spatializer_plugin = Some(rdr.read_aligned_string()?);
            m_ambisonic_decoder_plugin = Some(rdr.read_aligned_string()?);
            m_disable_audio = Some(AudioManagerMDisableAudioVariant::parse(rdr, ver)?);
            m_virtualize_effects = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2018, 3, 14) {
            m_virtual_voice_count = Some(rdr.read_i32()?);
            m_real_voice_count = Some(AudioManagerMRealVoiceCountVariant::parse(rdr, ver)?);
            m_spatializer_plugin = Some(rdr.read_aligned_string()?);
            m_ambisonic_decoder_plugin = Some(rdr.read_aligned_string()?);
            m_disable_audio = Some(AudioManagerMDisableAudioVariant::parse(rdr, ver)?);
            m_virtualize_effects = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_requested_dsp_buffer_size = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_virtual_voice_count = Some(rdr.read_i32()?);
            m_real_voice_count = Some(AudioManagerMRealVoiceCountVariant::parse(rdr, ver)?);
            m_spatializer_plugin = Some(rdr.read_aligned_string()?);
            m_ambisonic_decoder_plugin = Some(rdr.read_aligned_string()?);
            m_disable_audio = Some(AudioManagerMDisableAudioVariant::parse(rdr, ver)?);
            m_virtualize_effects = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 3, 6) {
            m_virtual_voice_count = Some(rdr.read_i32()?);
            m_real_voice_count = Some(AudioManagerMRealVoiceCountVariant::parse(rdr, ver)?);
            m_spatializer_plugin = Some(rdr.read_aligned_string()?);
            m_disable_audio = Some(AudioManagerMDisableAudioVariant::parse(rdr, ver)?);
            m_virtualize_effects = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 2, 0) {
            m_virtual_voice_count = Some(rdr.read_i32()?);
            m_real_voice_count = Some(AudioManagerMRealVoiceCountVariant::parse(rdr, ver)?);
            m_spatializer_plugin = Some(rdr.read_aligned_string()?);
            m_disable_audio = Some(AudioManagerMDisableAudioVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_virtual_voice_count = Some(rdr.read_i32()?);
            m_real_voice_count = Some(AudioManagerMRealVoiceCountVariant::parse(rdr, ver)?);
            m_disable_audio = Some(AudioManagerMDisableAudioVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 2, 0) {
            m_disable_audio = Some(AudioManagerMDisableAudioVariant::parse(rdr, ver)?);
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

impl crate::ToValue for AudioManager {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Volume".to_string(), self.m_volume.to_value())),
            crate::opt_field("m_SpeedOfSound", &self.m_speed_of_sound),
            Some(("Rolloff Scale".to_string(), self.rolloff_scale.to_value())),
            Some(("Doppler Factor".to_string(), self.doppler_factor.to_value())),
            Some((
                "Default Speaker Mode".to_string(),
                self.default_speaker_mode.to_value(),
            )),
            crate::opt_field("m_SampleRate", &self.m_sample_rate),
            Some((
                "m_DSPBufferSize".to_string(),
                self.m_dsp_buffer_size.to_value(),
            )),
            crate::opt_field("m_DisableAudio", &self.m_disable_audio),
            crate::opt_field("m_VirtualVoiceCount", &self.m_virtual_voice_count),
            crate::opt_field("m_RealVoiceCount", &self.m_real_voice_count),
            crate::opt_field("m_SpatializerPlugin", &self.m_spatializer_plugin),
            crate::opt_field("m_AmbisonicDecoderPlugin", &self.m_ambisonic_decoder_plugin),
            crate::opt_field("m_VirtualizeEffects", &self.m_virtualize_effects),
            crate::opt_field(
                "m_RequestedDSPBufferSize",
                &self.m_requested_dsp_buffer_size,
            ),
            crate::opt_field("m_AudioFoundation", &self.m_audio_foundation),
            crate::opt_field("m_OutputChannelLayout", &self.m_output_channel_layout),
            crate::opt_field("m_OutputSamplingRate", &self.m_output_sampling_rate),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Volume" => (self.m_volume).inspect_value(path, offset, limit),
                "m_SpeedOfSound" => (self.m_speed_of_sound).inspect_value(path, offset, limit),
                "Rolloff Scale" => (self.rolloff_scale).inspect_value(path, offset, limit),
                "Doppler Factor" => (self.doppler_factor).inspect_value(path, offset, limit),
                "Default Speaker Mode" => {
                    (self.default_speaker_mode).inspect_value(path, offset, limit)
                }
                "m_SampleRate" => (self.m_sample_rate).inspect_value(path, offset, limit),
                "m_DSPBufferSize" => (self.m_dsp_buffer_size).inspect_value(path, offset, limit),
                "m_DisableAudio" => (self.m_disable_audio).inspect_value(path, offset, limit),
                "m_VirtualVoiceCount" => {
                    (self.m_virtual_voice_count).inspect_value(path, offset, limit)
                }
                "m_RealVoiceCount" => (self.m_real_voice_count).inspect_value(path, offset, limit),
                "m_SpatializerPlugin" => {
                    (self.m_spatializer_plugin).inspect_value(path, offset, limit)
                }
                "m_AmbisonicDecoderPlugin" => {
                    (self.m_ambisonic_decoder_plugin).inspect_value(path, offset, limit)
                }
                "m_VirtualizeEffects" => {
                    (self.m_virtualize_effects).inspect_value(path, offset, limit)
                }
                "m_RequestedDSPBufferSize" => {
                    (self.m_requested_dsp_buffer_size).inspect_value(path, offset, limit)
                }
                "m_AudioFoundation" => (self.m_audio_foundation).inspect_value(path, offset, limit),
                "m_OutputChannelLayout" => {
                    (self.m_output_channel_layout).inspect_value(path, offset, limit)
                }
                "m_OutputSamplingRate" => {
                    (self.m_output_sampling_rate).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_volume).inspect_value(path, offset, limit))
                .map(|v| ("m_Volume".to_string(), v)),
            ((self.m_speed_of_sound).inspect_value(path, offset, limit))
                .map(|v| ("m_SpeedOfSound".to_string(), v)),
            ((self.rolloff_scale).inspect_value(path, offset, limit))
                .map(|v| ("Rolloff Scale".to_string(), v)),
            ((self.doppler_factor).inspect_value(path, offset, limit))
                .map(|v| ("Doppler Factor".to_string(), v)),
            ((self.default_speaker_mode).inspect_value(path, offset, limit))
                .map(|v| ("Default Speaker Mode".to_string(), v)),
            ((self.m_sample_rate).inspect_value(path, offset, limit))
                .map(|v| ("m_SampleRate".to_string(), v)),
            ((self.m_dsp_buffer_size).inspect_value(path, offset, limit))
                .map(|v| ("m_DSPBufferSize".to_string(), v)),
            ((self.m_disable_audio).inspect_value(path, offset, limit))
                .map(|v| ("m_DisableAudio".to_string(), v)),
            ((self.m_virtual_voice_count).inspect_value(path, offset, limit))
                .map(|v| ("m_VirtualVoiceCount".to_string(), v)),
            ((self.m_real_voice_count).inspect_value(path, offset, limit))
                .map(|v| ("m_RealVoiceCount".to_string(), v)),
            ((self.m_spatializer_plugin).inspect_value(path, offset, limit))
                .map(|v| ("m_SpatializerPlugin".to_string(), v)),
            ((self.m_ambisonic_decoder_plugin).inspect_value(path, offset, limit))
                .map(|v| ("m_AmbisonicDecoderPlugin".to_string(), v)),
            ((self.m_virtualize_effects).inspect_value(path, offset, limit))
                .map(|v| ("m_VirtualizeEffects".to_string(), v)),
            ((self.m_requested_dsp_buffer_size).inspect_value(path, offset, limit))
                .map(|v| ("m_RequestedDSPBufferSize".to_string(), v)),
            ((self.m_audio_foundation).inspect_value(path, offset, limit))
                .map(|v| ("m_AudioFoundation".to_string(), v)),
            ((self.m_output_channel_layout).inspect_value(path, offset, limit))
                .map(|v| ("m_OutputChannelLayout".to_string(), v)),
            ((self.m_output_sampling_rate).inspect_value(path, offset, limit))
                .map(|v| ("m_OutputSamplingRate".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
