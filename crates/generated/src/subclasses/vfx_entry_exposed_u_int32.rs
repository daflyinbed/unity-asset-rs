//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct VfxEntryExposed_UInt32 {
    pub m_value: u32,
    pub m_name: String,
    pub m_overridden: bool,
}

impl crate::Parse for VfxEntryExposed_UInt32 {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_value = rdr.read_u32()?;
        rdr.align(4)?;
        let m_name = rdr.read_aligned_string()?;
        let m_overridden = rdr.read_bool()?;
        rdr.align(4)?;

        Ok(VfxEntryExposed_UInt32 {
            m_value,
            m_name,
            m_overridden,
        })
    }
}

impl crate::ToValue for VfxEntryExposed_UInt32 {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Value".to_string(), self.m_value.to_value())),
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some(("m_Overridden".to_string(), self.m_overridden.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Value" => (self.m_value).inspect_value(path, offset, limit),
                "m_Name" => (self.m_name).inspect_value(path, offset, limit),
                "m_Overridden" => (self.m_overridden).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_value).inspect_value(path, offset, limit)).map(|v| ("m_Value".to_string(), v)),
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_overridden).inspect_value(path, offset, limit))
                .map(|v| ("m_Overridden".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
