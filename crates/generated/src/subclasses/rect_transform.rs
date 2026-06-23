//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vector2f::Vector2f;

#[derive(Debug, Clone)]
pub struct RectTransform {
    pub m_object_hide_flags: u32,
    pub m_prefab_parent_object: PPtr,
    pub m_prefab_internal: PPtr,
    pub m_game_object: PPtr,
    pub m_anchor_min: Vector2f,
    pub m_anchor_max: Vector2f,
    pub m_position: Vector2f,
    pub m_size_delta: Vector2f,
    pub m_pivot: Vector2f,
}

impl crate::Parse for RectTransform {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_prefab_parent_object = crate::PPtr::parse(r, ver)?;
        let m_prefab_internal = crate::PPtr::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_anchor_min = Vector2f::parse(r, ver)?;
        let m_anchor_max = Vector2f::parse(r, ver)?;
        let m_position = Vector2f::parse(r, ver)?;
        let m_size_delta = Vector2f::parse(r, ver)?;
        let m_pivot = Vector2f::parse(r, ver)?;

        Ok(RectTransform {
            m_object_hide_flags,
            m_prefab_parent_object,
            m_prefab_internal,
            m_game_object,
            m_anchor_min,
            m_anchor_max,
            m_position,
            m_size_delta,
            m_pivot,
        })
    }
}
