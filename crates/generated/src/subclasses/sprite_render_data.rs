//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::rectf::Rectf;
use crate::subclasses::sprite_vertex::SpriteVertex;
use crate::subclasses::vector2f::Vector2f;

#[derive(Debug, Clone)]
pub struct SpriteRenderData {
    pub texture: PPtr,
    pub vertices: Vec<SpriteVertex>,
    pub indices: Vec<u16>,
    pub texture_rect: Rectf,
    pub texture_rect_offset: Vector2f,
    pub settings_raw: u32,
}

impl crate::Parse for SpriteRenderData {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let texture = crate::PPtr::parse(r, ver)?;
        let vertices = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { SpriteVertex::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let indices = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u16() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let texture_rect = Rectf::parse(r, ver)?;
        let texture_rect_offset = Vector2f::parse(r, ver)?;
        let settings_raw = r.read_u32()?;

        Ok(SpriteRenderData {
            texture,
            vertices,
            indices,
            texture_rect,
            texture_rect_offset,
            settings_raw,
        })
    }
}
