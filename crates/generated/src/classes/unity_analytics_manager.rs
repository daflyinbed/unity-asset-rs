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
            rdr.align(4)?;
            m_test_config_url = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
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
