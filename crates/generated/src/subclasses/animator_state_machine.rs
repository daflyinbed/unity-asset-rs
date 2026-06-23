//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::child_animator_state::ChildAnimatorState;
use crate::subclasses::child_animator_state_machine::ChildAnimatorStateMachine;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct AnimatorStateMachine {
    pub m_object_hide_flags: u32,
    pub m_prefab_parent_object: PPtr,
    pub m_prefab_internal: PPtr,
    pub m_name: String,
    pub m_child_states: Vec<ChildAnimatorState>,
    pub m_child_state_machines: Vec<ChildAnimatorStateMachine>,
    pub m_any_state_transitions: Vec<PPtr>,
    pub m_entry_transitions: Vec<PPtr>,
    pub m_state_machine_transitions: Vec<(PPtr, Vec<PPtr>)>,
    pub m_state_machine_behaviours: Vec<PPtr>,
    pub m_any_state_position: Vector3f,
    pub m_entry_position: Vector3f,
    pub m_exit_position: Vector3f,
    pub m_parent_state_machine_position: Vector3f,
    pub m_default_state: PPtr,
}

impl crate::Parse for AnimatorStateMachine {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_prefab_parent_object = crate::PPtr::parse(r, ver)?;
        let m_prefab_internal = crate::PPtr::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_child_states = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ChildAnimatorState::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_child_state_machines = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ChildAnimatorStateMachine::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_any_state_transitions = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_entry_transitions = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_state_machine_transitions = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = crate::PPtr::parse(r, ver)?; let b = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_state_machine_behaviours = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_any_state_position = Vector3f::parse(r, ver)?;
        let m_entry_position = Vector3f::parse(r, ver)?;
        let m_exit_position = Vector3f::parse(r, ver)?;
        let m_parent_state_machine_position = Vector3f::parse(r, ver)?;
        let m_default_state = crate::PPtr::parse(r, ver)?;

        Ok(AnimatorStateMachine {
            m_object_hide_flags,
            m_prefab_parent_object,
            m_prefab_internal,
            m_name,
            m_child_states,
            m_child_state_machines,
            m_any_state_transitions,
            m_entry_transitions,
            m_state_machine_transitions,
            m_state_machine_behaviours,
            m_any_state_position,
            m_entry_position,
            m_exit_position,
            m_parent_state_machine_position,
            m_default_state,
        })
    }
}
