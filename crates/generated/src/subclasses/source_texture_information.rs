//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct SourceTextureInformation {
    pub width: i32,
    pub height: i32,
    pub does_texture_contain_alpha: bool,
    pub does_texture_contain_color: Option<bool>,
    pub source_was_hdr: Option<bool>,
}

impl crate::Parse for SourceTextureInformation {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut width = None;
        let mut height = None;
        let mut does_texture_contain_alpha = None;
        let mut does_texture_contain_color = None;
        let mut source_was_hdr = None;
        width = Some(rdr.read_i32()?);
        height = Some(rdr.read_i32()?);
        does_texture_contain_alpha = Some(rdr.read_bool()?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            source_was_hdr = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            does_texture_contain_color = Some(rdr.read_bool()?);
        } else {
        }

        Ok(SourceTextureInformation {
            width: width.unwrap(),
            height: height.unwrap(),
            does_texture_contain_alpha: does_texture_contain_alpha.unwrap(),
            does_texture_contain_color,
            source_was_hdr,
        })
    }
}

impl crate::ToValue for SourceTextureInformation {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("width".to_string(), self.width.to_value())),
            Some(("height".to_string(), self.height.to_value())),
            Some((
                "doesTextureContainAlpha".to_string(),
                self.does_texture_contain_alpha.to_value(),
            )),
            crate::opt_field("doesTextureContainColor", &self.does_texture_contain_color),
            crate::opt_field("sourceWasHDR", &self.source_was_hdr),
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
                "doesTextureContainAlpha" => {
                    (self.does_texture_contain_alpha).inspect_value(path, offset, limit)
                }
                "doesTextureContainColor" => {
                    (self.does_texture_contain_color).inspect_value(path, offset, limit)
                }
                "sourceWasHDR" => (self.source_was_hdr).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.width).inspect_value(path, offset, limit)).map(|v| ("width".to_string(), v)),
            ((self.height).inspect_value(path, offset, limit)).map(|v| ("height".to_string(), v)),
            ((self.does_texture_contain_alpha).inspect_value(path, offset, limit))
                .map(|v| ("doesTextureContainAlpha".to_string(), v)),
            ((self.does_texture_contain_color).inspect_value(path, offset, limit))
                .map(|v| ("doesTextureContainColor".to_string(), v)),
            ((self.source_was_hdr).inspect_value(path, offset, limit))
                .map(|v| ("sourceWasHDR".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
