//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::bit_field::BitField;
use crate::subclasses::vector2f::Vector2f;

#[derive(Debug, Clone)]
pub struct EdgeCollider2D {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_material: PPtr,
    pub m_density: Option<f32>,
    pub m_is_trigger: bool,
    pub m_points: Vec<Vector2f>,
    pub m_used_by_effector: Option<bool>,
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
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_u8()?;
        r.align(4)?;
        let m_material = crate::PPtr::parse(r, ver)?;
        let m_density = if *ver >= crate::UnityVersion::new(5, 3, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_is_trigger = r.read_bool()?;
        let m_points = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Vector2f::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_used_by_effector = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_include_layers = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(BitField::parse(r, ver)?)
        } else {
            None
        };
        let m_offset = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(Vector2f::parse(r, ver)?)
        } else {
            None
        };
        let m_exclude_layers = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(BitField::parse(r, ver)?)
        } else {
            None
        };
        let m_used_by_composite = if *ver >= crate::UnityVersion::new(5, 6, 0) {
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
        let m_force_send_layers = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(BitField::parse(r, ver)?)
        } else {
            None
        };
        let m_edge_radius = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
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
        let m_adjacent_start_point = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(Vector2f::parse(r, ver)?)
        } else {
            None
        };
        let m_callback_layers = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(BitField::parse(r, ver)?)
        } else {
            None
        };
        let m_adjacent_end_point = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(Vector2f::parse(r, ver)?)
        } else {
            None
        };
        let m_use_adjacent_start_point = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_use_adjacent_end_point = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
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

        Ok(EdgeCollider2D {
            m_game_object,
            m_enabled,
            m_material,
            m_density,
            m_is_trigger,
            m_points,
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
