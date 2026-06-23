//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct SpringJoint {
    pub m_object_hide_flags: u32,
    pub m_extension_ptr: PPtr,
    pub m_game_object: PPtr,
    pub m_connected_body: PPtr,
    pub m_anchor: Vector3f,
    pub m_spring: f32,
    pub m_damper: f32,
    pub m_min_distance: f32,
    pub m_max_distance: f32,
    pub m_break_force: f32,
    pub m_break_torque: f32,
}

impl crate::Parse for SpringJoint {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_extension_ptr = crate::PPtr::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_connected_body = crate::PPtr::parse(r, ver)?;
        let m_anchor = Vector3f::parse(r, ver)?;
        let m_spring = r.read_f32()?;
        let m_damper = r.read_f32()?;
        let m_min_distance = r.read_f32()?;
        let m_max_distance = r.read_f32()?;
        let m_break_force = r.read_f32()?;
        let m_break_torque = r.read_f32()?;

        Ok(SpringJoint {
            m_object_hide_flags,
            m_extension_ptr,
            m_game_object,
            m_connected_body,
            m_anchor,
            m_spring,
            m_damper,
            m_min_distance,
            m_max_distance,
            m_break_force,
            m_break_torque,
        })
    }
}
