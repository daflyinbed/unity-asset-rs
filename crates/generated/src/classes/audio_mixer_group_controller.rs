//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::guid::Guid;

#[derive(Debug, Clone)]
pub struct AudioMixerGroupController {
    pub m_name: String,
    pub m_audio_mixer: PPtr,
    pub m_group_id: Guid,
    pub m_children: Vec<PPtr>,
}

impl crate::Parse for AudioMixerGroupController {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        let m_audio_mixer = crate::PPtr::parse(rdr, ver)?;
        let m_group_id = Guid::parse(rdr, ver)?;
        let m_children = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;

        Ok(AudioMixerGroupController {
            m_name,
            m_audio_mixer,
            m_group_id,
            m_children,
        })
    }
}
