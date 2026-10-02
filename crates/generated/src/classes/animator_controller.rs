//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::controller_constant::ControllerConstant;
use crate::subclasses::state_machine_behaviour_vector_description::StateMachineBehaviourVectorDescription;

/// 版本枚举：AnimatorControllerMNameVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AnimatorControllerMNameVariant {
    Default(String),
    V5_1_2(String),
}

impl crate::Parse for AnimatorControllerMNameVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 1, 2) {
            let v = rdr.read_aligned_string()?;
            rdr.align(4)?;
            Ok(Self::V5_1_2(v))
        } else {
            let v = rdr.read_aligned_string()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for AnimatorControllerMNameVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_1_2(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_1_2(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：AnimatorControllerMAnimationClipsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AnimatorControllerMAnimationClipsVariant {
    Default(Vec<PPtr>),
    V5_0(Vec<PPtr>),
    V2017_1(Vec<PPtr>),
}

impl crate::Parse for AnimatorControllerMAnimationClipsVariant {
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
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| crate::PPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::V5_0(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| crate::PPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for AnimatorControllerMAnimationClipsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_0(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_0(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
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
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| crate::PPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::V5_4(v))
        } else if *ver >= crate::UnityVersion::new(5, 2, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| crate::PPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V5_2(v))
        } else if *ver >= crate::UnityVersion::new(5, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| crate::PPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::V5_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| crate::PPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for AnimatorControllerMStateMachineBehavioursVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_1(v) => v.to_value(),
            Self::V5_2(v) => v.to_value(),
            Self::V5_4(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_1(v) => (v).inspect_value(path, offset, limit),
            Self::V5_2(v) => (v).inspect_value(path, offset, limit),
            Self::V5_4(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
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
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 1, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V5_1(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for AnimatorControllerMMultiThreadedStateMachineVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct AnimatorController {
    pub m_name: AnimatorControllerMNameVariant,
    pub m_controller_size: u32,
    pub m_evaluate_transitions_on_start: Option<bool>,
    pub m_controller: ControllerConstant,
    pub m_tos: Vec<(u32, String)>,
    pub m_animation_clips: AnimatorControllerMAnimationClipsVariant,
    pub m_state_machine_behaviour_vector_description:
        Option<StateMachineBehaviourVectorDescription>,
    pub m_state_machine_behaviours: Option<AnimatorControllerMStateMachineBehavioursVariant>,
    pub m_multi_threaded_state_machine: Option<AnimatorControllerMMultiThreadedStateMachineVariant>,
}

impl crate::Parse for AnimatorController {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
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
        m_name = Some(AnimatorControllerMNameVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(6000, 4, 0) {
            m_evaluate_transitions_on_start = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
        }
        m_controller_size = Some(rdr.read_u32()?);
        m_controller = Some(ControllerConstant::parse(rdr, ver)?);
        m_tos = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| {
                        ({
                            let a = rdr.read_u32()?;
                            let b = rdr.read_aligned_string()?;
                            Ok((a, b))
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        m_animation_clips = Some(AnimatorControllerMAnimationClipsVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            m_state_machine_behaviour_vector_description =
                Some(StateMachineBehaviourVectorDescription::parse(rdr, ver)?);
            m_state_machine_behaviours = Some(
                AnimatorControllerMStateMachineBehavioursVariant::parse(rdr, ver)?,
            );
            m_multi_threaded_state_machine = Some(
                AnimatorControllerMMultiThreadedStateMachineVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(5, 2, 0) {
            m_state_machine_behaviour_vector_description =
                Some(StateMachineBehaviourVectorDescription::parse(rdr, ver)?);
            m_state_machine_behaviours = Some(
                AnimatorControllerMStateMachineBehavioursVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_state_machine_behaviour_vector_description =
                Some(StateMachineBehaviourVectorDescription::parse(rdr, ver)?);
            m_state_machine_behaviours = Some(
                AnimatorControllerMStateMachineBehavioursVariant::parse(rdr, ver)?,
            );
            m_multi_threaded_state_machine = Some(
                AnimatorControllerMMultiThreadedStateMachineVariant::parse(rdr, ver)?,
            );
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

impl crate::ToValue for AnimatorController {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some((
                "m_ControllerSize".to_string(),
                self.m_controller_size.to_value(),
            )),
            crate::opt_field(
                "m_EvaluateTransitionsOnStart",
                &self.m_evaluate_transitions_on_start,
            ),
            Some(("m_Controller".to_string(), self.m_controller.to_value())),
            Some(("m_TOS".to_string(), self.m_tos.to_value())),
            Some((
                "m_AnimationClips".to_string(),
                self.m_animation_clips.to_value(),
            )),
            crate::opt_field(
                "m_StateMachineBehaviourVectorDescription",
                &self.m_state_machine_behaviour_vector_description,
            ),
            crate::opt_field("m_StateMachineBehaviours", &self.m_state_machine_behaviours),
            crate::opt_field(
                "m_MultiThreadedStateMachine",
                &self.m_multi_threaded_state_machine,
            ),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Name" => (self.m_name).inspect_value(path, offset, limit),
                "m_ControllerSize" => (self.m_controller_size).inspect_value(path, offset, limit),
                "m_EvaluateTransitionsOnStart" => {
                    (self.m_evaluate_transitions_on_start).inspect_value(path, offset, limit)
                }
                "m_Controller" => (self.m_controller).inspect_value(path, offset, limit),
                "m_TOS" => (self.m_tos).inspect_value(path, offset, limit),
                "m_AnimationClips" => (self.m_animation_clips).inspect_value(path, offset, limit),
                "m_StateMachineBehaviourVectorDescription" => (self
                    .m_state_machine_behaviour_vector_description)
                    .inspect_value(path, offset, limit),
                "m_StateMachineBehaviours" => {
                    (self.m_state_machine_behaviours).inspect_value(path, offset, limit)
                }
                "m_MultiThreadedStateMachine" => {
                    (self.m_multi_threaded_state_machine).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_controller_size).inspect_value(path, offset, limit))
                .map(|v| ("m_ControllerSize".to_string(), v)),
            ((self.m_evaluate_transitions_on_start).inspect_value(path, offset, limit))
                .map(|v| ("m_EvaluateTransitionsOnStart".to_string(), v)),
            ((self.m_controller).inspect_value(path, offset, limit))
                .map(|v| ("m_Controller".to_string(), v)),
            ((self.m_tos).inspect_value(path, offset, limit)).map(|v| ("m_TOS".to_string(), v)),
            ((self.m_animation_clips).inspect_value(path, offset, limit))
                .map(|v| ("m_AnimationClips".to_string(), v)),
            ((self.m_state_machine_behaviour_vector_description)
                .inspect_value(path, offset, limit))
            .map(|v| ("m_StateMachineBehaviourVectorDescription".to_string(), v)),
            ((self.m_state_machine_behaviours).inspect_value(path, offset, limit))
                .map(|v| ("m_StateMachineBehaviours".to_string(), v)),
            ((self.m_multi_threaded_state_machine).inspect_value(path, offset, limit))
                .map(|v| ("m_MultiThreadedStateMachine".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
