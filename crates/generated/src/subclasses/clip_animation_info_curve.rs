//! 自动生成。请勿手动修改。

use crate::subclasses::animation_curve::AnimationCurve;


/// 版本枚举：ClipAnimationInfoCurveCurveVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ClipAnimationInfoCurveCurveVariant {
    Default(Box<AnimationCurve>),
    V2017_1(Box<AnimationCurve>),
}

impl crate::Parse for ClipAnimationInfoCurveCurveVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { Ok(Box::new(AnimationCurve::parse(rdr, ver)?)) } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { Ok(Box::new(AnimationCurve::parse(rdr, ver)?)) } )?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct ClipAnimationInfoCurve {
    pub name: String,
    pub curve: ClipAnimationInfoCurveCurveVariant,
}

impl crate::Parse for ClipAnimationInfoCurve {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let name = rdr.read_aligned_string()?;
        rdr.align(4)?;
        let curve = ClipAnimationInfoCurveCurveVariant::parse(rdr, ver)?;

        Ok(ClipAnimationInfoCurve {
            name,
            curve,
        })
    }
}
