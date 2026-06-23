//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vector2f::Vector2f;

#[derive(Debug, Clone)]
pub struct Physics2DSettings {
    pub m_object_hide_flags: u32,
    pub m_gravity: Vector2f,
    pub m_default_material: PPtr,
    pub m_velocity_iterations: i32,
    pub m_position_iterations: i32,
    pub m_raycasts_hit_triggers: bool,
    pub m_layer_collision_matrix: Vec<u32>,
}

impl crate::Parse for Physics2DSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_gravity = Vector2f::parse(r, ver)?;
        let m_default_material = crate::PPtr::parse(r, ver)?;
        let m_velocity_iterations = r.read_i32()?;
        let m_position_iterations = r.read_i32()?;
        let m_raycasts_hit_triggers = r.read_bool()?;
        r.align(4)?;
        let m_layer_collision_matrix = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u32() }).collect::<Result<Vec<_>, _>>() } )?;

        Ok(Physics2DSettings {
            m_object_hide_flags,
            m_gravity,
            m_default_material,
            m_velocity_iterations,
            m_position_iterations,
            m_raycasts_hit_triggers,
            m_layer_collision_matrix,
        })
    }
}
