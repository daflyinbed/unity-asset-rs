//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct StateRange {
    pub m_start_index: u32,
    pub m_count: u32,
}

impl crate::Parse for StateRange {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_start_index = rdr.read_u32()?;
        let m_count = rdr.read_u32()?;

        Ok(StateRange {
            m_start_index,
            m_count,
        })
    }
}

impl crate::ToValue for StateRange {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_StartIndex".to_string(), self.m_start_index.to_value())),
            Some(("m_Count".to_string(), self.m_count.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_StartIndex" => (self.m_start_index).inspect_value(path, offset, limit),
                "m_Count" => (self.m_count).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_start_index).inspect_value(path, offset, limit))
                .map(|v| ("m_StartIndex".to_string(), v)),
            ((self.m_count).inspect_value(path, offset, limit)).map(|v| ("m_Count".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
