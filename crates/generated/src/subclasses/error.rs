//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct Error {
    pub message: String,
    pub file_path: String,
    pub start_line: u32,
    pub start_char: u32,
    pub severity: i32,
}

impl crate::Parse for Error {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let message = rdr.read_aligned_string()?;
        let file_path = rdr.read_aligned_string()?;
        let start_line = rdr.read_u32()?;
        let start_char = rdr.read_u32()?;
        let severity = rdr.read_i32()?;

        Ok(Error {
            message,
            file_path,
            start_line,
            start_char,
            severity,
        })
    }
}

impl crate::ToValue for Error {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("message".to_string(), self.message.to_value())),
            Some(("filePath".to_string(), self.file_path.to_value())),
            Some(("startLine".to_string(), self.start_line.to_value())),
            Some(("startChar".to_string(), self.start_char.to_value())),
            Some(("severity".to_string(), self.severity.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "message" => (self.message).inspect_value(path, offset, limit),
                "filePath" => (self.file_path).inspect_value(path, offset, limit),
                "startLine" => (self.start_line).inspect_value(path, offset, limit),
                "startChar" => (self.start_char).inspect_value(path, offset, limit),
                "severity" => (self.severity).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.message).inspect_value(path, offset, limit)).map(|v| ("message".to_string(), v)),
            ((self.file_path).inspect_value(path, offset, limit))
                .map(|v| ("filePath".to_string(), v)),
            ((self.start_line).inspect_value(path, offset, limit))
                .map(|v| ("startLine".to_string(), v)),
            ((self.start_char).inspect_value(path, offset, limit))
                .map(|v| ("startChar".to_string(), v)),
            ((self.severity).inspect_value(path, offset, limit))
                .map(|v| ("severity".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
