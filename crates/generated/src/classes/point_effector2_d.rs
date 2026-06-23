//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::bit_field::BitField;
use super::effector2_d::Effector2D;

#[derive(Debug, Clone)]
pub struct PointEffector2D {
    pub parent: Effector2D,
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_collider_mask: BitField,
    pub m_use_collider_mask: Option<bool>,
    pub m_force_magnitude: f32,
    pub m_force_variation: f32,
    pub m_distance_scale: f32,
    pub m_drag: Option<f32>,
    pub m_angular_drag: Option<f32>,
    pub m_force_source: u8,
    pub m_force_target: u8,
    pub m_force_mode: u8,
    pub m_linear_damping: Option<f32>,
    pub m_angular_damping: Option<f32>,
}

impl crate::Parse for PointEffector2D {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = Effector2D::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_u8()?;
        r.align(4)?;
        let m_collider_mask = BitField::parse(r, ver)?;
        let m_use_collider_mask = if *ver >= crate::UnityVersion::new(5, 0, 2) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_force_magnitude = r.read_f32()?;
        let m_force_variation = r.read_f32()?;
        let m_distance_scale = r.read_f32()?;
        let m_drag = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_angular_drag = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_force_source = r.read_u8()?;
        let m_force_target = r.read_u8()?;
        let m_force_mode = r.read_u8()?;
        r.align(4)?;
        let m_linear_damping = if *ver >= crate::UnityVersion::new(6000, 1, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_angular_damping = if *ver >= crate::UnityVersion::new(6000, 1, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };

        Ok(PointEffector2D {
            parent,
            m_game_object,
            m_enabled,
            m_collider_mask,
            m_use_collider_mask,
            m_force_magnitude,
            m_force_variation,
            m_distance_scale,
            m_drag,
            m_angular_drag,
            m_force_source,
            m_force_target,
            m_force_mode,
            m_linear_damping,
            m_angular_damping,
        })
    }
}
