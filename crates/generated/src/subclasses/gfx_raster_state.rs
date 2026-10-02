//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct GfxRasterState {
    pub cull_mode: i32,
    pub depth_bias: i32,
    pub slope_scaled_depth_bias: f32,
    pub depth_clip: u8,
    pub conservative: u8,
}

impl crate::Parse for GfxRasterState {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let cull_mode = rdr.read_i32()?;
        let depth_bias = rdr.read_i32()?;
        let slope_scaled_depth_bias = rdr.read_f32()?;
        let depth_clip = rdr.read_u8()?;
        let conservative = rdr.read_u8()?;

        Ok(GfxRasterState {
            cull_mode,
            depth_bias,
            slope_scaled_depth_bias,
            depth_clip,
            conservative,
        })
    }
}

impl crate::ToValue for GfxRasterState {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("cullMode".to_string(), self.cull_mode.to_value())),
            Some(("depthBias".to_string(), self.depth_bias.to_value())),
            Some((
                "slopeScaledDepthBias".to_string(),
                self.slope_scaled_depth_bias.to_value(),
            )),
            Some(("depthClip".to_string(), self.depth_clip.to_value())),
            Some(("conservative".to_string(), self.conservative.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "cullMode" => (self.cull_mode).inspect_value(path, offset, limit),
                "depthBias" => (self.depth_bias).inspect_value(path, offset, limit),
                "slopeScaledDepthBias" => {
                    (self.slope_scaled_depth_bias).inspect_value(path, offset, limit)
                }
                "depthClip" => (self.depth_clip).inspect_value(path, offset, limit),
                "conservative" => (self.conservative).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.cull_mode).inspect_value(path, offset, limit))
                .map(|v| ("cullMode".to_string(), v)),
            ((self.depth_bias).inspect_value(path, offset, limit))
                .map(|v| ("depthBias".to_string(), v)),
            ((self.slope_scaled_depth_bias).inspect_value(path, offset, limit))
                .map(|v| ("slopeScaledDepthBias".to_string(), v)),
            ((self.depth_clip).inspect_value(path, offset, limit))
                .map(|v| ("depthClip".to_string(), v)),
            ((self.conservative).inspect_value(path, offset, limit))
                .map(|v| ("conservative".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
