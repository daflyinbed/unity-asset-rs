//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct MetroFtaFileType {
    pub content_type: String,
    pub file_type: String,
}

impl crate::Parse for MetroFtaFileType {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let content_type = rdr.read_aligned_string()?;
        let file_type = rdr.read_aligned_string()?;

        Ok(MetroFtaFileType {
            content_type,
            file_type,
        })
    }
}

impl crate::ToValue for MetroFtaFileType {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("contentType".to_string(), self.content_type.to_value())),
            Some(("fileType".to_string(), self.file_type.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "contentType" => (self.content_type).inspect_value(path, offset, limit),
                "fileType" => (self.file_type).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.content_type).inspect_value(path, offset, limit))
                .map(|v| ("contentType".to_string(), v)),
            ((self.file_type).inspect_value(path, offset, limit))
                .map(|v| ("fileType".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
