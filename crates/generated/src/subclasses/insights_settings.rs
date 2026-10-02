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
        let m_stack_trace_url = rdr.read_aligned_string()?;
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

impl crate::ToValue for InsightsSettings {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_EventUrl".to_string(), self.m_event_url.to_value())),
            Some((
                "m_StackTraceUrl".to_string(),
                self.m_stack_trace_url.to_value(),
            )),
            Some((
                "m_EngineDiagnosticsEnabled".to_string(),
                self.m_engine_diagnostics_enabled.to_value(),
            )),
            Some(("m_Enabled".to_string(), self.m_enabled.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_EventUrl" => (self.m_event_url).inspect_value(path, offset, limit),
                "m_StackTraceUrl" => (self.m_stack_trace_url).inspect_value(path, offset, limit),
                "m_EngineDiagnosticsEnabled" => {
                    (self.m_engine_diagnostics_enabled).inspect_value(path, offset, limit)
                }
                "m_Enabled" => (self.m_enabled).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_event_url).inspect_value(path, offset, limit))
                .map(|v| ("m_EventUrl".to_string(), v)),
            ((self.m_stack_trace_url).inspect_value(path, offset, limit))
                .map(|v| ("m_StackTraceUrl".to_string(), v)),
            ((self.m_engine_diagnostics_enabled).inspect_value(path, offset, limit))
                .map(|v| ("m_EngineDiagnosticsEnabled".to_string(), v)),
            ((self.m_enabled).inspect_value(path, offset, limit))
                .map(|v| ("m_Enabled".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
