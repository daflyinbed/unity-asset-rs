//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::cloth_constrain_coefficients::ClothConstrainCoefficients;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct SkinnedCloth {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_bending_stiffness: f32,
    pub m_stretching_stiffness: f32,
    pub m_damping: f32,
    pub m_thickness: f32,
    pub m_use_gravity: bool,
    pub m_self_collision: bool,
    pub m_external_acceleration: Vector3f,
    pub m_random_acceleration: Vector3f,
    pub m_world_velocity_scale: f32,
    pub m_world_acceleration_scale: f32,
    pub m_coefficients: Vec<ClothConstrainCoefficients>,
}

impl crate::Parse for SkinnedCloth {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(rdr, ver)?;
        let m_enabled = rdr.read_u8()?;
        rdr.align(4)?;
        let m_bending_stiffness = rdr.read_f32()?;
        let m_stretching_stiffness = rdr.read_f32()?;
        let m_damping = rdr.read_f32()?;
        let m_thickness = rdr.read_f32()?;
        let m_use_gravity = rdr.read_bool()?;
        let m_self_collision = rdr.read_bool()?;
        rdr.align(4)?;
        let m_external_acceleration = Vector3f::parse(rdr, ver)?;
        let m_random_acceleration = Vector3f::parse(rdr, ver)?;
        let m_world_velocity_scale = rdr.read_f32()?;
        let m_world_acceleration_scale = rdr.read_f32()?;
        let m_coefficients = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| ClothConstrainCoefficients::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;

        Ok(SkinnedCloth {
            m_game_object,
            m_enabled,
            m_bending_stiffness,
            m_stretching_stiffness,
            m_damping,
            m_thickness,
            m_use_gravity,
            m_self_collision,
            m_external_acceleration,
            m_random_acceleration,
            m_world_velocity_scale,
            m_world_acceleration_scale,
            m_coefficients,
        })
    }
}
