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
        rdr.align(4)?;
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
