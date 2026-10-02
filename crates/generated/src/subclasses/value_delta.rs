//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct ValueDelta {
    pub m_start: f32,
    pub m_stop: f32,
}

impl crate::Parse for ValueDelta {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_start = rdr.read_f32()?;
        let m_stop = rdr.read_f32()?;

        Ok(ValueDelta { m_start, m_stop })
    }
}

impl crate::ToValue for ValueDelta {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Start".to_string(), self.m_start.to_value())),
            Some(("m_Stop".to_string(), self.m_stop.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Start" => (self.m_start).inspect_value(path, offset, limit),
                "m_Stop" => (self.m_stop).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_start).inspect_value(path, offset, limit)).map(|v| ("m_Start".to_string(), v)),
            ((self.m_stop).inspect_value(path, offset, limit)).map(|v| ("m_Stop".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
