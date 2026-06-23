//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::cloth_attachment::ClothAttachment;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct InteractiveCloth {
    pub m_object_hide_flags: u32,
    pub m_extension_ptr: PPtr,
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_bending_stiffness: f32,
    pub m_stretching_stiffness: f32,
    pub m_damping: f32,
    pub m_thickness: f32,
    pub m_use_gravity: bool,
    pub m_self_collision: bool,
    pub m_external_acceleration: Vector3f,
    pub m_random_acceleration: Vector3f,
    pub m_mesh: PPtr,
    pub m_friction: f32,
    pub m_density: f32,
    pub m_pressure: f32,
    pub m_collision_response: f32,
    pub m_attachment_tear_factor: f32,
    pub m_attachment_response: f32,
    pub m_tear_factor: f32,
    pub m_attached_colliders: Vec<ClothAttachment>,
}

impl crate::Parse for InteractiveCloth {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_extension_ptr = crate::PPtr::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_u8()?;
        r.align(4)?;
        let m_bending_stiffness = r.read_f32()?;
        let m_stretching_stiffness = r.read_f32()?;
        let m_damping = r.read_f32()?;
        let m_thickness = r.read_f32()?;
        let m_use_gravity = r.read_bool()?;
        let m_self_collision = r.read_bool()?;
        r.align(4)?;
        let m_external_acceleration = Vector3f::parse(r, ver)?;
        let m_random_acceleration = Vector3f::parse(r, ver)?;
        let m_mesh = crate::PPtr::parse(r, ver)?;
        let m_friction = r.read_f32()?;
        let m_density = r.read_f32()?;
        let m_pressure = r.read_f32()?;
        let m_collision_response = r.read_f32()?;
        let m_attachment_tear_factor = r.read_f32()?;
        let m_attachment_response = r.read_f32()?;
        let m_tear_factor = r.read_f32()?;
        let m_attached_colliders = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ClothAttachment::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;

        Ok(InteractiveCloth {
            m_object_hide_flags,
            m_extension_ptr,
            m_game_object,
            m_enabled,
            m_bending_stiffness,
            m_stretching_stiffness,
            m_damping,
            m_thickness,
            m_use_gravity,
            m_self_collision,
            m_external_acceleration,
            m_random_acceleration,
            m_mesh,
            m_friction,
            m_density,
            m_pressure,
            m_collision_response,
            m_attachment_tear_factor,
            m_attachment_response,
            m_tear_factor,
            m_attached_colliders,
        })
    }
}
