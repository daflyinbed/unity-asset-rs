//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::bit_field::BitField;

#[derive(Debug, Clone)]
pub struct PlatformEffector2D {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_collider_mask: BitField,
    pub m_use_collider_mask: Option<bool>,
    pub m_one_way: Option<bool>,
    pub m_use_one_way: Option<bool>,
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
        let m_one_way = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_use_one_way = if *ver >= crate::UnityVersion::new(5, 0, 1) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_side_friction = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_use_side_friction = if *ver >= crate::UnityVersion::new(5, 0, 1) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_rotational_offset = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_side_bounce = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_use_side_bounce = if *ver >= crate::UnityVersion::new(5, 0, 1) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_surface_arc = if *ver >= crate::UnityVersion::new(5, 1, 2) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_use_one_way_grouping = if *ver >= crate::UnityVersion::new(5, 2, 4) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_side_angle_variance = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_side_arc = if *ver >= crate::UnityVersion::new(5, 2, 2) {
            Some(r.read_f32()?)
        } else {
            None
        };

        Ok(PlatformEffector2D {
            m_game_object,
            m_enabled,
            m_collider_mask,
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
