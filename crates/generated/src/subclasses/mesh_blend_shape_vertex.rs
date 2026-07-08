//! 自动生成。请勿手动修改。

use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct MeshBlendShapeVertex {
    pub vertex: Vector3f,
    pub normal: Vector3f,
    pub tangent: Vector3f,
    pub index: i32,
}

impl crate::Parse for MeshBlendShapeVertex {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let vertex = Vector3f::parse(rdr, ver)?;
        let normal = Vector3f::parse(rdr, ver)?;
        let tangent = Vector3f::parse(rdr, ver)?;
        let index = rdr.read_i32()?;

        Ok(MeshBlendShapeVertex {
            vertex,
            normal,
            tangent,
            index,
        })
    }
}
