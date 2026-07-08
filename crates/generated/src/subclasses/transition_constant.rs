//! 自动生成。请勿手动修改。

use crate::subclasses::offset_ptr::OffsetPtr;


/// 版本枚举：TransitionConstantMAtomicVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum TransitionConstantMAtomicVariant {
    Default(bool),
    V4_5(bool),
}

impl crate::Parse for TransitionConstantMAtomicVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(4, 5, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V4_5(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：TransitionConstantMHasExitTimeVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum TransitionConstantMHasExitTimeVariant {
    Default(bool),
    V5_1(bool),
}

impl crate::Parse for TransitionConstantMHasExitTimeVariant {
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
#[derive(Debug, Clone)]
pub struct TransitionConstant {
    pub m_condition_constant_array: Vec<OffsetPtr>,
    pub m_destination_state: u32,
    pub m_id: u32,
    pub m_full_path_id: Option<u32>,
    pub m_user_id: u32,
    pub m_transition_duration: f32,
    pub m_transition_offset: f32,
    pub m_atomic: Option<TransitionConstantMAtomicVariant>,
    pub m_can_transition_to_self: Option<bool>,
    pub m_exit_time: Option<f32>,
    pub m_has_exit_time: Option<TransitionConstantMHasExitTimeVariant>,
    pub m_interruption_source: Option<i32>,
    pub m_has_fixed_duration: Option<bool>,
    pub m_ordered_interruption: Option<bool>,
}

impl crate::Parse for TransitionConstant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_condition_constant_array = None;
        let mut m_destination_state = None;
        let mut m_id = None;
        let mut m_full_path_id = None;
        let mut m_user_id = None;
        let mut m_transition_duration = None;
        let mut m_transition_offset = None;
        let mut m_atomic = None;
        let mut m_can_transition_to_self = None;
        let mut m_exit_time = None;
        let mut m_has_exit_time = None;
        let mut m_interruption_source = None;
        let mut m_has_fixed_duration = None;
        let mut m_ordered_interruption = None;
            m_condition_constant_array = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { OffsetPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            m_destination_state = Some(rdr.read_u32()?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_full_path_id = Some(rdr.read_u32()?);
        } else {
        }
            m_id = Some(rdr.read_u32()?);
            m_user_id = Some(rdr.read_u32()?);
            m_transition_duration = Some(rdr.read_f32()?);
            m_transition_offset = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(5, 1, 0) {
            m_exit_time = Some(rdr.read_f32()?);
            m_has_exit_time = Some(TransitionConstantMHasExitTimeVariant::parse(rdr, ver)?);
            m_has_fixed_duration = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_interruption_source = Some(rdr.read_i32()?);
            m_ordered_interruption = Some(rdr.read_bool()?);
            m_can_transition_to_self = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_exit_time = Some(rdr.read_f32()?);
            m_has_exit_time = Some(TransitionConstantMHasExitTimeVariant::parse(rdr, ver)?);
            m_interruption_source = Some(rdr.read_i32()?);
            m_ordered_interruption = Some(rdr.read_bool()?);
            m_can_transition_to_self = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(4, 5, 0) {
            m_atomic = Some(TransitionConstantMAtomicVariant::parse(rdr, ver)?);
            m_can_transition_to_self = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
            m_atomic = Some(TransitionConstantMAtomicVariant::parse(rdr, ver)?);
        }

        Ok(TransitionConstant {
            m_condition_constant_array: m_condition_constant_array.unwrap(),
            m_destination_state: m_destination_state.unwrap(),
            m_id: m_id.unwrap(),
            m_full_path_id,
            m_user_id: m_user_id.unwrap(),
            m_transition_duration: m_transition_duration.unwrap(),
            m_transition_offset: m_transition_offset.unwrap(),
            m_atomic,
            m_can_transition_to_self,
            m_exit_time,
            m_has_exit_time,
            m_interruption_source,
            m_has_fixed_duration,
            m_ordered_interruption,
        })
    }
}
