//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vector4f::Vector4f;

#[derive(Debug, Clone)]
pub struct ClothRenderer {
    pub m_game_object: PPtr,
    pub m_enabled: bool,
    pub m_cast_shadows: bool,
    pub m_receive_shadows: bool,
    pub m_lightmap_index: u8,
    pub m_lightmap_tiling_offset: Vector4f,
    pub m_materials: Vec<PPtr>,
    pub m_subset_indices: Vec<u32>,
    pub m_static_batch_root: PPtr,
    pub m_pause_when_not_visible: bool,
    pub m_use_light_probes: Option<bool>,
    pub m_light_probe_anchor: Option<PPtr>,
    pub m_sorting_layer: Option<i16>,
    pub m_sorting_layer_id: Option<u32>,
    pub m_sorting_order: Option<i16>,
}

impl crate::Parse for ClothRenderer {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_bool()?;
        let m_cast_shadows = r.read_bool()?;
        let m_receive_shadows = r.read_bool()?;
        let m_lightmap_index = r.read_u8()?;
        let m_lightmap_tiling_offset = Vector4f::parse(r, ver)?;
        let m_materials = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_subset_indices = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u32() }).collect::<Result<Vec<_>, _>>() } )?;
        let m_static_batch_root = crate::PPtr::parse(r, ver)?;
        let m_pause_when_not_visible = r.read_bool()?;
        let m_use_light_probes = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_light_probe_anchor = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = crate::PPtr::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_sorting_layer = if *ver >= crate::UnityVersion::new(4, 3, 0) {
            Some(r.read_i16()?)
        } else {
            None
        };
        let m_sorting_layer_id = if *ver >= crate::UnityVersion::new(4, 5, 0) {
            Some(r.read_u32()?)
        } else {
            None
        };
        let m_sorting_order = if *ver >= crate::UnityVersion::new(4, 3, 0) {
            let v = r.read_i16()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(ClothRenderer {
            m_game_object,
            m_enabled,
            m_cast_shadows,
            m_receive_shadows,
            m_lightmap_index,
            m_lightmap_tiling_offset,
            m_materials,
            m_subset_indices,
            m_static_batch_root,
            m_pause_when_not_visible,
            m_use_light_probes,
            m_light_probe_anchor,
            m_sorting_layer,
            m_sorting_layer_id,
            m_sorting_order,
        })
    }
}
