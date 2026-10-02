//! 自动生成。请勿手动修改。

use crate::subclasses::fast_property_name::FastPropertyName;

/// 版本枚举：SerializedShaderFloatValueNameVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SerializedShaderFloatValueNameVariant {
    Default(Box<FastPropertyName>),
    V5_6(String),
}

impl crate::Parse for SerializedShaderFloatValueNameVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = rdr.read_aligned_string()?;
            Ok(Self::V5_6(v))
        } else {
            let v = ({ Ok(Box::new(FastPropertyName::parse(rdr, ver)?)) })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for SerializedShaderFloatValueNameVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_6(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_6(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct SerializedShaderFloatValue {
    pub val: f32,
    pub name: SerializedShaderFloatValueNameVariant,
}

impl crate::Parse for SerializedShaderFloatValue {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let val = rdr.read_f32()?;
        let name = SerializedShaderFloatValueNameVariant::parse(rdr, ver)?;

        Ok(SerializedShaderFloatValue { val, name })
    }
}

impl crate::ToValue for SerializedShaderFloatValue {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("val".to_string(), self.val.to_value())),
            Some(("name".to_string(), self.name.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "val" => (self.val).inspect_value(path, offset, limit),
                "name" => (self.name).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.val).inspect_value(path, offset, limit)).map(|v| ("val".to_string(), v)),
            ((self.name).inspect_value(path, offset, limit)).map(|v| ("name".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
