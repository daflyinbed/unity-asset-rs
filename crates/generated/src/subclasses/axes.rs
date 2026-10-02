//! 自动生成。请勿手动修改。

use crate::subclasses::float3::float3;
use crate::subclasses::float4::float4;
use crate::subclasses::limit::Limit;

/// 版本枚举：AxesMSgnVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AxesMSgnVariant {
    Default(Box<float4>),
    V5_4(Box<float3>),
}

impl crate::Parse for AxesMSgnVariant {
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

impl crate::ToValue for AxesMSgnVariant {
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
pub struct Axes {
    pub m_pre_q: float4,
    pub m_post_q: float4,
    pub m_sgn: AxesMSgnVariant,
    pub m_limit: Limit,
    pub m_length: f32,
    pub m_type: u32,
}

impl crate::Parse for Axes {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_pre_q = float4::parse(rdr, ver)?;
        let m_post_q = float4::parse(rdr, ver)?;
        let m_sgn = AxesMSgnVariant::parse(rdr, ver)?;
        let m_limit = Limit::parse(rdr, ver)?;
        let m_length = rdr.read_f32()?;
        let m_type = rdr.read_u32()?;

        Ok(Axes {
            m_pre_q,
            m_post_q,
            m_sgn,
            m_limit,
            m_length,
            m_type,
        })
    }
}

impl crate::ToValue for Axes {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_PreQ".to_string(), self.m_pre_q.to_value())),
            Some(("m_PostQ".to_string(), self.m_post_q.to_value())),
            Some(("m_Sgn".to_string(), self.m_sgn.to_value())),
            Some(("m_Limit".to_string(), self.m_limit.to_value())),
            Some(("m_Length".to_string(), self.m_length.to_value())),
            Some(("m_Type".to_string(), self.m_type.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_PreQ" => (self.m_pre_q).inspect_value(path, offset, limit),
                "m_PostQ" => (self.m_post_q).inspect_value(path, offset, limit),
                "m_Sgn" => (self.m_sgn).inspect_value(path, offset, limit),
                "m_Limit" => (self.m_limit).inspect_value(path, offset, limit),
                "m_Length" => (self.m_length).inspect_value(path, offset, limit),
                "m_Type" => (self.m_type).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_pre_q).inspect_value(path, offset, limit)).map(|v| ("m_PreQ".to_string(), v)),
            ((self.m_post_q).inspect_value(path, offset, limit))
                .map(|v| ("m_PostQ".to_string(), v)),
            ((self.m_sgn).inspect_value(path, offset, limit)).map(|v| ("m_Sgn".to_string(), v)),
            ((self.m_limit).inspect_value(path, offset, limit)).map(|v| ("m_Limit".to_string(), v)),
            ((self.m_length).inspect_value(path, offset, limit))
                .map(|v| ("m_Length".to_string(), v)),
            ((self.m_type).inspect_value(path, offset, limit)).map(|v| ("m_Type".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
