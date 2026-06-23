//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::guid::Guid;
use crate::subclasses::sprite_atlas_data::SpriteAtlasData;

#[derive(Debug, Clone)]
pub struct CachedSpriteAtlasRuntimeData {
    pub textures: Vec<PPtr>,
    pub alpha_textures: Vec<PPtr>,
    pub frames: Vec<((Guid, i64), SpriteAtlasData)>,
}

impl crate::Parse for CachedSpriteAtlasRuntimeData {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let textures = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let alpha_textures = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let frames = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = ( { let a = Guid::parse(r, ver)?; let b = r.read_i64()?; Ok((a, b)) } )?; let b = SpriteAtlasData::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;

        Ok(CachedSpriteAtlasRuntimeData {
            textures,
            alpha_textures,
            frames,
        })
    }
}
