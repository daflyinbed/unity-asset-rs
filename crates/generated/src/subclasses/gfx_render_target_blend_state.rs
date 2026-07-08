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
