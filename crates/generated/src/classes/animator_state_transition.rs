//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::animator_condition::AnimatorCondition;

/// 版本枚举：AnimatorStateTransitionMConditionsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AnimatorStateTransitionMConditionsVariant {
    Default(Vec<AnimatorCondition>),
    V2017_1(Vec<AnimatorCondition>),
}

impl crate::Parse for AnimatorStateTransitionMConditionsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| AnimatorCondition::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| AnimatorCondition::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for AnimatorStateTransitionMConditionsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：AnimatorStateTransitionMDestinationStateVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AnimatorStateTransitionMDestinationStateVariant {
    Default(PPtr),
    V5_0(PPtr),
}

impl crate::Parse for AnimatorStateTransitionMDestinationStateVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = crate::PPtr::parse(rdr, ver)?;
            Ok(Self::V5_0(v))
        } else {
            let v = crate::PPtr::parse(rdr, ver)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for AnimatorStateTransitionMDestinationStateVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_0(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_0(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：AnimatorStateTransitionMHasExitTimeVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AnimatorStateTransitionMHasExitTimeVariant {
    Default(bool),
    V5_1(bool),
}

impl crate::Parse for AnimatorStateTransitionMHasExitTimeVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 1, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V5_1(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for AnimatorStateTransitionMHasExitTimeVariant {
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
pub struct AnimatorStateTransition {
    pub m_name: String,
    pub m_source_state: Option<PPtr>,
    pub m_conditions: AnimatorStateTransitionMConditionsVariant,
    pub m_destination_state: AnimatorStateTransitionMDestinationStateVariant,
    pub m_destination_state_machine: Option<PPtr>,
    pub m_transition_duration: f32,
    pub m_transition_offset: f32,
    pub m_solo: bool,
    pub m_mute: bool,
    pub m_atomic: Option<bool>,
    pub m_is_exit: Option<bool>,
    pub m_can_transition_to_self: Option<bool>,
    pub m_exit_time: Option<f32>,
    pub m_has_exit_time: Option<AnimatorStateTransitionMHasExitTimeVariant>,
    pub m_interruption_source: Option<i32>,
    pub m_has_fixed_duration: Option<bool>,
    pub m_ordered_interruption: Option<bool>,
}

impl crate::Parse for AnimatorStateTransition {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut m_source_state = None;
        let mut m_conditions = None;
        let mut m_destination_state = None;
        let mut m_destination_state_machine = None;
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
        m_name = Some(rdr.read_aligned_string()?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
        } else {
            m_source_state = Some(crate::PPtr::parse(rdr, ver)?);
            m_destination_state = Some(AnimatorStateTransitionMDestinationStateVariant::parse(
                rdr, ver,
            )?);
            m_transition_duration = Some(rdr.read_f32()?);
            m_transition_offset = Some(rdr.read_f32()?);
        }
        m_conditions = Some(AnimatorStateTransitionMConditionsVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_destination_state_machine = Some(crate::PPtr::parse(rdr, ver)?);
            m_destination_state = Some(AnimatorStateTransitionMDestinationStateVariant::parse(
                rdr, ver,
            )?);
        } else {
            m_atomic = Some(rdr.read_bool()?);
        }
        m_solo = Some(rdr.read_bool()?);
        m_mute = Some(rdr.read_bool()?);
        if *ver >= crate::UnityVersion::new(5, 1, 0) {
            m_is_exit = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_transition_duration = Some(rdr.read_f32()?);
            m_transition_offset = Some(rdr.read_f32()?);
            m_exit_time = Some(rdr.read_f32()?);
            m_has_exit_time = Some(AnimatorStateTransitionMHasExitTimeVariant::parse(rdr, ver)?);
            m_has_fixed_duration = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_interruption_source = Some(rdr.read_i32()?);
            m_ordered_interruption = Some(rdr.read_bool()?);
            m_can_transition_to_self = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_is_exit = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_transition_duration = Some(rdr.read_f32()?);
            m_transition_offset = Some(rdr.read_f32()?);
            m_exit_time = Some(rdr.read_f32()?);
            m_has_exit_time = Some(AnimatorStateTransitionMHasExitTimeVariant::parse(rdr, ver)?);
            m_interruption_source = Some(rdr.read_i32()?);
            m_ordered_interruption = Some(rdr.read_bool()?);
            m_can_transition_to_self = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(4, 5, 0) {
            m_can_transition_to_self = Some(rdr.read_bool()?);
        } else {
        }

        Ok(AnimatorStateTransition {
            m_name: m_name.unwrap(),
            m_source_state,
            m_conditions: m_conditions.unwrap(),
            m_destination_state: m_destination_state.unwrap(),
            m_destination_state_machine,
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

impl crate::ToValue for AnimatorStateTransition {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            crate::opt_field("m_SourceState", &self.m_source_state),
            Some(("m_Conditions".to_string(), self.m_conditions.to_value())),
            Some((
                "m_DestinationState".to_string(),
                self.m_destination_state.to_value(),
            )),
            crate::opt_field(
                "m_DestinationStateMachine",
                &self.m_destination_state_machine,
            ),
            Some((
                "m_TransitionDuration".to_string(),
                self.m_transition_duration.to_value(),
            )),
            Some((
                "m_TransitionOffset".to_string(),
                self.m_transition_offset.to_value(),
            )),
            Some(("m_Solo".to_string(), self.m_solo.to_value())),
            Some(("m_Mute".to_string(), self.m_mute.to_value())),
            crate::opt_field("m_Atomic", &self.m_atomic),
            crate::opt_field("m_IsExit", &self.m_is_exit),
            crate::opt_field("m_CanTransitionToSelf", &self.m_can_transition_to_self),
            crate::opt_field("m_ExitTime", &self.m_exit_time),
            crate::opt_field("m_HasExitTime", &self.m_has_exit_time),
            crate::opt_field("m_InterruptionSource", &self.m_interruption_source),
            crate::opt_field("m_HasFixedDuration", &self.m_has_fixed_duration),
            crate::opt_field("m_OrderedInterruption", &self.m_ordered_interruption),
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
                "m_SourceState" => (self.m_source_state).inspect_value(path, offset, limit),
                "m_Conditions" => (self.m_conditions).inspect_value(path, offset, limit),
                "m_DestinationState" => {
                    (self.m_destination_state).inspect_value(path, offset, limit)
                }
                "m_DestinationStateMachine" => {
                    (self.m_destination_state_machine).inspect_value(path, offset, limit)
                }
                "m_TransitionDuration" => {
                    (self.m_transition_duration).inspect_value(path, offset, limit)
                }
                "m_TransitionOffset" => {
                    (self.m_transition_offset).inspect_value(path, offset, limit)
                }
                "m_Solo" => (self.m_solo).inspect_value(path, offset, limit),
                "m_Mute" => (self.m_mute).inspect_value(path, offset, limit),
                "m_Atomic" => (self.m_atomic).inspect_value(path, offset, limit),
                "m_IsExit" => (self.m_is_exit).inspect_value(path, offset, limit),
                "m_CanTransitionToSelf" => {
                    (self.m_can_transition_to_self).inspect_value(path, offset, limit)
                }
                "m_ExitTime" => (self.m_exit_time).inspect_value(path, offset, limit),
                "m_HasExitTime" => (self.m_has_exit_time).inspect_value(path, offset, limit),
                "m_InterruptionSource" => {
                    (self.m_interruption_source).inspect_value(path, offset, limit)
                }
                "m_HasFixedDuration" => {
                    (self.m_has_fixed_duration).inspect_value(path, offset, limit)
                }
                "m_OrderedInterruption" => {
                    (self.m_ordered_interruption).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_source_state).inspect_value(path, offset, limit))
                .map(|v| ("m_SourceState".to_string(), v)),
            ((self.m_conditions).inspect_value(path, offset, limit))
                .map(|v| ("m_Conditions".to_string(), v)),
            ((self.m_destination_state).inspect_value(path, offset, limit))
                .map(|v| ("m_DestinationState".to_string(), v)),
            ((self.m_destination_state_machine).inspect_value(path, offset, limit))
                .map(|v| ("m_DestinationStateMachine".to_string(), v)),
            ((self.m_transition_duration).inspect_value(path, offset, limit))
                .map(|v| ("m_TransitionDuration".to_string(), v)),
            ((self.m_transition_offset).inspect_value(path, offset, limit))
                .map(|v| ("m_TransitionOffset".to_string(), v)),
            ((self.m_solo).inspect_value(path, offset, limit)).map(|v| ("m_Solo".to_string(), v)),
            ((self.m_mute).inspect_value(path, offset, limit)).map(|v| ("m_Mute".to_string(), v)),
            ((self.m_atomic).inspect_value(path, offset, limit))
                .map(|v| ("m_Atomic".to_string(), v)),
            ((self.m_is_exit).inspect_value(path, offset, limit))
                .map(|v| ("m_IsExit".to_string(), v)),
            ((self.m_can_transition_to_self).inspect_value(path, offset, limit))
                .map(|v| ("m_CanTransitionToSelf".to_string(), v)),
            ((self.m_exit_time).inspect_value(path, offset, limit))
                .map(|v| ("m_ExitTime".to_string(), v)),
            ((self.m_has_exit_time).inspect_value(path, offset, limit))
                .map(|v| ("m_HasExitTime".to_string(), v)),
            ((self.m_interruption_source).inspect_value(path, offset, limit))
                .map(|v| ("m_InterruptionSource".to_string(), v)),
            ((self.m_has_fixed_duration).inspect_value(path, offset, limit))
                .map(|v| ("m_HasFixedDuration".to_string(), v)),
            ((self.m_ordered_interruption).inspect_value(path, offset, limit))
                .map(|v| ("m_OrderedInterruption".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
