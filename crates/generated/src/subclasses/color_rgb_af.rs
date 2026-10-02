//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct ColorRgbAf {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl crate::Parse for ColorRgbAf {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let r = rdr.read_f32()?;
        let g = rdr.read_f32()?;
        let b = rdr.read_f32()?;
        let a = rdr.read_f32()?;

        Ok(ColorRgbAf { r, g, b, a })
    }
}

impl crate::ToValue for ColorRgbAf {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("r".to_string(), self.r.to_value())),
            Some(("g".to_string(), self.g.to_value())),
            Some(("b".to_string(), self.b.to_value())),
            Some(("a".to_string(), self.a.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "r" => (self.r).inspect_value(path, offset, limit),
                "g" => (self.g).inspect_value(path, offset, limit),
                "b" => (self.b).inspect_value(path, offset, limit),
                "a" => (self.a).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.r).inspect_value(path, offset, limit)).map(|v| ("r".to_string(), v)),
            ((self.g).inspect_value(path, offset, limit)).map(|v| ("g".to_string(), v)),
            ((self.b).inspect_value(path, offset, limit)).map(|v| ("b".to_string(), v)),
            ((self.a).inspect_value(path, offset, limit)).map(|v| ("a".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
