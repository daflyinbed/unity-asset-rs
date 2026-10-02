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

impl crate::ToValue for SerializedShaderRtBlendState {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("srcBlend".to_string(), self.src_blend.to_value())),
            Some(("destBlend".to_string(), self.dest_blend.to_value())),
            Some(("srcBlendAlpha".to_string(), self.src_blend_alpha.to_value())),
            Some((
                "destBlendAlpha".to_string(),
                self.dest_blend_alpha.to_value(),
            )),
            Some(("blendOp".to_string(), self.blend_op.to_value())),
            Some(("blendOpAlpha".to_string(), self.blend_op_alpha.to_value())),
            Some(("colMask".to_string(), self.col_mask.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "srcBlend" => (self.src_blend).inspect_value(path, offset, limit),
                "destBlend" => (self.dest_blend).inspect_value(path, offset, limit),
                "srcBlendAlpha" => (self.src_blend_alpha).inspect_value(path, offset, limit),
                "destBlendAlpha" => (self.dest_blend_alpha).inspect_value(path, offset, limit),
                "blendOp" => (self.blend_op).inspect_value(path, offset, limit),
                "blendOpAlpha" => (self.blend_op_alpha).inspect_value(path, offset, limit),
                "colMask" => (self.col_mask).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.src_blend).inspect_value(path, offset, limit))
                .map(|v| ("srcBlend".to_string(), v)),
            ((self.dest_blend).inspect_value(path, offset, limit))
                .map(|v| ("destBlend".to_string(), v)),
            ((self.src_blend_alpha).inspect_value(path, offset, limit))
                .map(|v| ("srcBlendAlpha".to_string(), v)),
            ((self.dest_blend_alpha).inspect_value(path, offset, limit))
                .map(|v| ("destBlendAlpha".to_string(), v)),
            ((self.blend_op).inspect_value(path, offset, limit))
                .map(|v| ("blendOp".to_string(), v)),
            ((self.blend_op_alpha).inspect_value(path, offset, limit))
                .map(|v| ("blendOpAlpha".to_string(), v)),
            ((self.col_mask).inspect_value(path, offset, limit))
                .map(|v| ("colMask".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
