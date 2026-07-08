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
        let m_state_machine_index = rdr.read_u32()?;
        let m_state_machine_motion_set_index = if *ver >= crate::UnityVersion::new(4, 3, 0) {
            Some(rdr.read_u32()?)
        } else {
            None
        };
        let m_state_machine_synchronized_layer_index = if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            Some(rdr.read_u32()?)
        } else {
            None
        };
        let m_body_mask = HumanPoseMask::parse(rdr, ver)?;
        let m_skeleton_mask = OffsetPtr::parse(rdr, ver)?;
        let m_binding = rdr.read_u32()?;
        let int_m_layer_blending_mode = rdr.read_i32()?;
        let m_default_weight = rdr.read_f32()?;
        let m_ik_pass = rdr.read_bool()?;
        let m_synced_layer_affects_timing = rdr.read_bool()?;
        rdr.align(4)?;

        Ok(LayerConstant {
            m_state_machine_index,
            m_state_machine_motion_set_index,
            m_state_machine_synchronized_layer_index,
            m_body_mask,
            m_skeleton_mask,
            m_binding,
            int_m_layer_blending_mode,
            m_default_weight,
            m_ik_pass,
            m_synced_layer_affects_timing,
        })
    }
}
