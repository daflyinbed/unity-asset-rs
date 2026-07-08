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
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let pass = SerializedShaderFloatValue::parse(rdr, ver)?;
        let fail = SerializedShaderFloatValue::parse(rdr, ver)?;
        let z_fail = SerializedShaderFloatValue::parse(rdr, ver)?;
        let comp = SerializedShaderFloatValue::parse(rdr, ver)?;

        Ok(SerializedStencilOp {
            pass,
            fail,
            z_fail,
            comp,
        })
    }
}
