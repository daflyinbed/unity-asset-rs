//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::animation_curve::AnimationCurve;

/// 版本枚举：AudioSourceMResourceVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AudioSourceMResourceVariant {
    Default(PPtr),
    V6000_3(PPtr),
}

impl crate::Parse for AudioSourceMResourceVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(6000, 3, 0) {
            let v = crate::PPtr::parse(rdr, ver)?;
            Ok(Self::V6000_3(v))
        } else {
            let v = crate::PPtr::parse(rdr, ver)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for AudioSourceMResourceVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V6000_3(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V6000_3(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：AudioSourceMuteVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AudioSourceMuteVariant {
    Default(bool),
    V5_2(bool),
}

impl crate::Parse for AudioSourceMuteVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 2, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V5_2(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for AudioSourceMuteVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_2(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_2(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：AudioSourceSpatializeVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AudioSourceSpatializeVariant {
    Default(bool),
    V5_5(bool),
}

impl crate::Parse for AudioSourceSpatializeVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V5_5(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for AudioSourceSpatializeVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_5(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_5(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：AudioSourceBypassEffectsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AudioSourceBypassEffectsVariant {
    Default(bool),
    V4_2(bool),
}

impl crate::Parse for AudioSourceBypassEffectsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(4, 2, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V4_2(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for AudioSourceBypassEffectsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V4_2(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V4_2(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct AudioSource {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_audio_clip: PPtr,
    pub output_audio_mixer_group: Option<PPtr>,
    pub m_play_on_awake: bool,
    pub m_volume: f32,
    pub m_resource: Option<AudioSourceMResourceVariant>,
    pub m_pitch: f32,
    pub r#loop: bool,
    pub mute: AudioSourceMuteVariant,
    pub priority: i32,
    pub doppler_level: f32,
    pub spatialize: Option<AudioSourceSpatializeVariant>,
    pub min_distance: f32,
    pub spatialize_post_effects: Option<bool>,
    pub max_distance: f32,
    pub pan2_d: f32,
    pub rolloff_mode: i32,
    pub bypass_effects: AudioSourceBypassEffectsVariant,
    pub rolloff_custom_curve: AnimationCurve,
    pub bypass_listener_effects: Option<bool>,
    pub pan_level_custom_curve: AnimationCurve,
    pub bypass_reverb_zones: Option<bool>,
    pub spread_custom_curve: AnimationCurve,
    pub reverb_zone_mix_custom_curve: Option<AnimationCurve>,
}

impl crate::Parse for AudioSource {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut m_enabled = None;
        let mut m_audio_clip = None;
        let mut output_audio_mixer_group = None;
        let mut m_play_on_awake = None;
        let mut m_volume = None;
        let mut m_resource = None;
        let mut m_pitch = None;
        let mut r#loop = None;
        let mut mute = None;
        let mut priority = None;
        let mut doppler_level = None;
        let mut spatialize = None;
        let mut min_distance = None;
        let mut spatialize_post_effects = None;
        let mut max_distance = None;
        let mut pan2_d = None;
        let mut rolloff_mode = None;
        let mut bypass_effects = None;
        let mut rolloff_custom_curve = None;
        let mut bypass_listener_effects = None;
        let mut pan_level_custom_curve = None;
        let mut bypass_reverb_zones = None;
        let mut spread_custom_curve = None;
        let mut reverb_zone_mix_custom_curve = None;
        m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
        m_enabled = Some(rdr.read_u8()?);
        rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            output_audio_mixer_group = Some(crate::PPtr::parse(rdr, ver)?);
        } else {
        }
        m_audio_clip = Some(crate::PPtr::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            m_resource = Some(AudioSourceMResourceVariant::parse(rdr, ver)?);
        } else {
        }
        m_play_on_awake = Some(rdr.read_bool()?);
        rdr.align(4)?;
        m_volume = Some(rdr.read_f32()?);
        m_pitch = Some(rdr.read_f32()?);
        r#loop = Some(rdr.read_bool()?);
        mute = Some(AudioSourceMuteVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            spatialize = Some(AudioSourceSpatializeVariant::parse(rdr, ver)?);
            spatialize_post_effects = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 2, 0) {
            spatialize = Some(AudioSourceSpatializeVariant::parse(rdr, ver)?);
        } else {
        }
        priority = Some(rdr.read_i32()?);
        doppler_level = Some(rdr.read_f32()?);
        min_distance = Some(rdr.read_f32()?);
        max_distance = Some(rdr.read_f32()?);
        pan2_d = Some(rdr.read_f32()?);
        rolloff_mode = Some(rdr.read_i32()?);
        bypass_effects = Some(AudioSourceBypassEffectsVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(4, 2, 0) {
            bypass_listener_effects = Some(rdr.read_bool()?);
            bypass_reverb_zones = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
        }
        rolloff_custom_curve = Some(AnimationCurve::parse(rdr, ver)?);
        pan_level_custom_curve = Some(AnimationCurve::parse(rdr, ver)?);
        spread_custom_curve = Some(AnimationCurve::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            reverb_zone_mix_custom_curve = Some(AnimationCurve::parse(rdr, ver)?);
        } else {
        }

        Ok(AudioSource {
            m_game_object: m_game_object.unwrap(),
            m_enabled: m_enabled.unwrap(),
            m_audio_clip: m_audio_clip.unwrap(),
            output_audio_mixer_group,
            m_play_on_awake: m_play_on_awake.unwrap(),
            m_volume: m_volume.unwrap(),
            m_resource,
            m_pitch: m_pitch.unwrap(),
            r#loop: r#loop.unwrap(),
            mute: mute.unwrap(),
            priority: priority.unwrap(),
            doppler_level: doppler_level.unwrap(),
            spatialize,
            min_distance: min_distance.unwrap(),
            spatialize_post_effects,
            max_distance: max_distance.unwrap(),
            pan2_d: pan2_d.unwrap(),
            rolloff_mode: rolloff_mode.unwrap(),
            bypass_effects: bypass_effects.unwrap(),
            rolloff_custom_curve: rolloff_custom_curve.unwrap(),
            bypass_listener_effects,
            pan_level_custom_curve: pan_level_custom_curve.unwrap(),
            bypass_reverb_zones,
            spread_custom_curve: spread_custom_curve.unwrap(),
            reverb_zone_mix_custom_curve,
        })
    }
}

impl crate::ToValue for AudioSource {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_GameObject".to_string(), self.m_game_object.to_value())),
            Some(("m_Enabled".to_string(), self.m_enabled.to_value())),
            Some(("m_audioClip".to_string(), self.m_audio_clip.to_value())),
            crate::opt_field("OutputAudioMixerGroup", &self.output_audio_mixer_group),
            Some(("m_PlayOnAwake".to_string(), self.m_play_on_awake.to_value())),
            Some(("m_Volume".to_string(), self.m_volume.to_value())),
            crate::opt_field("m_Resource", &self.m_resource),
            Some(("m_Pitch".to_string(), self.m_pitch.to_value())),
            Some(("Loop".to_string(), self.r#loop.to_value())),
            Some(("Mute".to_string(), self.mute.to_value())),
            Some(("Priority".to_string(), self.priority.to_value())),
            Some(("DopplerLevel".to_string(), self.doppler_level.to_value())),
            crate::opt_field("Spatialize", &self.spatialize),
            Some(("MinDistance".to_string(), self.min_distance.to_value())),
            crate::opt_field("SpatializePostEffects", &self.spatialize_post_effects),
            Some(("MaxDistance".to_string(), self.max_distance.to_value())),
            Some(("Pan2D".to_string(), self.pan2_d.to_value())),
            Some(("rolloffMode".to_string(), self.rolloff_mode.to_value())),
            Some(("BypassEffects".to_string(), self.bypass_effects.to_value())),
            Some((
                "rolloffCustomCurve".to_string(),
                self.rolloff_custom_curve.to_value(),
            )),
            crate::opt_field("BypassListenerEffects", &self.bypass_listener_effects),
            Some((
                "panLevelCustomCurve".to_string(),
                self.pan_level_custom_curve.to_value(),
            )),
            crate::opt_field("BypassReverbZones", &self.bypass_reverb_zones),
            Some((
                "spreadCustomCurve".to_string(),
                self.spread_custom_curve.to_value(),
            )),
            crate::opt_field(
                "reverbZoneMixCustomCurve",
                &self.reverb_zone_mix_custom_curve,
            ),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_GameObject" => (self.m_game_object).inspect_value(path, offset, limit),
                "m_Enabled" => (self.m_enabled).inspect_value(path, offset, limit),
                "m_audioClip" => (self.m_audio_clip).inspect_value(path, offset, limit),
                "OutputAudioMixerGroup" => {
                    (self.output_audio_mixer_group).inspect_value(path, offset, limit)
                }
                "m_PlayOnAwake" => (self.m_play_on_awake).inspect_value(path, offset, limit),
                "m_Volume" => (self.m_volume).inspect_value(path, offset, limit),
                "m_Resource" => (self.m_resource).inspect_value(path, offset, limit),
                "m_Pitch" => (self.m_pitch).inspect_value(path, offset, limit),
                "Loop" => (self.r#loop).inspect_value(path, offset, limit),
                "Mute" => (self.mute).inspect_value(path, offset, limit),
                "Priority" => (self.priority).inspect_value(path, offset, limit),
                "DopplerLevel" => (self.doppler_level).inspect_value(path, offset, limit),
                "Spatialize" => (self.spatialize).inspect_value(path, offset, limit),
                "MinDistance" => (self.min_distance).inspect_value(path, offset, limit),
                "SpatializePostEffects" => {
                    (self.spatialize_post_effects).inspect_value(path, offset, limit)
                }
                "MaxDistance" => (self.max_distance).inspect_value(path, offset, limit),
                "Pan2D" => (self.pan2_d).inspect_value(path, offset, limit),
                "rolloffMode" => (self.rolloff_mode).inspect_value(path, offset, limit),
                "BypassEffects" => (self.bypass_effects).inspect_value(path, offset, limit),
                "rolloffCustomCurve" => {
                    (self.rolloff_custom_curve).inspect_value(path, offset, limit)
                }
                "BypassListenerEffects" => {
                    (self.bypass_listener_effects).inspect_value(path, offset, limit)
                }
                "panLevelCustomCurve" => {
                    (self.pan_level_custom_curve).inspect_value(path, offset, limit)
                }
                "BypassReverbZones" => {
                    (self.bypass_reverb_zones).inspect_value(path, offset, limit)
                }
                "spreadCustomCurve" => {
                    (self.spread_custom_curve).inspect_value(path, offset, limit)
                }
                "reverbZoneMixCustomCurve" => {
                    (self.reverb_zone_mix_custom_curve).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_game_object).inspect_value(path, offset, limit))
                .map(|v| ("m_GameObject".to_string(), v)),
            ((self.m_enabled).inspect_value(path, offset, limit))
                .map(|v| ("m_Enabled".to_string(), v)),
            ((self.m_audio_clip).inspect_value(path, offset, limit))
                .map(|v| ("m_audioClip".to_string(), v)),
            ((self.output_audio_mixer_group).inspect_value(path, offset, limit))
                .map(|v| ("OutputAudioMixerGroup".to_string(), v)),
            ((self.m_play_on_awake).inspect_value(path, offset, limit))
                .map(|v| ("m_PlayOnAwake".to_string(), v)),
            ((self.m_volume).inspect_value(path, offset, limit))
                .map(|v| ("m_Volume".to_string(), v)),
            ((self.m_resource).inspect_value(path, offset, limit))
                .map(|v| ("m_Resource".to_string(), v)),
            ((self.m_pitch).inspect_value(path, offset, limit)).map(|v| ("m_Pitch".to_string(), v)),
            ((self.r#loop).inspect_value(path, offset, limit)).map(|v| ("Loop".to_string(), v)),
            ((self.mute).inspect_value(path, offset, limit)).map(|v| ("Mute".to_string(), v)),
            ((self.priority).inspect_value(path, offset, limit))
                .map(|v| ("Priority".to_string(), v)),
            ((self.doppler_level).inspect_value(path, offset, limit))
                .map(|v| ("DopplerLevel".to_string(), v)),
            ((self.spatialize).inspect_value(path, offset, limit))
                .map(|v| ("Spatialize".to_string(), v)),
            ((self.min_distance).inspect_value(path, offset, limit))
                .map(|v| ("MinDistance".to_string(), v)),
            ((self.spatialize_post_effects).inspect_value(path, offset, limit))
                .map(|v| ("SpatializePostEffects".to_string(), v)),
            ((self.max_distance).inspect_value(path, offset, limit))
                .map(|v| ("MaxDistance".to_string(), v)),
            ((self.pan2_d).inspect_value(path, offset, limit)).map(|v| ("Pan2D".to_string(), v)),
            ((self.rolloff_mode).inspect_value(path, offset, limit))
                .map(|v| ("rolloffMode".to_string(), v)),
            ((self.bypass_effects).inspect_value(path, offset, limit))
                .map(|v| ("BypassEffects".to_string(), v)),
            ((self.rolloff_custom_curve).inspect_value(path, offset, limit))
                .map(|v| ("rolloffCustomCurve".to_string(), v)),
            ((self.bypass_listener_effects).inspect_value(path, offset, limit))
                .map(|v| ("BypassListenerEffects".to_string(), v)),
            ((self.pan_level_custom_curve).inspect_value(path, offset, limit))
                .map(|v| ("panLevelCustomCurve".to_string(), v)),
            ((self.bypass_reverb_zones).inspect_value(path, offset, limit))
                .map(|v| ("BypassReverbZones".to_string(), v)),
            ((self.spread_custom_curve).inspect_value(path, offset, limit))
                .map(|v| ("spreadCustomCurve".to_string(), v)),
            ((self.reverb_zone_mix_custom_curve).inspect_value(path, offset, limit))
                .map(|v| ("reverbZoneMixCustomCurve".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
