//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct MeshParticleEmitter {
    pub m_game_object: PPtr,
    pub m_enabled: bool,
    pub m_emit: bool,
    pub min_size: f32,
    pub max_size: f32,
    pub min_energy: f32,
    pub max_energy: f32,
    pub min_emission: f32,
    pub max_emission: f32,
    pub world_velocity: Vector3f,
    pub local_velocity: Vector3f,
    pub rnd_velocity: Vector3f,
    pub emitter_velocity_scale: f32,
    pub tangent_velocity: Vector3f,
    pub angular_velocity: f32,
    pub rnd_angular_velocity: f32,
    pub rnd_rotation: bool,
    pub simulate_in_worldspace: bool,
    pub m_one_shot: bool,
    pub m_interpolate_triangles: bool,
    pub m_systematic: bool,
    pub m_min_normal_velocity: f32,
    pub m_max_normal_velocity: f32,
    pub m_mesh: PPtr,
}

impl crate::Parse for MeshParticleEmitter {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(rdr, ver)?;
        let m_enabled = rdr.read_bool()?;
        let m_emit = rdr.read_bool()?;
        rdr.align(4)?;
        let min_size = rdr.read_f32()?;
        let max_size = rdr.read_f32()?;
        let min_energy = rdr.read_f32()?;
        let max_energy = rdr.read_f32()?;
        let min_emission = rdr.read_f32()?;
        let max_emission = rdr.read_f32()?;
        let world_velocity = Vector3f::parse(rdr, ver)?;
        let local_velocity = Vector3f::parse(rdr, ver)?;
        let rnd_velocity = Vector3f::parse(rdr, ver)?;
        let emitter_velocity_scale = rdr.read_f32()?;
        let tangent_velocity = Vector3f::parse(rdr, ver)?;
        let angular_velocity = rdr.read_f32()?;
        let rnd_angular_velocity = rdr.read_f32()?;
        let rnd_rotation = rdr.read_bool()?;
        let simulate_in_worldspace = rdr.read_bool()?;
        let m_one_shot = rdr.read_bool()?;
        let m_interpolate_triangles = rdr.read_bool()?;
        let m_systematic = rdr.read_bool()?;
        rdr.align(4)?;
        let m_min_normal_velocity = rdr.read_f32()?;
        let m_max_normal_velocity = rdr.read_f32()?;
        let m_mesh = crate::PPtr::parse(rdr, ver)?;

        Ok(MeshParticleEmitter {
            m_game_object,
            m_enabled,
            m_emit,
            min_size,
            max_size,
            min_energy,
            max_energy,
            min_emission,
            max_emission,
            world_velocity,
            local_velocity,
            rnd_velocity,
            emitter_velocity_scale,
            tangent_velocity,
            angular_velocity,
            rnd_angular_velocity,
            rnd_rotation,
            simulate_in_worldspace,
            m_one_shot,
            m_interpolate_triangles,
            m_systematic,
            m_min_normal_velocity,
            m_max_normal_velocity,
            m_mesh,
        })
    }
}
