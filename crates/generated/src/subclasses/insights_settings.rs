//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct InsightsSettings {
    pub m_event_url: String,
    pub m_stack_trace_url: String,
    pub m_engine_diagnostics_enabled: bool,
    pub m_enabled: bool,
}

impl crate::Parse for InsightsSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_event_url = rdr.read_aligned_string()?;
        rdr.align(4)?;
        let m_stack_trace_url = rdr.read_aligned_string()?;
        rdr.align(4)?;
        let m_engine_diagnostics_enabled = rdr.read_bool()?;
        let m_enabled = rdr.read_bool()?;
        rdr.align(4)?;

        Ok(InsightsSettings {
            m_event_url,
            m_stack_trace_url,
            m_engine_diagnostics_enabled,
            m_enabled,
        })
    }
}
