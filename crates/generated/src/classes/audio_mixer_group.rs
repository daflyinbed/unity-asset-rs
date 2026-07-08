//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::guid::Guid;


/// 版本枚举：AudioMixerGroupMChildrenVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AudioMixerGroupMChildrenVariant {
    Default(Vec<PPtr>),
    V2017_1(Vec<PPtr>),
}

impl crate::Parse for AudioMixerGroupMChildrenVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct AudioMixerGroup {
    pub m_name: String,
    pub m_audio_mixer: PPtr,
    pub m_group_id: Guid,
    pub m_children: AudioMixerGroupMChildrenVariant,
}

impl crate::Parse for AudioMixerGroup {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        rdr.align(4)?;
        let m_audio_mixer = crate::PPtr::parse(rdr, ver)?;
        let m_group_id = Guid::parse(rdr, ver)?;
        let m_children = AudioMixerGroupMChildrenVariant::parse(rdr, ver)?;

        Ok(AudioMixerGroup {
            m_name,
            m_audio_mixer,
            m_group_id,
            m_children,
        })
    }
}
