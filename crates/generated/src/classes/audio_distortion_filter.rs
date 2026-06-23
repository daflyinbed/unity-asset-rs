//! 自动生成。请勿手动修改。

use crate::PPtr;
use super::audio_filter::AudioFilter;

#[derive(Debug, Clone)]
pub struct AudioDistortionFilter {
    pub parent: AudioFilter,
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_distortion_level: f32,
}

impl crate::Parse for AudioDistortionFilter {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = AudioFilter::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_u8()?;
        r.align(4)?;
        let m_distortion_level = r.read_f32()?;

        Ok(AudioDistortionFilter {
            parent,
            m_game_object,
            m_enabled,
            m_distortion_level,
        })
    }
}
