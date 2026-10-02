//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct TilemapTileDirtyData {
    pub m_dirty_index: u32,
}

impl crate::Parse for TilemapTileDirtyData {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_dirty_index = rdr.read_u32()?;

        Ok(TilemapTileDirtyData { m_dirty_index })
    }
}

impl crate::ToValue for TilemapTileDirtyData {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [Some((
            "m_DirtyIndex".to_string(),
            self.m_dirty_index.to_value(),
        ))]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_DirtyIndex" => (self.m_dirty_index).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_dirty_index).inspect_value(path, offset, limit))
                .map(|v| ("m_DirtyIndex".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
