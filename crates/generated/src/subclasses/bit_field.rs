//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct BitField {
    pub m_bits: u32,
}

impl crate::Parse for BitField {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_bits = rdr.read_u32()?;

        Ok(BitField { m_bits })
    }
}

impl crate::ToValue for BitField {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> =
            [Some(("m_Bits".to_string(), self.m_bits.to_value()))]
                .into_iter()
                .flatten()
                .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Bits" => (self.m_bits).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_bits).inspect_value(path, offset, limit)).map(|v| ("m_Bits".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
