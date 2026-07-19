//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::child_animator_state::ChildAnimatorState;
use crate::subclasses::child_animator_state_machine::ChildAnimatorStateMachine;
use crate::subclasses::vector3f::Vector3f;

/// 版本枚举：AnimatorStateMachineMDefaultStateVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AnimatorStateMachineMDefaultStateVariant {
    Default(PPtr),
    V5_0(PPtr),
    V6000_4(PPtr),
}

impl crate::Parse for AnimatorStateMachineMDefaultStateVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(6000, 4, 0) {
            let v = crate::PPtr::parse(rdr, ver)?;
            rdr.align(4)?;
            Ok(Self::V6000_4(v))
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = crate::PPtr::parse(rdr, ver)?;
            Ok(Self::V5_0(v))
        } else {
            let v = crate::PPtr::parse(rdr, ver)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：AnimatorStateMachineMChildStatesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AnimatorStateMachineMChildStatesVariant {
    Default(Vec<ChildAnimatorState>),
    V2017_1(Vec<ChildAnimatorState>),
}

impl crate::Parse for AnimatorStateMachineMChildStatesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| ChildAnimatorState::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| ChildAnimatorState::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：AnimatorStateMachineMChildStateMachinesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AnimatorStateMachineMChildStateMachinesVariant {
    Default(Vec<ChildAnimatorStateMachine>),
    V2017_1(Vec<ChildAnimatorStateMachine>),
}

impl crate::Parse for AnimatorStateMachineMChildStateMachinesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| ChildAnimatorStateMachine::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| ChildAnimatorStateMachine::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：AnimatorStateMachineMAnyStateTransitionsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AnimatorStateMachineMAnyStateTransitionsVariant {
    Default(Vec<PPtr>),
    V2017_1(Vec<PPtr>),
}

impl crate::Parse for AnimatorStateMachineMAnyStateTransitionsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| crate::PPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| crate::PPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：AnimatorStateMachineMEntryTransitionsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AnimatorStateMachineMEntryTransitionsVariant {
    Default(Vec<PPtr>),
    V2017_1(Vec<PPtr>),
}

impl crate::Parse for AnimatorStateMachineMEntryTransitionsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| crate::PPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| crate::PPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：AnimatorStateMachineMStateMachineBehavioursVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AnimatorStateMachineMStateMachineBehavioursVariant {
    Default(Vec<PPtr>),
    V2017_1(Vec<PPtr>),
}

impl crate::Parse for AnimatorStateMachineMStateMachineBehavioursVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| crate::PPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| crate::PPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct AnimatorStateMachine {
    pub m_name: String,
    pub m_default_state: AnimatorStateMachineMDefaultStateVariant,
    pub m_child_states: Option<AnimatorStateMachineMChildStatesVariant>,
    pub m_states: Option<Vec<PPtr>>,
    pub m_child_state_machines: Option<AnimatorStateMachineMChildStateMachinesVariant>,
    pub m_child_state_machine: Option<Vec<PPtr>>,
    pub m_any_state_transitions: Option<AnimatorStateMachineMAnyStateTransitionsVariant>,
    pub m_child_state_machine_position: Option<Vec<Vector3f>>,
    pub m_entry_transitions: Option<AnimatorStateMachineMEntryTransitionsVariant>,
    pub m_local_transitions: Option<Vec<(PPtr, Vec<PPtr>)>>,
    pub m_ordered_transitions: Option<Vec<(PPtr, Vec<PPtr>)>>,
    pub m_state_machine_transitions: Option<Vec<(PPtr, Vec<PPtr>)>>,
    pub m_motion_set_count: Option<i32>,
    pub m_state_machine_behaviours: Option<AnimatorStateMachineMStateMachineBehavioursVariant>,
    pub m_any_state_position: Vector3f,
    pub m_parent_state_machine_position: Vector3f,
    pub m_entry_position: Option<Vector3f>,
    pub m_exit_position: Option<Vector3f>,
}

impl crate::Parse for AnimatorStateMachine {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
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
        m_name = Some(rdr.read_aligned_string()?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_child_states = Some(AnimatorStateMachineMChildStatesVariant::parse(rdr, ver)?);
            m_child_state_machines = Some(AnimatorStateMachineMChildStateMachinesVariant::parse(
                rdr, ver,
            )?);
            m_any_state_transitions = Some(AnimatorStateMachineMAnyStateTransitionsVariant::parse(
                rdr, ver,
            )?);
            m_entry_transitions = Some(AnimatorStateMachineMEntryTransitionsVariant::parse(
                rdr, ver,
            )?);
            m_state_machine_transitions = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = crate::PPtr::parse(rdr, ver)?;
                                let b = ({
                                    let n = rdr.read_i32()? as usize;
                                    (0..n)
                                        .map(|_| crate::PPtr::parse(rdr, ver))
                                        .collect::<Result<Vec<_>, _>>()
                                })?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_state_machine_behaviours = Some(
                AnimatorStateMachineMStateMachineBehavioursVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(4, 2, 0) {
            m_default_state = Some(AnimatorStateMachineMDefaultStateVariant::parse(rdr, ver)?);
            m_states = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| crate::PPtr::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_child_state_machine = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| crate::PPtr::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_child_state_machine_position = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| Vector3f::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_ordered_transitions = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = crate::PPtr::parse(rdr, ver)?;
                                let b = ({
                                    let n = rdr.read_i32()? as usize;
                                    (0..n)
                                        .map(|_| crate::PPtr::parse(rdr, ver))
                                        .collect::<Result<Vec<_>, _>>()
                                })?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_motion_set_count = Some(rdr.read_i32()?);
        } else {
            m_default_state = Some(AnimatorStateMachineMDefaultStateVariant::parse(rdr, ver)?);
            m_states = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| crate::PPtr::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_child_state_machine = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| crate::PPtr::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_child_state_machine_position = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| Vector3f::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_local_transitions = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = crate::PPtr::parse(rdr, ver)?;
                                let b = ({
                                    let n = rdr.read_i32()? as usize;
                                    (0..n)
                                        .map(|_| crate::PPtr::parse(rdr, ver))
                                        .collect::<Result<Vec<_>, _>>()
                                })?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_ordered_transitions = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = crate::PPtr::parse(rdr, ver)?;
                                let b = ({
                                    let n = rdr.read_i32()? as usize;
                                    (0..n)
                                        .map(|_| crate::PPtr::parse(rdr, ver))
                                        .collect::<Result<Vec<_>, _>>()
                                })?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_motion_set_count = Some(rdr.read_i32()?);
        }
        m_any_state_position = Some(Vector3f::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_entry_position = Some(Vector3f::parse(rdr, ver)?);
            m_exit_position = Some(Vector3f::parse(rdr, ver)?);
        } else {
        }
        m_parent_state_machine_position = Some(Vector3f::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_default_state = Some(AnimatorStateMachineMDefaultStateVariant::parse(rdr, ver)?);
        } else {
        }

        Ok(AnimatorStateMachine {
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
