//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::bit_field::BitField;
use crate::subclasses::polygon2_d::Polygon2D;
use crate::subclasses::sub_collider::SubCollider;
use crate::subclasses::vector2f::Vector2f;


/// 版本枚举：CompositeCollider2DMUsedByEffectorVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum CompositeCollider2DMUsedByEffectorVariant {
    Default(bool),
    V2023_1(bool),
}

impl crate::Parse for CompositeCollider2DMUsedByEffectorVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Ok(Self::V2023_1(v))
        } else {
            let v = r.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：CompositeCollider2DMColliderPathsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum CompositeCollider2DMColliderPathsVariant {
    Default(Vec<SubCollider>),
    V2017_1(Vec<SubCollider>),
}

impl crate::Parse for CompositeCollider2DMColliderPathsVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { SubCollider::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { SubCollider::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：CompositeCollider2DMCompositePathsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum CompositeCollider2DMCompositePathsVariant {
    Default(Polygon2D),
    V2017_1(Polygon2D),
}

impl crate::Parse for CompositeCollider2DMCompositePathsVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = Polygon2D::parse(r, ver)?;
            r.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = Polygon2D::parse(r, ver)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct CompositeCollider2D {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_density: f32,
    pub m_material: PPtr,
    pub m_is_trigger: bool,
    pub m_include_layers: Option<BitField>,
    pub m_used_by_effector: CompositeCollider2DMUsedByEffectorVariant,
    pub m_exclude_layers: Option<BitField>,
    pub m_used_by_composite: Option<bool>,
    pub m_layer_override_priority: Option<i32>,
    pub m_offset: Vector2f,
    pub m_force_send_layers: Option<BitField>,
    pub m_geometry_type: i32,
    pub m_force_receive_layers: Option<BitField>,
    pub m_generation_type: i32,
    pub m_contact_capture_layers: Option<BitField>,
    pub m_edge_radius: f32,
    pub m_callback_layers: Option<BitField>,
    pub m_collider_paths: CompositeCollider2DMColliderPathsVariant,
    pub m_composite_paths: CompositeCollider2DMCompositePathsVariant,
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
        let mut m_game_object = None;
        let mut m_enabled = None;
        let mut m_density = None;
        let mut m_material = None;
        let mut m_is_trigger = None;
        let mut m_include_layers = None;
        let mut m_used_by_effector = None;
        let mut m_exclude_layers = None;
        let mut m_used_by_composite = None;
        let mut m_layer_override_priority = None;
        let mut m_offset = None;
        let mut m_force_send_layers = None;
        let mut m_geometry_type = None;
        let mut m_force_receive_layers = None;
        let mut m_generation_type = None;
        let mut m_contact_capture_layers = None;
        let mut m_edge_radius = None;
        let mut m_callback_layers = None;
        let mut m_collider_paths = None;
        let mut m_composite_paths = None;
        let mut m_vertex_distance = None;
        let mut m_composite_operation = None;
        let mut m_offset_distance = None;
        let mut m_composite_order = None;
        let mut m_use_delaunay_mesh = None;
        let mut m_composite_game_object = None;
            m_game_object = Some(crate::PPtr::parse(r, ver)?);
            m_enabled = Some(r.read_u8()?);
            r.align(4)?;
            m_density = Some(r.read_f32()?);
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
            m_is_trigger = Some(r.read_bool()?);
            m_used_by_effector = Some(CompositeCollider2DMUsedByEffectorVariant::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            m_composite_operation = Some(r.read_i32()?);
            m_composite_order = Some(r.read_i32()?);
        } else {
            m_used_by_composite = Some(r.read_bool()?);
            r.align(4)?;
        }
            m_offset = Some(Vector2f::parse(r, ver)?);
            m_geometry_type = Some(r.read_i32()?);
            m_generation_type = Some(r.read_i32()?);
            m_edge_radius = Some(r.read_f32()?);
            m_collider_paths = Some(CompositeCollider2DMColliderPathsVariant::parse(r, ver)?);
            m_composite_paths = Some(CompositeCollider2DMCompositePathsVariant::parse(r, ver)?);
            m_vertex_distance = Some(r.read_f32()?);
        if *ver >= crate::UnityVersion::new(2022, 1, 18) {
            m_offset_distance = Some(r.read_f32()?);
            m_use_delaunay_mesh = Some(r.read_bool()?);
            r.align(4)?;
            m_composite_game_object = Some(crate::PPtr::parse(r, ver)?);
        } else if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            m_offset_distance = Some(r.read_f32()?);
            m_use_delaunay_mesh = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 1, 3) {
            m_offset_distance = Some(r.read_f32()?);
        } else {
        }

        Ok(CompositeCollider2D {
            m_game_object: m_game_object.unwrap(),
            m_enabled: m_enabled.unwrap(),
            m_density: m_density.unwrap(),
            m_material: m_material.unwrap(),
            m_is_trigger: m_is_trigger.unwrap(),
            m_include_layers,
            m_used_by_effector: m_used_by_effector.unwrap(),
            m_exclude_layers,
            m_used_by_composite,
            m_layer_override_priority,
            m_offset: m_offset.unwrap(),
            m_force_send_layers,
            m_geometry_type: m_geometry_type.unwrap(),
            m_force_receive_layers,
            m_generation_type: m_generation_type.unwrap(),
            m_contact_capture_layers,
            m_edge_radius: m_edge_radius.unwrap(),
            m_callback_layers,
            m_collider_paths: m_collider_paths.unwrap(),
            m_composite_paths: m_composite_paths.unwrap(),
            m_vertex_distance: m_vertex_distance.unwrap(),
            m_composite_operation,
            m_offset_distance,
            m_composite_order,
            m_use_delaunay_mesh,
            m_composite_game_object,
        })
    }
}
