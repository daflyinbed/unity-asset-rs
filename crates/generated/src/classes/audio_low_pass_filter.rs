//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::animation_curve::AnimationCurve;

#[derive(Debug, Clone)]
pub struct AudioLowPassFilter {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_cutoff_frequency: Option<f32>,
    pub m_lowpass_resonance_q: f32,
    pub lowpass_level_custom_curve: AnimationCurve,
}

impl crate::Parse for AudioLowPassFilter {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut m_enabled = None;
        let mut m_cutoff_frequency = None;
        let mut m_lowpass_resonance_q = None;
        let mut lowpass_level_custom_curve = None;
            m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
            m_enabled = Some(rdr.read_u8()?);
            rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(5, 2, 0) {
        } else {
            m_cutoff_frequency = Some(rdr.read_f32()?);
        }
            m_lowpass_resonance_q = Some(rdr.read_f32()?);
            lowpass_level_custom_curve = Some(AnimationCurve::parse(rdr, ver)?);

        Ok(AudioLowPassFilter {
            m_game_object: m_game_object.unwrap(),
            m_enabled: m_enabled.unwrap(),
            m_cutoff_frequency,
            m_lowpass_resonance_q: m_lowpass_resonance_q.unwrap(),
            lowpass_level_custom_curve: lowpass_level_custom_curve.unwrap(),
        })
    }
}
