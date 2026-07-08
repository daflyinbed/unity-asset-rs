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
        let m_shapes = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { PhysicsShape::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;
        let m_vertices = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { Vector2f::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;

        Ok(PhysicsShapeGroup2D {
            m_shapes,
            m_vertices,
        })
    }
}
