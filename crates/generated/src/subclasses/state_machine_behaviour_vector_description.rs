//! 自动生成。请勿手动修改。

use crate::subclasses::state_key::StateKey;
use crate::subclasses::state_range::StateRange;

/// 版本枚举：StateMachineBehaviourVectorDescriptionMStateMachineBehaviourIndicesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum StateMachineBehaviourVectorDescriptionMStateMachineBehaviourIndicesVariant {
    Default(Vec<u32>),
    V2017_1(Vec<u32>),
}

impl crate::Parse for StateMachineBehaviourVectorDescriptionMStateMachineBehaviourIndicesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_u32())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_u32())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for StateMachineBehaviourVectorDescriptionMStateMachineBehaviourIndicesVariant {
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
pub struct StateMachineBehaviourVectorDescription {
    pub m_state_machine_behaviour_ranges: Vec<(StateKey, StateRange)>,
    pub m_state_machine_behaviour_indices:
        StateMachineBehaviourVectorDescriptionMStateMachineBehaviourIndicesVariant,
}

impl crate::Parse for StateMachineBehaviourVectorDescription {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_state_machine_behaviour_ranges = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| {
                    ({
                        let a = StateKey::parse(rdr, ver)?;
                        let b = StateRange::parse(rdr, ver)?;
                        Ok((a, b))
                    })
                })
                .collect::<Result<Vec<_>, _>>()
        })?;
        let m_state_machine_behaviour_indices =
            StateMachineBehaviourVectorDescriptionMStateMachineBehaviourIndicesVariant::parse(
                rdr, ver,
            )?;

        Ok(StateMachineBehaviourVectorDescription {
            m_state_machine_behaviour_ranges,
            m_state_machine_behaviour_indices,
        })
    }
}

impl crate::ToValue for StateMachineBehaviourVectorDescription {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "m_StateMachineBehaviourRanges".to_string(),
                self.m_state_machine_behaviour_ranges.to_value(),
            )),
            Some((
                "m_StateMachineBehaviourIndices".to_string(),
                self.m_state_machine_behaviour_indices.to_value(),
            )),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_StateMachineBehaviourRanges" => {
                    (self.m_state_machine_behaviour_ranges).inspect_value(path, offset, limit)
                }
                "m_StateMachineBehaviourIndices" => {
                    (self.m_state_machine_behaviour_indices).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_state_machine_behaviour_ranges).inspect_value(path, offset, limit))
                .map(|v| ("m_StateMachineBehaviourRanges".to_string(), v)),
            ((self.m_state_machine_behaviour_indices).inspect_value(path, offset, limit))
                .map(|v| ("m_StateMachineBehaviourIndices".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
