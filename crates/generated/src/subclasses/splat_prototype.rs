//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vector2f::Vector2f;
use crate::subclasses::vector4f::Vector4f;

#[derive(Debug, Clone)]
pub struct SplatPrototype {
    pub texture: PPtr,
    pub tile_size: Vector2f,
    pub normal_map: Option<PPtr>,
    pub tile_offset: Vector2f,
    pub specular_metallic: Option<Vector4f>,
    pub smoothness: Option<f32>,
}

impl crate::Parse for SplatPrototype {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut texture = None;
        let mut tile_size = None;
        let mut normal_map = None;
        let mut tile_offset = None;
        let mut specular_metallic = None;
        let mut smoothness = None;
        texture = Some(crate::PPtr::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(4, 0, 0) {
            normal_map = Some(crate::PPtr::parse(rdr, ver)?);
        } else {
        }
        tile_size = Some(Vector2f::parse(rdr, ver)?);
        tile_offset = Some(Vector2f::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 0, 1) {
            specular_metallic = Some(Vector4f::parse(rdr, ver)?);
            smoothness = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            specular_metallic = Some(Vector4f::parse(rdr, ver)?);
        } else {
        }

        Ok(SplatPrototype {
            texture: texture.unwrap(),
            tile_size: tile_size.unwrap(),
            normal_map,
            tile_offset: tile_offset.unwrap(),
            specular_metallic,
            smoothness,
        })
    }
}

impl crate::ToValue for SplatPrototype {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("texture".to_string(), self.texture.to_value())),
            Some(("tileSize".to_string(), self.tile_size.to_value())),
            crate::opt_field("normalMap", &self.normal_map),
            Some(("tileOffset".to_string(), self.tile_offset.to_value())),
            crate::opt_field("specularMetallic", &self.specular_metallic),
            crate::opt_field("smoothness", &self.smoothness),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "texture" => (self.texture).inspect_value(path, offset, limit),
                "tileSize" => (self.tile_size).inspect_value(path, offset, limit),
                "normalMap" => (self.normal_map).inspect_value(path, offset, limit),
                "tileOffset" => (self.tile_offset).inspect_value(path, offset, limit),
                "specularMetallic" => (self.specular_metallic).inspect_value(path, offset, limit),
                "smoothness" => (self.smoothness).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.texture).inspect_value(path, offset, limit)).map(|v| ("texture".to_string(), v)),
            ((self.tile_size).inspect_value(path, offset, limit))
                .map(|v| ("tileSize".to_string(), v)),
            ((self.normal_map).inspect_value(path, offset, limit))
                .map(|v| ("normalMap".to_string(), v)),
            ((self.tile_offset).inspect_value(path, offset, limit))
                .map(|v| ("tileOffset".to_string(), v)),
            ((self.specular_metallic).inspect_value(path, offset, limit))
                .map(|v| ("specularMetallic".to_string(), v)),
            ((self.smoothness).inspect_value(path, offset, limit))
                .map(|v| ("smoothness".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
