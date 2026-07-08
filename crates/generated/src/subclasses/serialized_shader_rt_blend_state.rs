//! 自动生成。请勿手动修改。

use crate::subclasses::serialized_shader_float_value::SerializedShaderFloatValue;

#[derive(Debug, Clone)]
pub struct SerializedShaderRtBlendState {
    pub src_blend: SerializedShaderFloatValue,
    pub dest_blend: SerializedShaderFloatValue,
    pub src_blend_alpha: SerializedShaderFloatValue,
    pub dest_blend_alpha: SerializedShaderFloatValue,
    pub blend_op: SerializedShaderFloatValue,
    pub blend_op_alpha: SerializedShaderFloatValue,
    pub col_mask: SerializedShaderFloatValue,
}

impl crate::Parse for SerializedShaderRtBlendState {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let src_blend = SerializedShaderFloatValue::parse(rdr, ver)?;
        let dest_blend = SerializedShaderFloatValue::parse(rdr, ver)?;
        let src_blend_alpha = SerializedShaderFloatValue::parse(rdr, ver)?;
        let dest_blend_alpha = SerializedShaderFloatValue::parse(rdr, ver)?;
        let blend_op = SerializedShaderFloatValue::parse(rdr, ver)?;
        let blend_op_alpha = SerializedShaderFloatValue::parse(rdr, ver)?;
        let col_mask = SerializedShaderFloatValue::parse(rdr, ver)?;

        Ok(SerializedShaderRtBlendState {
            src_blend,
            dest_blend,
            src_blend_alpha,
            dest_blend_alpha,
            blend_op,
            blend_op_alpha,
            col_mask,
        })
    }
}
