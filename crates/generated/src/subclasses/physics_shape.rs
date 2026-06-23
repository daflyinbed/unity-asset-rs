//! 自动生成。请勿手动修改。

use crate::subclasses::vector2f::Vector2f;

#[derive(Debug, Clone)]
pub struct PhysicsShape {
    pub m_shape_type: i32,
    pub m_radius: f32,
    pub m_vertex_start_index: i32,
    pub m_vertex_count: i32,
    pub m_use_adjacent_start: i32,
    pub m_use_adjacent_end: i32,
    pub m_adjacent_start: Vector2f,
    pub m_adjacent_end: Vector2f,
}

impl crate::Parse for PhysicsShape {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_shape_type = r.read_i32()?;
        let m_radius = r.read_f32()?;
        let m_vertex_start_index = r.read_i32()?;
        let m_vertex_count = r.read_i32()?;
        let m_use_adjacent_start = r.read_i32()?;
        let m_use_adjacent_end = r.read_i32()?;
        let m_adjacent_start = Vector2f::parse(r, ver)?;
        let m_adjacent_end = Vector2f::parse(r, ver)?;

        Ok(PhysicsShape {
            m_shape_type,
            m_radius,
            m_vertex_start_index,
            m_vertex_count,
            m_use_adjacent_start,
            m_use_adjacent_end,
            m_adjacent_start,
            m_adjacent_end,
        })
    }
}
