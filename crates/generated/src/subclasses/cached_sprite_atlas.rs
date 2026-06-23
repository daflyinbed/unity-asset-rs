//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::guid::Guid;
use crate::subclasses::sprite_render_data::SpriteRenderData;

#[derive(Debug, Clone)]
pub struct CachedSpriteAtlas {
    pub textures: Vec<PPtr>,
    pub frames: Vec<((Guid, i32), SpriteRenderData)>,
}

impl crate::Parse for CachedSpriteAtlas {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let textures = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let frames = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = ( { let a = Guid::parse(r, ver)?; let b = r.read_i32()?; Ok((a, b)) } )?; let b = SpriteRenderData::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(CachedSpriteAtlas {
            textures,
            frames,
        })
    }
}
