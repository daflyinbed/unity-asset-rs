//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::color_rgb_af::ColorRgbAf;
use crate::subclasses::vector2f::Vector2f;
use crate::subclasses::vector4f::Vector4f;

#[derive(Debug, Clone)]
pub struct TerrainLayer {
    pub m_name: String,
    pub m_diffuse_texture: PPtr,
    pub m_normal_map_texture: PPtr,
    pub m_mask_map_texture: PPtr,
    pub m_tile_size: Vector2f,
    pub m_tile_offset: Vector2f,
    pub m_specular: ColorRgbAf,
    pub m_metallic: f32,
    pub m_smoothness: f32,
    pub m_normal_scale: f32,
    pub m_diffuse_remap_min: Vector4f,
    pub m_diffuse_remap_max: Vector4f,
    pub m_mask_map_remap_min: Vector4f,
    pub m_mask_map_remap_max: Vector4f,
    pub m_smoothness_source: Option<i32>,
}

impl crate::Parse for TerrainLayer {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        let m_diffuse_texture = crate::PPtr::parse(rdr, ver)?;
        let m_normal_map_texture = crate::PPtr::parse(rdr, ver)?;
        let m_mask_map_texture = crate::PPtr::parse(rdr, ver)?;
        let m_tile_size = Vector2f::parse(rdr, ver)?;
        let m_tile_offset = Vector2f::parse(rdr, ver)?;
        let m_specular = ColorRgbAf::parse(rdr, ver)?;
        let m_metallic = rdr.read_f32()?;
        let m_smoothness = rdr.read_f32()?;
        let m_normal_scale = rdr.read_f32()?;
        let m_diffuse_remap_min = Vector4f::parse(rdr, ver)?;
        let m_diffuse_remap_max = Vector4f::parse(rdr, ver)?;
        let m_mask_map_remap_min = Vector4f::parse(rdr, ver)?;
        let m_mask_map_remap_max = Vector4f::parse(rdr, ver)?;
        let m_smoothness_source = if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            Some(rdr.read_i32()?)
        } else {
            None
        };

        Ok(TerrainLayer {
            m_name,
            m_diffuse_texture,
            m_normal_map_texture,
            m_mask_map_texture,
            m_tile_size,
            m_tile_offset,
            m_specular,
            m_metallic,
            m_smoothness,
            m_normal_scale,
            m_diffuse_remap_min,
            m_diffuse_remap_max,
            m_mask_map_remap_min,
            m_mask_map_remap_max,
            m_smoothness_source,
        })
    }
}

impl crate::ToValue for TerrainLayer {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some((
                "m_DiffuseTexture".to_string(),
                self.m_diffuse_texture.to_value(),
            )),
            Some((
                "m_NormalMapTexture".to_string(),
                self.m_normal_map_texture.to_value(),
            )),
            Some((
                "m_MaskMapTexture".to_string(),
                self.m_mask_map_texture.to_value(),
            )),
            Some(("m_TileSize".to_string(), self.m_tile_size.to_value())),
            Some(("m_TileOffset".to_string(), self.m_tile_offset.to_value())),
            Some(("m_Specular".to_string(), self.m_specular.to_value())),
            Some(("m_Metallic".to_string(), self.m_metallic.to_value())),
            Some(("m_Smoothness".to_string(), self.m_smoothness.to_value())),
            Some(("m_NormalScale".to_string(), self.m_normal_scale.to_value())),
            Some((
                "m_DiffuseRemapMin".to_string(),
                self.m_diffuse_remap_min.to_value(),
            )),
            Some((
                "m_DiffuseRemapMax".to_string(),
                self.m_diffuse_remap_max.to_value(),
            )),
            Some((
                "m_MaskMapRemapMin".to_string(),
                self.m_mask_map_remap_min.to_value(),
            )),
            Some((
                "m_MaskMapRemapMax".to_string(),
                self.m_mask_map_remap_max.to_value(),
            )),
            crate::opt_field("m_SmoothnessSource", &self.m_smoothness_source),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Name" => (self.m_name).inspect_value(path, offset, limit),
                "m_DiffuseTexture" => (self.m_diffuse_texture).inspect_value(path, offset, limit),
                "m_NormalMapTexture" => {
                    (self.m_normal_map_texture).inspect_value(path, offset, limit)
                }
                "m_MaskMapTexture" => (self.m_mask_map_texture).inspect_value(path, offset, limit),
                "m_TileSize" => (self.m_tile_size).inspect_value(path, offset, limit),
                "m_TileOffset" => (self.m_tile_offset).inspect_value(path, offset, limit),
                "m_Specular" => (self.m_specular).inspect_value(path, offset, limit),
                "m_Metallic" => (self.m_metallic).inspect_value(path, offset, limit),
                "m_Smoothness" => (self.m_smoothness).inspect_value(path, offset, limit),
                "m_NormalScale" => (self.m_normal_scale).inspect_value(path, offset, limit),
                "m_DiffuseRemapMin" => {
                    (self.m_diffuse_remap_min).inspect_value(path, offset, limit)
                }
                "m_DiffuseRemapMax" => {
                    (self.m_diffuse_remap_max).inspect_value(path, offset, limit)
                }
                "m_MaskMapRemapMin" => {
                    (self.m_mask_map_remap_min).inspect_value(path, offset, limit)
                }
                "m_MaskMapRemapMax" => {
                    (self.m_mask_map_remap_max).inspect_value(path, offset, limit)
                }
                "m_SmoothnessSource" => {
                    (self.m_smoothness_source).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_diffuse_texture).inspect_value(path, offset, limit))
                .map(|v| ("m_DiffuseTexture".to_string(), v)),
            ((self.m_normal_map_texture).inspect_value(path, offset, limit))
                .map(|v| ("m_NormalMapTexture".to_string(), v)),
            ((self.m_mask_map_texture).inspect_value(path, offset, limit))
                .map(|v| ("m_MaskMapTexture".to_string(), v)),
            ((self.m_tile_size).inspect_value(path, offset, limit))
                .map(|v| ("m_TileSize".to_string(), v)),
            ((self.m_tile_offset).inspect_value(path, offset, limit))
                .map(|v| ("m_TileOffset".to_string(), v)),
            ((self.m_specular).inspect_value(path, offset, limit))
                .map(|v| ("m_Specular".to_string(), v)),
            ((self.m_metallic).inspect_value(path, offset, limit))
                .map(|v| ("m_Metallic".to_string(), v)),
            ((self.m_smoothness).inspect_value(path, offset, limit))
                .map(|v| ("m_Smoothness".to_string(), v)),
            ((self.m_normal_scale).inspect_value(path, offset, limit))
                .map(|v| ("m_NormalScale".to_string(), v)),
            ((self.m_diffuse_remap_min).inspect_value(path, offset, limit))
                .map(|v| ("m_DiffuseRemapMin".to_string(), v)),
            ((self.m_diffuse_remap_max).inspect_value(path, offset, limit))
                .map(|v| ("m_DiffuseRemapMax".to_string(), v)),
            ((self.m_mask_map_remap_min).inspect_value(path, offset, limit))
                .map(|v| ("m_MaskMapRemapMin".to_string(), v)),
            ((self.m_mask_map_remap_max).inspect_value(path, offset, limit))
                .map(|v| ("m_MaskMapRemapMax".to_string(), v)),
            ((self.m_smoothness_source).inspect_value(path, offset, limit))
                .map(|v| ("m_SmoothnessSource".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
