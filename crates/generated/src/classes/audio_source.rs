//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::animation_curve::AnimationCurve;
use crate::subclasses::extension_property_value::ExtensionPropertyValue;
use super::audio_behaviour::AudioBehaviour;

#[derive(Debug, Clone)]
pub struct AudioSource {
    pub parent: AudioBehaviour,
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_audio_clip: PPtr,
    pub output_audio_mixer_group: Option<PPtr>,
    pub m_play_on_awake: bool,
    pub m_volume: f32,
    pub m_resource: Option<PPtr>,
    pub m_pitch: f32,
    pub r#loop: bool,
    pub mute: bool,
    pub priority: i32,
    pub doppler_level: f32,
    pub spatialize: Option<bool>,
    pub min_distance: f32,
    pub spatialize_post_effects: Option<bool>,
    pub max_distance: f32,
    pub pan2_d: f32,
    pub rolloff_mode: i32,
    pub bypass_effects: bool,
    pub rolloff_custom_curve: AnimationCurve,
    pub bypass_listener_effects: Option<bool>,
    pub pan_level_custom_curve: AnimationCurve,
    pub bypass_reverb_zones: Option<bool>,
    pub spread_custom_curve: AnimationCurve,
    pub reverb_zone_mix_custom_curve: Option<AnimationCurve>,
    pub m_extension_property_values: Option<Vec<ExtensionPropertyValue>>,
}

impl crate::Parse for AudioSource {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = AudioBehaviour::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_u8()?;
        r.align(4)?;
        let m_audio_clip = crate::PPtr::parse(r, ver)?;
        let output_audio_mixer_group = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(crate::PPtr::parse(r, ver)?)
        } else {
            None
        };
        let m_play_on_awake = r.read_bool()?;
        r.align(4)?;
        let m_volume = r.read_f32()?;
        let m_resource = if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            Some(crate::PPtr::parse(r, ver)?)
        } else {
            None
        };
        let m_pitch = r.read_f32()?;
        let r#loop = r.read_bool()?;
        let mute = r.read_bool()?;
        r.align(4)?;
        let priority = r.read_i32()?;
        let doppler_level = r.read_f32()?;
        let spatialize = if *ver >= crate::UnityVersion::new(5, 2, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let min_distance = r.read_f32()?;
        let spatialize_post_effects = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let max_distance = r.read_f32()?;
        let pan2_d = r.read_f32()?;
        let rolloff_mode = r.read_i32()?;
        let bypass_effects = r.read_bool()?;
        r.align(4)?;
        let rolloff_custom_curve = AnimationCurve::parse(r, ver)?;
        let bypass_listener_effects = if *ver >= crate::UnityVersion::new(4, 2, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let pan_level_custom_curve = AnimationCurve::parse(r, ver)?;
        let bypass_reverb_zones = if *ver >= crate::UnityVersion::new(4, 2, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let spread_custom_curve = AnimationCurve::parse(r, ver)?;
        let reverb_zone_mix_custom_curve = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(AnimationCurve::parse(r, ver)?)
        } else {
            None
        };
        let m_extension_property_values = if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ExtensionPropertyValue::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(AudioSource {
            parent,
            m_game_object,
            m_enabled,
            m_audio_clip,
            output_audio_mixer_group,
            m_play_on_awake,
            m_volume,
            m_resource,
            m_pitch,
            r#loop,
            mute,
            priority,
            doppler_level,
            spatialize,
            min_distance,
            spatialize_post_effects,
            max_distance,
            pan2_d,
            rolloff_mode,
            bypass_effects,
            rolloff_custom_curve,
            bypass_listener_effects,
            pan_level_custom_curve,
            bypass_reverb_zones,
            spread_custom_curve,
            reverb_zone_mix_custom_curve,
            m_extension_property_values,
        })
    }
}
