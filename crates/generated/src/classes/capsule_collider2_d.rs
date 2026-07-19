//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::bit_field::BitField;
use crate::subclasses::vector2f::Vector2f;

/// 版本枚举：CapsuleCollider2DMUsedByEffectorVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum CapsuleCollider2DMUsedByEffectorVariant {
    Default(bool),
    V5_6(bool),
    V2023_1(bool),
}

impl crate::Parse for CapsuleCollider2DMUsedByEffectorVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V2023_1(v))
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V5_6(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct CapsuleCollider2D {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_density: f32,
    pub m_material: PPtr,
    pub m_is_trigger: bool,
    pub m_include_layers: Option<BitField>,
    pub m_used_by_effector: CapsuleCollider2DMUsedByEffectorVariant,
    pub m_exclude_layers: Option<BitField>,
    pub m_offset: Vector2f,
    pub m_used_by_composite: Option<bool>,
    pub m_layer_override_priority: Option<i32>,
    pub m_size: Vector2f,
    pub m_force_send_layers: Option<BitField>,
    pub m_direction: i32,
    pub m_force_receive_layers: Option<BitField>,
    pub m_contact_capture_layers: Option<BitField>,
    pub m_callback_layers: Option<BitField>,
    pub m_composite_operation: Option<i32>,
    pub m_composite_order: Option<i32>,
}

impl crate::Parse for CapsuleCollider2D {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut m_enabled = None;
        let mut m_density = None;
        let mut m_material = None;
        let mut m_is_trigger = None;
        let mut m_include_layers = None;
        let mut m_used_by_effector = None;
        let mut m_exclude_layers = None;
        let mut m_offset = None;
        let mut m_used_by_composite = None;
        let mut m_layer_override_priority = None;
        let mut m_size = None;
        let mut m_force_send_layers = None;
        let mut m_direction = None;
        let mut m_force_receive_layers = None;
        let mut m_contact_capture_layers = None;
        let mut m_callback_layers = None;
        let mut m_composite_operation = None;
        let mut m_composite_order = None;
        m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
        m_enabled = Some(rdr.read_u8()?);
        rdr.align(4)?;
        m_density = Some(rdr.read_f32()?);
        m_material = Some(crate::PPtr::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            m_include_layers = Some(BitField::parse(rdr, ver)?);
            m_exclude_layers = Some(BitField::parse(rdr, ver)?);
            m_layer_override_priority = Some(rdr.read_i32()?);
            m_force_send_layers = Some(BitField::parse(rdr, ver)?);
            m_force_receive_layers = Some(BitField::parse(rdr, ver)?);
            m_contact_capture_layers = Some(BitField::parse(rdr, ver)?);
            m_callback_layers = Some(BitField::parse(rdr, ver)?);
        } else {
        }
        m_is_trigger = Some(rdr.read_bool()?);
        m_used_by_effector = Some(CapsuleCollider2DMUsedByEffectorVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            m_composite_operation = Some(rdr.read_i32()?);
            m_composite_order = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_used_by_composite = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
        }
        m_offset = Some(Vector2f::parse(rdr, ver)?);
        m_size = Some(Vector2f::parse(rdr, ver)?);
        m_direction = Some(rdr.read_i32()?);

        Ok(CapsuleCollider2D {
            m_game_object: m_game_object.unwrap(),
            m_enabled: m_enabled.unwrap(),
            m_density: m_density.unwrap(),
            m_material: m_material.unwrap(),
            m_is_trigger: m_is_trigger.unwrap(),
            m_include_layers,
            m_used_by_effector: m_used_by_effector.unwrap(),
            m_exclude_layers,
            m_offset: m_offset.unwrap(),
            m_used_by_composite,
            m_layer_override_priority,
            m_size: m_size.unwrap(),
            m_force_send_layers,
            m_direction: m_direction.unwrap(),
            m_force_receive_layers,
            m_contact_capture_layers,
            m_callback_layers,
            m_composite_operation,
            m_composite_order,
        })
    }
}
