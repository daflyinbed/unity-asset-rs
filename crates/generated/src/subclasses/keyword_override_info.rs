//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct KeywordOverrideInfo {
    pub name: String,
    pub keep_in_build: bool,
}

impl crate::Parse for KeywordOverrideInfo {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let name = rdr.read_aligned_string()?;
        let keep_in_build = rdr.read_bool()?;

        Ok(KeywordOverrideInfo {
            name,
            keep_in_build,
        })
    }
}

impl crate::ToValue for KeywordOverrideInfo {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("name".to_string(), self.name.to_value())),
            Some(("keepInBuild".to_string(), self.keep_in_build.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "name" => (self.name).inspect_value(path, offset, limit),
                "keepInBuild" => (self.keep_in_build).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.name).inspect_value(path, offset, limit)).map(|v| ("name".to_string(), v)),
            ((self.keep_in_build).inspect_value(path, offset, limit))
                .map(|v| ("keepInBuild".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
