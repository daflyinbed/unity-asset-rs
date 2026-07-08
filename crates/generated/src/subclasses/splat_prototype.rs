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
