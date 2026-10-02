//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct SourceLocation {
    pub m_file: String,
    pub m_position: u32,
}

impl crate::Parse for SourceLocation {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_file = rdr.read_aligned_string()?;
        let m_position = rdr.read_u32()?;

        Ok(SourceLocation { m_file, m_position })
    }
}

impl crate::ToValue for SourceLocation {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_File".to_string(), self.m_file.to_value())),
            Some(("m_Position".to_string(), self.m_position.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_File" => (self.m_file).inspect_value(path, offset, limit),
                "m_Position" => (self.m_position).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_file).inspect_value(path, offset, limit)).map(|v| ("m_File".to_string(), v)),
            ((self.m_position).inspect_value(path, offset, limit))
                .map(|v| ("m_Position".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
