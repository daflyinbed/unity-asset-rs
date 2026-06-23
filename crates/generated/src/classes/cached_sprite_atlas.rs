//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::guid::Guid;
use crate::subclasses::sprite_render_data::SpriteRenderData;
use super::named_object::NamedObject;

#[derive(Debug, Clone)]
pub struct CachedSpriteAtlas {
    pub parent: NamedObject,
    pub textures: Vec<PPtr>,
    pub frames: Vec<((Guid, i32), SpriteRenderData)>,
    pub alpha_textures: Option<Vec<PPtr>>,
}

impl crate::Parse for CachedSpriteAtlas {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = NamedObject::parse(r, ver)?;
        let textures = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let frames = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = ( { let a = Guid::parse(r, ver)?; let b = r.read_i32()?; Ok((a, b)) } )?; let b = SpriteRenderData::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let alpha_textures = if *ver >= crate::UnityVersion::new(5, 2, 0) {
            Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };

        Ok(CachedSpriteAtlas {
            parent,
            textures,
            frames,
            alpha_textures,
        })
    }
}
