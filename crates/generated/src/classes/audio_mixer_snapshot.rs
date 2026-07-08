//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::guid::Guid;

#[derive(Debug, Clone)]
pub struct AudioMixerSnapshot {
    pub m_name: String,
    pub m_audio_mixer: PPtr,
    pub m_snapshot_id: Guid,
}

impl crate::Parse for AudioMixerSnapshot {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        let m_audio_mixer = crate::PPtr::parse(rdr, ver)?;
        let m_snapshot_id = Guid::parse(rdr, ver)?;

        Ok(AudioMixerSnapshot {
            m_name,
            m_audio_mixer,
            m_snapshot_id,
        })
    }
}
