//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct StateMachine {
    pub m_object_hide_flags: u32,
    pub m_prefab_parent_object: PPtr,
    pub m_prefab_internal: PPtr,
    pub m_name: String,
    pub m_default_state: PPtr,
    pub m_states: Vec<PPtr>,
    pub m_child_state_machine: Vec<PPtr>,
    pub m_child_state_machine_position: Vec<Vector3f>,
    pub m_local_transitions: Vec<(PPtr, Vec<PPtr>)>,
    pub m_ordered_transitions: Vec<(PPtr, Vec<PPtr>)>,
    pub m_motion_set_count: i32,
    pub m_any_state_position: Vector3f,
    pub m_parent_state_machine_position: Vector3f,
}

impl crate::Parse for StateMachine {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_prefab_parent_object = crate::PPtr::parse(r, ver)?;
        let m_prefab_internal = crate::PPtr::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_default_state = crate::PPtr::parse(r, ver)?;
        let m_states = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_child_state_machine = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_child_state_machine_position = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Vector3f::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_local_transitions = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = crate::PPtr::parse(r, ver)?; let b = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_ordered_transitions = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = crate::PPtr::parse(r, ver)?; let b = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_motion_set_count = r.read_i32()?;
        let m_any_state_position = Vector3f::parse(r, ver)?;
        let m_parent_state_machine_position = Vector3f::parse(r, ver)?;

        Ok(StateMachine {
            m_object_hide_flags,
            m_prefab_parent_object,
            m_prefab_internal,
            m_name,
            m_default_state,
            m_states,
            m_child_state_machine,
            m_child_state_machine_position,
            m_local_transitions,
            m_ordered_transitions,
            m_motion_set_count,
            m_any_state_position,
            m_parent_state_machine_position,
        })
    }
}
