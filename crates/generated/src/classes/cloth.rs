//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::cloth_constrain_coefficients::ClothConstrainCoefficients;
use crate::subclasses::vector3f::Vector3f;
use super::behaviour::Behaviour;

#[derive(Debug, Clone)]
pub struct Cloth {
    pub parent: Behaviour,
    pub m_game_object: PPtr,
    pub m_enabled: Option<u8>,
    pub m_stretching_stiffness: Option<f32>,
    pub m_bending_stiffness: Option<f32>,
    pub m_use_tethers: Option<bool>,
    pub m_use_gravity: Option<bool>,
    pub m_damping: Option<f32>,
    pub m_external_acceleration: Option<Vector3f>,
    pub m_random_acceleration: Option<Vector3f>,
    pub m_world_velocity_scale: Option<f32>,
    pub m_world_acceleration_scale: Option<f32>,
    pub m_friction: Option<f32>,
    pub m_collision_mass_scale: Option<f32>,
    pub m_use_continuous_collision: Option<bool>,
    pub m_use_virtual_particles: Option<bool>,
    pub m_solver_frequency: Option<u32>,
    pub m_sleep_threshold: Option<f32>,
    pub m_coefficients: Option<Vec<ClothConstrainCoefficients>>,
    pub m_capsule_colliders: Option<Vec<PPtr>>,
    pub m_sphere_colliders: Option<Vec<(PPtr, PPtr)>>,
    pub m_self_collision_distance: Option<f32>,
    pub m_self_collision_stiffness: Option<f32>,
    pub m_self_and_inter_collision_indices: Option<Vec<u32>>,
    pub m_virtual_particle_weights: Option<Vec<Vector3f>>,
    pub m_virtual_particle_indices: Option<Vec<u32>>,
}

impl crate::Parse for Cloth {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = Behaviour::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = r.read_u8()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_stretching_stiffness = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_bending_stiffness = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_use_tethers = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_use_gravity = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_damping = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_external_acceleration = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(Vector3f::parse(r, ver)?)
        } else {
            None
        };
        let m_random_acceleration = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(Vector3f::parse(r, ver)?)
        } else {
            None
        };
        let m_world_velocity_scale = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_world_acceleration_scale = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_friction = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_collision_mass_scale = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_use_continuous_collision = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_use_virtual_particles = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_solver_frequency = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_u32()?)
        } else {
            None
        };
        let m_sleep_threshold = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_coefficients = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ClothConstrainCoefficients::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_capsule_colliders = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };
        let m_sphere_colliders = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = crate::PPtr::parse(r, ver)?; let b = crate::PPtr::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };
        let m_self_collision_distance = if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_self_collision_stiffness = if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_self_and_inter_collision_indices = if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u32() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_virtual_particle_weights = if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Vector3f::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_virtual_particle_indices = if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u32() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(Cloth {
            parent,
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
            m_self_collision_distance,
            m_self_collision_stiffness,
            m_self_and_inter_collision_indices,
            m_virtual_particle_weights,
            m_virtual_particle_indices,
        })
    }
}
