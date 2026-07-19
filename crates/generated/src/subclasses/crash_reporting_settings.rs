//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct CrashReportingSettings {
    pub m_event_url: String,
    pub m_enabled: Option<bool>,
    pub m_native_event_url: Option<String>,
    pub m_enable_cloud_diagnostics_reporting: Option<bool>,
    pub m_log_buffer_size: Option<u32>,
}

impl crate::Parse for CrashReportingSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_event_url = None;
        let mut m_enabled = None;
        let mut m_native_event_url = None;
        let mut m_enable_cloud_diagnostics_reporting = None;
        let mut m_log_buffer_size = None;
        m_event_url = Some(rdr.read_aligned_string()?);
        if *ver >= crate::UnityVersion::new(6000, 2, 0) {
            m_enable_cloud_diagnostics_reporting = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_log_buffer_size = Some(rdr.read_u32()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(6000, 1, 0) {
            m_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_log_buffer_size = Some(rdr.read_u32()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(6000, 0, 70) {
            m_enable_cloud_diagnostics_reporting = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_log_buffer_size = Some(rdr.read_u32()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            m_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_log_buffer_size = Some(rdr.read_u32()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2022, 3, 74) {
            m_enable_cloud_diagnostics_reporting = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_log_buffer_size = Some(rdr.read_u32()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            m_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_log_buffer_size = Some(rdr.read_u32()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_native_event_url = Some(rdr.read_aligned_string()?);
            m_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
            m_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
        }

        Ok(CrashReportingSettings {
            m_event_url: m_event_url.unwrap(),
            m_enabled,
            m_native_event_url,
            m_enable_cloud_diagnostics_reporting,
            m_log_buffer_size,
        })
    }
}
