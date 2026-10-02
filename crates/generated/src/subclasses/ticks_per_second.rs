//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct TicksPerSecond {
    pub m_denominator: u32,
    pub m_numerator: u32,
}

impl crate::Parse for TicksPerSecond {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_denominator = rdr.read_u32()?;
        let m_numerator = rdr.read_u32()?;

        Ok(TicksPerSecond {
            m_denominator,
            m_numerator,
        })
    }
}

impl crate::ToValue for TicksPerSecond {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Denominator".to_string(), self.m_denominator.to_value())),
            Some(("m_Numerator".to_string(), self.m_numerator.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Denominator" => (self.m_denominator).inspect_value(path, offset, limit),
                "m_Numerator" => (self.m_numerator).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_denominator).inspect_value(path, offset, limit))
                .map(|v| ("m_Denominator".to_string(), v)),
            ((self.m_numerator).inspect_value(path, offset, limit))
                .map(|v| ("m_Numerator".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
