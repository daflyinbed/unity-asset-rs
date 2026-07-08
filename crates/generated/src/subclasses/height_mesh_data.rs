//! 自动生成。请勿手动修改。

use crate::subclasses::aabb::Aabb;
use crate::subclasses::height_mesh_bv_node::HeightMeshBvNode;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct HeightMeshData {
    pub m_vertices: Vec<Vector3f>,
    pub m_indices: Vec<i32>,
    pub m_bounds: Aabb,
    pub m_nodes: Vec<HeightMeshBvNode>,
}

impl crate::Parse for HeightMeshData {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_vertices = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Vector3f::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_indices = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i32() }).collect::<Result<Vec<_>, _>>() } )?;
        let m_bounds = Aabb::parse(r, ver)?;
        let m_nodes = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { HeightMeshBvNode::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;

        Ok(HeightMeshData {
            m_vertices,
            m_indices,
            m_bounds,
            m_nodes,
        })
    }
}
