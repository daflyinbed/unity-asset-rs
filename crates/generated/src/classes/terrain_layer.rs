//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::color_rgba::ColorRgba;
use crate::subclasses::vector2f::Vector2f;
use crate::subclasses::vector4f::Vector4f;
use super::named_object::NamedObject;

#[derive(Debug, Clone)]
pub struct TerrainLayer {
    pub parent: NamedObject,
    pub m_name: String,
    pub m_diffuse_texture: PPtr,
    pub m_normal_map_texture: PPtr,
    pub m_mask_map_texture: PPtr,
    pub m_tile_size: Vector2f,
    pub m_tile_offset: Vector2f,
    pub m_specular: ColorRgba,
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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = NamedObject::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_diffuse_texture = crate::PPtr::parse(r, ver)?;
        let m_normal_map_texture = crate::PPtr::parse(r, ver)?;
        let m_mask_map_texture = crate::PPtr::parse(r, ver)?;
        let m_tile_size = Vector2f::parse(r, ver)?;
        let m_tile_offset = Vector2f::parse(r, ver)?;
        let m_specular = ColorRgba::parse(r, ver)?;
        let m_metallic = r.read_f32()?;
        let m_smoothness = r.read_f32()?;
        let m_normal_scale = r.read_f32()?;
        let m_diffuse_remap_min = Vector4f::parse(r, ver)?;
        let m_diffuse_remap_max = Vector4f::parse(r, ver)?;
        let m_mask_map_remap_min = Vector4f::parse(r, ver)?;
        let m_mask_map_remap_max = Vector4f::parse(r, ver)?;
        let m_smoothness_source = if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };

        Ok(TerrainLayer {
            parent,
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
