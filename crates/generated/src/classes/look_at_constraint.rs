//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::constraint_source::ConstraintSource;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct LookAtConstraint {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_weight: f32,
    pub m_rotation_at_rest: Vector3f,
    pub m_rotation_offset: Vector3f,
    pub m_roll: f32,
    pub m_world_up_object: PPtr,
    pub m_use_up_object: bool,
    pub m_is_contraint_active: Option<bool>,
    pub m_active: Option<bool>,
    pub m_sources: Vec<ConstraintSource>,
}

impl crate::Parse for LookAtConstraint {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut m_enabled = None;
        let mut m_weight = None;
        let mut m_rotation_at_rest = None;
        let mut m_rotation_offset = None;
        let mut m_roll = None;
        let mut m_world_up_object = None;
        let mut m_use_up_object = None;
        let mut m_is_contraint_active = None;
        let mut m_active = None;
        let mut m_sources = None;
            m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
            m_enabled = Some(rdr.read_u8()?);
            rdr.align(4)?;
            m_weight = Some(rdr.read_f32()?);
            m_rotation_at_rest = Some(Vector3f::parse(rdr, ver)?);
            m_rotation_offset = Some(Vector3f::parse(rdr, ver)?);
            m_roll = Some(rdr.read_f32()?);
            m_world_up_object = Some(crate::PPtr::parse(rdr, ver)?);
            m_use_up_object = Some(rdr.read_bool()?);
            rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            m_active = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
            m_is_contraint_active = Some(rdr.read_bool()?);
            rdr.align(4)?;
        }
            m_sources = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ConstraintSource::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;

        Ok(LookAtConstraint {
            m_game_object: m_game_object.unwrap(),
            m_enabled: m_enabled.unwrap(),
            m_weight: m_weight.unwrap(),
            m_rotation_at_rest: m_rotation_at_rest.unwrap(),
            m_rotation_offset: m_rotation_offset.unwrap(),
            m_roll: m_roll.unwrap(),
            m_world_up_object: m_world_up_object.unwrap(),
            m_use_up_object: m_use_up_object.unwrap(),
            m_is_contraint_active,
            m_active,
            m_sources: m_sources.unwrap(),
        })
    }
}
