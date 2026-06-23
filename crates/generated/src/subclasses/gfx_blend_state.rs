//! 自动生成。请勿手动修改。

use crate::subclasses::gfx_render_target_blend_state::GfxRenderTargetBlendState;

#[derive(Debug, Clone)]
pub struct GfxBlendState {
    pub rt: Vec<GfxRenderTargetBlendState>,
    pub separate_mrt_blend: u8,
    pub alpha_to_mask: u8,
}

impl crate::Parse for GfxBlendState {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let rt = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { GfxRenderTargetBlendState::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let separate_mrt_blend = r.read_u8()?;
        let alpha_to_mask = r.read_u8()?;
        r.align(4)?;

        Ok(GfxBlendState {
            rt,
            separate_mrt_blend,
            alpha_to_mask,
        })
    }
}
