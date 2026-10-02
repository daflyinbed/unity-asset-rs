//! 自动生成。请勿手动修改。

use crate::subclasses::fast_property_name::FastPropertyName;
use crate::subclasses::serialized_shader_float_value::SerializedShaderFloatValue;

/// 版本枚举：SerializedShaderVectorValueNameVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SerializedShaderVectorValueNameVariant {
    Default(Box<FastPropertyName>),
    V5_6(String),
}

impl crate::Parse for SerializedShaderVectorValueNameVariant {
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

impl crate::ToValue for SerializedShaderVectorValueNameVariant {
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
pub struct SerializedShaderVectorValue {
    pub x: SerializedShaderFloatValue,
    pub y: SerializedShaderFloatValue,
    pub z: SerializedShaderFloatValue,
    pub w: SerializedShaderFloatValue,
    pub name: SerializedShaderVectorValueNameVariant,
}

impl crate::Parse for SerializedShaderVectorValue {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let x = SerializedShaderFloatValue::parse(rdr, ver)?;
        let y = SerializedShaderFloatValue::parse(rdr, ver)?;
        let z = SerializedShaderFloatValue::parse(rdr, ver)?;
        let w = SerializedShaderFloatValue::parse(rdr, ver)?;
        let name = SerializedShaderVectorValueNameVariant::parse(rdr, ver)?;

        Ok(SerializedShaderVectorValue { x, y, z, w, name })
    }
}

impl crate::ToValue for SerializedShaderVectorValue {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("x".to_string(), self.x.to_value())),
            Some(("y".to_string(), self.y.to_value())),
            Some(("z".to_string(), self.z.to_value())),
            Some(("w".to_string(), self.w.to_value())),
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
                "x" => (self.x).inspect_value(path, offset, limit),
                "y" => (self.y).inspect_value(path, offset, limit),
                "z" => (self.z).inspect_value(path, offset, limit),
                "w" => (self.w).inspect_value(path, offset, limit),
                "name" => (self.name).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.x).inspect_value(path, offset, limit)).map(|v| ("x".to_string(), v)),
            ((self.y).inspect_value(path, offset, limit)).map(|v| ("y".to_string(), v)),
            ((self.z).inspect_value(path, offset, limit)).map(|v| ("z".to_string(), v)),
            ((self.w).inspect_value(path, offset, limit)).map(|v| ("w".to_string(), v)),
            ((self.name).inspect_value(path, offset, limit)).map(|v| ("name".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
