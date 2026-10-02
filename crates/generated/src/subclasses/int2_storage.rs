//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct int2_storage {
    pub x: i32,
    pub y: i32,
}

impl crate::Parse for int2_storage {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let x = rdr.read_i32()?;
        let y = rdr.read_i32()?;

        Ok(int2_storage { x, y })
    }
}

impl crate::ToValue for int2_storage {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("x".to_string(), self.x.to_value())),
            Some(("y".to_string(), self.y.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "x" => (self.x).inspect_value(path, offset, limit),
                "y" => (self.y).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.x).inspect_value(path, offset, limit)).map(|v| ("x".to_string(), v)),
            ((self.y).inspect_value(path, offset, limit)).map(|v| ("y".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
