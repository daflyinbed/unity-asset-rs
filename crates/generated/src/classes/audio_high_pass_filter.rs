//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct AudioHighPassFilter {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_cutoff_frequency: f32,
    pub m_highpass_resonance_q: f32,
}

impl crate::Parse for AudioHighPassFilter {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(rdr, ver)?;
        let m_enabled = rdr.read_u8()?;
        rdr.align(4)?;
        let m_cutoff_frequency = rdr.read_f32()?;
        let m_highpass_resonance_q = rdr.read_f32()?;

        Ok(AudioHighPassFilter {
            m_game_object,
            m_enabled,
            m_cutoff_frequency,
            m_highpass_resonance_q,
        })
    }
}

impl crate::ToValue for AudioHighPassFilter {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_GameObject".to_string(), self.m_game_object.to_value())),
            Some(("m_Enabled".to_string(), self.m_enabled.to_value())),
            Some((
                "m_CutoffFrequency".to_string(),
                self.m_cutoff_frequency.to_value(),
            )),
            Some((
                "m_HighpassResonanceQ".to_string(),
                self.m_highpass_resonance_q.to_value(),
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
                "m_HighpassResonanceQ" => {
                    (self.m_highpass_resonance_q).inspect_value(path, offset, limit)
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
            ((self.m_highpass_resonance_q).inspect_value(path, offset, limit))
                .map(|v| ("m_HighpassResonanceQ".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
