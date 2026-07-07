//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct OffMeshLink {
    pub m_game_object: PPtr,
    pub m_start: PPtr,
    pub m_nav_mesh_layer: Option<u32>,
    pub m_enabled: Option<u8>,
    pub m_end: PPtr,
    pub m_area_index: Option<u32>,
    pub m_dt_poly_ref: Option<u32>,
    pub m_agent_type_id: Option<i32>,
    pub m_cost_override: f32,
    pub m_bi_directional: bool,
    pub m_activated: bool,
    pub m_auto_update_positions: Option<bool>,
}

impl crate::Parse for OffMeshLink {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_start = crate::PPtr::parse(r, ver)?;
        let m_nav_mesh_layer = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            Some(r.read_u32()?)
        } else {
            None
        };
        let m_enabled = if *ver >= crate::UnityVersion::new(4, 3, 0) {
            let v = r.read_u8()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_end = crate::PPtr::parse(r, ver)?;
        let m_area_index = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_u32()?)
        } else {
            None
        };
        let m_dt_poly_ref = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(r.read_u32()?)
        } else {
            None
        };
        let m_agent_type_id = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_cost_override = r.read_f32()?;
        r.align(4)?;
        let m_bi_directional = r.read_bool()?;
        let m_activated = r.read_bool()?;
        let m_auto_update_positions = if *ver >= crate::UnityVersion::new(4, 3, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };

        Ok(OffMeshLink {
            m_game_object,
            m_start,
            m_nav_mesh_layer,
            m_enabled,
            m_end,
            m_area_index,
            m_dt_poly_ref,
            m_agent_type_id,
            m_cost_override,
            m_bi_directional,
            m_activated,
            m_auto_update_positions,
        })
    }
}
