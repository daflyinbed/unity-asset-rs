//! 自动生成。请勿手动修改。

use crate::subclasses::physics_shape::PhysicsShape;
use crate::subclasses::vector2f::Vector2f;

#[derive(Debug, Clone)]
pub struct PhysicsShapeGroup2D {
    pub m_shapes: Vec<PhysicsShape>,
    pub m_vertices: Vec<Vector2f>,
}

impl crate::Parse for PhysicsShapeGroup2D {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_shapes = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| PhysicsShape::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let m_vertices = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| Vector2f::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;

        Ok(PhysicsShapeGroup2D {
            m_shapes,
            m_vertices,
        })
    }
}

impl crate::ToValue for PhysicsShapeGroup2D {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Shapes".to_string(), self.m_shapes.to_value())),
            Some(("m_Vertices".to_string(), self.m_vertices.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Shapes" => (self.m_shapes).inspect_value(path, offset, limit),
                "m_Vertices" => (self.m_vertices).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_shapes).inspect_value(path, offset, limit))
                .map(|v| ("m_Shapes".to_string(), v)),
            ((self.m_vertices).inspect_value(path, offset, limit))
                .map(|v| ("m_Vertices".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
