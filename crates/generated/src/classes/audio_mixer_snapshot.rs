//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::guid::Guid;
use super::named_object::NamedObject;

#[derive(Debug, Clone)]
pub struct AudioMixerSnapshot {
    pub parent: NamedObject,
    pub m_name: String,
    pub m_audio_mixer: PPtr,
    pub m_snapshot_id: Guid,
}

impl crate::Parse for AudioMixerSnapshot {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = NamedObject::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_audio_mixer = crate::PPtr::parse(r, ver)?;
        let m_snapshot_id = Guid::parse(r, ver)?;

        Ok(AudioMixerSnapshot {
            parent,
            m_name,
            m_audio_mixer,
            m_snapshot_id,
        })
    }
}
