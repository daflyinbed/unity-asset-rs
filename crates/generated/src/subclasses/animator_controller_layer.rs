//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct AnimatorControllerLayer {
    pub m_name: String,
    pub m_type: i32,
    pub m_state_machine: PPtr,
    pub m_mask: PPtr,
    pub m_skeleton_mask: PPtr,
    pub m_blending_mode: i32,
    pub m_synced_layer_index: i32,
    pub m_state_machine_motion_set_index: i32,
    pub m_ik_pass: bool,
}

impl crate::Parse for AnimatorControllerLayer {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        let m_type = rdr.read_i32()?;
        let m_state_machine = crate::PPtr::parse(rdr, ver)?;
        let m_mask = crate::PPtr::parse(rdr, ver)?;
        let m_skeleton_mask = crate::PPtr::parse(rdr, ver)?;
        let m_blending_mode = rdr.read_i32()?;
        let m_synced_layer_index = rdr.read_i32()?;
        let m_state_machine_motion_set_index = rdr.read_i32()?;
        let m_ik_pass = rdr.read_bool()?;

        Ok(AnimatorControllerLayer {
            m_name,
            m_type,
            m_state_machine,
            m_mask,
            m_skeleton_mask,
            m_blending_mode,
            m_synced_layer_index,
            m_state_machine_motion_set_index,
            m_ik_pass,
        })
    }
}

impl crate::ToValue for AnimatorControllerLayer {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some(("m_Type".to_string(), self.m_type.to_value())),
            Some((
                "m_StateMachine".to_string(),
                self.m_state_machine.to_value(),
            )),
            Some(("m_Mask".to_string(), self.m_mask.to_value())),
            Some((
                "m_SkeletonMask".to_string(),
                self.m_skeleton_mask.to_value(),
            )),
            Some((
                "m_BlendingMode".to_string(),
                self.m_blending_mode.to_value(),
            )),
            Some((
                "m_SyncedLayerIndex".to_string(),
                self.m_synced_layer_index.to_value(),
            )),
            Some((
                "m_StateMachineMotionSetIndex".to_string(),
                self.m_state_machine_motion_set_index.to_value(),
            )),
            Some(("m_IKPass".to_string(), self.m_ik_pass.to_value())),
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
                "m_Type" => (self.m_type).inspect_value(path, offset, limit),
                "m_StateMachine" => (self.m_state_machine).inspect_value(path, offset, limit),
                "m_Mask" => (self.m_mask).inspect_value(path, offset, limit),
                "m_SkeletonMask" => (self.m_skeleton_mask).inspect_value(path, offset, limit),
                "m_BlendingMode" => (self.m_blending_mode).inspect_value(path, offset, limit),
                "m_SyncedLayerIndex" => {
                    (self.m_synced_layer_index).inspect_value(path, offset, limit)
                }
                "m_StateMachineMotionSetIndex" => {
                    (self.m_state_machine_motion_set_index).inspect_value(path, offset, limit)
                }
                "m_IKPass" => (self.m_ik_pass).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_type).inspect_value(path, offset, limit)).map(|v| ("m_Type".to_string(), v)),
            ((self.m_state_machine).inspect_value(path, offset, limit))
                .map(|v| ("m_StateMachine".to_string(), v)),
            ((self.m_mask).inspect_value(path, offset, limit)).map(|v| ("m_Mask".to_string(), v)),
            ((self.m_skeleton_mask).inspect_value(path, offset, limit))
                .map(|v| ("m_SkeletonMask".to_string(), v)),
            ((self.m_blending_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_BlendingMode".to_string(), v)),
            ((self.m_synced_layer_index).inspect_value(path, offset, limit))
                .map(|v| ("m_SyncedLayerIndex".to_string(), v)),
            ((self.m_state_machine_motion_set_index).inspect_value(path, offset, limit))
                .map(|v| ("m_StateMachineMotionSetIndex".to_string(), v)),
            ((self.m_ik_pass).inspect_value(path, offset, limit))
                .map(|v| ("m_IKPass".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
