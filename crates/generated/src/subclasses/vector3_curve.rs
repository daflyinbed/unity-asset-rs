//! 自动生成。请勿手动修改。

use crate::subclasses::animation_curve::AnimationCurve;


/// 版本枚举：Vector3CurveCurveVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum Vector3CurveCurveVariant {
    Default(Box<AnimationCurve>),
    V2017_1(Box<AnimationCurve>),
}

impl crate::Parse for Vector3CurveCurveVariant {
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
pub struct Vector3Curve {
    pub curve: Vector3CurveCurveVariant,
    pub path: String,
}

impl crate::Parse for Vector3Curve {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let curve = Vector3CurveCurveVariant::parse(rdr, ver)?;
        let path = rdr.read_aligned_string()?;
        rdr.align(4)?;

        Ok(Vector3Curve {
            curve,
            path,
        })
    }
}
