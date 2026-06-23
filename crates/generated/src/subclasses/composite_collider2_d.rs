//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::polygon2_d::Polygon2D;
use crate::subclasses::sub_collider::SubCollider;
use crate::subclasses::vector2f::Vector2f;

#[derive(Debug, Clone)]
pub struct CompositeCollider2D {
    pub m_object_hide_flags: u32,
    pub m_prefab_parent_object: PPtr,
    pub m_prefab_internal: PPtr,
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_density: f32,
    pub m_material: PPtr,
    pub m_is_trigger: bool,
    pub m_used_by_effector: bool,
    pub m_used_by_composite: bool,
    pub m_offset: Vector2f,
    pub m_geometry_type: i32,
    pub m_generation_type: i32,
    pub m_collider_paths: Vec<SubCollider>,
    pub m_composite_paths: Polygon2D,
    pub m_vertex_distance: f32,
}

impl crate::Parse for CompositeCollider2D {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_prefab_parent_object = crate::PPtr::parse(r, ver)?;
        let m_prefab_internal = crate::PPtr::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_u8()?;
        r.align(4)?;
        let m_density = r.read_f32()?;
        let m_material = crate::PPtr::parse(r, ver)?;
        let m_is_trigger = r.read_bool()?;
        let m_used_by_effector = r.read_bool()?;
        let m_used_by_composite = r.read_bool()?;
        r.align(4)?;
        let m_offset = Vector2f::parse(r, ver)?;
        let m_geometry_type = r.read_i32()?;
        let m_generation_type = r.read_i32()?;
        let m_collider_paths = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { SubCollider::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_composite_paths = Polygon2D::parse(r, ver)?;
        let m_vertex_distance = r.read_f32()?;

        Ok(CompositeCollider2D {
            m_object_hide_flags,
            m_prefab_parent_object,
            m_prefab_internal,
            m_game_object,
            m_enabled,
            m_density,
            m_material,
            m_is_trigger,
            m_used_by_effector,
            m_used_by_composite,
            m_offset,
            m_geometry_type,
            m_generation_type,
            m_collider_paths,
            m_composite_paths,
            m_vertex_distance,
        })
    }
}
