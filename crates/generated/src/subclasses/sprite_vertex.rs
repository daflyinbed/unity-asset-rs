//! 自动生成。请勿手动修改。

use crate::subclasses::vector2f::Vector2f;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct SpriteVertex {
    pub pos: Vector3f,
    pub uv: Option<Vector2f>,
}

impl crate::Parse for SpriteVertex {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut pos = None;
        let mut uv = None;
            pos = Some(Vector3f::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(4, 5, 0) {
        } else {
            uv = Some(Vector2f::parse(rdr, ver)?);
        }

        Ok(SpriteVertex {
            pos: pos.unwrap(),
            uv,
        })
    }
}
