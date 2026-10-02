//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct PPtr_PrefabInstance {
    pub m_file_id: i32,
    pub m_path_id: i64,
}

impl crate::Parse for PPtr_PrefabInstance {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_file_id = rdr.read_i32()?;
        let m_path_id = rdr.read_i64()?;

        Ok(PPtr_PrefabInstance {
            m_file_id,
            m_path_id,
        })
    }
}

impl crate::ToValue for PPtr_PrefabInstance {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_FileID".to_string(), self.m_file_id.to_value())),
            Some(("m_PathID".to_string(), self.m_path_id.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_FileID" => (self.m_file_id).inspect_value(path, offset, limit),
                "m_PathID" => (self.m_path_id).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_file_id).inspect_value(path, offset, limit))
                .map(|v| ("m_FileID".to_string(), v)),
            ((self.m_path_id).inspect_value(path, offset, limit))
                .map(|v| ("m_PathID".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
