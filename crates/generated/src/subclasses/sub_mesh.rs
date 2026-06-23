//! 自动生成。请勿手动修改。

use crate::subclasses::aabb::Aabb;

#[derive(Debug, Clone)]
pub struct SubMesh {
    pub first_byte: u32,
    pub index_count: u32,
    pub is_tri_strip: u32,
    pub triangle_count: u32,
    pub first_vertex: u32,
    pub vertex_count: u32,
    pub local_aabb: Aabb,
}

impl crate::Parse for SubMesh {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let first_byte = r.read_u32()?;
        let index_count = r.read_u32()?;
        let is_tri_strip = r.read_u32()?;
        let triangle_count = r.read_u32()?;
        let first_vertex = r.read_u32()?;
        let vertex_count = r.read_u32()?;
        let local_aabb = Aabb::parse(r, ver)?;

        Ok(SubMesh {
            first_byte,
            index_count,
            is_tri_strip,
            triangle_count,
            first_vertex,
            vertex_count,
            local_aabb,
        })
    }
}
