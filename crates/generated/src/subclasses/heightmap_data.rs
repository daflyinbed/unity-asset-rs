//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::matrix4x4f::Matrix4x4f;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct HeightmapData {
    pub position: Option<Vector3f>,
    pub surface_to_terrain: Option<Matrix4x4f>,
    pub terrain_data: PPtr,
    pub is_rotated: Option<bool>,
}

impl crate::Parse for HeightmapData {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let position = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(Vector3f::parse(rdr, ver)?)
        } else {
            None
        };
        let surface_to_terrain = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(Matrix4x4f::parse(rdr, ver)?)
        } else {
            None
        };
        let terrain_data = crate::PPtr::parse(rdr, ver)?;
        let is_rotated = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(rdr.read_bool()?)
        } else {
            None
        };

        Ok(HeightmapData {
            position,
            surface_to_terrain,
            terrain_data,
            is_rotated,
        })
    }
}
