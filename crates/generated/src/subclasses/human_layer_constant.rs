//! 自动生成。请勿手动修改。

use crate::subclasses::human_pose_mask::HumanPoseMask;
use crate::subclasses::offset_ptr::OffsetPtr;

#[derive(Debug, Clone)]
pub struct HumanLayerConstant {
    pub m_state_machine_index: u32,
    pub m_state_machine_motion_set_index: u32,
    pub m_body_mask: HumanPoseMask,
    pub m_skeleton_mask: OffsetPtr,
    pub m_binding: u32,
    pub int_m_layer_blending_mode: i32,
    pub m_ik_pass: bool,
}

impl crate::Parse for HumanLayerConstant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_state_machine_index = r.read_u32()?;
        let m_state_machine_motion_set_index = r.read_u32()?;
        let m_body_mask = HumanPoseMask::parse(r, ver)?;
        let m_skeleton_mask = OffsetPtr::parse(r, ver)?;
        let m_binding = r.read_u32()?;
        let int_m_layer_blending_mode = r.read_i32()?;
        let m_ik_pass = r.read_bool()?;
        r.align(4)?;

        Ok(HumanLayerConstant {
            m_state_machine_index,
            m_state_machine_motion_set_index,
            m_body_mask,
            m_skeleton_mask,
            m_binding,
            int_m_layer_blending_mode,
            m_ik_pass,
        })
    }
}
