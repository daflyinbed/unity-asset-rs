//! 自动生成。请勿手动修改。

use crate::subclasses::human_pose_mask::HumanPoseMask;
use crate::subclasses::offset_ptr::OffsetPtr;

/// 版本枚举：LayerConstantMIkPassVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum LayerConstantMIkPassVariant {
    Default(bool),
    V4_2(bool),
}

impl crate::Parse for LayerConstantMIkPassVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(4, 2, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V4_2(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for LayerConstantMIkPassVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V4_2(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V4_2(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct LayerConstant {
    pub m_state_machine_index: u32,
    pub m_state_machine_motion_set_index: Option<u32>,
    pub m_state_machine_synchronized_layer_index: Option<u32>,
    pub m_body_mask: HumanPoseMask,
    pub m_skeleton_mask: OffsetPtr,
    pub m_binding: u32,
    pub int_m_layer_blending_mode: i32,
    pub m_ik_pass: LayerConstantMIkPassVariant,
    pub m_default_weight: Option<f32>,
    pub m_synced_layer_affects_timing: Option<bool>,
}

impl crate::Parse for LayerConstant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_state_machine_index = None;
        let mut m_state_machine_motion_set_index = None;
        let mut m_state_machine_synchronized_layer_index = None;
        let mut m_body_mask = None;
        let mut m_skeleton_mask = None;
        let mut m_binding = None;
        let mut int_m_layer_blending_mode = None;
        let mut m_ik_pass = None;
        let mut m_default_weight = None;
        let mut m_synced_layer_affects_timing = None;
        m_state_machine_index = Some(rdr.read_u32()?);
        if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_state_machine_synchronized_layer_index = Some(rdr.read_u32()?);
        } else {
            m_state_machine_motion_set_index = Some(rdr.read_u32()?);
        }
        m_body_mask = Some(HumanPoseMask::parse(rdr, ver)?);
        m_skeleton_mask = Some(OffsetPtr::parse(rdr, ver)?);
        m_binding = Some(rdr.read_u32()?);
        int_m_layer_blending_mode = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(4, 2, 0) {
            m_default_weight = Some(rdr.read_f32()?);
        } else {
        }
        m_ik_pass = Some(LayerConstantMIkPassVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(4, 2, 0) {
            m_synced_layer_affects_timing = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
        }

        Ok(LayerConstant {
            m_state_machine_index: m_state_machine_index.unwrap(),
            m_state_machine_motion_set_index,
            m_state_machine_synchronized_layer_index,
            m_body_mask: m_body_mask.unwrap(),
            m_skeleton_mask: m_skeleton_mask.unwrap(),
            m_binding: m_binding.unwrap(),
            int_m_layer_blending_mode: int_m_layer_blending_mode.unwrap(),
            m_ik_pass: m_ik_pass.unwrap(),
            m_default_weight,
            m_synced_layer_affects_timing,
        })
    }
}

impl crate::ToValue for LayerConstant {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "m_StateMachineIndex".to_string(),
                self.m_state_machine_index.to_value(),
            )),
            crate::opt_field(
                "m_StateMachineMotionSetIndex",
                &self.m_state_machine_motion_set_index,
            ),
            crate::opt_field(
                "m_StateMachineSynchronizedLayerIndex",
                &self.m_state_machine_synchronized_layer_index,
            ),
            Some(("m_BodyMask".to_string(), self.m_body_mask.to_value())),
            Some((
                "m_SkeletonMask".to_string(),
                self.m_skeleton_mask.to_value(),
            )),
            Some(("m_Binding".to_string(), self.m_binding.to_value())),
            Some((
                "(int&)m_LayerBlendingMode".to_string(),
                self.int_m_layer_blending_mode.to_value(),
            )),
            Some(("m_IKPass".to_string(), self.m_ik_pass.to_value())),
            crate::opt_field("m_DefaultWeight", &self.m_default_weight),
            crate::opt_field(
                "m_SyncedLayerAffectsTiming",
                &self.m_synced_layer_affects_timing,
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
                "m_StateMachineIndex" => {
                    (self.m_state_machine_index).inspect_value(path, offset, limit)
                }
                "m_StateMachineMotionSetIndex" => {
                    (self.m_state_machine_motion_set_index).inspect_value(path, offset, limit)
                }
                "m_StateMachineSynchronizedLayerIndex" => (self
                    .m_state_machine_synchronized_layer_index)
                    .inspect_value(path, offset, limit),
                "m_BodyMask" => (self.m_body_mask).inspect_value(path, offset, limit),
                "m_SkeletonMask" => (self.m_skeleton_mask).inspect_value(path, offset, limit),
                "m_Binding" => (self.m_binding).inspect_value(path, offset, limit),
                "(int&)m_LayerBlendingMode" => {
                    (self.int_m_layer_blending_mode).inspect_value(path, offset, limit)
                }
                "m_IKPass" => (self.m_ik_pass).inspect_value(path, offset, limit),
                "m_DefaultWeight" => (self.m_default_weight).inspect_value(path, offset, limit),
                "m_SyncedLayerAffectsTiming" => {
                    (self.m_synced_layer_affects_timing).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_state_machine_index).inspect_value(path, offset, limit))
                .map(|v| ("m_StateMachineIndex".to_string(), v)),
            ((self.m_state_machine_motion_set_index).inspect_value(path, offset, limit))
                .map(|v| ("m_StateMachineMotionSetIndex".to_string(), v)),
            ((self.m_state_machine_synchronized_layer_index).inspect_value(path, offset, limit))
                .map(|v| ("m_StateMachineSynchronizedLayerIndex".to_string(), v)),
            ((self.m_body_mask).inspect_value(path, offset, limit))
                .map(|v| ("m_BodyMask".to_string(), v)),
            ((self.m_skeleton_mask).inspect_value(path, offset, limit))
                .map(|v| ("m_SkeletonMask".to_string(), v)),
            ((self.m_binding).inspect_value(path, offset, limit))
                .map(|v| ("m_Binding".to_string(), v)),
            ((self.int_m_layer_blending_mode).inspect_value(path, offset, limit))
                .map(|v| ("(int&)m_LayerBlendingMode".to_string(), v)),
            ((self.m_ik_pass).inspect_value(path, offset, limit))
                .map(|v| ("m_IKPass".to_string(), v)),
            ((self.m_default_weight).inspect_value(path, offset, limit))
                .map(|v| ("m_DefaultWeight".to_string(), v)),
            ((self.m_synced_layer_affects_timing).inspect_value(path, offset, limit))
                .map(|v| ("m_SyncedLayerAffectsTiming".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
