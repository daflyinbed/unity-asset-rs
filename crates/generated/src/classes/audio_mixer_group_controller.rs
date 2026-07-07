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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_audio_mixer = crate::PPtr::parse(r, ver)?;
        let m_group_id = Guid::parse(r, ver)?;
        let m_children = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(AudioMixerGroupController {
            m_name,
            m_audio_mixer,
            m_group_id,
            m_children,
        })
    }
}
