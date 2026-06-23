//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::physics_shape_group2_d::PhysicsShapeGroup2D;
use crate::subclasses::vector2f::Vector2f;

#[derive(Debug, Clone)]
pub struct CustomCollider2D {
    pub m_object_hide_flags: u32,
    pub m_corresponding_source_object: PPtr,
    pub m_prefab_instance: PPtr,
    pub m_prefab_asset: PPtr,
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_density: f32,
    pub m_material: PPtr,
    pub m_is_trigger: bool,
    pub m_used_by_effector: bool,
    pub m_used_by_composite: bool,
    pub m_offset: Vector2f,
    pub m_custom_shapes: PhysicsShapeGroup2D,
}

impl crate::Parse for CustomCollider2D {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_corresponding_source_object = crate::PPtr::parse(r, ver)?;
        let m_prefab_instance = crate::PPtr::parse(r, ver)?;
        let m_prefab_asset = crate::PPtr::parse(r, ver)?;
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
        let m_custom_shapes = PhysicsShapeGroup2D::parse(r, ver)?;
        r.align(4)?;

        Ok(CustomCollider2D {
            m_object_hide_flags,
            m_corresponding_source_object,
            m_prefab_instance,
            m_prefab_asset,
            m_game_object,
            m_enabled,
            m_density,
            m_material,
            m_is_trigger,
            m_used_by_effector,
            m_used_by_composite,
            m_offset,
            m_custom_shapes,
        })
    }
}
