//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::bit_field::BitField;
use crate::subclasses::vector2f::Vector2f;


/// 版本枚举：EdgeCollider2DMIsTriggerVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum EdgeCollider2DMIsTriggerVariant {
    Default(bool),
    V5_0(bool),
}

impl crate::Parse for EdgeCollider2DMIsTriggerVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = r.read_bool()?;
            Ok(Self::V5_0(v))
        } else {
            let v = r.read_bool()?;
            r.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：EdgeCollider2DMPointsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum EdgeCollider2DMPointsVariant {
    Default(Vec<Vector2f>),
    V2017_1(Vec<Vector2f>),
}

impl crate::Parse for EdgeCollider2DMPointsVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Vector2f::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Vector2f::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：EdgeCollider2DMUsedByEffectorVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum EdgeCollider2DMUsedByEffectorVariant {
    Default(bool),
    V5_6(bool),
    V2023_1(bool),
}

impl crate::Parse for EdgeCollider2DMUsedByEffectorVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Ok(Self::V2023_1(v))
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = r.read_bool()?;
            Ok(Self::V5_6(v))
        } else {
            let v = r.read_bool()?;
            r.align(4)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct EdgeCollider2D {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_material: PPtr,
    pub m_density: Option<f32>,
    pub m_is_trigger: EdgeCollider2DMIsTriggerVariant,
    pub m_points: EdgeCollider2DMPointsVariant,
    pub m_used_by_effector: Option<EdgeCollider2DMUsedByEffectorVariant>,
    pub m_include_layers: Option<BitField>,
    pub m_offset: Option<Vector2f>,
    pub m_exclude_layers: Option<BitField>,
    pub m_used_by_composite: Option<bool>,
    pub m_layer_override_priority: Option<i32>,
    pub m_force_send_layers: Option<BitField>,
    pub m_edge_radius: Option<f32>,
    pub m_force_receive_layers: Option<BitField>,
    pub m_contact_capture_layers: Option<BitField>,
    pub m_adjacent_start_point: Option<Vector2f>,
    pub m_callback_layers: Option<BitField>,
    pub m_adjacent_end_point: Option<Vector2f>,
    pub m_use_adjacent_start_point: Option<bool>,
    pub m_use_adjacent_end_point: Option<bool>,
    pub m_composite_operation: Option<i32>,
    pub m_composite_order: Option<i32>,
}

impl crate::Parse for EdgeCollider2D {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut m_enabled = None;
        let mut m_material = None;
        let mut m_density = None;
        let mut m_is_trigger = None;
        let mut m_points = None;
        let mut m_used_by_effector = None;
        let mut m_include_layers = None;
        let mut m_offset = None;
        let mut m_exclude_layers = None;
        let mut m_used_by_composite = None;
        let mut m_layer_override_priority = None;
        let mut m_force_send_layers = None;
        let mut m_edge_radius = None;
        let mut m_force_receive_layers = None;
        let mut m_contact_capture_layers = None;
        let mut m_adjacent_start_point = None;
        let mut m_callback_layers = None;
        let mut m_adjacent_end_point = None;
        let mut m_use_adjacent_start_point = None;
        let mut m_use_adjacent_end_point = None;
        let mut m_composite_operation = None;
        let mut m_composite_order = None;
            m_game_object = Some(crate::PPtr::parse(r, ver)?);
            m_enabled = Some(r.read_u8()?);
            r.align(4)?;
        if *ver >= crate::UnityVersion::new(5, 3, 0) {
            m_density = Some(r.read_f32()?);
        } else {
        }
            m_material = Some(crate::PPtr::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            m_include_layers = Some(BitField::parse(r, ver)?);
            m_exclude_layers = Some(BitField::parse(r, ver)?);
            m_layer_override_priority = Some(r.read_i32()?);
            m_force_send_layers = Some(BitField::parse(r, ver)?);
            m_force_receive_layers = Some(BitField::parse(r, ver)?);
            m_contact_capture_layers = Some(BitField::parse(r, ver)?);
            m_callback_layers = Some(BitField::parse(r, ver)?);
        } else {
        }
            m_is_trigger = Some(EdgeCollider2DMIsTriggerVariant::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            m_used_by_effector = Some(EdgeCollider2DMUsedByEffectorVariant::parse(r, ver)?);
            m_composite_operation = Some(r.read_i32()?);
            m_composite_order = Some(r.read_i32()?);
            m_offset = Some(Vector2f::parse(r, ver)?);
            m_edge_radius = Some(r.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_used_by_effector = Some(EdgeCollider2DMUsedByEffectorVariant::parse(r, ver)?);
            m_used_by_composite = Some(r.read_bool()?);
            r.align(4)?;
            m_offset = Some(Vector2f::parse(r, ver)?);
            m_edge_radius = Some(r.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_used_by_effector = Some(EdgeCollider2DMUsedByEffectorVariant::parse(r, ver)?);
            m_offset = Some(Vector2f::parse(r, ver)?);
        } else {
        }
            m_points = Some(EdgeCollider2DMPointsVariant::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            m_adjacent_start_point = Some(Vector2f::parse(r, ver)?);
            m_adjacent_end_point = Some(Vector2f::parse(r, ver)?);
            m_use_adjacent_start_point = Some(r.read_bool()?);
            m_use_adjacent_end_point = Some(r.read_bool()?);
            r.align(4)?;
        } else {
        }

        Ok(EdgeCollider2D {
            m_game_object: m_game_object.unwrap(),
            m_enabled: m_enabled.unwrap(),
            m_material: m_material.unwrap(),
            m_density,
            m_is_trigger: m_is_trigger.unwrap(),
            m_points: m_points.unwrap(),
            m_used_by_effector,
            m_include_layers,
            m_offset,
            m_exclude_layers,
            m_used_by_composite,
            m_layer_override_priority,
            m_force_send_layers,
            m_edge_radius,
            m_force_receive_layers,
            m_contact_capture_layers,
            m_adjacent_start_point,
            m_callback_layers,
            m_adjacent_end_point,
            m_use_adjacent_start_point,
            m_use_adjacent_end_point,
            m_composite_operation,
            m_composite_order,
        })
    }
}
