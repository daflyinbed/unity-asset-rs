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
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| crate::PPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| crate::PPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for CachedSpriteAtlasTexturesVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
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
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| {
                        ({
                            let a = ({
                                let a = Guid::parse(rdr, ver)?;
                                let b = rdr.read_i64()?;
                                Ok((a, b))
                            })?;
                            let b = SpriteRenderData::parse(rdr, ver)?;
                            Ok((a, b))
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::V5_0(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| {
                        ({
                            let a = ({
                                let a = Guid::parse(rdr, ver)?;
                                let b = rdr.read_i32()?;
                                Ok((a, b))
                            })?;
                            let b = SpriteRenderData::parse(rdr, ver)?;
                            Ok((a, b))
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for CachedSpriteAtlasFramesVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_0(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_0(v) => (v).inspect_value(path, offset, limit),
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
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| crate::PPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| crate::PPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for CachedSpriteAtlasAlphaTexturesVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
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
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut textures = None;
        let mut frames = None;
        let mut alpha_textures = None;
        textures = Some(CachedSpriteAtlasTexturesVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 2, 0) {
            alpha_textures = Some(CachedSpriteAtlasAlphaTexturesVariant::parse(rdr, ver)?);
        } else {
        }
        frames = Some(CachedSpriteAtlasFramesVariant::parse(rdr, ver)?);

        Ok(CachedSpriteAtlas {
            textures: textures.unwrap(),
            frames: frames.unwrap(),
            alpha_textures,
        })
    }
}

impl crate::ToValue for CachedSpriteAtlas {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("textures".to_string(), self.textures.to_value())),
            Some(("frames".to_string(), self.frames.to_value())),
            crate::opt_field("alphaTextures", &self.alpha_textures),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "textures" => (self.textures).inspect_value(path, offset, limit),
                "frames" => (self.frames).inspect_value(path, offset, limit),
                "alphaTextures" => (self.alpha_textures).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.textures).inspect_value(path, offset, limit))
                .map(|v| ("textures".to_string(), v)),
            ((self.frames).inspect_value(path, offset, limit)).map(|v| ("frames".to_string(), v)),
            ((self.alpha_textures).inspect_value(path, offset, limit))
                .map(|v| ("alphaTextures".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
