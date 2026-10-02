//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::guid::Guid;
use crate::subclasses::hash128::Hash128;
use crate::subclasses::sprite_atlas_data::SpriteAtlasData;

#[derive(Debug, Clone)]
pub struct CachedSpriteAtlasRuntimeData {
    pub textures: Vec<PPtr>,
    pub alpha_textures: Vec<PPtr>,
    pub frames: Vec<((Guid, i64), SpriteAtlasData)>,
    pub current_packing_hash: Option<Hash128>,
}

impl crate::Parse for CachedSpriteAtlasRuntimeData {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let textures = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| crate::PPtr::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let alpha_textures = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| crate::PPtr::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let frames = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| {
                    ({
                        let a = ({
                            let a = Guid::parse(rdr, ver)?;
                            let b = rdr.read_i64()?;
                            Ok((a, b))
                        })?;
                        let b = SpriteAtlasData::parse(rdr, ver)?;
                        Ok((a, b))
                    })
                })
                .collect::<Result<Vec<_>, _>>()
        })?;
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

impl crate::ToValue for CachedSpriteAtlasRuntimeData {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("textures".to_string(), self.textures.to_value())),
            Some(("alphaTextures".to_string(), self.alpha_textures.to_value())),
            Some(("frames".to_string(), self.frames.to_value())),
            crate::opt_field("currentPackingHash", &self.current_packing_hash),
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
                "alphaTextures" => (self.alpha_textures).inspect_value(path, offset, limit),
                "frames" => (self.frames).inspect_value(path, offset, limit),
                "currentPackingHash" => {
                    (self.current_packing_hash).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.textures).inspect_value(path, offset, limit))
                .map(|v| ("textures".to_string(), v)),
            ((self.alpha_textures).inspect_value(path, offset, limit))
                .map(|v| ("alphaTextures".to_string(), v)),
            ((self.frames).inspect_value(path, offset, limit)).map(|v| ("frames".to_string(), v)),
            ((self.current_packing_hash).inspect_value(path, offset, limit))
                .map(|v| ("currentPackingHash".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
