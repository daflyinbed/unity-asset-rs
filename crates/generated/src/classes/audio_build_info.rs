//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct AudioBuildInfo {
    pub m_is_audio_disabled: bool,
    pub m_audio_clip_count: i32,
    pub m_audio_mixer_count: i32,
}

impl crate::Parse for AudioBuildInfo {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_is_audio_disabled = rdr.read_bool()?;
        rdr.align(4)?;
        let m_audio_clip_count = rdr.read_i32()?;
        let m_audio_mixer_count = rdr.read_i32()?;

        Ok(AudioBuildInfo {
            m_is_audio_disabled,
            m_audio_clip_count,
            m_audio_mixer_count,
        })
    }
}

impl crate::ToValue for AudioBuildInfo {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "m_IsAudioDisabled".to_string(),
                self.m_is_audio_disabled.to_value(),
            )),
            Some((
                "m_AudioClipCount".to_string(),
                self.m_audio_clip_count.to_value(),
            )),
            Some((
                "m_AudioMixerCount".to_string(),
                self.m_audio_mixer_count.to_value(),
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
                "m_IsAudioDisabled" => {
                    (self.m_is_audio_disabled).inspect_value(path, offset, limit)
                }
                "m_AudioClipCount" => (self.m_audio_clip_count).inspect_value(path, offset, limit),
                "m_AudioMixerCount" => {
                    (self.m_audio_mixer_count).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_is_audio_disabled).inspect_value(path, offset, limit))
                .map(|v| ("m_IsAudioDisabled".to_string(), v)),
            ((self.m_audio_clip_count).inspect_value(path, offset, limit))
                .map(|v| ("m_AudioClipCount".to_string(), v)),
            ((self.m_audio_mixer_count).inspect_value(path, offset, limit))
                .map(|v| ("m_AudioMixerCount".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
