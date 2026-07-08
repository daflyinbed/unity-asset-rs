//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct GfxDepthState {
    pub depth_write: u8,
    pub depth_func: i8,
}

impl crate::Parse for GfxDepthState {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let depth_write = rdr.read_u8()?;
        let depth_func = rdr.read_i8()?;
        rdr.align(4)?;

        Ok(GfxDepthState {
            depth_write,
            depth_func,
        })
    }
}
