//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct StreamedResource {
    pub m_source: String,
    pub m_offset: u64,
    pub m_size: u64,
}

impl crate::Parse for StreamedResource {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_source = rdr.read_aligned_string()?;
        let m_offset = rdr.read_u64()?;
        let m_size = rdr.read_u64()?;

        Ok(StreamedResource {
            m_source,
            m_offset,
            m_size,
        })
    }
}

impl crate::ToValue for StreamedResource {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Source".to_string(), self.m_source.to_value())),
            Some(("m_Offset".to_string(), self.m_offset.to_value())),
            Some(("m_Size".to_string(), self.m_size.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Source" => (self.m_source).inspect_value(path, offset, limit),
                "m_Offset" => (self.m_offset).inspect_value(path, offset, limit),
                "m_Size" => (self.m_size).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_source).inspect_value(path, offset, limit))
                .map(|v| ("m_Source".to_string(), v)),
            ((self.m_offset).inspect_value(path, offset, limit))
                .map(|v| ("m_Offset".to_string(), v)),
            ((self.m_size).inspect_value(path, offset, limit)).map(|v| ("m_Size".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
