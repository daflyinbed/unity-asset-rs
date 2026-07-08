//! 自动生成。请勿手动修改。

use crate::subclasses::keyframe::Keyframe;


/// 版本枚举：AnimationCurveMCurveVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AnimationCurveMCurveVariant {
    Default(Vec<Keyframe>),
    V2017_1(Vec<Keyframe>),
}

impl crate::Parse for AnimationCurveMCurveVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { Keyframe::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { Keyframe::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct AnimationCurve {
    pub m_curve: AnimationCurveMCurveVariant,
    pub m_pre_infinity: i32,
    pub m_post_infinity: i32,
    pub m_rotation_order: Option<i32>,
}

impl crate::Parse for AnimationCurve {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_curve = AnimationCurveMCurveVariant::parse(rdr, ver)?;
        let m_pre_infinity = rdr.read_i32()?;
        let m_post_infinity = rdr.read_i32()?;
        let m_rotation_order = if *ver >= crate::UnityVersion::new(5, 3, 0) {
            Some(rdr.read_i32()?)
        } else {
            None
        };

        Ok(AnimationCurve {
            m_curve,
            m_pre_infinity,
            m_post_infinity,
            m_rotation_order,
        })
    }
}
