//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::bit_field::BitField;
use super::effector2_d::Effector2D;

#[derive(Debug, Clone)]
pub struct SurfaceEffector2D {
    pub parent: Effector2D,
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_collider_mask: BitField,
    pub m_use_collider_mask: Option<bool>,
    pub m_speed: f32,
    pub m_speed_variation: f32,
    pub m_force_scale: Option<f32>,
    pub m_use_contact_force: Option<bool>,
    pub m_use_friction: Option<bool>,
    pub m_use_bounce: Option<bool>,
}

impl crate::Parse for SurfaceEffector2D {
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
        let m_speed = r.read_f32()?;
        let m_speed_variation = r.read_f32()?;
        let m_force_scale = if *ver >= crate::UnityVersion::new(5, 0, 1) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_use_contact_force = if *ver >= crate::UnityVersion::new(5, 0, 1) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_use_friction = if *ver >= crate::UnityVersion::new(5, 0, 1) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_use_bounce = if *ver >= crate::UnityVersion::new(5, 0, 1) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(SurfaceEffector2D {
            parent,
            m_game_object,
            m_enabled,
            m_collider_mask,
            m_use_collider_mask,
            m_speed,
            m_speed_variation,
            m_force_scale,
            m_use_contact_force,
            m_use_friction,
            m_use_bounce,
        })
    }
}
