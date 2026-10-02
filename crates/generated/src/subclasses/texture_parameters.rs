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
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let width = rdr.read_i32()?;
        let height = rdr.read_i32()?;
        let mip_levels = rdr.read_i32()?;
        let texture_format = rdr.read_i32()?;

        Ok(TextureParameters {
            width,
            height,
            mip_levels,
            texture_format,
        })
    }
}

impl crate::ToValue for TextureParameters {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("width".to_string(), self.width.to_value())),
            Some(("height".to_string(), self.height.to_value())),
            Some(("mipLevels".to_string(), self.mip_levels.to_value())),
            Some(("textureFormat".to_string(), self.texture_format.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "width" => (self.width).inspect_value(path, offset, limit),
                "height" => (self.height).inspect_value(path, offset, limit),
                "mipLevels" => (self.mip_levels).inspect_value(path, offset, limit),
                "textureFormat" => (self.texture_format).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.width).inspect_value(path, offset, limit)).map(|v| ("width".to_string(), v)),
            ((self.height).inspect_value(path, offset, limit)).map(|v| ("height".to_string(), v)),
            ((self.mip_levels).inspect_value(path, offset, limit))
                .map(|v| ("mipLevels".to_string(), v)),
            ((self.texture_format).inspect_value(path, offset, limit))
                .map(|v| ("textureFormat".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
