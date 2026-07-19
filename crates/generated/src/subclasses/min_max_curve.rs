//! 自动生成。请勿手动修改。

use crate::subclasses::animation_curve::AnimationCurve;

/// 版本枚举：MinMaxCurveMinMaxStateVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum MinMaxCurveMinMaxStateVariant {
    Default(i16),
    V5_6_1(u16),
}

impl crate::Parse for MinMaxCurveMinMaxStateVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 6, 1) {
            let v = rdr.read_u16()?;
            rdr.align(4)?;
            Ok(Self::V5_6_1(v))
        } else {
            let v = rdr.read_i16()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct MinMaxCurve {
    pub scalar: f32,
    pub min_max_state: MinMaxCurveMinMaxStateVariant,
    pub max_curve: AnimationCurve,
    pub min_curve: AnimationCurve,
    pub min_scalar: Option<f32>,
}

impl crate::Parse for MinMaxCurve {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut scalar = None;
        let mut min_max_state = None;
        let mut max_curve = None;
        let mut min_curve = None;
        let mut min_scalar = None;
        if *ver >= crate::UnityVersion::new(5, 6, 0) {
            min_max_state = Some(MinMaxCurveMinMaxStateVariant::parse(rdr, ver)?);
        } else {
        }
        scalar = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(5, 6, 1) {
            min_scalar = Some(rdr.read_f32()?);
        } else {
        }
        max_curve = Some(AnimationCurve::parse(rdr, ver)?);
        min_curve = Some(AnimationCurve::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 6, 0) {
        } else {
            min_max_state = Some(MinMaxCurveMinMaxStateVariant::parse(rdr, ver)?);
        }

        Ok(MinMaxCurve {
            scalar: scalar.unwrap(),
            min_max_state: min_max_state.unwrap(),
            max_curve: max_curve.unwrap(),
            min_curve: min_curve.unwrap(),
            min_scalar,
        })
    }
}
