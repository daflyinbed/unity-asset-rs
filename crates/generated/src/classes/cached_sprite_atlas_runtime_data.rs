//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::guid::Guid;
use crate::subclasses::hash128::Hash128;
use crate::subclasses::sprite_atlas_data::SpriteAtlasData;


/// 版本枚举：CachedSpriteAtlasRuntimeDataFramesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum CachedSpriteAtlasRuntimeDataFramesVariant {
    Default(Vec<((Guid, i64), SpriteAtlasData)>),
    V2020_2(Vec<((Guid, i64), SpriteAtlasData)>),
}

impl crate::Parse for CachedSpriteAtlasRuntimeDataFramesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = ( { let a = Guid::parse(rdr, ver)?; let b = rdr.read_i64()?; Ok((a, b)) } )?; let b = SpriteAtlasData::parse(rdr, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2020_2(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = ( { let a = Guid::parse(rdr, ver)?; let b = rdr.read_i64()?; Ok((a, b)) } )?; let b = SpriteAtlasData::parse(rdr, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct CachedSpriteAtlasRuntimeData {
    pub textures: Vec<PPtr>,
    pub alpha_textures: Vec<PPtr>,
    pub frames: CachedSpriteAtlasRuntimeDataFramesVariant,
    pub current_packing_hash: Option<Hash128>,
}

impl crate::Parse for CachedSpriteAtlasRuntimeData {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let textures = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;
        let alpha_textures = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;
        let frames = CachedSpriteAtlasRuntimeDataFramesVariant::parse(rdr, ver)?;
        let current_packing_hash = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(Hash128::parse(rdr, ver)?)
        } else {
            None
        };

        Ok(CachedSpriteAtlasRuntimeData {
            textures,
            alpha_textures,
            frames,
            current_packing_hash,
        })
    }
}
