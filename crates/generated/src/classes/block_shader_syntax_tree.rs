//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct BlockShaderSyntaxTree {
    pub m_name: String,
    pub file_path: String,
    pub source: String,
}

impl crate::Parse for BlockShaderSyntaxTree {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        let file_path = rdr.read_aligned_string()?;
        let source = rdr.read_aligned_string()?;

        Ok(BlockShaderSyntaxTree {
            m_name,
            file_path,
            source,
        })
    }
}

impl crate::ToValue for BlockShaderSyntaxTree {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some(("filePath".to_string(), self.file_path.to_value())),
            Some(("source".to_string(), self.source.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Name" => (self.m_name).inspect_value(path, offset, limit),
                "filePath" => (self.file_path).inspect_value(path, offset, limit),
                "source" => (self.source).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.file_path).inspect_value(path, offset, limit))
                .map(|v| ("filePath".to_string(), v)),
            ((self.source).inspect_value(path, offset, limit)).map(|v| ("source".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
