//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::child_animator_state::ChildAnimatorState;
use crate::subclasses::child_animator_state_machine::ChildAnimatorStateMachine;
use crate::subclasses::vector3f::Vector3f;


/// 版本枚举：StateMachineMDefaultStateVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum StateMachineMDefaultStateVariant {
    Default(PPtr),
    V5_0(PPtr),
    V6000_4(PPtr),
}

impl crate::Parse for StateMachineMDefaultStateVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(6000, 4, 0) {
            let v = crate::PPtr::parse(r, ver)?;
            r.align(4)?;
            Ok(Self::V6000_4(v))
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = crate::PPtr::parse(r, ver)?;
            Ok(Self::V5_0(v))
        } else {
            let v = crate::PPtr::parse(r, ver)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：StateMachineMChildStatesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum StateMachineMChildStatesVariant {
    Default(Vec<ChildAnimatorState>),
    V2017_1(Vec<ChildAnimatorState>),
}

impl crate::Parse for StateMachineMChildStatesVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ChildAnimatorState::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ChildAnimatorState::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：StateMachineMChildStateMachinesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum StateMachineMChildStateMachinesVariant {
    Default(Vec<ChildAnimatorStateMachine>),
    V2017_1(Vec<ChildAnimatorStateMachine>),
}

impl crate::Parse for StateMachineMChildStateMachinesVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ChildAnimatorStateMachine::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ChildAnimatorStateMachine::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：StateMachineMAnyStateTransitionsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum StateMachineMAnyStateTransitionsVariant {
    Default(Vec<PPtr>),
    V2017_1(Vec<PPtr>),
}

impl crate::Parse for StateMachineMAnyStateTransitionsVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：StateMachineMEntryTransitionsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum StateMachineMEntryTransitionsVariant {
    Default(Vec<PPtr>),
    V2017_1(Vec<PPtr>),
}

impl crate::Parse for StateMachineMEntryTransitionsVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：StateMachineMStateMachineTransitionsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum StateMachineMStateMachineTransitionsVariant {
    Default(Vec<(PPtr, Vec<PPtr>)>),
    V2017_1(Vec<(PPtr, Vec<PPtr>)>),
}

impl crate::Parse for StateMachineMStateMachineTransitionsVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = crate::PPtr::parse(r, ver)?; let b = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = crate::PPtr::parse(r, ver)?; let b = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：StateMachineMStateMachineBehavioursVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum StateMachineMStateMachineBehavioursVariant {
    Default(Vec<PPtr>),
    V2017_1(Vec<PPtr>),
}

impl crate::Parse for StateMachineMStateMachineBehavioursVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct StateMachine {
    pub m_name: String,
    pub m_default_state: StateMachineMDefaultStateVariant,
    pub m_child_states: Option<StateMachineMChildStatesVariant>,
    pub m_states: Option<Vec<PPtr>>,
    pub m_child_state_machines: Option<StateMachineMChildStateMachinesVariant>,
    pub m_child_state_machine: Option<Vec<PPtr>>,
    pub m_any_state_transitions: Option<StateMachineMAnyStateTransitionsVariant>,
    pub m_child_state_machine_position: Option<Vec<Vector3f>>,
    pub m_entry_transitions: Option<StateMachineMEntryTransitionsVariant>,
    pub m_local_transitions: Option<Vec<(PPtr, Vec<PPtr>)>>,
    pub m_ordered_transitions: Option<Vec<(PPtr, Vec<PPtr>)>>,
    pub m_state_machine_transitions: Option<StateMachineMStateMachineTransitionsVariant>,
    pub m_motion_set_count: Option<i32>,
    pub m_state_machine_behaviours: Option<StateMachineMStateMachineBehavioursVariant>,
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
        let mut m_name = None;
        let mut m_default_state = None;
        let mut m_child_states = None;
        let mut m_states = None;
        let mut m_child_state_machines = None;
        let mut m_child_state_machine = None;
        let mut m_any_state_transitions = None;
        let mut m_child_state_machine_position = None;
        let mut m_entry_transitions = None;
        let mut m_local_transitions = None;
        let mut m_ordered_transitions = None;
        let mut m_state_machine_transitions = None;
        let mut m_motion_set_count = None;
        let mut m_state_machine_behaviours = None;
        let mut m_any_state_position = None;
        let mut m_parent_state_machine_position = None;
        let mut m_entry_position = None;
        let mut m_exit_position = None;
            m_name = Some(r.read_aligned_string()?);
            r.align(4)?;
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_child_states = Some(StateMachineMChildStatesVariant::parse(r, ver)?);
            m_child_state_machines = Some(StateMachineMChildStateMachinesVariant::parse(r, ver)?);
            m_any_state_transitions = Some(StateMachineMAnyStateTransitionsVariant::parse(r, ver)?);
            m_entry_transitions = Some(StateMachineMEntryTransitionsVariant::parse(r, ver)?);
            m_state_machine_transitions = Some(StateMachineMStateMachineTransitionsVariant::parse(r, ver)?);
            m_state_machine_behaviours = Some(StateMachineMStateMachineBehavioursVariant::parse(r, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 2, 0) {
            m_default_state = Some(StateMachineMDefaultStateVariant::parse(r, ver)?);
            m_states = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            m_child_state_machine = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            m_child_state_machine_position = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { Vector3f::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            m_ordered_transitions = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = crate::PPtr::parse(r, ver)?; let b = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            m_motion_set_count = Some(r.read_i32()?);
        } else {
            m_default_state = Some(StateMachineMDefaultStateVariant::parse(r, ver)?);
            m_states = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            m_child_state_machine = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            m_child_state_machine_position = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { Vector3f::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            m_local_transitions = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = crate::PPtr::parse(r, ver)?; let b = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            m_ordered_transitions = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = crate::PPtr::parse(r, ver)?; let b = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            m_motion_set_count = Some(r.read_i32()?);
        }
            m_any_state_position = Some(Vector3f::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_entry_position = Some(Vector3f::parse(r, ver)?);
            m_exit_position = Some(Vector3f::parse(r, ver)?);
        } else {
        }
            m_parent_state_machine_position = Some(Vector3f::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_default_state = Some(StateMachineMDefaultStateVariant::parse(r, ver)?);
        } else {
        }

        Ok(StateMachine {
            m_name: m_name.unwrap(),
            m_default_state: m_default_state.unwrap(),
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
            m_any_state_position: m_any_state_position.unwrap(),
            m_parent_state_machine_position: m_parent_state_machine_position.unwrap(),
            m_entry_position,
            m_exit_position,
        })
    }
}
