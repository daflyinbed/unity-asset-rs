//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct UnityPurchasingSettings {
    pub m_enabled: bool,
    pub m_test_mode: bool,
}

impl crate::Parse for UnityPurchasingSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_enabled = rdr.read_bool()?;
        let m_test_mode = rdr.read_bool()?;
        rdr.align(4)?;

        Ok(UnityPurchasingSettings {
            m_enabled,
            m_test_mode,
        })
    }
}

impl crate::ToValue for UnityPurchasingSettings {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Enabled".to_string(), self.m_enabled.to_value())),
            Some(("m_TestMode".to_string(), self.m_test_mode.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Enabled" => (self.m_enabled).inspect_value(path, offset, limit),
                "m_TestMode" => (self.m_test_mode).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_enabled).inspect_value(path, offset, limit))
                .map(|v| ("m_Enabled".to_string(), v)),
            ((self.m_test_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_TestMode".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
