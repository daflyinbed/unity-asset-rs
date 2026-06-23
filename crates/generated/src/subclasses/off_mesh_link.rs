//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct OffMeshLink {
    pub m_object_hide_flags: u32,
    pub m_prefab_parent_object: PPtr,
    pub m_prefab_internal: PPtr,
    pub m_game_object: PPtr,
    pub m_start: PPtr,
    pub m_end: PPtr,
    pub m_dt_poly_ref: u32,
    pub m_cost_override: f32,
    pub m_bi_directional: bool,
    pub m_activated: bool,
}

impl crate::Parse for OffMeshLink {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_prefab_parent_object = crate::PPtr::parse(r, ver)?;
        let m_prefab_internal = crate::PPtr::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_start = crate::PPtr::parse(r, ver)?;
        let m_end = crate::PPtr::parse(r, ver)?;
        let m_dt_poly_ref = r.read_u32()?;
        let m_cost_override = r.read_f32()?;
        r.align(4)?;
        let m_bi_directional = r.read_bool()?;
        let m_activated = r.read_bool()?;

        Ok(OffMeshLink {
            m_object_hide_flags,
            m_prefab_parent_object,
            m_prefab_internal,
            m_game_object,
            m_start,
            m_end,
            m_dt_poly_ref,
            m_cost_override,
            m_bi_directional,
            m_activated,
        })
    }
}
