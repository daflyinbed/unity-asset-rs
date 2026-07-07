//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::classes::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct HeightmapData {
    pub position: Vector3f,
    pub terrain_data: PPtr,
}

impl crate::Parse for HeightmapData {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let position = Vector3f::parse(r, ver)?;
        let terrain_data = crate::PPtr::parse(r, ver)?;

        Ok(HeightmapData {
            position,
            terrain_data,
        })
    }
}
