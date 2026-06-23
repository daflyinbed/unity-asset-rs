//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct UnityAnalyticsManager {
    pub m_object_hide_flags: u32,
    pub m_enabled: bool,
    pub m_initialize_on_startup: bool,
    pub m_test_mode: bool,
    pub m_test_event_url: String,
    pub m_test_config_url: String,
}

impl crate::Parse for UnityAnalyticsManager {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_enabled = r.read_bool()?;
        let m_initialize_on_startup = r.read_bool()?;
        let m_test_mode = r.read_bool()?;
        r.align(4)?;
        let m_test_event_url = r.read_aligned_string()?;
        r.align(4)?;
        let m_test_config_url = r.read_aligned_string()?;
        r.align(4)?;

        Ok(UnityAnalyticsManager {
            m_object_hide_flags,
            m_enabled,
            m_initialize_on_startup,
            m_test_mode,
            m_test_event_url,
            m_test_config_url,
        })
    }
}
