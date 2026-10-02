//! 自动生成。请勿手动修改。

use crate::PPtr;

/// 版本枚举：AudioEchoFilterMDelayVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AudioEchoFilterMDelayVariant {
    Default(u32),
    V2022_2(f32),
}

impl crate::Parse for AudioEchoFilterMDelayVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            let v = rdr.read_f32()?;
            Ok(Self::V2022_2(v))
        } else {
            let v = rdr.read_u32()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for AudioEchoFilterMDelayVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2022_2(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2022_2(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct AudioEchoFilter {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_delay: AudioEchoFilterMDelayVariant,
    pub m_decay_ratio: f32,
    pub m_wet_mix: f32,
    pub m_dry_mix: f32,
}

impl crate::Parse for AudioEchoFilter {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(rdr, ver)?;
        let m_enabled = rdr.read_u8()?;
        rdr.align(4)?;
        let m_delay = AudioEchoFilterMDelayVariant::parse(rdr, ver)?;
        let m_decay_ratio = rdr.read_f32()?;
        let m_wet_mix = rdr.read_f32()?;
        let m_dry_mix = rdr.read_f32()?;

        Ok(AudioEchoFilter {
            m_game_object,
            m_enabled,
            m_delay,
            m_decay_ratio,
            m_wet_mix,
            m_dry_mix,
        })
    }
}

impl crate::ToValue for AudioEchoFilter {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_GameObject".to_string(), self.m_game_object.to_value())),
            Some(("m_Enabled".to_string(), self.m_enabled.to_value())),
            Some(("m_Delay".to_string(), self.m_delay.to_value())),
            Some(("m_DecayRatio".to_string(), self.m_decay_ratio.to_value())),
            Some(("m_WetMix".to_string(), self.m_wet_mix.to_value())),
            Some(("m_DryMix".to_string(), self.m_dry_mix.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_GameObject" => (self.m_game_object).inspect_value(path, offset, limit),
                "m_Enabled" => (self.m_enabled).inspect_value(path, offset, limit),
                "m_Delay" => (self.m_delay).inspect_value(path, offset, limit),
                "m_DecayRatio" => (self.m_decay_ratio).inspect_value(path, offset, limit),
                "m_WetMix" => (self.m_wet_mix).inspect_value(path, offset, limit),
                "m_DryMix" => (self.m_dry_mix).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_game_object).inspect_value(path, offset, limit))
                .map(|v| ("m_GameObject".to_string(), v)),
            ((self.m_enabled).inspect_value(path, offset, limit))
                .map(|v| ("m_Enabled".to_string(), v)),
            ((self.m_delay).inspect_value(path, offset, limit)).map(|v| ("m_Delay".to_string(), v)),
            ((self.m_decay_ratio).inspect_value(path, offset, limit))
                .map(|v| ("m_DecayRatio".to_string(), v)),
            ((self.m_wet_mix).inspect_value(path, offset, limit))
                .map(|v| ("m_WetMix".to_string(), v)),
            ((self.m_dry_mix).inspect_value(path, offset, limit))
                .map(|v| ("m_DryMix".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
