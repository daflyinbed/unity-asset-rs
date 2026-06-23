//! 自动生成。请勿手动修改。

use super::object::Object;

#[derive(Debug, Clone)]
pub struct AudioBuildInfo {
    pub parent: Object,
    pub m_is_audio_disabled: bool,
    pub m_audio_clip_count: i32,
    pub m_audio_mixer_count: i32,
}

impl crate::Parse for AudioBuildInfo {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = Object::parse(r, ver)?;
        let m_is_audio_disabled = r.read_bool()?;
        r.align(4)?;
        let m_audio_clip_count = r.read_i32()?;
        let m_audio_mixer_count = r.read_i32()?;

        Ok(AudioBuildInfo {
            parent,
            m_is_audio_disabled,
            m_audio_clip_count,
            m_audio_mixer_count,
        })
    }
}
