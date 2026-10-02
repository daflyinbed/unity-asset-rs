//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct NetworkViewId {
    pub m_id: u32,
    pub m_type: u32,
}

impl crate::Parse for NetworkViewId {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_id = rdr.read_u32()?;
        let m_type = rdr.read_u32()?;

        Ok(NetworkViewId { m_id, m_type })
    }
}

impl crate::ToValue for NetworkViewId {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_ID".to_string(), self.m_id.to_value())),
            Some(("m_Type".to_string(), self.m_type.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_ID" => (self.m_id).inspect_value(path, offset, limit),
                "m_Type" => (self.m_type).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_id).inspect_value(path, offset, limit)).map(|v| ("m_ID".to_string(), v)),
            ((self.m_type).inspect_value(path, offset, limit)).map(|v| ("m_Type".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
