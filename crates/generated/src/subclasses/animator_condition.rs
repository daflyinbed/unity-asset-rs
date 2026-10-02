//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct AnimatorCondition {
    pub m_condition_mode: i32,
    pub m_condition_event: String,
    pub m_event_treshold: f32,
    pub m_exit_time: Option<f32>,
}

impl crate::Parse for AnimatorCondition {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_condition_mode = None;
        let mut m_condition_event = None;
        let mut m_event_treshold = None;
        let mut m_exit_time = None;
        m_condition_mode = Some(rdr.read_i32()?);
        m_condition_event = Some(rdr.read_aligned_string()?);
        m_event_treshold = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
        } else {
            m_exit_time = Some(rdr.read_f32()?);
        }

        Ok(AnimatorCondition {
            m_condition_mode: m_condition_mode.unwrap(),
            m_condition_event: m_condition_event.unwrap(),
            m_event_treshold: m_event_treshold.unwrap(),
            m_exit_time,
        })
    }
}

impl crate::ToValue for AnimatorCondition {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "m_ConditionMode".to_string(),
                self.m_condition_mode.to_value(),
            )),
            Some((
                "m_ConditionEvent".to_string(),
                self.m_condition_event.to_value(),
            )),
            Some((
                "m_EventTreshold".to_string(),
                self.m_event_treshold.to_value(),
            )),
            crate::opt_field("m_ExitTime", &self.m_exit_time),
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
                "m_ConditionEvent" => (self.m_condition_event).inspect_value(path, offset, limit),
                "m_EventTreshold" => (self.m_event_treshold).inspect_value(path, offset, limit),
                "m_ExitTime" => (self.m_exit_time).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_condition_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_ConditionMode".to_string(), v)),
            ((self.m_condition_event).inspect_value(path, offset, limit))
                .map(|v| ("m_ConditionEvent".to_string(), v)),
            ((self.m_event_treshold).inspect_value(path, offset, limit))
                .map(|v| ("m_EventTreshold".to_string(), v)),
            ((self.m_exit_time).inspect_value(path, offset, limit))
                .map(|v| ("m_ExitTime".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
