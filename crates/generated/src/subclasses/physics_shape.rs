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
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_shape_type = rdr.read_i32()?;
        let m_radius = rdr.read_f32()?;
        let m_vertex_start_index = rdr.read_i32()?;
        let m_vertex_count = rdr.read_i32()?;
        let m_use_adjacent_start = rdr.read_i32()?;
        let m_use_adjacent_end = rdr.read_i32()?;
        let m_adjacent_start = Vector2f::parse(rdr, ver)?;
        let m_adjacent_end = Vector2f::parse(rdr, ver)?;

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
