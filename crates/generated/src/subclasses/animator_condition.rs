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
