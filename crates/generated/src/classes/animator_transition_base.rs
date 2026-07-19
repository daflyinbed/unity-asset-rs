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
