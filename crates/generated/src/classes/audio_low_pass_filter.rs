//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::animation_curve::AnimationCurve;


/// 版本枚举：AudioLowPassFilterLowpassLevelCustomCurveVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AudioLowPassFilterLowpassLevelCustomCurveVariant {
    Default(AnimationCurve),
    V2017_1(AnimationCurve),
}

impl crate::Parse for AudioLowPassFilterLowpassLevelCustomCurveVariant {
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
pub struct AudioLowPassFilter {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_cutoff_frequency: Option<f32>,
    pub m_lowpass_resonance_q: f32,
    pub lowpass_level_custom_curve: AudioLowPassFilterLowpassLevelCustomCurveVariant,
}

impl crate::Parse for AudioLowPassFilter {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_u8()?;
        r.align(4)?;
        let m_cutoff_frequency = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_lowpass_resonance_q = r.read_f32()?;
        let lowpass_level_custom_curve = AudioLowPassFilterLowpassLevelCustomCurveVariant::parse(r, ver)?;

        Ok(AudioLowPassFilter {
            m_game_object,
            m_enabled,
            m_cutoff_frequency,
            m_lowpass_resonance_q,
            lowpass_level_custom_curve,
        })
    }
}
