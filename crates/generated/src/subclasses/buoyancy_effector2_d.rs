//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::bit_field::BitField;

#[derive(Debug, Clone)]
pub struct BuoyancyEffector2D {
    pub m_object_hide_flags: u32,
    pub m_prefab_parent_object: PPtr,
    pub m_prefab_internal: PPtr,
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_use_collider_mask: bool,
    pub m_collider_mask: BitField,
    pub m_surface_level: f32,
    pub m_density: f32,
    pub m_linear_drag: f32,
    pub m_angular_drag: f32,
    pub m_flow_angle: f32,
    pub m_flow_magnitude: f32,
    pub m_flow_variation: f32,
}

impl crate::Parse for BuoyancyEffector2D {
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
        let m_use_collider_mask = r.read_bool()?;
        r.align(4)?;
        let m_collider_mask = BitField::parse(r, ver)?;
        let m_surface_level = r.read_f32()?;
        let m_density = r.read_f32()?;
        let m_linear_drag = r.read_f32()?;
        let m_angular_drag = r.read_f32()?;
        let m_flow_angle = r.read_f32()?;
        let m_flow_magnitude = r.read_f32()?;
        let m_flow_variation = r.read_f32()?;

        Ok(BuoyancyEffector2D {
            m_object_hide_flags,
            m_prefab_parent_object,
            m_prefab_internal,
            m_game_object,
            m_enabled,
            m_use_collider_mask,
            m_collider_mask,
            m_surface_level,
            m_density,
            m_linear_drag,
            m_angular_drag,
            m_flow_angle,
            m_flow_magnitude,
            m_flow_variation,
        })
    }
}
