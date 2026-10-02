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

impl crate::ToValue for MinMaxCurveMinMaxStateVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_6_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_6_1(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for MinMaxCurve {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("scalar".to_string(), self.scalar.to_value())),
            Some(("minMaxState".to_string(), self.min_max_state.to_value())),
            Some(("maxCurve".to_string(), self.max_curve.to_value())),
            Some(("minCurve".to_string(), self.min_curve.to_value())),
            crate::opt_field("minScalar", &self.min_scalar),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "scalar" => (self.scalar).inspect_value(path, offset, limit),
                "minMaxState" => (self.min_max_state).inspect_value(path, offset, limit),
                "maxCurve" => (self.max_curve).inspect_value(path, offset, limit),
                "minCurve" => (self.min_curve).inspect_value(path, offset, limit),
                "minScalar" => (self.min_scalar).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.scalar).inspect_value(path, offset, limit)).map(|v| ("scalar".to_string(), v)),
            ((self.min_max_state).inspect_value(path, offset, limit))
                .map(|v| ("minMaxState".to_string(), v)),
            ((self.max_curve).inspect_value(path, offset, limit))
                .map(|v| ("maxCurve".to_string(), v)),
            ((self.min_curve).inspect_value(path, offset, limit))
                .map(|v| ("minCurve".to_string(), v)),
            ((self.min_scalar).inspect_value(path, offset, limit))
                .map(|v| ("minScalar".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
