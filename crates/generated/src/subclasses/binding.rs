//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct Binding {
    pub m_set: u32,
    pub m_slot: u32,
}

impl crate::Parse for Binding {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_set = rdr.read_u32()?;
        let m_slot = rdr.read_u32()?;

        Ok(Binding { m_set, m_slot })
    }
}

impl crate::ToValue for Binding {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Set".to_string(), self.m_set.to_value())),
            Some(("m_Slot".to_string(), self.m_slot.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Set" => (self.m_set).inspect_value(path, offset, limit),
                "m_Slot" => (self.m_slot).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_set).inspect_value(path, offset, limit)).map(|v| ("m_Set".to_string(), v)),
            ((self.m_slot).inspect_value(path, offset, limit)).map(|v| ("m_Slot".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
