//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct PhysicsManager {
    pub m_object_hide_flags: u32,
    pub m_gravity: Vector3f,
    pub m_default_material: PPtr,
    pub m_bounce_threshold: f32,
    pub m_sleep_velocity: f32,
    pub m_sleep_angular_velocity: f32,
    pub m_max_angular_velocity: f32,
    pub m_min_penetration_for_penalty: f32,
    pub m_solver_iteration_count: i32,
    pub m_raycasts_hit_triggers: bool,
    pub m_layer_collision_matrix: Vec<u32>,
}

impl crate::Parse for PhysicsManager {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_gravity = Vector3f::parse(r, ver)?;
        let m_default_material = crate::PPtr::parse(r, ver)?;
        let m_bounce_threshold = r.read_f32()?;
        let m_sleep_velocity = r.read_f32()?;
        let m_sleep_angular_velocity = r.read_f32()?;
        let m_max_angular_velocity = r.read_f32()?;
        let m_min_penetration_for_penalty = r.read_f32()?;
        let m_solver_iteration_count = r.read_i32()?;
        let m_raycasts_hit_triggers = r.read_bool()?;
        r.align(4)?;
        let m_layer_collision_matrix = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u32() }).collect::<Result<Vec<_>, _>>() } )?;

        Ok(PhysicsManager {
            m_object_hide_flags,
            m_gravity,
            m_default_material,
            m_bounce_threshold,
            m_sleep_velocity,
            m_sleep_angular_velocity,
            m_max_angular_velocity,
            m_min_penetration_for_penalty,
            m_solver_iteration_count,
            m_raycasts_hit_triggers,
            m_layer_collision_matrix,
        })
    }
}
