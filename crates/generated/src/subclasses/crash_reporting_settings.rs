//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct CrashReportingSettings {
    pub m_event_url: String,
    pub m_enabled: bool,
}

impl crate::Parse for CrashReportingSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_event_url = r.read_aligned_string()?;
        r.align(4)?;
        let m_enabled = r.read_bool()?;
        r.align(4)?;

        Ok(CrashReportingSettings {
            m_event_url,
            m_enabled,
        })
    }
}
