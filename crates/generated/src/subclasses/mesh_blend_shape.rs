//! 自动生成。请勿手动修改。

use crate::subclasses::vector3f::Vector3f;


/// 版本枚举：MeshBlendShapeHasTangentsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum MeshBlendShapeHasTangentsVariant {
    Default(bool),
    V4_3(bool),
}

impl crate::Parse for MeshBlendShapeHasTangentsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(4, 3, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V4_3(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct MeshBlendShape {
    pub name: Option<String>,
    pub first_vertex: u32,
    pub vertex_count: u32,
    pub has_normals: bool,
    pub aabb_min_delta: Option<Vector3f>,
    pub has_tangents: MeshBlendShapeHasTangentsVariant,
    pub aabb_max_delta: Option<Vector3f>,
}

impl crate::Parse for MeshBlendShape {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut name = None;
        let mut first_vertex = None;
        let mut vertex_count = None;
        let mut has_normals = None;
        let mut aabb_min_delta = None;
        let mut has_tangents = None;
        let mut aabb_max_delta = None;
        if *ver >= crate::UnityVersion::new(4, 3, 0) {
        } else {
            name = Some(rdr.read_aligned_string()?);
        }
            first_vertex = Some(rdr.read_u32()?);
            vertex_count = Some(rdr.read_u32()?);
        if *ver >= crate::UnityVersion::new(4, 3, 0) {
        } else {
            aabb_min_delta = Some(Vector3f::parse(rdr, ver)?);
            aabb_max_delta = Some(Vector3f::parse(rdr, ver)?);
        }
            has_normals = Some(rdr.read_bool()?);
            has_tangents = Some(MeshBlendShapeHasTangentsVariant::parse(rdr, ver)?);

        Ok(MeshBlendShape {
            name,
            first_vertex: first_vertex.unwrap(),
            vertex_count: vertex_count.unwrap(),
            has_normals: has_normals.unwrap(),
            aabb_min_delta,
            has_tangents: has_tangents.unwrap(),
            aabb_max_delta,
        })
    }
}
