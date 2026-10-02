//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct DateTime {
    pub ticks: i64,
}

impl crate::Parse for DateTime {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let ticks = rdr.read_i64()?;

        Ok(DateTime { ticks })
    }
}

impl crate::ToValue for DateTime {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> =
            [Some(("ticks".to_string(), self.ticks.to_value()))]
                .into_iter()
                .flatten()
                .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "ticks" => (self.ticks).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.ticks).inspect_value(path, offset, limit)).map(|v| ("ticks".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
