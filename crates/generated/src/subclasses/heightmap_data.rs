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
        let mut position = None;
        let mut surface_to_terrain = None;
        let mut terrain_data = None;
        let mut is_rotated = None;
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            surface_to_terrain = Some(Matrix4x4f::parse(rdr, ver)?);
        } else {
            position = Some(Vector3f::parse(rdr, ver)?);
        }
            terrain_data = Some(crate::PPtr::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            is_rotated = Some(rdr.read_bool()?);
        } else {
        }

        Ok(HeightmapData {
            position,
            surface_to_terrain,
            terrain_data: terrain_data.unwrap(),
            is_rotated,
        })
    }
}
