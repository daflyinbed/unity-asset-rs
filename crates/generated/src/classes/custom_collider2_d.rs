//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::bit_field::BitField;
use crate::subclasses::physics_shape_group2_d::PhysicsShapeGroup2D;
use crate::subclasses::vector2f::Vector2f;
use super::collider2_d::Collider2D;

#[derive(Debug, Clone)]
pub struct CustomCollider2D {
    pub parent: Collider2D,
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_density: f32,
    pub m_material: PPtr,
    pub m_is_trigger: bool,
    pub m_include_layers: Option<BitField>,
    pub m_used_by_effector: bool,
    pub m_exclude_layers: Option<BitField>,
    pub m_used_by_composite: Option<bool>,
    pub m_layer_override_priority: Option<i32>,
    pub m_offset: Vector2f,
    pub m_force_send_layers: Option<BitField>,
    pub m_custom_shapes: PhysicsShapeGroup2D,
    pub m_force_receive_layers: Option<BitField>,
    pub m_contact_capture_layers: Option<BitField>,
    pub m_callback_layers: Option<BitField>,
    pub m_composite_operation: Option<i32>,
    pub m_composite_order: Option<i32>,
}

impl crate::Parse for CustomCollider2D {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = Collider2D::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_u8()?;
        r.align(4)?;
        let m_density = r.read_f32()?;
        let m_material = crate::PPtr::parse(r, ver)?;
        let m_is_trigger = r.read_bool()?;
        let m_include_layers = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(BitField::parse(r, ver)?)
        } else {
            None
        };
        let m_used_by_effector = r.read_bool()?;
        let m_exclude_layers = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(BitField::parse(r, ver)?)
        } else {
            None
        };
        let m_used_by_composite = if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_layer_override_priority = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_offset = Vector2f::parse(r, ver)?;
        let m_force_send_layers = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(BitField::parse(r, ver)?)
        } else {
            None
        };
        let m_custom_shapes = PhysicsShapeGroup2D::parse(r, ver)?;
        r.align(4)?;
        let m_force_receive_layers = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(BitField::parse(r, ver)?)
        } else {
            None
        };
        let m_contact_capture_layers = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(BitField::parse(r, ver)?)
        } else {
            None
        };
        let m_callback_layers = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(BitField::parse(r, ver)?)
        } else {
            None
        };
        let m_composite_operation = if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_composite_order = if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };

        Ok(CustomCollider2D {
            parent,
            m_game_object,
            m_enabled,
            m_density,
            m_material,
            m_is_trigger,
            m_include_layers,
            m_used_by_effector,
            m_exclude_layers,
            m_used_by_composite,
            m_layer_override_priority,
            m_offset,
            m_force_send_layers,
            m_custom_shapes,
            m_force_receive_layers,
            m_contact_capture_layers,
            m_callback_layers,
            m_composite_operation,
            m_composite_order,
        })
    }
}
