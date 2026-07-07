//! 自动生成。请勿手动修改。

use crate::classes::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct HeightMeshBvNode {
    pub min: Vector3f,
    pub max: Vector3f,
    pub i: i32,
    pub n: i32,
}

impl crate::Parse for HeightMeshBvNode {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let min = Vector3f::parse(r, ver)?;
        let max = Vector3f::parse(r, ver)?;
        let i = r.read_i32()?;
        let n = r.read_i32()?;

        Ok(HeightMeshBvNode {
            min,
            max,
            i,
            n,
        })
    }
}
