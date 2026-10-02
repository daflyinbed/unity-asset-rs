//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct AudioContainerElement {
    pub m_name: String,
    pub m_audio_clip: PPtr,
    pub m_volume: f32,
    pub m_enabled: bool,
}

impl crate::Parse for AudioContainerElement {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        let m_audio_clip = crate::PPtr::parse(rdr, ver)?;
        let m_volume = rdr.read_f32()?;
        let m_enabled = rdr.read_bool()?;

        Ok(AudioContainerElement {
            m_name,
            m_audio_clip,
            m_volume,
            m_enabled,
        })
    }
}

impl crate::ToValue for AudioContainerElement {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some(("m_AudioClip".to_string(), self.m_audio_clip.to_value())),
            Some(("m_Volume".to_string(), self.m_volume.to_value())),
            Some(("m_Enabled".to_string(), self.m_enabled.to_value())),
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
                "m_AudioClip" => (self.m_audio_clip).inspect_value(path, offset, limit),
                "m_Volume" => (self.m_volume).inspect_value(path, offset, limit),
                "m_Enabled" => (self.m_enabled).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_audio_clip).inspect_value(path, offset, limit))
                .map(|v| ("m_AudioClip".to_string(), v)),
            ((self.m_volume).inspect_value(path, offset, limit))
                .map(|v| ("m_Volume".to_string(), v)),
            ((self.m_enabled).inspect_value(path, offset, limit))
                .map(|v| ("m_Enabled".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
