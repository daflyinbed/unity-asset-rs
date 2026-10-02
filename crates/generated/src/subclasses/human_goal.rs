//! 自动生成。请勿手动修改。

use crate::subclasses::float3::float3;
use crate::subclasses::float4::float4;
use crate::subclasses::xform::xform;

/// 版本枚举：HumanGoalMHintTVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum HumanGoalMHintTVariant {
    Default(Box<float4>),
    V5_4(Box<float3>),
}

impl crate::Parse for HumanGoalMHintTVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = ({ Ok(Box::new(float3::parse(rdr, ver)?)) })?;
            Ok(Self::V5_4(v))
        } else {
            let v = ({ Ok(Box::new(float4::parse(rdr, ver)?)) })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for HumanGoalMHintTVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_4(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_4(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct HumanGoal {
    pub m_x: xform,
    pub m_weight_t: f32,
    pub m_weight_r: f32,
    pub m_hint_t: Option<HumanGoalMHintTVariant>,
    pub m_hint_weight_t: Option<f32>,
}

impl crate::Parse for HumanGoal {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_x = xform::parse(rdr, ver)?;
        let m_weight_t = rdr.read_f32()?;
        let m_weight_r = rdr.read_f32()?;
        let m_hint_t = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(HumanGoalMHintTVariant::parse(rdr, ver)?)
        } else {
            None
        };
        let m_hint_weight_t = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };

        Ok(HumanGoal {
            m_x,
            m_weight_t,
            m_weight_r,
            m_hint_t,
            m_hint_weight_t,
        })
    }
}

impl crate::ToValue for HumanGoal {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_X".to_string(), self.m_x.to_value())),
            Some(("m_WeightT".to_string(), self.m_weight_t.to_value())),
            Some(("m_WeightR".to_string(), self.m_weight_r.to_value())),
            crate::opt_field("m_HintT", &self.m_hint_t),
            crate::opt_field("m_HintWeightT", &self.m_hint_weight_t),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_X" => (self.m_x).inspect_value(path, offset, limit),
                "m_WeightT" => (self.m_weight_t).inspect_value(path, offset, limit),
                "m_WeightR" => (self.m_weight_r).inspect_value(path, offset, limit),
                "m_HintT" => (self.m_hint_t).inspect_value(path, offset, limit),
                "m_HintWeightT" => (self.m_hint_weight_t).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_x).inspect_value(path, offset, limit)).map(|v| ("m_X".to_string(), v)),
            ((self.m_weight_t).inspect_value(path, offset, limit))
                .map(|v| ("m_WeightT".to_string(), v)),
            ((self.m_weight_r).inspect_value(path, offset, limit))
                .map(|v| ("m_WeightR".to_string(), v)),
            ((self.m_hint_t).inspect_value(path, offset, limit))
                .map(|v| ("m_HintT".to_string(), v)),
            ((self.m_hint_weight_t).inspect_value(path, offset, limit))
                .map(|v| ("m_HintWeightT".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
