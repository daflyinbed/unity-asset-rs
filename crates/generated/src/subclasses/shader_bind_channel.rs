//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct ShaderBindChannel {
    pub source: i8,
    pub target: i8,
}

impl crate::Parse for ShaderBindChannel {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let source = rdr.read_i8()?;
        let target = rdr.read_i8()?;

        Ok(ShaderBindChannel { source, target })
    }
}

impl crate::ToValue for ShaderBindChannel {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("source".to_string(), self.source.to_value())),
            Some(("target".to_string(), self.target.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "source" => (self.source).inspect_value(path, offset, limit),
                "target" => (self.target).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.source).inspect_value(path, offset, limit)).map(|v| ("source".to_string(), v)),
            ((self.target).inspect_value(path, offset, limit)).map(|v| ("target".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
