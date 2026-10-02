//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct PropertyNameNested {
    pub id: String,
}

impl crate::Parse for PropertyNameNested {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let id = rdr.read_aligned_string()?;
        rdr.align(4)?;

        Ok(PropertyNameNested { id })
    }
}

impl crate::ToValue for PropertyNameNested {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [Some(("id".to_string(), self.id.to_value()))]
            .into_iter()
            .flatten()
            .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "id" => (self.id).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> =
            vec![((self.id).inspect_value(path, offset, limit)).map(|v| ("id".to_string(), v))]
                .into_iter()
                .flatten()
                .collect();
        Some(crate::Value::Struct(fields))
    }
}
