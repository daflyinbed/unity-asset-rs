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
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u32() }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u32() }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct StateMachineBehaviourVectorDescription {
    pub m_state_machine_behaviour_ranges: Vec<(StateKey, StateRange)>,
    pub m_state_machine_behaviour_indices: StateMachineBehaviourVectorDescriptionMStateMachineBehaviourIndicesVariant,
}

impl crate::Parse for StateMachineBehaviourVectorDescription {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_state_machine_behaviour_ranges = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = StateKey::parse(rdr, ver)?; let b = StateRange::parse(rdr, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_state_machine_behaviour_indices = StateMachineBehaviourVectorDescriptionMStateMachineBehaviourIndicesVariant::parse(rdr, ver)?;

        Ok(StateMachineBehaviourVectorDescription {
            m_state_machine_behaviour_ranges,
            m_state_machine_behaviour_indices,
        })
    }
}
