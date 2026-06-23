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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let blend_state = GfxBlendState::parse(r, ver)?;
        r.align(4)?;
        let raster_state = GfxRasterState::parse(r, ver)?;
        let depth_state = GfxDepthState::parse(r, ver)?;
        r.align(4)?;
        let stencil_state = GfxStencilState::parse(r, ver)?;
        let stencil_ref = r.read_i32()?;
        let mask = r.read_i32()?;

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
