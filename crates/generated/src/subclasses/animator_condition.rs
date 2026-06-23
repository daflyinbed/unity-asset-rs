//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct AnimatorCondition {
    pub m_condition_mode: i32,
    pub m_condition_event: String,
    pub m_event_treshold: f32,
}

impl crate::Parse for AnimatorCondition {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_condition_mode = r.read_i32()?;
        let m_condition_event = r.read_aligned_string()?;
        r.align(4)?;
        let m_event_treshold = r.read_f32()?;

        Ok(AnimatorCondition {
            m_condition_mode,
            m_condition_event,
            m_event_treshold,
        })
    }
}
