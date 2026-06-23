//! 自动生成。请勿手动修改。

use crate::PPtr;
use super::audio_filter::AudioFilter;

#[derive(Debug, Clone)]
pub struct AudioEchoFilter {
    pub parent: AudioFilter,
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_delay: u32,
    pub m_decay_ratio: f32,
    pub m_wet_mix: f32,
    pub m_dry_mix: f32,
}

impl crate::Parse for AudioEchoFilter {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = AudioFilter::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_u8()?;
        r.align(4)?;
        let m_delay = r.read_u32()?;
        let m_decay_ratio = r.read_f32()?;
        let m_wet_mix = r.read_f32()?;
        let m_dry_mix = r.read_f32()?;

        Ok(AudioEchoFilter {
            parent,
            m_game_object,
            m_enabled,
            m_delay,
            m_decay_ratio,
            m_wet_mix,
            m_dry_mix,
        })
    }
}
