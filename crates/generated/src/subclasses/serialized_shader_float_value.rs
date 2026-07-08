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
            let v = ( { Ok(Box::new(FastPropertyName::parse(rdr, ver)?)) } )?;
            Ok(Self::Default(v))
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

        Ok(SerializedShaderFloatValue {
            val,
            name,
        })
    }
}
