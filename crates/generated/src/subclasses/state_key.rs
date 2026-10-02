//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct StateKey {
    pub m_state_id: u32,
    pub m_layer_index: i32,
}

impl crate::Parse for StateKey {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_state_id = rdr.read_u32()?;
        let m_layer_index = rdr.read_i32()?;

        Ok(StateKey {
            m_state_id,
            m_layer_index,
        })
    }
}

impl crate::ToValue for StateKey {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_StateID".to_string(), self.m_state_id.to_value())),
            Some(("m_LayerIndex".to_string(), self.m_layer_index.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_StateID" => (self.m_state_id).inspect_value(path, offset, limit),
                "m_LayerIndex" => (self.m_layer_index).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_state_id).inspect_value(path, offset, limit))
                .map(|v| ("m_StateID".to_string(), v)),
            ((self.m_layer_index).inspect_value(path, offset, limit))
                .map(|v| ("m_LayerIndex".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
