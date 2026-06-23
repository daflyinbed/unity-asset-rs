//! 自动生成。请勿手动修改。

use crate::subclasses::channel::Channel;
use super::base_animation_track::BaseAnimationTrack;

#[derive(Debug, Clone)]
pub struct NewAnimationTrack {
    pub parent: BaseAnimationTrack,
    pub m_name: String,
    pub m_curves: Vec<Channel>,
    pub m_class_id: i32,
}

impl crate::Parse for NewAnimationTrack {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = BaseAnimationTrack::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_curves = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Channel::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_class_id = r.read_i32()?;

        Ok(NewAnimationTrack {
            parent,
            m_name,
            m_curves,
            m_class_id,
        })
    }
}
