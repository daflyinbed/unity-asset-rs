//! 自动生成。请勿手动修改。

use crate::subclasses::serialized_shader_float_value::SerializedShaderFloatValue;

#[derive(Debug, Clone)]
pub struct SerializedStencilOp {
    pub pass: SerializedShaderFloatValue,
    pub fail: SerializedShaderFloatValue,
    pub z_fail: SerializedShaderFloatValue,
    pub comp: SerializedShaderFloatValue,
}

impl crate::Parse for SerializedStencilOp {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let pass = SerializedShaderFloatValue::parse(r, ver)?;
        r.align(4)?;
        let fail = SerializedShaderFloatValue::parse(r, ver)?;
        r.align(4)?;
        let z_fail = SerializedShaderFloatValue::parse(r, ver)?;
        r.align(4)?;
        let comp = SerializedShaderFloatValue::parse(r, ver)?;
        r.align(4)?;

        Ok(SerializedStencilOp {
            pass,
            fail,
            z_fail,
            comp,
        })
    }
}
