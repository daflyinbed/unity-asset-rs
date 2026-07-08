//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::bit_field::BitField;

#[derive(Debug, Clone)]
pub struct WorldParticleCollider {
    pub m_game_object: PPtr,
    pub m_bounce_factor: f32,
    pub m_collision_energy_loss: f32,
    pub m_collides_with: BitField,
    pub m_send_collision_message: bool,
    pub m_min_kill_velocity: f32,
}

impl crate::Parse for WorldParticleCollider {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(rdr, ver)?;
        let m_bounce_factor = rdr.read_f32()?;
        let m_collision_energy_loss = rdr.read_f32()?;
        let m_collides_with = BitField::parse(rdr, ver)?;
        let m_send_collision_message = rdr.read_bool()?;
        rdr.align(4)?;
        let m_min_kill_velocity = rdr.read_f32()?;

        Ok(WorldParticleCollider {
            m_game_object,
            m_bounce_factor,
            m_collision_energy_loss,
            m_collides_with,
            m_send_collision_message,
            m_min_kill_velocity,
        })
    }
}
