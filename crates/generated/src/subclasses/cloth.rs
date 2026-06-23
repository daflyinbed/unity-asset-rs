//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::cloth_constrain_coefficients::ClothConstrainCoefficients;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct Cloth {
    pub m_object_hide_flags: u32,
    pub m_prefab_parent_object: PPtr,
    pub m_prefab_internal: PPtr,
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_stretching_stiffness: f32,
    pub m_bending_stiffness: f32,
    pub m_use_tethers: bool,
    pub m_use_gravity: bool,
    pub m_damping: f32,
    pub m_external_acceleration: Vector3f,
    pub m_random_acceleration: Vector3f,
    pub m_world_velocity_scale: f32,
    pub m_world_acceleration_scale: f32,
    pub m_friction: f32,
    pub m_collision_mass_scale: f32,
    pub m_use_continuous_collision: bool,
    pub m_use_virtual_particles: bool,
    pub m_solver_frequency: u32,
    pub m_sleep_threshold: f32,
    pub m_coefficients: Vec<ClothConstrainCoefficients>,
    pub m_capsule_colliders: Vec<PPtr>,
    pub m_sphere_colliders: Vec<(PPtr, PPtr)>,
}

impl crate::Parse for Cloth {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_prefab_parent_object = crate::PPtr::parse(r, ver)?;
        let m_prefab_internal = crate::PPtr::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_u8()?;
        r.align(4)?;
        let m_stretching_stiffness = r.read_f32()?;
        let m_bending_stiffness = r.read_f32()?;
        let m_use_tethers = r.read_bool()?;
        let m_use_gravity = r.read_bool()?;
        r.align(4)?;
        let m_damping = r.read_f32()?;
        let m_external_acceleration = Vector3f::parse(r, ver)?;
        let m_random_acceleration = Vector3f::parse(r, ver)?;
        let m_world_velocity_scale = r.read_f32()?;
        let m_world_acceleration_scale = r.read_f32()?;
        let m_friction = r.read_f32()?;
        let m_collision_mass_scale = r.read_f32()?;
        let m_use_continuous_collision = r.read_bool()?;
        let m_use_virtual_particles = r.read_bool()?;
        r.align(4)?;
        let m_solver_frequency = r.read_u32()?;
        let m_sleep_threshold = r.read_f32()?;
        let m_coefficients = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ClothConstrainCoefficients::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_capsule_colliders = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_sphere_colliders = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = crate::PPtr::parse(r, ver)?; let b = crate::PPtr::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;

        Ok(Cloth {
            m_object_hide_flags,
            m_prefab_parent_object,
            m_prefab_internal,
            m_game_object,
            m_enabled,
            m_stretching_stiffness,
            m_bending_stiffness,
            m_use_tethers,
            m_use_gravity,
            m_damping,
            m_external_acceleration,
            m_random_acceleration,
            m_world_velocity_scale,
            m_world_acceleration_scale,
            m_friction,
            m_collision_mass_scale,
            m_use_continuous_collision,
            m_use_virtual_particles,
            m_solver_frequency,
            m_sleep_threshold,
            m_coefficients,
            m_capsule_colliders,
            m_sphere_colliders,
        })
    }
}
