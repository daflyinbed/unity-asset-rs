//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::animator_condition::AnimatorCondition;
use crate::subclasses::condition::Condition;


/// 版本枚举：TransitionMConditionsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum TransitionMConditionsVariant {
    Default(Vec<Condition>),
    V5_0(Vec<AnimatorCondition>),
}

impl crate::Parse for TransitionMConditionsVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { AnimatorCondition::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Ok(Self::V5_0(v))
        } else {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Condition::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：TransitionMDstStateVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum TransitionMDstStateVariant {
    Default(PPtr),
    V5_0(PPtr),
}

impl crate::Parse for TransitionMDstStateVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = crate::PPtr::parse(r, ver)?;
            Ok(Self::V5_0(v))
        } else {
            let v = crate::PPtr::parse(r, ver)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：TransitionMHasExitTimeVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum TransitionMHasExitTimeVariant {
    Default(bool),
    V5_1(bool),
}

impl crate::Parse for TransitionMHasExitTimeVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 1, 0) {
            let v = r.read_bool()?;
            Ok(Self::V5_1(v))
        } else {
            let v = r.read_bool()?;
            r.align(4)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct Transition {
    pub m_name: String,
    pub m_src_state: Option<PPtr>,
    pub m_conditions: TransitionMConditionsVariant,
    pub m_dst_state: TransitionMDstStateVariant,
    pub m_dst_state_machine: Option<PPtr>,
    pub m_transition_duration: f32,
    pub m_transition_offset: f32,
    pub m_solo: bool,
    pub m_mute: bool,
    pub m_atomic: Option<bool>,
    pub m_is_exit: Option<bool>,
    pub m_can_transition_to_self: Option<bool>,
    pub m_exit_time: Option<f32>,
    pub m_has_exit_time: Option<TransitionMHasExitTimeVariant>,
    pub m_interruption_source: Option<i32>,
    pub m_has_fixed_duration: Option<bool>,
    pub m_ordered_interruption: Option<bool>,
}

impl crate::Parse for Transition {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut m_src_state = None;
        let mut m_conditions = None;
        let mut m_dst_state = None;
        let mut m_dst_state_machine = None;
        let mut m_transition_duration = None;
        let mut m_transition_offset = None;
        let mut m_solo = None;
        let mut m_mute = None;
        let mut m_atomic = None;
        let mut m_is_exit = None;
        let mut m_can_transition_to_self = None;
        let mut m_exit_time = None;
        let mut m_has_exit_time = None;
        let mut m_interruption_source = None;
        let mut m_has_fixed_duration = None;
        let mut m_ordered_interruption = None;
            m_name = Some(r.read_aligned_string()?);
            r.align(4)?;
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
        } else {
            m_src_state = Some(crate::PPtr::parse(r, ver)?);
            m_dst_state = Some(TransitionMDstStateVariant::parse(r, ver)?);
            m_transition_duration = Some(r.read_f32()?);
            m_transition_offset = Some(r.read_f32()?);
        }
            m_conditions = Some(TransitionMConditionsVariant::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_dst_state_machine = Some(crate::PPtr::parse(r, ver)?);
            m_dst_state = Some(TransitionMDstStateVariant::parse(r, ver)?);
        } else {
            m_atomic = Some(r.read_bool()?);
        }
            m_solo = Some(r.read_bool()?);
            m_mute = Some(r.read_bool()?);
        if *ver >= crate::UnityVersion::new(5, 1, 0) {
            m_is_exit = Some(r.read_bool()?);
            r.align(4)?;
            m_transition_duration = Some(r.read_f32()?);
            m_transition_offset = Some(r.read_f32()?);
            m_exit_time = Some(r.read_f32()?);
            m_has_exit_time = Some(TransitionMHasExitTimeVariant::parse(r, ver)?);
            m_has_fixed_duration = Some(r.read_bool()?);
            r.align(4)?;
            m_interruption_source = Some(r.read_i32()?);
            m_ordered_interruption = Some(r.read_bool()?);
            m_can_transition_to_self = Some(r.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_is_exit = Some(r.read_bool()?);
            r.align(4)?;
            m_transition_duration = Some(r.read_f32()?);
            m_transition_offset = Some(r.read_f32()?);
            m_exit_time = Some(r.read_f32()?);
            m_has_exit_time = Some(TransitionMHasExitTimeVariant::parse(r, ver)?);
            m_interruption_source = Some(r.read_i32()?);
            m_ordered_interruption = Some(r.read_bool()?);
            m_can_transition_to_self = Some(r.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(4, 5, 0) {
            m_can_transition_to_self = Some(r.read_bool()?);
        } else {
        }

        Ok(Transition {
            m_name: m_name.unwrap(),
            m_src_state,
            m_conditions: m_conditions.unwrap(),
            m_dst_state: m_dst_state.unwrap(),
            m_dst_state_machine,
            m_transition_duration: m_transition_duration.unwrap(),
            m_transition_offset: m_transition_offset.unwrap(),
            m_solo: m_solo.unwrap(),
            m_mute: m_mute.unwrap(),
            m_atomic,
            m_is_exit,
            m_can_transition_to_self,
            m_exit_time,
            m_has_exit_time,
            m_interruption_source,
            m_has_fixed_duration,
            m_ordered_interruption,
        })
    }
}
