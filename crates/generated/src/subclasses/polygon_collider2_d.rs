//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::polygon2_d::Polygon2D;

#[derive(Debug, Clone)]
pub struct PolygonCollider2D {
    pub m_object_hide_flags: u32,
    pub m_prefab_parent_object: PPtr,
    pub m_prefab_internal: PPtr,
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_material: PPtr,
    pub m_is_trigger: bool,
    pub m_poly: Polygon2D,
}

impl crate::Parse for PolygonCollider2D {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_prefab_parent_object = crate::PPtr::parse(r, ver)?;
        let m_prefab_internal = crate::PPtr::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_u8()?;
        r.align(4)?;
        let m_material = crate::PPtr::parse(r, ver)?;
        let m_is_trigger = r.read_bool()?;
        r.align(4)?;
        let m_poly = Polygon2D::parse(r, ver)?;

        Ok(PolygonCollider2D {
            m_object_hide_flags,
            m_prefab_parent_object,
            m_prefab_internal,
            m_game_object,
            m_enabled,
            m_material,
            m_is_trigger,
            m_poly,
        })
    }
}
