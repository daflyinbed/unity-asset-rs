//! 自动生成。请勿手动修改。

use crate::subclasses::offset_ptr::OffsetPtr;

#[derive(Debug, Clone)]
pub struct StateMachineConstant {
    pub m_state_constant_array: Vec<OffsetPtr>,
    pub m_any_state_transition_constant_array: Vec<OffsetPtr>,
    pub m_default_state: u32,
    pub m_motion_set_count: u32,
}

impl crate::Parse for StateMachineConstant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_state_constant_array = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { OffsetPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_any_state_transition_constant_array = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { OffsetPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_default_state = r.read_u32()?;
        let m_motion_set_count = r.read_u32()?;

        Ok(StateMachineConstant {
            m_state_constant_array,
            m_any_state_transition_constant_array,
            m_default_state,
            m_motion_set_count,
        })
    }
}
