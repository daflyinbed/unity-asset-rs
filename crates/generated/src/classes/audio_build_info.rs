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
