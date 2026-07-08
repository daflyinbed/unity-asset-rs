//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::controller_constant::ControllerConstant;
use crate::subclasses::state_machine_behaviour_vector_description::StateMachineBehaviourVectorDescription;


/// 版本枚举：AnimatorControllerMAnimationClipsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AnimatorControllerMAnimationClipsVariant {
    Default(Vec<PPtr>),
    V5_0(Vec<PPtr>),
    V2017_1(Vec<PPtr>),
}

impl crate::Parse for AnimatorControllerMAnimationClipsVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Ok(Self::V2017_1(v))
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::V5_0(v))
        } else {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：AnimatorControllerMStateMachineBehaviourVectorDescriptionVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AnimatorControllerMStateMachineBehaviourVectorDescriptionVariant {
    Default(StateMachineBehaviourVectorDescription),
    V2017_1(StateMachineBehaviourVectorDescription),
}

impl crate::Parse for AnimatorControllerMStateMachineBehaviourVectorDescriptionVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = StateMachineBehaviourVectorDescription::parse(r, ver)?;
            r.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = StateMachineBehaviourVectorDescription::parse(r, ver)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：AnimatorControllerMStateMachineBehavioursVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AnimatorControllerMStateMachineBehavioursVariant {
    Default(Vec<PPtr>),
    V5_1(Vec<PPtr>),
    V5_2(Vec<PPtr>),
    V5_4(Vec<PPtr>),
    V2017_1(Vec<PPtr>),
}

impl crate::Parse for AnimatorControllerMStateMachineBehavioursVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Ok(Self::V2017_1(v))
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::V5_4(v))
        } else if *ver >= crate::UnityVersion::new(5, 2, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Ok(Self::V5_2(v))
        } else if *ver >= crate::UnityVersion::new(5, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::V5_1(v))
        } else {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：AnimatorControllerMMultiThreadedStateMachineVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AnimatorControllerMMultiThreadedStateMachineVariant {
    Default(bool),
    V5_1(bool),
}

impl crate::Parse for AnimatorControllerMMultiThreadedStateMachineVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 1, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Ok(Self::V5_1(v))
        } else {
            let v = r.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct AnimatorController {
    pub m_name: String,
    pub m_controller_size: u32,
    pub m_evaluate_transitions_on_start: Option<bool>,
    pub m_controller: ControllerConstant,
    pub m_tos: Vec<(u32, String)>,
    pub m_animation_clips: AnimatorControllerMAnimationClipsVariant,
    pub m_state_machine_behaviour_vector_description: Option<AnimatorControllerMStateMachineBehaviourVectorDescriptionVariant>,
    pub m_state_machine_behaviours: Option<AnimatorControllerMStateMachineBehavioursVariant>,
    pub m_multi_threaded_state_machine: Option<AnimatorControllerMMultiThreadedStateMachineVariant>,
}

impl crate::Parse for AnimatorController {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut m_controller_size = None;
        let mut m_evaluate_transitions_on_start = None;
        let mut m_controller = None;
        let mut m_tos = None;
        let mut m_animation_clips = None;
        let mut m_state_machine_behaviour_vector_description = None;
        let mut m_state_machine_behaviours = None;
        let mut m_multi_threaded_state_machine = None;
            m_name = Some(r.read_aligned_string()?);
            r.align(4)?;
        if *ver >= crate::UnityVersion::new(6000, 4, 0) {
            m_evaluate_transitions_on_start = Some(r.read_bool()?);
            r.align(4)?;
        } else {
        }
            m_controller_size = Some(r.read_u32()?);
            m_controller = Some(ControllerConstant::parse(r, ver)?);
            r.align(4)?;
            m_tos = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_u32()?; let b = r.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_animation_clips = Some(AnimatorControllerMAnimationClipsVariant::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            m_state_machine_behaviour_vector_description = Some(AnimatorControllerMStateMachineBehaviourVectorDescriptionVariant::parse(r, ver)?);
            m_state_machine_behaviours = Some(AnimatorControllerMStateMachineBehavioursVariant::parse(r, ver)?);
            m_multi_threaded_state_machine = Some(AnimatorControllerMMultiThreadedStateMachineVariant::parse(r, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 2, 0) {
            m_state_machine_behaviour_vector_description = Some(AnimatorControllerMStateMachineBehaviourVectorDescriptionVariant::parse(r, ver)?);
            m_state_machine_behaviours = Some(AnimatorControllerMStateMachineBehavioursVariant::parse(r, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_state_machine_behaviour_vector_description = Some(AnimatorControllerMStateMachineBehaviourVectorDescriptionVariant::parse(r, ver)?);
            m_state_machine_behaviours = Some(AnimatorControllerMStateMachineBehavioursVariant::parse(r, ver)?);
            m_multi_threaded_state_machine = Some(AnimatorControllerMMultiThreadedStateMachineVariant::parse(r, ver)?);
        } else {
        }

        Ok(AnimatorController {
            m_name: m_name.unwrap(),
            m_controller_size: m_controller_size.unwrap(),
            m_evaluate_transitions_on_start,
            m_controller: m_controller.unwrap(),
            m_tos: m_tos.unwrap(),
            m_animation_clips: m_animation_clips.unwrap(),
            m_state_machine_behaviour_vector_description,
            m_state_machine_behaviours,
            m_multi_threaded_state_machine,
        })
    }
}
