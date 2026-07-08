//! 自动生成。请勿手动修改。

use crate::subclasses::vector2f::Vector2f;
use crate::subclasses::vector4f::Vector4f;

#[derive(Debug, Clone)]
pub struct SpriteTilingProperty {
    pub border: Vector4f,
    pub pivot: Vector2f,
    pub old_size: Vector2f,
    pub new_size: Vector2f,
    pub adaptive_tiling_threshold: f32,
    pub draw_mode: i32,
    pub adaptive_tiling: bool,
}

impl crate::Parse for SpriteTilingProperty {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let border = Vector4f::parse(rdr, ver)?;
        let pivot = Vector2f::parse(rdr, ver)?;
        let old_size = Vector2f::parse(rdr, ver)?;
        let new_size = Vector2f::parse(rdr, ver)?;
        let adaptive_tiling_threshold = rdr.read_f32()?;
        let draw_mode = rdr.read_i32()?;
        let adaptive_tiling = rdr.read_bool()?;
        rdr.align(4)?;

        Ok(SpriteTilingProperty {
            border,
            pivot,
            old_size,
            new_size,
            adaptive_tiling_threshold,
            draw_mode,
            adaptive_tiling,
        })
    }
}
