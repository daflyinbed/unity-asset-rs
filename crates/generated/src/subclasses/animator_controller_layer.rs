//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct AnimatorControllerLayer {
    pub m_name: String,
    pub m_type: i32,
    pub m_state_machine: PPtr,
    pub m_human_mask: PPtr,
    pub m_skeleton_mask: PPtr,
    pub m_blending_mode: i32,
    pub m_synced_layer_index: i32,
    pub m_state_machine_motion_set_index: i32,
    pub m_ik_pass: bool,
}

impl crate::Parse for AnimatorControllerLayer {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_type = r.read_i32()?;
        let m_state_machine = crate::PPtr::parse(r, ver)?;
        let m_human_mask = crate::PPtr::parse(r, ver)?;
        let m_skeleton_mask = crate::PPtr::parse(r, ver)?;
        let m_blending_mode = r.read_i32()?;
        let m_synced_layer_index = r.read_i32()?;
        let m_state_machine_motion_set_index = r.read_i32()?;
        let m_ik_pass = r.read_bool()?;

        Ok(AnimatorControllerLayer {
            m_name,
            m_type,
            m_state_machine,
            m_human_mask,
            m_skeleton_mask,
            m_blending_mode,
            m_synced_layer_index,
            m_state_machine_motion_set_index,
            m_ik_pass,
        })
    }
}
