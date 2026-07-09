//! 自动生成。请勿手动修改。

use crate::subclasses::offset_ptr::OffsetPtr;

#[derive(Debug, Clone)]
pub struct StateMachineConstant {
    pub m_state_constant_array: Vec<OffsetPtr>,
    pub m_any_state_transition_constant_array: Vec<OffsetPtr>,
    pub m_default_state: u32,
    pub m_selector_state_constant_array: Option<Vec<OffsetPtr>>,
    pub m_synchronized_layer_count: u32,
    pub m_evaluate_transitions_on_start: Option<bool>,
}

impl crate::Parse for StateMachineConstant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_state_constant_array = None;
        let mut m_any_state_transition_constant_array = None;
        let mut m_default_state = None;
        let mut m_selector_state_constant_array = None;
        let mut m_synchronized_layer_count = None;
        let mut m_evaluate_transitions_on_start = None;
            m_state_constant_array = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { OffsetPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            m_any_state_transition_constant_array = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { OffsetPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_selector_state_constant_array = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { OffsetPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
        } else {
        }
            m_default_state = Some(rdr.read_u32()?);
            m_synchronized_layer_count = Some(rdr.read_u32()?);
        if *ver >= crate::UnityVersion::new(6000, 4, 0) {
            m_evaluate_transitions_on_start = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
        }

        Ok(StateMachineConstant {
            m_state_constant_array: m_state_constant_array.unwrap(),
            m_any_state_transition_constant_array: m_any_state_transition_constant_array.unwrap(),
            m_default_state: m_default_state.unwrap(),
            m_selector_state_constant_array,
            m_synchronized_layer_count: m_synchronized_layer_count.unwrap(),
            m_evaluate_transitions_on_start,
        })
    }
}
