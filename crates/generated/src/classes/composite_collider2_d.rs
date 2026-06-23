//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::bit_field::BitField;
use crate::subclasses::polygon2_d::Polygon2D;
use crate::subclasses::sub_collider::SubCollider;
use crate::subclasses::vector2f::Vector2f;
use super::collider2_d::Collider2D;

#[derive(Debug, Clone)]
pub struct CompositeCollider2D {
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
    pub m_geometry_type: i32,
    pub m_force_receive_layers: Option<BitField>,
    pub m_generation_type: i32,
    pub m_contact_capture_layers: Option<BitField>,
    pub m_collider_paths: Vec<SubCollider>,
    pub m_edge_radius: Option<f32>,
    pub m_callback_layers: Option<BitField>,
    pub m_composite_paths: Polygon2D,
    pub m_vertex_distance: f32,
    pub m_composite_operation: Option<i32>,
    pub m_offset_distance: Option<f32>,
    pub m_composite_order: Option<i32>,
    pub m_use_delaunay_mesh: Option<bool>,
    pub m_composite_game_object: Option<PPtr>,
}

impl crate::Parse for CompositeCollider2D {
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
        let m_offset = Vector2f::parse(r, ver)?;
        let m_force_send_layers = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(BitField::parse(r, ver)?)
        } else {
            None
        };
        let m_geometry_type = r.read_i32()?;
        let m_force_receive_layers = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(BitField::parse(r, ver)?)
        } else {
            None
        };
        let m_generation_type = r.read_i32()?;
        let m_contact_capture_layers = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(BitField::parse(r, ver)?)
        } else {
            None
        };
        let m_collider_paths = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { SubCollider::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_edge_radius = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_callback_layers = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(BitField::parse(r, ver)?)
        } else {
            None
        };
        let m_composite_paths = Polygon2D::parse(r, ver)?;
        let m_vertex_distance = r.read_f32()?;
        let m_composite_operation = if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_offset_distance = if *ver >= crate::UnityVersion::new(2019, 1, 3) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_composite_order = if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_use_delaunay_mesh = if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_composite_game_object = if *ver >= crate::UnityVersion::new(2022, 1, 18) {
            Some(crate::PPtr::parse(r, ver)?)
        } else {
            None
        };

        Ok(CompositeCollider2D {
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
            m_geometry_type,
            m_force_receive_layers,
            m_generation_type,
            m_contact_capture_layers,
            m_collider_paths,
            m_edge_radius,
            m_callback_layers,
            m_composite_paths,
            m_vertex_distance,
            m_composite_operation,
            m_offset_distance,
            m_composite_order,
            m_use_delaunay_mesh,
            m_composite_game_object,
        })
    }
}
