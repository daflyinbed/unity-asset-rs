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
