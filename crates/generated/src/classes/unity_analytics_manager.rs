//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct UnityAnalyticsManager {
    pub m_enabled: Option<bool>,
    pub m_initialize_on_startup: Option<bool>,
    pub m_test_mode: Option<bool>,
    pub m_test_event_url: Option<String>,
    pub m_test_config_url: Option<String>,
}

impl crate::Parse for UnityAnalyticsManager {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_enabled = None;
        let mut m_initialize_on_startup = None;
        let mut m_test_mode = None;
        let mut m_test_event_url = None;
        let mut m_test_config_url = None;
        if *ver >= crate::UnityVersion::new(5, 3, 0) {
        } else {
            m_enabled = Some(rdr.read_bool()?);
            m_initialize_on_startup = Some(rdr.read_bool()?);
            m_test_mode = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_test_event_url = Some(rdr.read_aligned_string()?);
            m_test_config_url = Some(rdr.read_aligned_string()?);
        }

        Ok(UnityAnalyticsManager {
            m_enabled,
            m_initialize_on_startup,
            m_test_mode,
            m_test_event_url,
            m_test_config_url,
        })
    }
}

impl crate::ToValue for UnityAnalyticsManager {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            crate::opt_field("m_Enabled", &self.m_enabled),
            crate::opt_field("m_InitializeOnStartup", &self.m_initialize_on_startup),
            crate::opt_field("m_TestMode", &self.m_test_mode),
            crate::opt_field("m_TestEventUrl", &self.m_test_event_url),
            crate::opt_field("m_TestConfigUrl", &self.m_test_config_url),
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
                "m_InitializeOnStartup" => {
                    (self.m_initialize_on_startup).inspect_value(path, offset, limit)
                }
                "m_TestMode" => (self.m_test_mode).inspect_value(path, offset, limit),
                "m_TestEventUrl" => (self.m_test_event_url).inspect_value(path, offset, limit),
                "m_TestConfigUrl" => (self.m_test_config_url).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_enabled).inspect_value(path, offset, limit))
                .map(|v| ("m_Enabled".to_string(), v)),
            ((self.m_initialize_on_startup).inspect_value(path, offset, limit))
                .map(|v| ("m_InitializeOnStartup".to_string(), v)),
            ((self.m_test_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_TestMode".to_string(), v)),
            ((self.m_test_event_url).inspect_value(path, offset, limit))
                .map(|v| ("m_TestEventUrl".to_string(), v)),
            ((self.m_test_config_url).inspect_value(path, offset, limit))
                .map(|v| ("m_TestConfigUrl".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
