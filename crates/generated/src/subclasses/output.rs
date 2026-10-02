//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct Output {
    pub has_empty_font_data: bool,
}

impl crate::Parse for Output {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let has_empty_font_data = rdr.read_bool()?;

        Ok(Output {
            has_empty_font_data,
        })
    }
}

impl crate::ToValue for Output {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [Some((
            "hasEmptyFontData".to_string(),
            self.has_empty_font_data.to_value(),
        ))]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "hasEmptyFontData" => (self.has_empty_font_data).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.has_empty_font_data).inspect_value(path, offset, limit))
                .map(|v| ("hasEmptyFontData".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
