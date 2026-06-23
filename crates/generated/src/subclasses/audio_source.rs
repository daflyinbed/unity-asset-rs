//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::animation_curve::AnimationCurve;

#[derive(Debug, Clone)]
pub struct AudioSource {
    pub m_object_hide_flags: u32,
    pub m_extension_ptr: PPtr,
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_audio_clip: PPtr,
    pub m_play_on_awake: bool,
    pub m_volume: f32,
    pub m_pitch: f32,
    pub r#loop: bool,
    pub mute: bool,
    pub priority: i32,
    pub doppler_level: f32,
    pub min_distance: f32,
    pub max_distance: f32,
    pub pan2_d: f32,
    pub rolloff_mode: i32,
    pub bypass_effects: bool,
    pub rolloff_custom_curve: AnimationCurve,
    pub pan_level_custom_curve: AnimationCurve,
    pub spread_custom_curve: AnimationCurve,
}

impl crate::Parse for AudioSource {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_extension_ptr = crate::PPtr::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_u8()?;
        r.align(4)?;
        let m_audio_clip = crate::PPtr::parse(r, ver)?;
        let m_play_on_awake = r.read_bool()?;
        r.align(4)?;
        let m_volume = r.read_f32()?;
        let m_pitch = r.read_f32()?;
        let r#loop = r.read_bool()?;
        let mute = r.read_bool()?;
        r.align(4)?;
        let priority = r.read_i32()?;
        let doppler_level = r.read_f32()?;
        let min_distance = r.read_f32()?;
        let max_distance = r.read_f32()?;
        let pan2_d = r.read_f32()?;
        let rolloff_mode = r.read_i32()?;
        let bypass_effects = r.read_bool()?;
        r.align(4)?;
        let rolloff_custom_curve = AnimationCurve::parse(r, ver)?;
        let pan_level_custom_curve = AnimationCurve::parse(r, ver)?;
        let spread_custom_curve = AnimationCurve::parse(r, ver)?;

        Ok(AudioSource {
            m_object_hide_flags,
            m_extension_ptr,
            m_game_object,
            m_enabled,
            m_audio_clip,
            m_play_on_awake,
            m_volume,
            m_pitch,
            r#loop,
            mute,
            priority,
            doppler_level,
            min_distance,
            max_distance,
            pan2_d,
            rolloff_mode,
            bypass_effects,
            rolloff_custom_curve,
            pan_level_custom_curve,
            spread_custom_curve,
        })
    }
}
