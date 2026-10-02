//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct KeywordTargetInfo {
    pub keyword_name: String,
    pub requirements: i32,
}

impl crate::Parse for KeywordTargetInfo {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let keyword_name = rdr.read_aligned_string()?;
        let requirements = rdr.read_i32()?;

        Ok(KeywordTargetInfo {
            keyword_name,
            requirements,
        })
    }
}

impl crate::ToValue for KeywordTargetInfo {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("keywordName".to_string(), self.keyword_name.to_value())),
            Some(("requirements".to_string(), self.requirements.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "keywordName" => (self.keyword_name).inspect_value(path, offset, limit),
                "requirements" => (self.requirements).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.keyword_name).inspect_value(path, offset, limit))
                .map(|v| ("keywordName".to_string(), v)),
            ((self.requirements).inspect_value(path, offset, limit))
                .map(|v| ("requirements".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
