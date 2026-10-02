//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct GfxRenderTargetBlendState {
    pub write_mask: u8,
    pub src_blend: u8,
    pub dst_blend: u8,
    pub src_blend_alpha: u8,
    pub dst_blend_alpha: u8,
    pub blend_op: u8,
    pub blend_op_alpha: u8,
}

impl crate::Parse for GfxRenderTargetBlendState {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let write_mask = rdr.read_u8()?;
        let src_blend = rdr.read_u8()?;
        let dst_blend = rdr.read_u8()?;
        let src_blend_alpha = rdr.read_u8()?;
        let dst_blend_alpha = rdr.read_u8()?;
        let blend_op = rdr.read_u8()?;
        let blend_op_alpha = rdr.read_u8()?;

        Ok(GfxRenderTargetBlendState {
            write_mask,
            src_blend,
            dst_blend,
            src_blend_alpha,
            dst_blend_alpha,
            blend_op,
            blend_op_alpha,
        })
    }
}

impl crate::ToValue for GfxRenderTargetBlendState {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("writeMask".to_string(), self.write_mask.to_value())),
            Some(("srcBlend".to_string(), self.src_blend.to_value())),
            Some(("dstBlend".to_string(), self.dst_blend.to_value())),
            Some(("srcBlendAlpha".to_string(), self.src_blend_alpha.to_value())),
            Some(("dstBlendAlpha".to_string(), self.dst_blend_alpha.to_value())),
            Some(("blendOp".to_string(), self.blend_op.to_value())),
            Some(("blendOpAlpha".to_string(), self.blend_op_alpha.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "writeMask" => (self.write_mask).inspect_value(path, offset, limit),
                "srcBlend" => (self.src_blend).inspect_value(path, offset, limit),
                "dstBlend" => (self.dst_blend).inspect_value(path, offset, limit),
                "srcBlendAlpha" => (self.src_blend_alpha).inspect_value(path, offset, limit),
                "dstBlendAlpha" => (self.dst_blend_alpha).inspect_value(path, offset, limit),
                "blendOp" => (self.blend_op).inspect_value(path, offset, limit),
                "blendOpAlpha" => (self.blend_op_alpha).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.write_mask).inspect_value(path, offset, limit))
                .map(|v| ("writeMask".to_string(), v)),
            ((self.src_blend).inspect_value(path, offset, limit))
                .map(|v| ("srcBlend".to_string(), v)),
            ((self.dst_blend).inspect_value(path, offset, limit))
                .map(|v| ("dstBlend".to_string(), v)),
            ((self.src_blend_alpha).inspect_value(path, offset, limit))
                .map(|v| ("srcBlendAlpha".to_string(), v)),
            ((self.dst_blend_alpha).inspect_value(path, offset, limit))
                .map(|v| ("dstBlendAlpha".to_string(), v)),
            ((self.blend_op).inspect_value(path, offset, limit))
                .map(|v| ("blendOp".to_string(), v)),
            ((self.blend_op_alpha).inspect_value(path, offset, limit))
                .map(|v| ("blendOpAlpha".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
