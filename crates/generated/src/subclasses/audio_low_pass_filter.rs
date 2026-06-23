//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::animation_curve::AnimationCurve;

#[derive(Debug, Clone)]
pub struct AudioLowPassFilter {
    pub m_object_hide_flags: u32,
    pub m_extension_ptr: PPtr,
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_cutoff_frequency: f32,
    pub m_lowpass_resonance_q: f32,
    pub lowpass_level_custom_curve: AnimationCurve,
}

impl crate::Parse for AudioLowPassFilter {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_extension_ptr = crate::PPtr::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_u8()?;
        r.align(4)?;
        let m_cutoff_frequency = r.read_f32()?;
        let m_lowpass_resonance_q = r.read_f32()?;
        let lowpass_level_custom_curve = AnimationCurve::parse(r, ver)?;

        Ok(AudioLowPassFilter {
            m_object_hide_flags,
            m_extension_ptr,
            m_game_object,
            m_enabled,
            m_cutoff_frequency,
            m_lowpass_resonance_q,
            lowpass_level_custom_curve,
        })
    }
}
