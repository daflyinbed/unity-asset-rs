//! 自动生成。请勿手动修改。

use crate::classes::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct MeshBlendShape {
    pub name: String,
    pub first_vertex: u32,
    pub vertex_count: u32,
    pub aabb_min_delta: Vector3f,
    pub aabb_max_delta: Vector3f,
    pub has_normals: bool,
    pub has_tangents: bool,
}

impl crate::Parse for MeshBlendShape {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let name = r.read_aligned_string()?;
        r.align(4)?;
        let first_vertex = r.read_u32()?;
        let vertex_count = r.read_u32()?;
        let aabb_min_delta = Vector3f::parse(r, ver)?;
        let aabb_max_delta = Vector3f::parse(r, ver)?;
        let has_normals = r.read_bool()?;
        let has_tangents = r.read_bool()?;

        Ok(MeshBlendShape {
            name,
            first_vertex,
            vertex_count,
            aabb_min_delta,
            aabb_max_delta,
            has_normals,
            has_tangents,
        })
    }
}
