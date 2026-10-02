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
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| Keyframe::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| Keyframe::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for AnimationCurveMCurveVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for AnimationCurve {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Curve".to_string(), self.m_curve.to_value())),
            Some(("m_PreInfinity".to_string(), self.m_pre_infinity.to_value())),
            Some((
                "m_PostInfinity".to_string(),
                self.m_post_infinity.to_value(),
            )),
            crate::opt_field("m_RotationOrder", &self.m_rotation_order),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Curve" => (self.m_curve).inspect_value(path, offset, limit),
                "m_PreInfinity" => (self.m_pre_infinity).inspect_value(path, offset, limit),
                "m_PostInfinity" => (self.m_post_infinity).inspect_value(path, offset, limit),
                "m_RotationOrder" => (self.m_rotation_order).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_curve).inspect_value(path, offset, limit)).map(|v| ("m_Curve".to_string(), v)),
            ((self.m_pre_infinity).inspect_value(path, offset, limit))
                .map(|v| ("m_PreInfinity".to_string(), v)),
            ((self.m_post_infinity).inspect_value(path, offset, limit))
                .map(|v| ("m_PostInfinity".to_string(), v)),
            ((self.m_rotation_order).inspect_value(path, offset, limit))
                .map(|v| ("m_RotationOrder".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
