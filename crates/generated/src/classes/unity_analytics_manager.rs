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
        let m_enabled = if *ver >= crate::UnityVersion::new(5, 2, 0) {
            Some(rdr.read_bool()?)
        } else {
            None
        };
        let m_initialize_on_startup = if *ver >= crate::UnityVersion::new(5, 2, 0) {
            Some(rdr.read_bool()?)
        } else {
            None
        };
        let m_test_mode = if *ver >= crate::UnityVersion::new(5, 2, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_test_event_url = if *ver >= crate::UnityVersion::new(5, 2, 0) {
            let v = rdr.read_aligned_string()?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_test_config_url = if *ver >= crate::UnityVersion::new(5, 2, 0) {
            let v = rdr.read_aligned_string()?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(UnityAnalyticsManager {
            m_enabled,
            m_initialize_on_startup,
            m_test_mode,
            m_test_event_url,
            m_test_config_url,
        })
    }
}
