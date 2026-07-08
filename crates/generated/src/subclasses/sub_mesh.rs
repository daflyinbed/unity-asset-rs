//! 自动生成。请勿手动修改。

use crate::subclasses::aabb::Aabb;

#[derive(Debug, Clone)]
pub struct SubMesh {
    pub first_byte: u32,
    pub index_count: u32,
    pub is_tri_strip: Option<u32>,
    pub topology: Option<i32>,
    pub triangle_count: Option<u32>,
    pub first_vertex: u32,
    pub base_vertex: Option<u32>,
    pub vertex_count: u32,
    pub local_aabb: Aabb,
}

impl crate::Parse for SubMesh {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut first_byte = None;
        let mut index_count = None;
        let mut is_tri_strip = None;
        let mut topology = None;
        let mut triangle_count = None;
        let mut first_vertex = None;
        let mut base_vertex = None;
        let mut vertex_count = None;
        let mut local_aabb = None;
            first_byte = Some(rdr.read_u32()?);
            index_count = Some(rdr.read_u32()?);
        if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            topology = Some(rdr.read_i32()?);
            base_vertex = Some(rdr.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            topology = Some(rdr.read_i32()?);
        } else {
            is_tri_strip = Some(rdr.read_u32()?);
            triangle_count = Some(rdr.read_u32()?);
        }
            first_vertex = Some(rdr.read_u32()?);
            vertex_count = Some(rdr.read_u32()?);
            local_aabb = Some(Aabb::parse(rdr, ver)?);

        Ok(SubMesh {
            first_byte: first_byte.unwrap(),
            index_count: index_count.unwrap(),
            is_tri_strip,
            topology,
            triangle_count,
            first_vertex: first_vertex.unwrap(),
            base_vertex,
            vertex_count: vertex_count.unwrap(),
            local_aabb: local_aabb.unwrap(),
        })
    }
}
