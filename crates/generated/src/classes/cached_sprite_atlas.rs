//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::guid::Guid;
use crate::subclasses::sprite_render_data::SpriteRenderData;


/// 版本枚举：CachedSpriteAtlasTexturesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum CachedSpriteAtlasTexturesVariant {
    Default(Vec<PPtr>),
    V2017_1(Vec<PPtr>),
}

impl crate::Parse for CachedSpriteAtlasTexturesVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：CachedSpriteAtlasFramesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum CachedSpriteAtlasFramesVariant {
    Default(Vec<((Guid, i32), SpriteRenderData)>),
    V5_0(Vec<((Guid, i64), SpriteRenderData)>),
}

impl crate::Parse for CachedSpriteAtlasFramesVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = ( { let a = Guid::parse(r, ver)?; let b = r.read_i64()?; Ok((a, b)) } )?; let b = SpriteRenderData::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Ok(Self::V5_0(v))
        } else {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = ( { let a = Guid::parse(r, ver)?; let b = r.read_i32()?; Ok((a, b)) } )?; let b = SpriteRenderData::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：CachedSpriteAtlasAlphaTexturesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum CachedSpriteAtlasAlphaTexturesVariant {
    Default(Vec<PPtr>),
    V2017_1(Vec<PPtr>),
}

impl crate::Parse for CachedSpriteAtlasAlphaTexturesVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct CachedSpriteAtlas {
    pub textures: CachedSpriteAtlasTexturesVariant,
    pub frames: CachedSpriteAtlasFramesVariant,
    pub alpha_textures: Option<CachedSpriteAtlasAlphaTexturesVariant>,
}

impl crate::Parse for CachedSpriteAtlas {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut textures = None;
        let mut frames = None;
        let mut alpha_textures = None;
            textures = Some(CachedSpriteAtlasTexturesVariant::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(5, 2, 0) {
            alpha_textures = Some(CachedSpriteAtlasAlphaTexturesVariant::parse(r, ver)?);
        } else {
        }
            frames = Some(CachedSpriteAtlasFramesVariant::parse(r, ver)?);

        Ok(CachedSpriteAtlas {
            textures: textures.unwrap(),
            frames: frames.unwrap(),
            alpha_textures,
        })
    }
}
