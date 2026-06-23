//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct TextureSettings {
    pub aniso_level: u32,
    pub compression_quality: u32,
    pub max_texture_size: u32,
    pub texture_compression: i32,
    pub color_space: i32,
    pub filter_mode: i32,
    pub generate_mip_maps: i32,
    pub readable: i32,
}

impl crate::Parse for TextureSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let aniso_level = r.read_u32()?;
        let compression_quality = r.read_u32()?;
        let max_texture_size = r.read_u32()?;
        let texture_compression = r.read_i32()?;
        let color_space = r.read_i32()?;
        let filter_mode = r.read_i32()?;
        let generate_mip_maps = r.read_i32()?;
        let readable = r.read_i32()?;
        r.align(4)?;

        Ok(TextureSettings {
            aniso_level,
            compression_quality,
            max_texture_size,
            texture_compression,
            color_space,
            filter_mode,
            generate_mip_maps,
            readable,
        })
    }
}
