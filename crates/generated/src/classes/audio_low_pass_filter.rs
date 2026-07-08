//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::animation_curve::AnimationCurve;


/// 版本枚举：AudioLowPassFilterLowpassLevelCustomCurveVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AudioLowPassFilterLowpassLevelCustomCurveVariant {
    Default(Box<AnimationCurve>),
    V2017_1(Box<AnimationCurve>),
}

impl crate::Parse for AudioLowPassFilterLowpassLevelCustomCurveVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { Ok(Box::new(AnimationCurve::parse(rdr, ver)?)) } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { Ok(Box::new(AnimationCurve::parse(rdr, ver)?)) } )?;
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
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(rdr, ver)?;
        let m_enabled = rdr.read_u8()?;
        rdr.align(4)?;
        let m_cutoff_frequency = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };
        let m_lowpass_resonance_q = rdr.read_f32()?;
        let lowpass_level_custom_curve = AudioLowPassFilterLowpassLevelCustomCurveVariant::parse(rdr, ver)?;

        Ok(AudioLowPassFilter {
            m_game_object,
            m_enabled,
            m_cutoff_frequency,
            m_lowpass_resonance_q,
            lowpass_level_custom_curve,
        })
    }
}
