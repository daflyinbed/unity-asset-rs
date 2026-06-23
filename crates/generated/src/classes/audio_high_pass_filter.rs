//! 自动生成。请勿手动修改。

use crate::PPtr;
use super::audio_filter::AudioFilter;

#[derive(Debug, Clone)]
pub struct AudioHighPassFilter {
    pub parent: AudioFilter,
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_cutoff_frequency: f32,
    pub m_highpass_resonance_q: f32,
}

impl crate::Parse for AudioHighPassFilter {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = AudioFilter::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_u8()?;
        r.align(4)?;
        let m_cutoff_frequency = r.read_f32()?;
        let m_highpass_resonance_q = r.read_f32()?;

        Ok(AudioHighPassFilter {
            parent,
            m_game_object,
            m_enabled,
            m_cutoff_frequency,
            m_highpass_resonance_q,
        })
    }
}
