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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_condition_mode = r.read_u32()?;
        let m_event_id = r.read_u32()?;
        let m_event_threshold = r.read_f32()?;
        let m_exit_time = r.read_f32()?;

        Ok(ConditionConstant {
            m_condition_mode,
            m_event_id,
            m_event_threshold,
            m_exit_time,
        })
    }
}
