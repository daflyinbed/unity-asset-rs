//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::bit_field::BitField;


/// 版本枚举：PointEffector2DMForceSourceVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PointEffector2DMForceSourceVariant {
    Default(u8),
    V5_5(i32),
}

impl crate::Parse for PointEffector2DMForceSourceVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            let v = r.read_i32()?;
            Ok(Self::V5_5(v))
        } else {
            let v = r.read_u8()?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：PointEffector2DMForceTargetVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PointEffector2DMForceTargetVariant {
    Default(u8),
    V5_5(i32),
}

impl crate::Parse for PointEffector2DMForceTargetVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            let v = r.read_i32()?;
            Ok(Self::V5_5(v))
        } else {
            let v = r.read_u8()?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：PointEffector2DMForceModeVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PointEffector2DMForceModeVariant {
    Default(u8),
    V5_5(i32),
}

impl crate::Parse for PointEffector2DMForceModeVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            let v = r.read_i32()?;
            Ok(Self::V5_5(v))
        } else {
            let v = r.read_u8()?;
            r.align(4)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct PointEffector2D {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_collider_mask: BitField,
    pub m_use_collider_mask: Option<bool>,
    pub m_force_magnitude: f32,
    pub m_force_variation: f32,
    pub m_distance_scale: f32,
    pub m_drag: Option<f32>,
    pub m_angular_drag: Option<f32>,
    pub m_force_source: PointEffector2DMForceSourceVariant,
    pub m_force_target: PointEffector2DMForceTargetVariant,
    pub m_force_mode: PointEffector2DMForceModeVariant,
    pub m_linear_damping: Option<f32>,
    pub m_angular_damping: Option<f32>,
}

impl crate::Parse for PointEffector2D {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut m_enabled = None;
        let mut m_collider_mask = None;
        let mut m_use_collider_mask = None;
        let mut m_force_magnitude = None;
        let mut m_force_variation = None;
        let mut m_distance_scale = None;
        let mut m_drag = None;
        let mut m_angular_drag = None;
        let mut m_force_source = None;
        let mut m_force_target = None;
        let mut m_force_mode = None;
        let mut m_linear_damping = None;
        let mut m_angular_damping = None;
            m_game_object = Some(crate::PPtr::parse(r, ver)?);
            m_enabled = Some(r.read_u8()?);
            r.align(4)?;
        if *ver >= crate::UnityVersion::new(5, 0, 2) {
            m_use_collider_mask = Some(r.read_bool()?);
            r.align(4)?;
        } else {
        }
            m_collider_mask = Some(BitField::parse(r, ver)?);
            m_force_magnitude = Some(r.read_f32()?);
            m_force_variation = Some(r.read_f32()?);
            m_distance_scale = Some(r.read_f32()?);
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
        } else {
            m_drag = Some(r.read_f32()?);
            m_angular_drag = Some(r.read_f32()?);
        }
            m_force_source = Some(PointEffector2DMForceSourceVariant::parse(r, ver)?);
            m_force_target = Some(PointEffector2DMForceTargetVariant::parse(r, ver)?);
            m_force_mode = Some(PointEffector2DMForceModeVariant::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(6000, 1, 0) {
            m_linear_damping = Some(r.read_f32()?);
            m_angular_damping = Some(r.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(5, 5, 0) {
            m_drag = Some(r.read_f32()?);
            m_angular_drag = Some(r.read_f32()?);
        } else {
        }

        Ok(PointEffector2D {
            m_game_object: m_game_object.unwrap(),
            m_enabled: m_enabled.unwrap(),
            m_collider_mask: m_collider_mask.unwrap(),
            m_use_collider_mask,
            m_force_magnitude: m_force_magnitude.unwrap(),
            m_force_variation: m_force_variation.unwrap(),
            m_distance_scale: m_distance_scale.unwrap(),
            m_drag,
            m_angular_drag,
            m_force_source: m_force_source.unwrap(),
            m_force_target: m_force_target.unwrap(),
            m_force_mode: m_force_mode.unwrap(),
            m_linear_damping,
            m_angular_damping,
        })
    }
}
