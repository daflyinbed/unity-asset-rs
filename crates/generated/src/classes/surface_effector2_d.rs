//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::bit_field::BitField;

#[derive(Debug, Clone)]
pub struct SurfaceEffector2D {
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
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut m_enabled = None;
        let mut m_collider_mask = None;
        let mut m_use_collider_mask = None;
        let mut m_speed = None;
        let mut m_speed_variation = None;
        let mut m_force_scale = None;
        let mut m_use_contact_force = None;
        let mut m_use_friction = None;
        let mut m_use_bounce = None;
            m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
            m_enabled = Some(rdr.read_u8()?);
            rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(5, 0, 2) {
            m_use_collider_mask = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
        }
            m_collider_mask = Some(BitField::parse(rdr, ver)?);
            m_speed = Some(rdr.read_f32()?);
            m_speed_variation = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(5, 0, 1) {
            m_force_scale = Some(rdr.read_f32()?);
            m_use_contact_force = Some(rdr.read_bool()?);
            m_use_friction = Some(rdr.read_bool()?);
            m_use_bounce = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
        }

        Ok(SurfaceEffector2D {
            m_game_object: m_game_object.unwrap(),
            m_enabled: m_enabled.unwrap(),
            m_collider_mask: m_collider_mask.unwrap(),
            m_use_collider_mask,
            m_speed: m_speed.unwrap(),
            m_speed_variation: m_speed_variation.unwrap(),
            m_force_scale,
            m_use_contact_force,
            m_use_friction,
            m_use_bounce,
        })
    }
}
