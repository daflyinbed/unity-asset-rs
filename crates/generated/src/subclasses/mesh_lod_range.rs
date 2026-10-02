//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct MeshLodRange {
    pub m_index_start: u32,
    pub m_index_count: u32,
}

impl crate::Parse for MeshLodRange {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_index_start = rdr.read_u32()?;
        let m_index_count = rdr.read_u32()?;
        rdr.align(4)?;

        Ok(MeshLodRange {
            m_index_start,
            m_index_count,
        })
    }
}

impl crate::ToValue for MeshLodRange {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_IndexStart".to_string(), self.m_index_start.to_value())),
            Some(("m_IndexCount".to_string(), self.m_index_count.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_IndexStart" => (self.m_index_start).inspect_value(path, offset, limit),
                "m_IndexCount" => (self.m_index_count).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_index_start).inspect_value(path, offset, limit))
                .map(|v| ("m_IndexStart".to_string(), v)),
            ((self.m_index_count).inspect_value(path, offset, limit))
                .map(|v| ("m_IndexCount".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
