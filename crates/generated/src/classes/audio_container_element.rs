//! 自动生成。请勿手动修改。

use crate::PPtr;
use super::named_object::NamedObject;

#[derive(Debug, Clone)]
pub struct AudioContainerElement {
    pub parent: NamedObject,
    pub m_name: String,
    pub m_audio_clip: PPtr,
    pub m_volume: f32,
    pub m_enabled: bool,
}

impl crate::Parse for AudioContainerElement {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = NamedObject::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_audio_clip = crate::PPtr::parse(r, ver)?;
        let m_volume = r.read_f32()?;
        let m_enabled = r.read_bool()?;

        Ok(AudioContainerElement {
            parent,
            m_name,
            m_audio_clip,
            m_volume,
            m_enabled,
        })
    }
}
