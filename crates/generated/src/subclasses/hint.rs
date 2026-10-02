//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct Hint {
    pub m_key: String,
    pub m_value: String,
}

impl crate::Parse for Hint {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_key = rdr.read_aligned_string()?;
        let m_value = rdr.read_aligned_string()?;

        Ok(Hint { m_key, m_value })
    }
}

impl crate::ToValue for Hint {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Key".to_string(), self.m_key.to_value())),
            Some(("m_Value".to_string(), self.m_value.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Key" => (self.m_key).inspect_value(path, offset, limit),
                "m_Value" => (self.m_value).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_key).inspect_value(path, offset, limit)).map(|v| ("m_Key".to_string(), v)),
            ((self.m_value).inspect_value(path, offset, limit)).map(|v| ("m_Value".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
