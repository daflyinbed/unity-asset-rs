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
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let rt = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| GfxRenderTargetBlendState::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        let separate_mrt_blend = rdr.read_u8()?;
        let alpha_to_mask = rdr.read_u8()?;
        rdr.align(4)?;

        Ok(GfxBlendState {
            rt,
            separate_mrt_blend,
            alpha_to_mask,
        })
    }
}

impl crate::ToValue for GfxBlendState {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("rt".to_string(), self.rt.to_value())),
            Some((
                "separateMRTBlend".to_string(),
                self.separate_mrt_blend.to_value(),
            )),
            Some(("alphaToMask".to_string(), self.alpha_to_mask.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "rt" => (self.rt).inspect_value(path, offset, limit),
                "separateMRTBlend" => (self.separate_mrt_blend).inspect_value(path, offset, limit),
                "alphaToMask" => (self.alpha_to_mask).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.rt).inspect_value(path, offset, limit)).map(|v| ("rt".to_string(), v)),
            ((self.separate_mrt_blend).inspect_value(path, offset, limit))
                .map(|v| ("separateMRTBlend".to_string(), v)),
            ((self.alpha_to_mask).inspect_value(path, offset, limit))
                .map(|v| ("alphaToMask".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
