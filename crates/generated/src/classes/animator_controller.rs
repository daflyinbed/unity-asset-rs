//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::controller_constant::ControllerConstant;
use crate::subclasses::state_machine_behaviour_vector_description::StateMachineBehaviourVectorDescription;
use super::named_object::NamedObject;
use super::runtime_animator_controller::RuntimeAnimatorController;

#[derive(Debug, Clone)]
pub enum AnimatorControllerParent {
    NamedObject(NamedObject),
    RuntimeAnimatorController(RuntimeAnimatorController),
}

#[derive(Debug, Clone)]
pub struct AnimatorController {
    pub parent: AnimatorControllerParent,
    pub m_name: String,
    pub m_controller_size: u32,
    pub m_evaluate_transitions_on_start: Option<bool>,
    pub m_controller: ControllerConstant,
    pub m_tos: Vec<(u32, String)>,
    pub m_animation_clips: Vec<PPtr>,
    pub m_state_machine_behaviour_vector_description: Option<StateMachineBehaviourVectorDescription>,
    pub m_state_machine_behaviours: Option<Vec<PPtr>>,
    pub m_multi_threaded_state_machine: Option<bool>,
}

impl crate::Parse for AnimatorController {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            AnimatorControllerParent::NamedObject(NamedObject::parse(r, ver)?)
        } else if *ver >= crate::UnityVersion::new(4, 1, 0) {
            AnimatorControllerParent::RuntimeAnimatorController(RuntimeAnimatorController::parse(r, ver)?)
        } else {
            panic!("无匹配的父类分支：version = {:?}", ver)
        };
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_controller_size = r.read_u32()?;
        let m_evaluate_transitions_on_start = if *ver >= crate::UnityVersion::new(6000, 4, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_controller = ControllerConstant::parse(r, ver)?;
        r.align(4)?;
        let m_tos = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_u32()?; let b = r.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_animation_clips = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_state_machine_behaviour_vector_description = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(StateMachineBehaviourVectorDescription::parse(r, ver)?)
        } else {
            None
        };
        let m_state_machine_behaviours = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_multi_threaded_state_machine = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };

        Ok(AnimatorController {
            parent,
            m_name,
            m_controller_size,
            m_evaluate_transitions_on_start,
            m_controller,
            m_tos,
            m_animation_clips,
            m_state_machine_behaviour_vector_description,
            m_state_machine_behaviours,
            m_multi_threaded_state_machine,
        })
    }
}
