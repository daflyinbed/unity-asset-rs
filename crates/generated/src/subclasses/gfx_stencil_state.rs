//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct GfxStencilState {
    pub stencil_enable: u8,
    pub read_mask: u8,
    pub write_mask: u8,
    pub padding: u8,
    pub stencil_func_front: u8,
    pub stencil_pass_op_front: u8,
    pub stencil_fail_op_front: u8,
    pub stencil_z_fail_op_front: u8,
    pub stencil_func_back: u8,
    pub stencil_pass_op_back: u8,
    pub stencil_fail_op_back: u8,
    pub stencil_z_fail_op_back: u8,
}

impl crate::Parse for GfxStencilState {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let stencil_enable = r.read_u8()?;
        let read_mask = r.read_u8()?;
        let write_mask = r.read_u8()?;
        let padding = r.read_u8()?;
        let stencil_func_front = r.read_u8()?;
        let stencil_pass_op_front = r.read_u8()?;
        let stencil_fail_op_front = r.read_u8()?;
        let stencil_z_fail_op_front = r.read_u8()?;
        let stencil_func_back = r.read_u8()?;
        let stencil_pass_op_back = r.read_u8()?;
        let stencil_fail_op_back = r.read_u8()?;
        let stencil_z_fail_op_back = r.read_u8()?;

        Ok(GfxStencilState {
            stencil_enable,
            read_mask,
            write_mask,
            padding,
            stencil_func_front,
            stencil_pass_op_front,
            stencil_fail_op_front,
            stencil_z_fail_op_front,
            stencil_func_back,
            stencil_pass_op_back,
            stencil_fail_op_back,
            stencil_z_fail_op_back,
        })
    }
}
