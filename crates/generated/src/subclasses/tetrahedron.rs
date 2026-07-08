//! 自动生成。请勿手动修改。

use crate::subclasses::matrix3x4f::Matrix3x4f;

#[derive(Debug, Clone)]
pub struct Tetrahedron {
    pub indices_0: i32,
    pub indices_1: i32,
    pub indices_2: i32,
    pub indices_3: i32,
    pub neighbors_0: i32,
    pub neighbors_1: i32,
    pub neighbors_2: i32,
    pub neighbors_3: i32,
    pub matrix: Matrix3x4f,
}

impl crate::Parse for Tetrahedron {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let indices_0 = rdr.read_i32()?;
        let indices_1 = rdr.read_i32()?;
        let indices_2 = rdr.read_i32()?;
        let indices_3 = rdr.read_i32()?;
        let neighbors_0 = rdr.read_i32()?;
        let neighbors_1 = rdr.read_i32()?;
        let neighbors_2 = rdr.read_i32()?;
        let neighbors_3 = rdr.read_i32()?;
        let matrix = Matrix3x4f::parse(rdr, ver)?;

        Ok(Tetrahedron {
            indices_0,
            indices_1,
            indices_2,
            indices_3,
            neighbors_0,
            neighbors_1,
            neighbors_2,
            neighbors_3,
            matrix,
        })
    }
}
