//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct TextureSettings {
    pub aniso_level: i32,
    pub compression_quality: i32,
    pub max_texture_size: i32,
    pub texture_compression: i32,
    pub filter_mode: i32,
    pub generate_mip_maps: bool,
    pub readable: bool,
    pub crunched_compression: bool,
    pub s_rgb: bool,
}

impl crate::Parse for TextureSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let aniso_level = rdr.read_i32()?;
        let compression_quality = rdr.read_i32()?;
        let max_texture_size = rdr.read_i32()?;
        let texture_compression = rdr.read_i32()?;
        let filter_mode = rdr.read_i32()?;
        let generate_mip_maps = rdr.read_bool()?;
        let readable = rdr.read_bool()?;
        let crunched_compression = rdr.read_bool()?;
        rdr.align(4)?;
        let s_rgb = rdr.read_bool()?;
        rdr.align(4)?;

        Ok(TextureSettings {
            aniso_level,
            compression_quality,
            max_texture_size,
            texture_compression,
            filter_mode,
            generate_mip_maps,
            readable,
            crunched_compression,
            s_rgb,
        })
    }
}
