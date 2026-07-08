//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::guid::Guid;
use crate::subclasses::parameter::Parameter;


/// 版本枚举：AudioMixerEffectControllerMParametersVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AudioMixerEffectControllerMParametersVariant {
    Default(Vec<Parameter>),
    V2017_1(Vec<Parameter>),
}

impl crate::Parse for AudioMixerEffectControllerMParametersVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { Parameter::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { Parameter::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct AudioMixerEffectController {
    pub m_name: String,
    pub m_effect_id: Guid,
    pub m_effect_name: String,
    pub m_mix_level: Guid,
    pub m_parameters: AudioMixerEffectControllerMParametersVariant,
    pub m_send_target: PPtr,
    pub m_enable_wet_mix: bool,
    pub m_bypass: bool,
}

impl crate::Parse for AudioMixerEffectController {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        let m_effect_id = Guid::parse(rdr, ver)?;
        let m_effect_name = rdr.read_aligned_string()?;
        let m_mix_level = Guid::parse(rdr, ver)?;
        let m_parameters = AudioMixerEffectControllerMParametersVariant::parse(rdr, ver)?;
        let m_send_target = crate::PPtr::parse(rdr, ver)?;
        let m_enable_wet_mix = rdr.read_bool()?;
        let m_bypass = rdr.read_bool()?;
        rdr.align(4)?;

        Ok(AudioMixerEffectController {
            m_name,
            m_effect_id,
            m_effect_name,
            m_mix_level,
            m_parameters,
            m_send_target,
            m_enable_wet_mix,
            m_bypass,
        })
    }
}
