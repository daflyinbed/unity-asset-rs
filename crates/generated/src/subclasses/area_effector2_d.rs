//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::bit_field::BitField;

#[derive(Debug, Clone)]
pub struct AreaEffector2D {
    pub m_object_hide_flags: u32,
    pub m_prefab_parent_object: PPtr,
    pub m_prefab_internal: PPtr,
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_collider_mask: BitField,
    pub m_force_direction: f32,
    pub m_force_magnitude: f32,
    pub m_force_variation: f32,
    pub m_drag: f32,
    pub m_angular_drag: f32,
    pub m_force_target: u8,
}

impl crate::Parse for AreaEffector2D {
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
        let m_collider_mask = BitField::parse(r, ver)?;
        let m_force_direction = r.read_f32()?;
        let m_force_magnitude = r.read_f32()?;
        let m_force_variation = r.read_f32()?;
        let m_drag = r.read_f32()?;
        let m_angular_drag = r.read_f32()?;
        let m_force_target = r.read_u8()?;
        r.align(4)?;

        Ok(AreaEffector2D {
            m_object_hide_flags,
            m_prefab_parent_object,
            m_prefab_internal,
            m_game_object,
            m_enabled,
            m_collider_mask,
            m_force_direction,
            m_force_magnitude,
            m_force_variation,
            m_drag,
            m_angular_drag,
            m_force_target,
        })
    }
}
