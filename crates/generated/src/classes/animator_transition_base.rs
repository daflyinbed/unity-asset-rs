//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::animator_condition::AnimatorCondition;

/// 版本枚举：AnimatorTransitionBaseMConditionsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AnimatorTransitionBaseMConditionsVariant {
    Default(Vec<AnimatorCondition>),
    V2017_1(Vec<AnimatorCondition>),
}

impl crate::Parse for AnimatorTransitionBaseMConditionsVariant {
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

impl crate::ToValue for AnimatorTransitionBaseMConditionsVariant {
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
#[derive(Debug, Clone)]
pub struct AnimatorTransitionBase {
    pub m_name: String,
    pub m_conditions: AnimatorTransitionBaseMConditionsVariant,
    pub m_destination_state_machine: PPtr,
    pub m_destination_state: PPtr,
    pub m_solo: bool,
    pub m_mute: bool,
    pub m_is_exit: bool,
}

impl crate::Parse for AnimatorTransitionBase {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        let m_conditions = AnimatorTransitionBaseMConditionsVariant::parse(rdr, ver)?;
        let m_destination_state_machine = crate::PPtr::parse(rdr, ver)?;
        let m_destination_state = crate::PPtr::parse(rdr, ver)?;
        let m_solo = rdr.read_bool()?;
        let m_mute = rdr.read_bool()?;
        let m_is_exit = rdr.read_bool()?;
        rdr.align(4)?;

        Ok(AnimatorTransitionBase {
            m_name,
            m_conditions,
            m_destination_state_machine,
            m_destination_state,
            m_solo,
            m_mute,
            m_is_exit,
        })
    }
}

impl crate::ToValue for AnimatorTransitionBase {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some(("m_Conditions".to_string(), self.m_conditions.to_value())),
            Some((
                "m_DestinationStateMachine".to_string(),
                self.m_destination_state_machine.to_value(),
            )),
            Some((
                "m_DestinationState".to_string(),
                self.m_destination_state.to_value(),
            )),
            Some(("m_Solo".to_string(), self.m_solo.to_value())),
            Some(("m_Mute".to_string(), self.m_mute.to_value())),
            Some(("m_IsExit".to_string(), self.m_is_exit.to_value())),
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
                "m_Conditions" => (self.m_conditions).inspect_value(path, offset, limit),
                "m_DestinationStateMachine" => {
                    (self.m_destination_state_machine).inspect_value(path, offset, limit)
                }
                "m_DestinationState" => {
                    (self.m_destination_state).inspect_value(path, offset, limit)
                }
                "m_Solo" => (self.m_solo).inspect_value(path, offset, limit),
                "m_Mute" => (self.m_mute).inspect_value(path, offset, limit),
                "m_IsExit" => (self.m_is_exit).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_conditions).inspect_value(path, offset, limit))
                .map(|v| ("m_Conditions".to_string(), v)),
            ((self.m_destination_state_machine).inspect_value(path, offset, limit))
                .map(|v| ("m_DestinationStateMachine".to_string(), v)),
            ((self.m_destination_state).inspect_value(path, offset, limit))
                .map(|v| ("m_DestinationState".to_string(), v)),
            ((self.m_solo).inspect_value(path, offset, limit)).map(|v| ("m_Solo".to_string(), v)),
            ((self.m_mute).inspect_value(path, offset, limit)).map(|v| ("m_Mute".to_string(), v)),
            ((self.m_is_exit).inspect_value(path, offset, limit))
                .map(|v| ("m_IsExit".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
