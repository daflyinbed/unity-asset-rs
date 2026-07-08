//! 自动生成。请勿手动修改。

use crate::subclasses::render_state_block::RenderStateBlock;

#[derive(Debug, Clone)]
pub struct RenderStateInfo {
    pub render_state: RenderStateBlock,
}

impl crate::Parse for RenderStateInfo {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let render_state = RenderStateBlock::parse(rdr, ver)?;

        Ok(RenderStateInfo {
            render_state,
        })
    }
}
