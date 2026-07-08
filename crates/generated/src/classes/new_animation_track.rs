//! 自动生成。请勿手动修改。

use crate::subclasses::channel::Channel;

#[derive(Debug, Clone)]
pub struct NewAnimationTrack {
    pub m_name: String,
    pub m_curves: Vec<Channel>,
    pub m_class_id: i32,
}

impl crate::Parse for NewAnimationTrack {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        rdr.align(4)?;
        let m_curves = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { Channel::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;
        let m_class_id = rdr.read_i32()?;

        Ok(NewAnimationTrack {
            m_name,
            m_curves,
            m_class_id,
        })
    }
}
