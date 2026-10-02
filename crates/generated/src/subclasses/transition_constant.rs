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

impl crate::ToValue for TransitionConstantMAtomicVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V4_5(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V4_5(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for TransitionConstantMHasExitTimeVariant {
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
        m_condition_constant_array = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| OffsetPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
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

impl crate::ToValue for TransitionConstant {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "m_ConditionConstantArray".to_string(),
                self.m_condition_constant_array.to_value(),
            )),
            Some((
                "m_DestinationState".to_string(),
                self.m_destination_state.to_value(),
            )),
            Some(("m_ID".to_string(), self.m_id.to_value())),
            crate::opt_field("m_FullPathID", &self.m_full_path_id),
            Some(("m_UserID".to_string(), self.m_user_id.to_value())),
            Some((
                "m_TransitionDuration".to_string(),
                self.m_transition_duration.to_value(),
            )),
            Some((
                "m_TransitionOffset".to_string(),
                self.m_transition_offset.to_value(),
            )),
            crate::opt_field("m_Atomic", &self.m_atomic),
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
                "m_ConditionConstantArray" => {
                    (self.m_condition_constant_array).inspect_value(path, offset, limit)
                }
                "m_DestinationState" => {
                    (self.m_destination_state).inspect_value(path, offset, limit)
                }
                "m_ID" => (self.m_id).inspect_value(path, offset, limit),
                "m_FullPathID" => (self.m_full_path_id).inspect_value(path, offset, limit),
                "m_UserID" => (self.m_user_id).inspect_value(path, offset, limit),
                "m_TransitionDuration" => {
                    (self.m_transition_duration).inspect_value(path, offset, limit)
                }
                "m_TransitionOffset" => {
                    (self.m_transition_offset).inspect_value(path, offset, limit)
                }
                "m_Atomic" => (self.m_atomic).inspect_value(path, offset, limit),
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
            ((self.m_condition_constant_array).inspect_value(path, offset, limit))
                .map(|v| ("m_ConditionConstantArray".to_string(), v)),
            ((self.m_destination_state).inspect_value(path, offset, limit))
                .map(|v| ("m_DestinationState".to_string(), v)),
            ((self.m_id).inspect_value(path, offset, limit)).map(|v| ("m_ID".to_string(), v)),
            ((self.m_full_path_id).inspect_value(path, offset, limit))
                .map(|v| ("m_FullPathID".to_string(), v)),
            ((self.m_user_id).inspect_value(path, offset, limit))
                .map(|v| ("m_UserID".to_string(), v)),
            ((self.m_transition_duration).inspect_value(path, offset, limit))
                .map(|v| ("m_TransitionDuration".to_string(), v)),
            ((self.m_transition_offset).inspect_value(path, offset, limit))
                .map(|v| ("m_TransitionOffset".to_string(), v)),
            ((self.m_atomic).inspect_value(path, offset, limit))
                .map(|v| ("m_Atomic".to_string(), v)),
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
