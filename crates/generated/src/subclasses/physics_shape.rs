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

impl crate::ToValue for PhysicsShape {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_ShapeType".to_string(), self.m_shape_type.to_value())),
            Some(("m_Radius".to_string(), self.m_radius.to_value())),
            Some((
                "m_VertexStartIndex".to_string(),
                self.m_vertex_start_index.to_value(),
            )),
            Some(("m_VertexCount".to_string(), self.m_vertex_count.to_value())),
            Some((
                "m_UseAdjacentStart".to_string(),
                self.m_use_adjacent_start.to_value(),
            )),
            Some((
                "m_UseAdjacentEnd".to_string(),
                self.m_use_adjacent_end.to_value(),
            )),
            Some((
                "m_AdjacentStart".to_string(),
                self.m_adjacent_start.to_value(),
            )),
            Some(("m_AdjacentEnd".to_string(), self.m_adjacent_end.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_ShapeType" => (self.m_shape_type).inspect_value(path, offset, limit),
                "m_Radius" => (self.m_radius).inspect_value(path, offset, limit),
                "m_VertexStartIndex" => {
                    (self.m_vertex_start_index).inspect_value(path, offset, limit)
                }
                "m_VertexCount" => (self.m_vertex_count).inspect_value(path, offset, limit),
                "m_UseAdjacentStart" => {
                    (self.m_use_adjacent_start).inspect_value(path, offset, limit)
                }
                "m_UseAdjacentEnd" => (self.m_use_adjacent_end).inspect_value(path, offset, limit),
                "m_AdjacentStart" => (self.m_adjacent_start).inspect_value(path, offset, limit),
                "m_AdjacentEnd" => (self.m_adjacent_end).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_shape_type).inspect_value(path, offset, limit))
                .map(|v| ("m_ShapeType".to_string(), v)),
            ((self.m_radius).inspect_value(path, offset, limit))
                .map(|v| ("m_Radius".to_string(), v)),
            ((self.m_vertex_start_index).inspect_value(path, offset, limit))
                .map(|v| ("m_VertexStartIndex".to_string(), v)),
            ((self.m_vertex_count).inspect_value(path, offset, limit))
                .map(|v| ("m_VertexCount".to_string(), v)),
            ((self.m_use_adjacent_start).inspect_value(path, offset, limit))
                .map(|v| ("m_UseAdjacentStart".to_string(), v)),
            ((self.m_use_adjacent_end).inspect_value(path, offset, limit))
                .map(|v| ("m_UseAdjacentEnd".to_string(), v)),
            ((self.m_adjacent_start).inspect_value(path, offset, limit))
                .map(|v| ("m_AdjacentStart".to_string(), v)),
            ((self.m_adjacent_end).inspect_value(path, offset, limit))
                .map(|v| ("m_AdjacentEnd".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
