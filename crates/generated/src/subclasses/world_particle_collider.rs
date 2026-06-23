//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::bit_field::BitField;

#[derive(Debug, Clone)]
pub struct WorldParticleCollider {
    pub m_object_hide_flags: u32,
    pub m_extension_ptr: PPtr,
    pub m_game_object: PPtr,
    pub m_bounce_factor: f32,
    pub m_collision_energy_loss: f32,
    pub m_collides_with: BitField,
    pub m_send_collision_message: bool,
    pub m_min_kill_velocity: f32,
}

impl crate::Parse for WorldParticleCollider {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_extension_ptr = crate::PPtr::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_bounce_factor = r.read_f32()?;
        let m_collision_energy_loss = r.read_f32()?;
        let m_collides_with = BitField::parse(r, ver)?;
        let m_send_collision_message = r.read_bool()?;
        r.align(4)?;
        let m_min_kill_velocity = r.read_f32()?;

        Ok(WorldParticleCollider {
            m_object_hide_flags,
            m_extension_ptr,
            m_game_object,
            m_bounce_factor,
            m_collision_energy_loss,
            m_collides_with,
            m_send_collision_message,
            m_min_kill_velocity,
        })
    }
}
