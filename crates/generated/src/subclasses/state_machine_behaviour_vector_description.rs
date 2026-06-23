//! 自动生成。请勿手动修改。

use crate::subclasses::state_key::StateKey;
use crate::subclasses::state_range::StateRange;

#[derive(Debug, Clone)]
pub struct StateMachineBehaviourVectorDescription {
    pub m_state_machine_behaviour_ranges: Vec<(StateKey, StateRange)>,
    pub m_state_machine_behaviour_indices: Vec<u32>,
}

impl crate::Parse for StateMachineBehaviourVectorDescription {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_state_machine_behaviour_ranges = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = StateKey::parse(r, ver)?; let b = StateRange::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_state_machine_behaviour_indices = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u32() }).collect::<Result<Vec<_>, _>>() } )?;

        Ok(StateMachineBehaviourVectorDescription {
            m_state_machine_behaviour_ranges,
            m_state_machine_behaviour_indices,
        })
    }
}
