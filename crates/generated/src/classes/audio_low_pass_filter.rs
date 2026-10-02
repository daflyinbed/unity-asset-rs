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

impl crate::ToValue for AudioLowPassFilter {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_GameObject".to_string(), self.m_game_object.to_value())),
            Some(("m_Enabled".to_string(), self.m_enabled.to_value())),
            crate::opt_field("m_CutoffFrequency", &self.m_cutoff_frequency),
            Some((
                "m_LowpassResonanceQ".to_string(),
                self.m_lowpass_resonance_q.to_value(),
            )),
            Some((
                "lowpassLevelCustomCurve".to_string(),
                self.lowpass_level_custom_curve.to_value(),
            )),
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
                "m_CutoffFrequency" => (self.m_cutoff_frequency).inspect_value(path, offset, limit),
                "m_LowpassResonanceQ" => {
                    (self.m_lowpass_resonance_q).inspect_value(path, offset, limit)
                }
                "lowpassLevelCustomCurve" => {
                    (self.lowpass_level_custom_curve).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_game_object).inspect_value(path, offset, limit))
                .map(|v| ("m_GameObject".to_string(), v)),
            ((self.m_enabled).inspect_value(path, offset, limit))
                .map(|v| ("m_Enabled".to_string(), v)),
            ((self.m_cutoff_frequency).inspect_value(path, offset, limit))
                .map(|v| ("m_CutoffFrequency".to_string(), v)),
            ((self.m_lowpass_resonance_q).inspect_value(path, offset, limit))
                .map(|v| ("m_LowpassResonanceQ".to_string(), v)),
            ((self.lowpass_level_custom_curve).inspect_value(path, offset, limit))
                .map(|v| ("lowpassLevelCustomCurve".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
