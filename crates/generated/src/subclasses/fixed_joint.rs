//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct FixedJoint {
    pub m_object_hide_flags: u32,
    pub m_extension_ptr: PPtr,
    pub m_game_object: PPtr,
    pub m_connected_body: PPtr,
    pub m_break_force: f32,
    pub m_break_torque: f32,
}

impl crate::Parse for FixedJoint {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_extension_ptr = crate::PPtr::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_connected_body = crate::PPtr::parse(r, ver)?;
        let m_break_force = r.read_f32()?;
        let m_break_torque = r.read_f32()?;

        Ok(FixedJoint {
            m_object_hide_flags,
            m_extension_ptr,
            m_game_object,
            m_connected_body,
            m_break_force,
            m_break_torque,
        })
    }
}
