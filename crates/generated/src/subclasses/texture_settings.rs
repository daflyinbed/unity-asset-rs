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

impl crate::ToValue for TextureSettings {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("anisoLevel".to_string(), self.aniso_level.to_value())),
            Some((
                "compressionQuality".to_string(),
                self.compression_quality.to_value(),
            )),
            Some((
                "maxTextureSize".to_string(),
                self.max_texture_size.to_value(),
            )),
            Some((
                "textureCompression".to_string(),
                self.texture_compression.to_value(),
            )),
            Some(("filterMode".to_string(), self.filter_mode.to_value())),
            Some((
                "generateMipMaps".to_string(),
                self.generate_mip_maps.to_value(),
            )),
            Some(("readable".to_string(), self.readable.to_value())),
            Some((
                "crunchedCompression".to_string(),
                self.crunched_compression.to_value(),
            )),
            Some(("sRGB".to_string(), self.s_rgb.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "anisoLevel" => (self.aniso_level).inspect_value(path, offset, limit),
                "compressionQuality" => {
                    (self.compression_quality).inspect_value(path, offset, limit)
                }
                "maxTextureSize" => (self.max_texture_size).inspect_value(path, offset, limit),
                "textureCompression" => {
                    (self.texture_compression).inspect_value(path, offset, limit)
                }
                "filterMode" => (self.filter_mode).inspect_value(path, offset, limit),
                "generateMipMaps" => (self.generate_mip_maps).inspect_value(path, offset, limit),
                "readable" => (self.readable).inspect_value(path, offset, limit),
                "crunchedCompression" => {
                    (self.crunched_compression).inspect_value(path, offset, limit)
                }
                "sRGB" => (self.s_rgb).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.aniso_level).inspect_value(path, offset, limit))
                .map(|v| ("anisoLevel".to_string(), v)),
            ((self.compression_quality).inspect_value(path, offset, limit))
                .map(|v| ("compressionQuality".to_string(), v)),
            ((self.max_texture_size).inspect_value(path, offset, limit))
                .map(|v| ("maxTextureSize".to_string(), v)),
            ((self.texture_compression).inspect_value(path, offset, limit))
                .map(|v| ("textureCompression".to_string(), v)),
            ((self.filter_mode).inspect_value(path, offset, limit))
                .map(|v| ("filterMode".to_string(), v)),
            ((self.generate_mip_maps).inspect_value(path, offset, limit))
                .map(|v| ("generateMipMaps".to_string(), v)),
            ((self.readable).inspect_value(path, offset, limit))
                .map(|v| ("readable".to_string(), v)),
            ((self.crunched_compression).inspect_value(path, offset, limit))
                .map(|v| ("crunchedCompression".to_string(), v)),
            ((self.s_rgb).inspect_value(path, offset, limit)).map(|v| ("sRGB".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
