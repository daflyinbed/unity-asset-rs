//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct MeshRenderer {
    pub m_object_hide_flags: u32,
    pub m_prefab_parent_object: PPtr,
    pub m_prefab_internal: PPtr,
    pub m_game_object: PPtr,
    pub m_enabled: bool,
    pub m_cast_shadows: u8,
    pub m_receive_shadows: bool,
    pub m_materials: Vec<PPtr>,
    pub m_subset_indices: Vec<u32>,
    pub m_static_batch_root: PPtr,
    pub m_use_light_probes: bool,
    pub m_reflection_probe_usage: i32,
    pub m_probe_anchor: PPtr,
    pub m_scale_in_lightmap: f32,
    pub m_preserve_u_vs: bool,
    pub m_important_gi: bool,
    pub m_auto_uv_max_distance: f32,
    pub m_auto_uv_max_angle: f32,
    pub m_lightmap_parameters: PPtr,
    pub m_sorting_layer_id: i32,
    pub m_sorting_order: i16,
}

impl crate::Parse for MeshRenderer {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_prefab_parent_object = crate::PPtr::parse(r, ver)?;
        let m_prefab_internal = crate::PPtr::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_bool()?;
        r.align(4)?;
        let m_cast_shadows = r.read_u8()?;
        let m_receive_shadows = r.read_bool()?;
        r.align(4)?;
        let m_materials = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_subset_indices = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u32() }).collect::<Result<Vec<_>, _>>() } )?;
        let m_static_batch_root = crate::PPtr::parse(r, ver)?;
        let m_use_light_probes = r.read_bool()?;
        r.align(4)?;
        let m_reflection_probe_usage = r.read_i32()?;
        let m_probe_anchor = crate::PPtr::parse(r, ver)?;
        let m_scale_in_lightmap = r.read_f32()?;
        let m_preserve_u_vs = r.read_bool()?;
        let m_important_gi = r.read_bool()?;
        r.align(4)?;
        let m_auto_uv_max_distance = r.read_f32()?;
        let m_auto_uv_max_angle = r.read_f32()?;
        let m_lightmap_parameters = crate::PPtr::parse(r, ver)?;
        r.align(4)?;
        let m_sorting_layer_id = r.read_i32()?;
        let m_sorting_order = r.read_i16()?;
        r.align(4)?;

        Ok(MeshRenderer {
            m_object_hide_flags,
            m_prefab_parent_object,
            m_prefab_internal,
            m_game_object,
            m_enabled,
            m_cast_shadows,
            m_receive_shadows,
            m_materials,
            m_subset_indices,
            m_static_batch_root,
            m_use_light_probes,
            m_reflection_probe_usage,
            m_probe_anchor,
            m_scale_in_lightmap,
            m_preserve_u_vs,
            m_important_gi,
            m_auto_uv_max_distance,
            m_auto_uv_max_angle,
            m_lightmap_parameters,
            m_sorting_layer_id,
            m_sorting_order,
        })
    }
}
