//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct TransformMaskElement {
    pub m_path: String,
    pub m_weight: f32,
}

impl crate::Parse for TransformMaskElement {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_path = rdr.read_aligned_string()?;
        let m_weight = rdr.read_f32()?;

        Ok(TransformMaskElement { m_path, m_weight })
    }
}

impl crate::ToValue for TransformMaskElement {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Path".to_string(), self.m_path.to_value())),
            Some(("m_Weight".to_string(), self.m_weight.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Path" => (self.m_path).inspect_value(path, offset, limit),
                "m_Weight" => (self.m_weight).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_path).inspect_value(path, offset, limit)).map(|v| ("m_Path".to_string(), v)),
            ((self.m_weight).inspect_value(path, offset, limit))
                .map(|v| ("m_Weight".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
