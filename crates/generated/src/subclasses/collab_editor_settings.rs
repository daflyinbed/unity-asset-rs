//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct CollabEditorSettings {
    pub in_progress_enabled: bool,
}

impl crate::Parse for CollabEditorSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let in_progress_enabled = rdr.read_bool()?;
        rdr.align(4)?;

        Ok(CollabEditorSettings {
            in_progress_enabled,
        })
    }
}

impl crate::ToValue for CollabEditorSettings {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [Some((
            "inProgressEnabled".to_string(),
            self.in_progress_enabled.to_value(),
        ))]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "inProgressEnabled" => {
                    (self.in_progress_enabled).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.in_progress_enabled).inspect_value(path, offset, limit))
                .map(|v| ("inProgressEnabled".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
