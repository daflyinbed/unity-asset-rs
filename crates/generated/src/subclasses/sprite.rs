//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::rectf::Rectf;
use crate::subclasses::sprite_render_data::SpriteRenderData;
use crate::subclasses::vector2f::Vector2f;

#[derive(Debug, Clone)]
pub struct Sprite {
    pub m_object_hide_flags: u32,
    pub m_prefab_parent_object: PPtr,
    pub m_prefab_internal: PPtr,
    pub m_name: String,
    pub m_rect: Rectf,
    pub m_offset: Vector2f,
    pub m_pixels_to_units: f32,
    pub m_extrude: u32,
    pub m_atlas_name: String,
    pub m_packing_tag: String,
    pub m_rd: SpriteRenderData,
}

impl crate::Parse for Sprite {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_prefab_parent_object = crate::PPtr::parse(r, ver)?;
        let m_prefab_internal = crate::PPtr::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_rect = Rectf::parse(r, ver)?;
        let m_offset = Vector2f::parse(r, ver)?;
        let m_pixels_to_units = r.read_f32()?;
        let m_extrude = r.read_u32()?;
        let m_atlas_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_packing_tag = r.read_aligned_string()?;
        r.align(4)?;
        let m_rd = SpriteRenderData::parse(r, ver)?;
        r.align(4)?;

        Ok(Sprite {
            m_object_hide_flags,
            m_prefab_parent_object,
            m_prefab_internal,
            m_name,
            m_rect,
            m_offset,
            m_pixels_to_units,
            m_extrude,
            m_atlas_name,
            m_packing_tag,
            m_rd,
        })
    }
}
