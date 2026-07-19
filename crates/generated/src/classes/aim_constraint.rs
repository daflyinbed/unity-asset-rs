//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::constraint_source::ConstraintSource;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct AimConstraint {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_weight: f32,
    pub m_rotation_at_rest: Vector3f,
    pub m_rotation_offset: Vector3f,
    pub m_aim_vector: Vector3f,
    pub m_up_vector: Vector3f,
    pub m_world_up_vector: Vector3f,
    pub m_world_up_object: PPtr,
    pub m_up_type: i32,
    pub m_affect_rotation_x: bool,
    pub m_affect_rotation_y: bool,
    pub m_affect_rotation_z: bool,
    pub m_is_contraint_active: Option<bool>,
    pub m_active: Option<bool>,
    pub m_sources: Vec<ConstraintSource>,
}

impl crate::Parse for AimConstraint {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut m_enabled = None;
        let mut m_weight = None;
        let mut m_rotation_at_rest = None;
        let mut m_rotation_offset = None;
        let mut m_aim_vector = None;
        let mut m_up_vector = None;
        let mut m_world_up_vector = None;
        let mut m_world_up_object = None;
        let mut m_up_type = None;
        let mut m_affect_rotation_x = None;
        let mut m_affect_rotation_y = None;
        let mut m_affect_rotation_z = None;
        let mut m_is_contraint_active = None;
        let mut m_active = None;
        let mut m_sources = None;
        m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
        m_enabled = Some(rdr.read_u8()?);
        rdr.align(4)?;
        m_weight = Some(rdr.read_f32()?);
        m_rotation_at_rest = Some(Vector3f::parse(rdr, ver)?);
        m_rotation_offset = Some(Vector3f::parse(rdr, ver)?);
        m_aim_vector = Some(Vector3f::parse(rdr, ver)?);
        m_up_vector = Some(Vector3f::parse(rdr, ver)?);
        m_world_up_vector = Some(Vector3f::parse(rdr, ver)?);
        m_world_up_object = Some(crate::PPtr::parse(rdr, ver)?);
        m_up_type = Some(rdr.read_i32()?);
        m_affect_rotation_x = Some(rdr.read_bool()?);
        m_affect_rotation_y = Some(rdr.read_bool()?);
        m_affect_rotation_z = Some(rdr.read_bool()?);
        if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            m_active = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
            m_is_contraint_active = Some(rdr.read_bool()?);
            rdr.align(4)?;
        }
        m_sources = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| ConstraintSource::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        rdr.align(4)?;

        Ok(AimConstraint {
            m_game_object: m_game_object.unwrap(),
            m_enabled: m_enabled.unwrap(),
            m_weight: m_weight.unwrap(),
            m_rotation_at_rest: m_rotation_at_rest.unwrap(),
            m_rotation_offset: m_rotation_offset.unwrap(),
            m_aim_vector: m_aim_vector.unwrap(),
            m_up_vector: m_up_vector.unwrap(),
            m_world_up_vector: m_world_up_vector.unwrap(),
            m_world_up_object: m_world_up_object.unwrap(),
            m_up_type: m_up_type.unwrap(),
            m_affect_rotation_x: m_affect_rotation_x.unwrap(),
            m_affect_rotation_y: m_affect_rotation_y.unwrap(),
            m_affect_rotation_z: m_affect_rotation_z.unwrap(),
            m_is_contraint_active,
            m_active,
            m_sources: m_sources.unwrap(),
        })
    }
}
