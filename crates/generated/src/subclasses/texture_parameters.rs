//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct TextureParameters {
    pub width: i32,
    pub height: i32,
    pub mip_levels: i32,
    pub texture_format: i32,
}

impl crate::Parse for TextureParameters {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let width = r.read_i32()?;
        let height = r.read_i32()?;
        let mip_levels = r.read_i32()?;
        let texture_format = r.read_i32()?;

        Ok(TextureParameters {
            width,
            height,
            mip_levels,
            texture_format,
        })
    }
}
