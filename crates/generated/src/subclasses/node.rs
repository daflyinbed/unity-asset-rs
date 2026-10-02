//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct Node {
    pub m_parent_id: i32,
    pub m_axes_id: i32,
}

impl crate::Parse for Node {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_parent_id = rdr.read_i32()?;
        let m_axes_id = rdr.read_i32()?;

        Ok(Node {
            m_parent_id,
            m_axes_id,
        })
    }
}

impl crate::ToValue for Node {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_ParentId".to_string(), self.m_parent_id.to_value())),
            Some(("m_AxesId".to_string(), self.m_axes_id.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_ParentId" => (self.m_parent_id).inspect_value(path, offset, limit),
                "m_AxesId" => (self.m_axes_id).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_parent_id).inspect_value(path, offset, limit))
                .map(|v| ("m_ParentId".to_string(), v)),
            ((self.m_axes_id).inspect_value(path, offset, limit))
                .map(|v| ("m_AxesId".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
