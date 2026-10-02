//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct UnitySceneHandle {
    pub value: i32,
}

impl crate::Parse for UnitySceneHandle {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let value = rdr.read_i32()?;

        Ok(UnitySceneHandle { value })
    }
}

impl crate::ToValue for UnitySceneHandle {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> =
            [Some(("value".to_string(), self.value.to_value()))]
                .into_iter()
                .flatten()
                .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "value" => (self.value).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.value).inspect_value(path, offset, limit)).map(|v| ("value".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
