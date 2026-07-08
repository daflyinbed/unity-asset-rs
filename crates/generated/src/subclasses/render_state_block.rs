//! 自动生成。请勿手动修改。

use crate::subclasses::gfx_blend_state::GfxBlendState;
use crate::subclasses::gfx_depth_state::GfxDepthState;
use crate::subclasses::gfx_raster_state::GfxRasterState;
use crate::subclasses::gfx_stencil_state::GfxStencilState;

#[derive(Debug, Clone)]
pub struct RenderStateBlock {
    pub blend_state: GfxBlendState,
    pub raster_state: GfxRasterState,
    pub depth_state: GfxDepthState,
    pub stencil_state: GfxStencilState,
    pub stencil_ref: i32,
    pub mask: i32,
}

impl crate::Parse for RenderStateBlock {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let blend_state = GfxBlendState::parse(rdr, ver)?;
        let raster_state = GfxRasterState::parse(rdr, ver)?;
        let depth_state = GfxDepthState::parse(rdr, ver)?;
        let stencil_state = GfxStencilState::parse(rdr, ver)?;
        let stencil_ref = rdr.read_i32()?;
        let mask = rdr.read_i32()?;

        Ok(RenderStateBlock {
            blend_state,
            raster_state,
            depth_state,
            stencil_state,
            stencil_ref,
            mask,
        })
    }
}
