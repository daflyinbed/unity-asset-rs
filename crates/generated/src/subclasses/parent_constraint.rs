//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::constraint_source::ConstraintSource;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct ParentConstraint {
    pub m_object_hide_flags: u32,
    pub m_prefab_parent_object: PPtr,
    pub m_prefab_internal: PPtr,
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_weight: f32,
    pub m_translation_at_rest: Vector3f,
    pub m_rotation_at_rest: Vector3f,
    pub m_translation_offsets: Vec<Vector3f>,
    pub m_rotation_offsets: Vec<Vector3f>,
    pub m_affect_translation_x: bool,
    pub m_affect_translation_y: bool,
    pub m_affect_translation_z: bool,
    pub m_affect_rotation_x: bool,
    pub m_affect_rotation_y: bool,
    pub m_affect_rotation_z: bool,
    pub m_is_contraint_active: bool,
    pub m_is_locked: bool,
    pub m_sources: Vec<ConstraintSource>,
}

impl crate::Parse for ParentConstraint {
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
        let m_weight = r.read_f32()?;
        let m_translation_at_rest = Vector3f::parse(r, ver)?;
        let m_rotation_at_rest = Vector3f::parse(r, ver)?;
        let m_translation_offsets = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Vector3f::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_rotation_offsets = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Vector3f::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_affect_translation_x = r.read_bool()?;
        let m_affect_translation_y = r.read_bool()?;
        let m_affect_translation_z = r.read_bool()?;
        let m_affect_rotation_x = r.read_bool()?;
        let m_affect_rotation_y = r.read_bool()?;
        let m_affect_rotation_z = r.read_bool()?;
        let m_is_contraint_active = r.read_bool()?;
        let m_is_locked = r.read_bool()?;
        r.align(4)?;
        let m_sources = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ConstraintSource::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(ParentConstraint {
            m_object_hide_flags,
            m_prefab_parent_object,
            m_prefab_internal,
            m_game_object,
            m_enabled,
            m_weight,
            m_translation_at_rest,
            m_rotation_at_rest,
            m_translation_offsets,
            m_rotation_offsets,
            m_affect_translation_x,
            m_affect_translation_y,
            m_affect_translation_z,
            m_affect_rotation_x,
            m_affect_rotation_y,
            m_affect_rotation_z,
            m_is_contraint_active,
            m_is_locked,
            m_sources,
        })
    }
}
