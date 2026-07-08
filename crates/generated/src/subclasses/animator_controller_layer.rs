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
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        let m_type = rdr.read_i32()?;
        let m_state_machine = crate::PPtr::parse(rdr, ver)?;
        let m_human_mask = crate::PPtr::parse(rdr, ver)?;
        let m_skeleton_mask = crate::PPtr::parse(rdr, ver)?;
        let m_blending_mode = rdr.read_i32()?;
        let m_synced_layer_index = rdr.read_i32()?;
        let m_state_machine_motion_set_index = rdr.read_i32()?;
        let m_ik_pass = rdr.read_bool()?;

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
