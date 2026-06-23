//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct Rigidbody2D {
    pub m_object_hide_flags: u32,
    pub m_prefab_parent_object: PPtr,
    pub m_prefab_internal: PPtr,
    pub m_game_object: PPtr,
    pub m_mass: f32,
    pub m_linear_drag: f32,
    pub m_angular_drag: f32,
    pub m_gravity_scale: f32,
    pub m_fixed_angle: bool,
    pub m_is_kinematic: bool,
    pub m_interpolate: u8,
    pub m_sleeping_mode: u8,
    pub m_collision_detection: u8,
}

impl crate::Parse for Rigidbody2D {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_prefab_parent_object = crate::PPtr::parse(r, ver)?;
        let m_prefab_internal = crate::PPtr::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_mass = r.read_f32()?;
        let m_linear_drag = r.read_f32()?;
        let m_angular_drag = r.read_f32()?;
        let m_gravity_scale = r.read_f32()?;
        let m_fixed_angle = r.read_bool()?;
        let m_is_kinematic = r.read_bool()?;
        let m_interpolate = r.read_u8()?;
        let m_sleeping_mode = r.read_u8()?;
        let m_collision_detection = r.read_u8()?;
        r.align(4)?;

        Ok(Rigidbody2D {
            m_object_hide_flags,
            m_prefab_parent_object,
            m_prefab_internal,
            m_game_object,
            m_mass,
            m_linear_drag,
            m_angular_drag,
            m_gravity_scale,
            m_fixed_angle,
            m_is_kinematic,
            m_interpolate,
            m_sleeping_mode,
            m_collision_detection,
        })
    }
}
