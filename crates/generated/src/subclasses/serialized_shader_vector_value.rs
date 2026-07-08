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
            rdr.align(4)?;
            Ok(Self::V5_6(v))
        } else {
            let v = ( { Ok(Box::new(FastPropertyName::parse(rdr, ver)?)) } )?;
            rdr.align(4)?;
            Ok(Self::Default(v))
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
        rdr.align(4)?;
        let y = SerializedShaderFloatValue::parse(rdr, ver)?;
        rdr.align(4)?;
        let z = SerializedShaderFloatValue::parse(rdr, ver)?;
        rdr.align(4)?;
        let w = SerializedShaderFloatValue::parse(rdr, ver)?;
        rdr.align(4)?;
        let name = SerializedShaderVectorValueNameVariant::parse(rdr, ver)?;

        Ok(SerializedShaderVectorValue {
            x,
            y,
            z,
            w,
            name,
        })
    }
}
