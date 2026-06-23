//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vector2f::Vector2f;

#[derive(Debug, Clone)]
pub struct SplatPrototype {
    pub texture: PPtr,
    pub tile_size: Vector2f,
    pub tile_offset: Vector2f,
}

impl crate::Parse for SplatPrototype {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let texture = crate::PPtr::parse(r, ver)?;
        let tile_size = Vector2f::parse(r, ver)?;
        let tile_offset = Vector2f::parse(r, ver)?;

        Ok(SplatPrototype {
            texture,
            tile_size,
            tile_offset,
        })
    }
}
