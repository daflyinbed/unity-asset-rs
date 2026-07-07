//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::guid::Guid;
use crate::subclasses::parameter::Parameter;

#[derive(Debug, Clone)]
pub struct AudioMixerEffectController {
    pub m_name: String,
    pub m_effect_id: Guid,
    pub m_effect_name: String,
    pub m_mix_level: Guid,
    pub m_parameters: Vec<Parameter>,
    pub m_send_target: PPtr,
    pub m_enable_wet_mix: bool,
    pub m_bypass: bool,
}

impl crate::Parse for AudioMixerEffectController {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_effect_id = Guid::parse(r, ver)?;
        let m_effect_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_mix_level = Guid::parse(r, ver)?;
        let m_parameters = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Parameter::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_send_target = crate::PPtr::parse(r, ver)?;
        let m_enable_wet_mix = r.read_bool()?;
        let m_bypass = r.read_bool()?;
        r.align(4)?;

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
