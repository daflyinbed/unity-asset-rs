//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct SerializedShaderDependency {
    pub from: String,
    pub to: String,
}

impl crate::Parse for SerializedShaderDependency {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let from = rdr.read_aligned_string()?;
        let to = rdr.read_aligned_string()?;

        Ok(SerializedShaderDependency { from, to })
    }
}

impl crate::ToValue for SerializedShaderDependency {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("from".to_string(), self.from.to_value())),
            Some(("to".to_string(), self.to.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "from" => (self.from).inspect_value(path, offset, limit),
                "to" => (self.to).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.from).inspect_value(path, offset, limit)).map(|v| ("from".to_string(), v)),
            ((self.to).inspect_value(path, offset, limit)).map(|v| ("to".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
