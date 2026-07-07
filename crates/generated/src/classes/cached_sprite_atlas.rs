//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::guid::Guid;
use crate::subclasses::sprite_render_data::SpriteRenderData;

#[derive(Debug, Clone)]
pub struct CachedSpriteAtlas {
    pub textures: Vec<PPtr>,
    pub frames: Vec<((Guid, i64), SpriteRenderData)>,
    pub alpha_textures: Option<Vec<PPtr>>,
}

impl crate::Parse for CachedSpriteAtlas {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let textures = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let frames = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = ( { let a = Guid::parse(r, ver)?; let b = r.read_i64()?; Ok((a, b)) } )?; let b = SpriteRenderData::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let alpha_textures = if *ver >= crate::UnityVersion::new(5, 2, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(CachedSpriteAtlas {
            textures,
            frames,
            alpha_textures,
        })
    }
}
