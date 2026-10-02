//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct ValueConstant {
    pub m_id: u32,
    pub m_type_id: Option<u32>,
    pub m_type: u32,
    pub m_index: u32,
}

impl crate::Parse for ValueConstant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_id = None;
        let mut m_type_id = None;
        let mut m_type = None;
        let mut m_index = None;
        m_id = Some(rdr.read_u32()?);
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
        } else {
            m_type_id = Some(rdr.read_u32()?);
        }
        m_type = Some(rdr.read_u32()?);
        m_index = Some(rdr.read_u32()?);

        Ok(ValueConstant {
            m_id: m_id.unwrap(),
            m_type_id,
            m_type: m_type.unwrap(),
            m_index: m_index.unwrap(),
        })
    }
}

impl crate::ToValue for ValueConstant {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_ID".to_string(), self.m_id.to_value())),
            crate::opt_field("m_TypeID", &self.m_type_id),
            Some(("m_Type".to_string(), self.m_type.to_value())),
            Some(("m_Index".to_string(), self.m_index.to_value())),
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
                "m_TypeID" => (self.m_type_id).inspect_value(path, offset, limit),
                "m_Type" => (self.m_type).inspect_value(path, offset, limit),
                "m_Index" => (self.m_index).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_id).inspect_value(path, offset, limit)).map(|v| ("m_ID".to_string(), v)),
            ((self.m_type_id).inspect_value(path, offset, limit))
                .map(|v| ("m_TypeID".to_string(), v)),
            ((self.m_type).inspect_value(path, offset, limit)).map(|v| ("m_Type".to_string(), v)),
            ((self.m_index).inspect_value(path, offset, limit)).map(|v| ("m_Index".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
