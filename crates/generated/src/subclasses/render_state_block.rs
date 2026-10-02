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

impl crate::ToValue for RenderStateBlock {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("blendState".to_string(), self.blend_state.to_value())),
            Some(("rasterState".to_string(), self.raster_state.to_value())),
            Some(("depthState".to_string(), self.depth_state.to_value())),
            Some(("stencilState".to_string(), self.stencil_state.to_value())),
            Some(("stencilRef".to_string(), self.stencil_ref.to_value())),
            Some(("mask".to_string(), self.mask.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "blendState" => (self.blend_state).inspect_value(path, offset, limit),
                "rasterState" => (self.raster_state).inspect_value(path, offset, limit),
                "depthState" => (self.depth_state).inspect_value(path, offset, limit),
                "stencilState" => (self.stencil_state).inspect_value(path, offset, limit),
                "stencilRef" => (self.stencil_ref).inspect_value(path, offset, limit),
                "mask" => (self.mask).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.blend_state).inspect_value(path, offset, limit))
                .map(|v| ("blendState".to_string(), v)),
            ((self.raster_state).inspect_value(path, offset, limit))
                .map(|v| ("rasterState".to_string(), v)),
            ((self.depth_state).inspect_value(path, offset, limit))
                .map(|v| ("depthState".to_string(), v)),
            ((self.stencil_state).inspect_value(path, offset, limit))
                .map(|v| ("stencilState".to_string(), v)),
            ((self.stencil_ref).inspect_value(path, offset, limit))
                .map(|v| ("stencilRef".to_string(), v)),
            ((self.mask).inspect_value(path, offset, limit)).map(|v| ("mask".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
