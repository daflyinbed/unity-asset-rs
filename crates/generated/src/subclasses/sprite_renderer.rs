//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::color_rgba::ColorRgba;
use crate::subclasses::vector4f::Vector4f;

#[derive(Debug, Clone)]
pub struct SpriteRenderer {
    pub m_object_hide_flags: u32,
    pub m_prefab_parent_object: PPtr,
    pub m_prefab_internal: PPtr,
    pub m_game_object: PPtr,
    pub m_enabled: bool,
    pub m_cast_shadows: bool,
    pub m_receive_shadows: bool,
    pub m_lightmap_index: u8,
    pub m_lightmap_tiling_offset: Vector4f,
    pub m_materials: Vec<PPtr>,
    pub m_subset_indices: Vec<u32>,
    pub m_static_batch_root: PPtr,
    pub m_use_light_probes: bool,
    pub m_light_probe_anchor: PPtr,
    pub m_scale_in_lightmap: f32,
    pub m_sorting_layer: i16,
    pub m_sorting_order: i16,
    pub m_sorting_layer_id: u32,
    pub m_sprite: PPtr,
    pub m_color: ColorRgba,
}

impl crate::Parse for SpriteRenderer {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_prefab_parent_object = crate::PPtr::parse(r, ver)?;
        let m_prefab_internal = crate::PPtr::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_bool()?;
        let m_cast_shadows = r.read_bool()?;
        let m_receive_shadows = r.read_bool()?;
        let m_lightmap_index = r.read_u8()?;
        let m_lightmap_tiling_offset = Vector4f::parse(r, ver)?;
        let m_materials = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_subset_indices = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u32() }).collect::<Result<Vec<_>, _>>() } )?;
        let m_static_batch_root = crate::PPtr::parse(r, ver)?;
        let m_use_light_probes = r.read_bool()?;
        r.align(4)?;
        let m_light_probe_anchor = crate::PPtr::parse(r, ver)?;
        let m_scale_in_lightmap = r.read_f32()?;
        r.align(4)?;
        let m_sorting_layer = r.read_i16()?;
        let m_sorting_order = r.read_i16()?;
        let m_sorting_layer_id = r.read_u32()?;
        let m_sprite = crate::PPtr::parse(r, ver)?;
        let m_color = ColorRgba::parse(r, ver)?;

        Ok(SpriteRenderer {
            m_object_hide_flags,
            m_prefab_parent_object,
            m_prefab_internal,
            m_game_object,
            m_enabled,
            m_cast_shadows,
            m_receive_shadows,
            m_lightmap_index,
            m_lightmap_tiling_offset,
            m_materials,
            m_subset_indices,
            m_static_batch_root,
            m_use_light_probes,
            m_light_probe_anchor,
            m_scale_in_lightmap,
            m_sorting_layer,
            m_sorting_order,
            m_sorting_layer_id,
            m_sprite,
            m_color,
        })
    }
}
