//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct PerformanceReportingSettings {
    pub m_enabled: bool,
}

impl crate::Parse for PerformanceReportingSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_enabled = r.read_bool()?;
        r.align(4)?;

        Ok(PerformanceReportingSettings {
            m_enabled,
        })
    }
}
