//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::classes::polygon2_d::Polygon2D;
use crate::subclasses::bit_field::BitField;
use crate::subclasses::sprite_tiling_property::SpriteTilingProperty;
use crate::subclasses::vector2f::Vector2f;

#[derive(Debug, Clone)]
pub struct PolygonCollider2D {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_material: PPtr,
    pub m_density: Option<f32>,
    pub m_is_trigger: bool,
    pub m_poly: Option<Polygon2D>,
    pub m_used_by_effector: Option<bool>,
    pub m_include_layers: Option<BitField>,
    pub m_offset: Option<Vector2f>,
    pub m_exclude_layers: Option<BitField>,
    pub m_points: Option<Polygon2D>,
    pub m_used_by_composite: Option<bool>,
    pub m_layer_override_priority: Option<i32>,
    pub m_force_send_layers: Option<BitField>,
    pub m_auto_tiling: Option<bool>,
    pub m_sprite_tiling_property: Option<SpriteTilingProperty>,
    pub m_force_receive_layers: Option<BitField>,
    pub m_contact_capture_layers: Option<BitField>,
    pub m_callback_layers: Option<BitField>,
    pub m_use_delaunay_mesh: Option<bool>,
    pub m_composite_operation: Option<i32>,
    pub m_composite_order: Option<i32>,
}

impl crate::Parse for PolygonCollider2D {
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
        let m_poly = if *ver >= crate::UnityVersion::new(4, 3, 0) {
            Some(Polygon2D::parse(r, ver)?)
        } else {
            None
        };
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
        let m_points = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = Polygon2D::parse(r, ver)?;
            r.align(4)?;
            Some(v)
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
        let m_auto_tiling = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_sprite_tiling_property = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = SpriteTilingProperty::parse(r, ver)?;
            r.align(4)?;
            Some(v)
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
        let m_callback_layers = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(BitField::parse(r, ver)?)
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

        Ok(PolygonCollider2D {
            m_game_object,
            m_enabled,
            m_material,
            m_density,
            m_is_trigger,
            m_poly,
            m_used_by_effector,
            m_include_layers,
            m_offset,
            m_exclude_layers,
            m_points,
            m_used_by_composite,
            m_layer_override_priority,
            m_force_send_layers,
            m_auto_tiling,
            m_sprite_tiling_property,
            m_force_receive_layers,
            m_contact_capture_layers,
            m_callback_layers,
            m_use_delaunay_mesh,
            m_composite_operation,
            m_composite_order,
        })
    }
}
