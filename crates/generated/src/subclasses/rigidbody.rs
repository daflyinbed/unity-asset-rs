//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct Rigidbody {
    pub m_object_hide_flags: u32,
    pub m_extension_ptr: PPtr,
    pub m_game_object: PPtr,
    pub m_mass: f32,
    pub m_drag: f32,
    pub m_angular_drag: f32,
    pub m_use_gravity: bool,
    pub m_is_kinematic: bool,
    pub m_interpolate: u8,
    pub m_constraints: i32,
    pub m_collision_detection: i32,
}

impl crate::Parse for Rigidbody {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_extension_ptr = crate::PPtr::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_mass = r.read_f32()?;
        let m_drag = r.read_f32()?;
        let m_angular_drag = r.read_f32()?;
        let m_use_gravity = r.read_bool()?;
        let m_is_kinematic = r.read_bool()?;
        let m_interpolate = r.read_u8()?;
        r.align(4)?;
        let m_constraints = r.read_i32()?;
        let m_collision_detection = r.read_i32()?;

        Ok(Rigidbody {
            m_object_hide_flags,
            m_extension_ptr,
            m_game_object,
            m_mass,
            m_drag,
            m_angular_drag,
            m_use_gravity,
            m_is_kinematic,
            m_interpolate,
            m_constraints,
            m_collision_detection,
        })
    }
}
