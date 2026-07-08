//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::animation_curve::AnimationCurve;


/// 版本枚举：FloatCurveCurveVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum FloatCurveCurveVariant {
    Default(Box<AnimationCurve>),
    V2017_1(Box<AnimationCurve>),
}

impl crate::Parse for FloatCurveCurveVariant {
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
pub struct FloatCurve {
    pub curve: FloatCurveCurveVariant,
    pub attribute: String,
    pub path: String,
    pub class_id: i32,
    pub script: PPtr,
    pub flags: Option<i32>,
}

impl crate::Parse for FloatCurve {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let curve = FloatCurveCurveVariant::parse(rdr, ver)?;
        let attribute = rdr.read_aligned_string()?;
        rdr.align(4)?;
        let path = rdr.read_aligned_string()?;
        rdr.align(4)?;
        let class_id = rdr.read_i32()?;
        let script = crate::PPtr::parse(rdr, ver)?;
        let flags = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(rdr.read_i32()?)
        } else {
            None
        };

        Ok(FloatCurve {
            curve,
            attribute,
            path,
            class_id,
            script,
            flags,
        })
    }
}
