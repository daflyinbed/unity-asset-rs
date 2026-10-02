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
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| Parameter::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| Parameter::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for AudioMixerEffectControllerMParametersVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for AudioMixerEffectController {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some(("m_EffectID".to_string(), self.m_effect_id.to_value())),
            Some(("m_EffectName".to_string(), self.m_effect_name.to_value())),
            Some(("m_MixLevel".to_string(), self.m_mix_level.to_value())),
            Some(("m_Parameters".to_string(), self.m_parameters.to_value())),
            Some(("m_SendTarget".to_string(), self.m_send_target.to_value())),
            Some((
                "m_EnableWetMix".to_string(),
                self.m_enable_wet_mix.to_value(),
            )),
            Some(("m_Bypass".to_string(), self.m_bypass.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Name" => (self.m_name).inspect_value(path, offset, limit),
                "m_EffectID" => (self.m_effect_id).inspect_value(path, offset, limit),
                "m_EffectName" => (self.m_effect_name).inspect_value(path, offset, limit),
                "m_MixLevel" => (self.m_mix_level).inspect_value(path, offset, limit),
                "m_Parameters" => (self.m_parameters).inspect_value(path, offset, limit),
                "m_SendTarget" => (self.m_send_target).inspect_value(path, offset, limit),
                "m_EnableWetMix" => (self.m_enable_wet_mix).inspect_value(path, offset, limit),
                "m_Bypass" => (self.m_bypass).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_effect_id).inspect_value(path, offset, limit))
                .map(|v| ("m_EffectID".to_string(), v)),
            ((self.m_effect_name).inspect_value(path, offset, limit))
                .map(|v| ("m_EffectName".to_string(), v)),
            ((self.m_mix_level).inspect_value(path, offset, limit))
                .map(|v| ("m_MixLevel".to_string(), v)),
            ((self.m_parameters).inspect_value(path, offset, limit))
                .map(|v| ("m_Parameters".to_string(), v)),
            ((self.m_send_target).inspect_value(path, offset, limit))
                .map(|v| ("m_SendTarget".to_string(), v)),
            ((self.m_enable_wet_mix).inspect_value(path, offset, limit))
                .map(|v| ("m_EnableWetMix".to_string(), v)),
            ((self.m_bypass).inspect_value(path, offset, limit))
                .map(|v| ("m_Bypass".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
