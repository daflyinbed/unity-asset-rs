//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::bit_field::BitField;


/// 版本枚举：PlatformEffector2DMUseOneWayVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PlatformEffector2DMUseOneWayVariant {
    Default(bool),
    V5_1_2(bool),
    V5_2_4(bool),
}

impl crate::Parse for PlatformEffector2DMUseOneWayVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 2, 4) {
            let v = r.read_bool()?;
            Ok(Self::V5_2_4(v))
        } else if *ver >= crate::UnityVersion::new(5, 1, 2) {
            let v = r.read_bool()?;
            r.align(4)?;
            Ok(Self::V5_1_2(v))
        } else {
            let v = r.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct PlatformEffector2D {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_collider_mask: BitField,
    pub m_use_collider_mask: Option<bool>,
    pub m_one_way: Option<bool>,
    pub m_use_one_way: Option<PlatformEffector2DMUseOneWayVariant>,
    pub m_side_friction: Option<bool>,
    pub m_use_side_friction: Option<bool>,
    pub m_rotational_offset: Option<f32>,
    pub m_side_bounce: Option<bool>,
    pub m_use_side_bounce: Option<bool>,
    pub m_surface_arc: Option<f32>,
    pub m_use_one_way_grouping: Option<bool>,
    pub m_side_angle_variance: Option<f32>,
    pub m_side_arc: Option<f32>,
}

impl crate::Parse for PlatformEffector2D {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut m_enabled = None;
        let mut m_collider_mask = None;
        let mut m_use_collider_mask = None;
        let mut m_one_way = None;
        let mut m_use_one_way = None;
        let mut m_side_friction = None;
        let mut m_use_side_friction = None;
        let mut m_rotational_offset = None;
        let mut m_side_bounce = None;
        let mut m_use_side_bounce = None;
        let mut m_surface_arc = None;
        let mut m_use_one_way_grouping = None;
        let mut m_side_angle_variance = None;
        let mut m_side_arc = None;
            m_game_object = Some(crate::PPtr::parse(r, ver)?);
            m_enabled = Some(r.read_u8()?);
            r.align(4)?;
        if *ver >= crate::UnityVersion::new(5, 0, 2) {
            m_use_collider_mask = Some(r.read_bool()?);
            r.align(4)?;
        } else {
        }
            m_collider_mask = Some(BitField::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            m_rotational_offset = Some(r.read_f32()?);
            m_use_one_way = Some(PlatformEffector2DMUseOneWayVariant::parse(r, ver)?);
            m_use_one_way_grouping = Some(r.read_bool()?);
            r.align(4)?;
            m_surface_arc = Some(r.read_f32()?);
            m_use_side_friction = Some(r.read_bool()?);
            m_use_side_bounce = Some(r.read_bool()?);
            r.align(4)?;
            m_side_arc = Some(r.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(5, 2, 4) {
            m_use_one_way = Some(PlatformEffector2DMUseOneWayVariant::parse(r, ver)?);
            m_use_one_way_grouping = Some(r.read_bool()?);
            r.align(4)?;
            m_surface_arc = Some(r.read_f32()?);
            m_use_side_friction = Some(r.read_bool()?);
            m_use_side_bounce = Some(r.read_bool()?);
            r.align(4)?;
            m_side_arc = Some(r.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(5, 2, 2) {
            m_use_one_way = Some(PlatformEffector2DMUseOneWayVariant::parse(r, ver)?);
            m_surface_arc = Some(r.read_f32()?);
            m_use_side_friction = Some(r.read_bool()?);
            m_use_side_bounce = Some(r.read_bool()?);
            r.align(4)?;
            m_side_arc = Some(r.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(5, 1, 2) {
            m_use_one_way = Some(PlatformEffector2DMUseOneWayVariant::parse(r, ver)?);
            m_surface_arc = Some(r.read_f32()?);
            m_use_side_friction = Some(r.read_bool()?);
            m_use_side_bounce = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 0, 1) {
            m_use_one_way = Some(PlatformEffector2DMUseOneWayVariant::parse(r, ver)?);
            m_use_side_friction = Some(r.read_bool()?);
            m_use_side_bounce = Some(r.read_bool()?);
            r.align(4)?;
            m_side_angle_variance = Some(r.read_f32()?);
        } else {
            m_one_way = Some(r.read_bool()?);
            m_side_friction = Some(r.read_bool()?);
            m_side_bounce = Some(r.read_bool()?);
            r.align(4)?;
            m_side_angle_variance = Some(r.read_f32()?);
        }

        Ok(PlatformEffector2D {
            m_game_object: m_game_object.unwrap(),
            m_enabled: m_enabled.unwrap(),
            m_collider_mask: m_collider_mask.unwrap(),
            m_use_collider_mask,
            m_one_way,
            m_use_one_way,
            m_side_friction,
            m_use_side_friction,
            m_rotational_offset,
            m_side_bounce,
            m_use_side_bounce,
            m_surface_arc,
            m_use_one_way_grouping,
            m_side_angle_variance,
            m_side_arc,
        })
    }
}
