//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct TextureImportInstructions {
    pub compressed_format: i32,
    pub uncompressed_format: i32,
    pub recommended_format: i32,
    pub usage_mode: i32,
    pub color_space: i32,
    pub width: i32,
    pub height: i32,
    pub compression_quality: i32,
}

impl crate::Parse for TextureImportInstructions {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let compressed_format = r.read_i32()?;
        let uncompressed_format = r.read_i32()?;
        let recommended_format = r.read_i32()?;
        let usage_mode = r.read_i32()?;
        let color_space = r.read_i32()?;
        let width = r.read_i32()?;
        let height = r.read_i32()?;
        let compression_quality = r.read_i32()?;

        Ok(TextureImportInstructions {
            compressed_format,
            uncompressed_format,
            recommended_format,
            usage_mode,
            color_space,
            width,
            height,
            compression_quality,
        })
    }
}
