//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct BuildStepMessage {
    pub r#type: i32,
    pub content: String,
}

impl crate::Parse for BuildStepMessage {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let r#type = rdr.read_i32()?;
        let content = rdr.read_aligned_string()?;

        Ok(BuildStepMessage { r#type, content })
    }
}

impl crate::ToValue for BuildStepMessage {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("type".to_string(), self.r#type.to_value())),
            Some(("content".to_string(), self.content.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "type" => (self.r#type).inspect_value(path, offset, limit),
                "content" => (self.content).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.r#type).inspect_value(path, offset, limit)).map(|v| ("type".to_string(), v)),
            ((self.content).inspect_value(path, offset, limit)).map(|v| ("content".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
