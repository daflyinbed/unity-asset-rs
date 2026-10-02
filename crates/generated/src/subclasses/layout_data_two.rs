//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct LayoutDataTwo {
    pub m_integer_value: i32,
    pub m_float_value: f32,
}

impl crate::Parse for LayoutDataTwo {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_integer_value = rdr.read_i32()?;
        let m_float_value = rdr.read_f32()?;

        Ok(LayoutDataTwo {
            m_integer_value,
            m_float_value,
        })
    }
}

impl crate::ToValue for LayoutDataTwo {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "m_IntegerValue".to_string(),
                self.m_integer_value.to_value(),
            )),
            Some(("m_FloatValue".to_string(), self.m_float_value.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_IntegerValue" => (self.m_integer_value).inspect_value(path, offset, limit),
                "m_FloatValue" => (self.m_float_value).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_integer_value).inspect_value(path, offset, limit))
                .map(|v| ("m_IntegerValue".to_string(), v)),
            ((self.m_float_value).inspect_value(path, offset, limit))
                .map(|v| ("m_FloatValue".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
