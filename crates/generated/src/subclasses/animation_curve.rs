//! 自动生成。请勿手动修改。

use crate::subclasses::keyframe::Keyframe;

#[derive(Debug, Clone)]
pub struct AnimationCurve {
    pub m_curve: Vec<Keyframe>,
    pub m_pre_infinity: i32,
    pub m_post_infinity: i32,
}

impl crate::Parse for AnimationCurve {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_curve = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Keyframe::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_pre_infinity = r.read_i32()?;
        let m_post_infinity = r.read_i32()?;

        Ok(AnimationCurve {
            m_curve,
            m_pre_infinity,
            m_post_infinity,
        })
    }
}
