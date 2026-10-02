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

impl crate::ToValue for WorldParticleCollider {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_GameObject".to_string(), self.m_game_object.to_value())),
            Some((
                "m_BounceFactor".to_string(),
                self.m_bounce_factor.to_value(),
            )),
            Some((
                "m_CollisionEnergyLoss".to_string(),
                self.m_collision_energy_loss.to_value(),
            )),
            Some((
                "m_CollidesWith".to_string(),
                self.m_collides_with.to_value(),
            )),
            Some((
                "m_SendCollisionMessage".to_string(),
                self.m_send_collision_message.to_value(),
            )),
            Some((
                "m_MinKillVelocity".to_string(),
                self.m_min_kill_velocity.to_value(),
            )),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_GameObject" => (self.m_game_object).inspect_value(path, offset, limit),
                "m_BounceFactor" => (self.m_bounce_factor).inspect_value(path, offset, limit),
                "m_CollisionEnergyLoss" => {
                    (self.m_collision_energy_loss).inspect_value(path, offset, limit)
                }
                "m_CollidesWith" => (self.m_collides_with).inspect_value(path, offset, limit),
                "m_SendCollisionMessage" => {
                    (self.m_send_collision_message).inspect_value(path, offset, limit)
                }
                "m_MinKillVelocity" => {
                    (self.m_min_kill_velocity).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_game_object).inspect_value(path, offset, limit))
                .map(|v| ("m_GameObject".to_string(), v)),
            ((self.m_bounce_factor).inspect_value(path, offset, limit))
                .map(|v| ("m_BounceFactor".to_string(), v)),
            ((self.m_collision_energy_loss).inspect_value(path, offset, limit))
                .map(|v| ("m_CollisionEnergyLoss".to_string(), v)),
            ((self.m_collides_with).inspect_value(path, offset, limit))
                .map(|v| ("m_CollidesWith".to_string(), v)),
            ((self.m_send_collision_message).inspect_value(path, offset, limit))
                .map(|v| ("m_SendCollisionMessage".to_string(), v)),
            ((self.m_min_kill_velocity).inspect_value(path, offset, limit))
                .map(|v| ("m_MinKillVelocity".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
