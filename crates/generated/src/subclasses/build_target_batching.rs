//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct BuildTargetBatching {
    pub m_build_target: String,
    pub m_static_batching: bool,
    pub m_dynamic_batching: bool,
}

impl crate::Parse for BuildTargetBatching {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_build_target = rdr.read_aligned_string()?;
        let m_static_batching = rdr.read_bool()?;
        let m_dynamic_batching = rdr.read_bool()?;
        rdr.align(4)?;

        Ok(BuildTargetBatching {
            m_build_target,
            m_static_batching,
            m_dynamic_batching,
        })
    }
}

impl crate::ToValue for BuildTargetBatching {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_BuildTarget".to_string(), self.m_build_target.to_value())),
            Some((
                "m_StaticBatching".to_string(),
                self.m_static_batching.to_value(),
            )),
            Some((
                "m_DynamicBatching".to_string(),
                self.m_dynamic_batching.to_value(),
            )),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_BuildTarget" => (self.m_build_target).inspect_value(path, offset, limit),
                "m_StaticBatching" => (self.m_static_batching).inspect_value(path, offset, limit),
                "m_DynamicBatching" => (self.m_dynamic_batching).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_build_target).inspect_value(path, offset, limit))
                .map(|v| ("m_BuildTarget".to_string(), v)),
            ((self.m_static_batching).inspect_value(path, offset, limit))
                .map(|v| ("m_StaticBatching".to_string(), v)),
            ((self.m_dynamic_batching).inspect_value(path, offset, limit))
                .map(|v| ("m_DynamicBatching".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
