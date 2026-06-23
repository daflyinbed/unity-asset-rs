//! 自动生成。请勿手动修改。

use crate::subclasses::offset_ptr::OffsetPtr;

#[derive(Debug, Clone)]
pub struct TransitionConstant {
    pub m_condition_constant_array: Vec<OffsetPtr>,
    pub m_destination_state: u32,
    pub m_id: u32,
    pub m_user_id: u32,
    pub m_transition_duration: f32,
    pub m_transition_offset: f32,
    pub m_atomic: bool,
}

impl crate::Parse for TransitionConstant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_condition_constant_array = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { OffsetPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_destination_state = r.read_u32()?;
        let m_id = r.read_u32()?;
        let m_user_id = r.read_u32()?;
        let m_transition_duration = r.read_f32()?;
        let m_transition_offset = r.read_f32()?;
        let m_atomic = r.read_bool()?;
        r.align(4)?;

        Ok(TransitionConstant {
            m_condition_constant_array,
            m_destination_state,
            m_id,
            m_user_id,
            m_transition_duration,
            m_transition_offset,
            m_atomic,
        })
    }
}
