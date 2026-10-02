//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct ConditionConstant {
    pub m_condition_mode: u32,
    pub m_event_id: u32,
    pub m_event_threshold: f32,
    pub m_exit_time: f32,
}

impl crate::Parse for ConditionConstant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_condition_mode = rdr.read_u32()?;
        let m_event_id = rdr.read_u32()?;
        let m_event_threshold = rdr.read_f32()?;
        let m_exit_time = rdr.read_f32()?;

        Ok(ConditionConstant {
            m_condition_mode,
            m_event_id,
            m_event_threshold,
            m_exit_time,
        })
    }
}

impl crate::ToValue for ConditionConstant {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "m_ConditionMode".to_string(),
                self.m_condition_mode.to_value(),
            )),
            Some(("m_EventID".to_string(), self.m_event_id.to_value())),
            Some((
                "m_EventThreshold".to_string(),
                self.m_event_threshold.to_value(),
            )),
            Some(("m_ExitTime".to_string(), self.m_exit_time.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_ConditionMode" => (self.m_condition_mode).inspect_value(path, offset, limit),
                "m_EventID" => (self.m_event_id).inspect_value(path, offset, limit),
                "m_EventThreshold" => (self.m_event_threshold).inspect_value(path, offset, limit),
                "m_ExitTime" => (self.m_exit_time).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_condition_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_ConditionMode".to_string(), v)),
            ((self.m_event_id).inspect_value(path, offset, limit))
                .map(|v| ("m_EventID".to_string(), v)),
            ((self.m_event_threshold).inspect_value(path, offset, limit))
                .map(|v| ("m_EventThreshold".to_string(), v)),
            ((self.m_exit_time).inspect_value(path, offset, limit))
                .map(|v| ("m_ExitTime".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
