//! 自动生成。请勿手动修改。

use crate::subclasses::human_pose_mask::HumanPoseMask;
use crate::subclasses::offset_ptr::OffsetPtr;

#[derive(Debug, Clone)]
pub struct LayerConstant {
    pub m_state_machine_index: u32,
    pub m_state_machine_motion_set_index: Option<u32>,
    pub m_state_machine_synchronized_layer_index: Option<u32>,
    pub m_body_mask: HumanPoseMask,
    pub m_skeleton_mask: OffsetPtr,
    pub m_binding: u32,
    pub int_m_layer_blending_mode: i32,
    pub m_default_weight: f32,
    pub m_ik_pass: bool,
    pub m_synced_layer_affects_timing: bool,
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
        let mut m_default_weight = None;
        let mut m_ik_pass = None;
        let mut m_synced_layer_affects_timing = None;
            m_state_machine_index = Some(rdr.read_u32()?);
        if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            m_state_machine_synchronized_layer_index = Some(rdr.read_u32()?);
        } else {
            m_state_machine_motion_set_index = Some(rdr.read_u32()?);
        }
            m_body_mask = Some(HumanPoseMask::parse(rdr, ver)?);
            m_skeleton_mask = Some(OffsetPtr::parse(rdr, ver)?);
            m_binding = Some(rdr.read_u32()?);
            int_m_layer_blending_mode = Some(rdr.read_i32()?);
            m_default_weight = Some(rdr.read_f32()?);
            m_ik_pass = Some(rdr.read_bool()?);
            m_synced_layer_affects_timing = Some(rdr.read_bool()?);
            rdr.align(4)?;

        Ok(LayerConstant {
            m_state_machine_index: m_state_machine_index.unwrap(),
            m_state_machine_motion_set_index,
            m_state_machine_synchronized_layer_index,
            m_body_mask: m_body_mask.unwrap(),
            m_skeleton_mask: m_skeleton_mask.unwrap(),
            m_binding: m_binding.unwrap(),
            int_m_layer_blending_mode: int_m_layer_blending_mode.unwrap(),
            m_default_weight: m_default_weight.unwrap(),
            m_ik_pass: m_ik_pass.unwrap(),
            m_synced_layer_affects_timing: m_synced_layer_affects_timing.unwrap(),
        })
    }
}
