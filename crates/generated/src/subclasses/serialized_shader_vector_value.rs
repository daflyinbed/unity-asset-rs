//! 自动生成。请勿手动修改。

use crate::subclasses::fast_property_name::FastPropertyName;
use crate::subclasses::serialized_shader_float_value::SerializedShaderFloatValue;

#[derive(Debug, Clone)]
pub struct SerializedShaderVectorValue {
    pub x: SerializedShaderFloatValue,
    pub y: SerializedShaderFloatValue,
    pub z: SerializedShaderFloatValue,
    pub w: SerializedShaderFloatValue,
    pub name: FastPropertyName,
}

impl crate::Parse for SerializedShaderVectorValue {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let x = SerializedShaderFloatValue::parse(r, ver)?;
        r.align(4)?;
        let y = SerializedShaderFloatValue::parse(r, ver)?;
        r.align(4)?;
        let z = SerializedShaderFloatValue::parse(r, ver)?;
        r.align(4)?;
        let w = SerializedShaderFloatValue::parse(r, ver)?;
        r.align(4)?;
        let name = FastPropertyName::parse(r, ver)?;
        r.align(4)?;

        Ok(SerializedShaderVectorValue {
            x,
            y,
            z,
            w,
            name,
        })
    }
}
