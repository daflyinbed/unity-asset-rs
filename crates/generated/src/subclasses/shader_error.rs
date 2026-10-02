//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct ShaderError {
    pub message: String,
    pub line: i32,
    pub warning: bool,
    pub program_error: bool,
}

impl crate::Parse for ShaderError {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let message = rdr.read_aligned_string()?;
        let line = rdr.read_i32()?;
        let warning = rdr.read_bool()?;
        let program_error = rdr.read_bool()?;
        rdr.align(4)?;

        Ok(ShaderError {
            message,
            line,
            warning,
            program_error,
        })
    }
}

impl crate::ToValue for ShaderError {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("message".to_string(), self.message.to_value())),
            Some(("line".to_string(), self.line.to_value())),
            Some(("warning".to_string(), self.warning.to_value())),
            Some(("programError".to_string(), self.program_error.to_value())),
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
                "line" => (self.line).inspect_value(path, offset, limit),
                "warning" => (self.warning).inspect_value(path, offset, limit),
                "programError" => (self.program_error).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.message).inspect_value(path, offset, limit)).map(|v| ("message".to_string(), v)),
            ((self.line).inspect_value(path, offset, limit)).map(|v| ("line".to_string(), v)),
            ((self.warning).inspect_value(path, offset, limit)).map(|v| ("warning".to_string(), v)),
            ((self.program_error).inspect_value(path, offset, limit))
                .map(|v| ("programError".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
