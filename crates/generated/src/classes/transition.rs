//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::condition::Condition;
use super::animator_transition_base::AnimatorTransitionBase;
use super::named_object::NamedObject;

#[derive(Debug, Clone)]
pub enum TransitionParent {
    NamedObject(NamedObject),
    AnimatorTransitionBase(AnimatorTransitionBase),
}

#[derive(Debug, Clone)]
pub struct Transition {
    pub parent: TransitionParent,
    pub m_name: String,
    pub m_src_state: Option<PPtr>,
    pub m_conditions: Vec<Condition>,
    pub m_dst_state: PPtr,
    pub m_dst_state_machine: Option<PPtr>,
    pub m_transition_duration: f32,
    pub m_transition_offset: f32,
    pub m_solo: bool,
    pub m_mute: bool,
    pub m_atomic: Option<bool>,
    pub m_is_exit: Option<bool>,
    pub m_can_transition_to_self: Option<bool>,
    pub m_exit_time: Option<f32>,
    pub m_has_exit_time: Option<bool>,
    pub m_interruption_source: Option<i32>,
    pub m_has_fixed_duration: Option<bool>,
    pub m_ordered_interruption: Option<bool>,
}

impl crate::Parse for Transition {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            TransitionParent::NamedObject(NamedObject::parse(r, ver)?)
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            TransitionParent::AnimatorTransitionBase(AnimatorTransitionBase::parse(r, ver)?)
        } else {
            panic!("无匹配的父类分支：version = {:?}", ver)
        };
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_src_state = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            Some(crate::PPtr::parse(r, ver)?)
        } else {
            None
        };
        let m_conditions = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Condition::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_dst_state = crate::PPtr::parse(r, ver)?;
        let m_dst_state_machine = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(crate::PPtr::parse(r, ver)?)
        } else {
            None
        };
        let m_transition_duration = r.read_f32()?;
        let m_transition_offset = r.read_f32()?;
        let m_solo = r.read_bool()?;
        let m_mute = r.read_bool()?;
        let m_atomic = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_is_exit = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_can_transition_to_self = if *ver >= crate::UnityVersion::new(4, 5, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_exit_time = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_has_exit_time = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_interruption_source = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_has_fixed_duration = if *ver >= crate::UnityVersion::new(5, 1, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_ordered_interruption = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };

        Ok(Transition {
            parent,
            m_name,
            m_src_state,
            m_conditions,
            m_dst_state,
            m_dst_state_machine,
            m_transition_duration,
            m_transition_offset,
            m_solo,
            m_mute,
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
