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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(6000, 3, 0) {
            let v = crate::PPtr::parse(r, ver)?;
            Ok(Self::V6000_3(v))
        } else {
            let v = crate::PPtr::parse(r, ver)?;
            Ok(Self::Default(v))
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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 2, 0) {
            let v = r.read_bool()?;
            Ok(Self::V5_2(v))
        } else {
            let v = r.read_bool()?;
            r.align(4)?;
            Ok(Self::Default(v))
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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            let v = r.read_bool()?;
            Ok(Self::V5_5(v))
        } else {
            let v = r.read_bool()?;
            r.align(4)?;
            Ok(Self::Default(v))
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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(4, 2, 0) {
            let v = r.read_bool()?;
            Ok(Self::V4_2(v))
        } else {
            let v = r.read_bool()?;
            r.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：AudioSourceRolloffCustomCurveVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AudioSourceRolloffCustomCurveVariant {
    Default(AnimationCurve),
    V2017_1(AnimationCurve),
}

impl crate::Parse for AudioSourceRolloffCustomCurveVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = AnimationCurve::parse(r, ver)?;
            r.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = AnimationCurve::parse(r, ver)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：AudioSourcePanLevelCustomCurveVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AudioSourcePanLevelCustomCurveVariant {
    Default(AnimationCurve),
    V2017_1(AnimationCurve),
}

impl crate::Parse for AudioSourcePanLevelCustomCurveVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = AnimationCurve::parse(r, ver)?;
            r.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = AnimationCurve::parse(r, ver)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：AudioSourceSpreadCustomCurveVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AudioSourceSpreadCustomCurveVariant {
    Default(AnimationCurve),
    V2017_1(AnimationCurve),
}

impl crate::Parse for AudioSourceSpreadCustomCurveVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = AnimationCurve::parse(r, ver)?;
            r.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = AnimationCurve::parse(r, ver)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：AudioSourceReverbZoneMixCustomCurveVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AudioSourceReverbZoneMixCustomCurveVariant {
    Default(AnimationCurve),
    V2017_1(AnimationCurve),
}

impl crate::Parse for AudioSourceReverbZoneMixCustomCurveVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = AnimationCurve::parse(r, ver)?;
            r.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = AnimationCurve::parse(r, ver)?;
            Ok(Self::Default(v))
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
    pub rolloff_custom_curve: AudioSourceRolloffCustomCurveVariant,
    pub bypass_listener_effects: Option<bool>,
    pub pan_level_custom_curve: AudioSourcePanLevelCustomCurveVariant,
    pub bypass_reverb_zones: Option<bool>,
    pub spread_custom_curve: AudioSourceSpreadCustomCurveVariant,
    pub reverb_zone_mix_custom_curve: Option<AudioSourceReverbZoneMixCustomCurveVariant>,
}

impl crate::Parse for AudioSource {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
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
            m_game_object = Some(crate::PPtr::parse(r, ver)?);
            m_enabled = Some(r.read_u8()?);
            r.align(4)?;
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            output_audio_mixer_group = Some(crate::PPtr::parse(r, ver)?);
        } else {
        }
            m_audio_clip = Some(crate::PPtr::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            m_resource = Some(AudioSourceMResourceVariant::parse(r, ver)?);
        } else {
        }
            m_play_on_awake = Some(r.read_bool()?);
            r.align(4)?;
            m_volume = Some(r.read_f32()?);
            m_pitch = Some(r.read_f32()?);
            r#loop = Some(r.read_bool()?);
            mute = Some(AudioSourceMuteVariant::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            spatialize = Some(AudioSourceSpatializeVariant::parse(r, ver)?);
            spatialize_post_effects = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 2, 0) {
            spatialize = Some(AudioSourceSpatializeVariant::parse(r, ver)?);
        } else {
        }
            priority = Some(r.read_i32()?);
            doppler_level = Some(r.read_f32()?);
            min_distance = Some(r.read_f32()?);
            max_distance = Some(r.read_f32()?);
            pan2_d = Some(r.read_f32()?);
            rolloff_mode = Some(r.read_i32()?);
            bypass_effects = Some(AudioSourceBypassEffectsVariant::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(4, 2, 0) {
            bypass_listener_effects = Some(r.read_bool()?);
            bypass_reverb_zones = Some(r.read_bool()?);
            r.align(4)?;
        } else {
        }
            rolloff_custom_curve = Some(AudioSourceRolloffCustomCurveVariant::parse(r, ver)?);
            pan_level_custom_curve = Some(AudioSourcePanLevelCustomCurveVariant::parse(r, ver)?);
            spread_custom_curve = Some(AudioSourceSpreadCustomCurveVariant::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            reverb_zone_mix_custom_curve = Some(AudioSourceReverbZoneMixCustomCurveVariant::parse(r, ver)?);
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
