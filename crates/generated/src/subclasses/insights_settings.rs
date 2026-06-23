//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct InsightsSettings {
    pub m_engine_diagnostics_enabled: bool,
    pub m_enabled: bool,
}

impl crate::Parse for InsightsSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_engine_diagnostics_enabled = r.read_bool()?;
        let m_enabled = r.read_bool()?;
        r.align(4)?;

        Ok(InsightsSettings {
            m_engine_diagnostics_enabled,
            m_enabled,
        })
    }
}
