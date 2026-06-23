//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::child_animator_state::ChildAnimatorState;
use crate::subclasses::child_animator_state_machine::ChildAnimatorStateMachine;
use crate::subclasses::vector3f::Vector3f;
use super::named_object::NamedObject;

#[derive(Debug, Clone)]
pub struct StateMachine {
    pub parent: NamedObject,
    pub m_name: String,
    pub m_default_state: PPtr,
    pub m_child_states: Option<Vec<ChildAnimatorState>>,
    pub m_states: Option<Vec<PPtr>>,
    pub m_child_state_machines: Option<Vec<ChildAnimatorStateMachine>>,
    pub m_child_state_machine: Option<Vec<PPtr>>,
    pub m_any_state_transitions: Option<Vec<PPtr>>,
    pub m_child_state_machine_position: Option<Vec<Vector3f>>,
    pub m_entry_transitions: Option<Vec<PPtr>>,
    pub m_local_transitions: Option<Vec<(PPtr, Vec<PPtr>)>>,
    pub m_ordered_transitions: Option<Vec<(PPtr, Vec<PPtr>)>>,
    pub m_state_machine_transitions: Option<Vec<(PPtr, Vec<PPtr>)>>,
    pub m_motion_set_count: Option<i32>,
    pub m_state_machine_behaviours: Option<Vec<PPtr>>,
    pub m_any_state_position: Vector3f,
    pub m_parent_state_machine_position: Vector3f,
    pub m_entry_position: Option<Vector3f>,
    pub m_exit_position: Option<Vector3f>,
}

impl crate::Parse for StateMachine {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = NamedObject::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_default_state = crate::PPtr::parse(r, ver)?;
        let m_child_states = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ChildAnimatorState::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };
        let m_states = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };
        let m_child_state_machines = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ChildAnimatorStateMachine::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };
        let m_child_state_machine = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };
        let m_any_state_transitions = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };
        let m_child_state_machine_position = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { Vector3f::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };
        let m_entry_transitions = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };
        let m_local_transitions = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = crate::PPtr::parse(r, ver)?; let b = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };
        let m_ordered_transitions = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = crate::PPtr::parse(r, ver)?; let b = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };
        let m_state_machine_transitions = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = crate::PPtr::parse(r, ver)?; let b = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };
        let m_motion_set_count = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_state_machine_behaviours = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };
        let m_any_state_position = Vector3f::parse(r, ver)?;
        let m_parent_state_machine_position = Vector3f::parse(r, ver)?;
        let m_entry_position = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(Vector3f::parse(r, ver)?)
        } else {
            None
        };
        let m_exit_position = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(Vector3f::parse(r, ver)?)
        } else {
            None
        };

        Ok(StateMachine {
            parent,
            m_name,
            m_default_state,
            m_child_states,
            m_states,
            m_child_state_machines,
            m_child_state_machine,
            m_any_state_transitions,
            m_child_state_machine_position,
            m_entry_transitions,
            m_local_transitions,
            m_ordered_transitions,
            m_state_machine_transitions,
            m_motion_set_count,
            m_state_machine_behaviours,
            m_any_state_position,
            m_parent_state_machine_position,
            m_entry_position,
            m_exit_position,
        })
    }
}
