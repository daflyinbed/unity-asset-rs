//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct ColorRgba32 {
    pub rgba: u32,
}

impl crate::Parse for ColorRgba32 {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let rgba = rdr.read_u32()?;

        Ok(ColorRgba32 { rgba })
    }
}

impl crate::ToValue for ColorRgba32 {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> =
            [Some(("rgba".to_string(), self.rgba.to_value()))]
                .into_iter()
                .flatten()
                .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "rgba" => (self.rgba).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> =
            vec![((self.rgba).inspect_value(path, offset, limit)).map(|v| ("rgba".to_string(), v))]
                .into_iter()
                .flatten()
                .collect();
        Some(crate::Value::Struct(fields))
    }
}
