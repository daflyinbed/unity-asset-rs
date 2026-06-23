//! 自动生成。请勿手动修改。

use crate::subclasses::vector2f::Vector2f;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct SpriteVertex {
    pub pos: Vector3f,
    pub uv: Vector2f,
}

impl crate::Parse for SpriteVertex {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let pos = Vector3f::parse(r, ver)?;
        let uv = Vector2f::parse(r, ver)?;

        Ok(SpriteVertex {
            pos,
            uv,
        })
    }
}
