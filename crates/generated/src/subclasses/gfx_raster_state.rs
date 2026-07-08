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
