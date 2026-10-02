//! 自动生成。请勿手动修改。

use crate::subclasses::float3::float3;
use crate::subclasses::float4::float4;

/// 版本枚举：LimitMMinVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum LimitMMinVariant {
    Default(Box<float4>),
    V5_4(Box<float3>),
}

impl crate::Parse for LimitMMinVariant {
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

impl crate::ToValue for LimitMMinVariant {
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

/// 版本枚举：LimitMMaxVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum LimitMMaxVariant {
    Default(Box<float4>),
    V5_4(Box<float3>),
}

impl crate::Parse for LimitMMaxVariant {
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

impl crate::ToValue for LimitMMaxVariant {
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
pub struct Limit {
    pub m_min: LimitMMinVariant,
    pub m_max: LimitMMaxVariant,
}

impl crate::Parse for Limit {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_min = LimitMMinVariant::parse(rdr, ver)?;
        let m_max = LimitMMaxVariant::parse(rdr, ver)?;

        Ok(Limit { m_min, m_max })
    }
}

impl crate::ToValue for Limit {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Min".to_string(), self.m_min.to_value())),
            Some(("m_Max".to_string(), self.m_max.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Min" => (self.m_min).inspect_value(path, offset, limit),
                "m_Max" => (self.m_max).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_min).inspect_value(path, offset, limit)).map(|v| ("m_Min".to_string(), v)),
            ((self.m_max).inspect_value(path, offset, limit)).map(|v| ("m_Max".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
