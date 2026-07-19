//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::bit_field::BitField;

/// 版本枚举：AreaEffector2DMForceTargetVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AreaEffector2DMForceTargetVariant {
    Default(u8),
    V5_5(i32),
}

impl crate::Parse for AreaEffector2DMForceTargetVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            let v = rdr.read_i32()?;
            Ok(Self::V5_5(v))
        } else {
            let v = rdr.read_u8()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct AreaEffector2D {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_collider_mask: BitField,
    pub m_use_collider_mask: Option<bool>,
    pub m_force_direction: Option<f32>,
    pub m_force_magnitude: f32,
    pub m_use_global_angle: Option<bool>,
    pub m_force_variation: f32,
    pub m_force_angle: Option<f32>,
    pub m_drag: Option<f32>,
    pub m_angular_drag: Option<f32>,
    pub m_force_target: AreaEffector2DMForceTargetVariant,
    pub m_linear_damping: Option<f32>,
    pub m_angular_damping: Option<f32>,
}

impl crate::Parse for AreaEffector2D {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut m_enabled = None;
        let mut m_collider_mask = None;
        let mut m_use_collider_mask = None;
        let mut m_force_direction = None;
        let mut m_force_magnitude = None;
        let mut m_use_global_angle = None;
        let mut m_force_variation = None;
        let mut m_force_angle = None;
        let mut m_drag = None;
        let mut m_angular_drag = None;
        let mut m_force_target = None;
        let mut m_linear_damping = None;
        let mut m_angular_damping = None;
        m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
        m_enabled = Some(rdr.read_u8()?);
        rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(5, 0, 2) {
            m_use_collider_mask = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
        }
        m_collider_mask = Some(BitField::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 0, 2) {
            m_use_global_angle = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_force_angle = Some(rdr.read_f32()?);
        } else {
            m_force_direction = Some(rdr.read_f32()?);
        }
        m_force_magnitude = Some(rdr.read_f32()?);
        m_force_variation = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(6000, 1, 0) {
            m_force_target = Some(AreaEffector2DMForceTargetVariant::parse(rdr, ver)?);
            m_linear_damping = Some(rdr.read_f32()?);
            m_angular_damping = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(5, 5, 0) {
            m_force_target = Some(AreaEffector2DMForceTargetVariant::parse(rdr, ver)?);
            m_drag = Some(rdr.read_f32()?);
            m_angular_drag = Some(rdr.read_f32()?);
        } else {
            m_drag = Some(rdr.read_f32()?);
            m_angular_drag = Some(rdr.read_f32()?);
            m_force_target = Some(AreaEffector2DMForceTargetVariant::parse(rdr, ver)?);
        }

        Ok(AreaEffector2D {
            m_game_object: m_game_object.unwrap(),
            m_enabled: m_enabled.unwrap(),
            m_collider_mask: m_collider_mask.unwrap(),
            m_use_collider_mask,
            m_force_direction,
            m_force_magnitude: m_force_magnitude.unwrap(),
            m_use_global_angle,
            m_force_variation: m_force_variation.unwrap(),
            m_force_angle,
            m_drag,
            m_angular_drag,
            m_force_target: m_force_target.unwrap(),
            m_linear_damping,
            m_angular_damping,
        })
    }
}
