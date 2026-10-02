//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::bit_field::BitField;

#[derive(Debug, Clone)]
pub struct SurfaceEffector2D {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_collider_mask: BitField,
    pub m_use_collider_mask: Option<bool>,
    pub m_speed: f32,
    pub m_speed_variation: f32,
    pub m_force_scale: Option<f32>,
    pub m_use_contact_force: Option<bool>,
    pub m_use_friction: Option<bool>,
    pub m_use_bounce: Option<bool>,
}

impl crate::Parse for SurfaceEffector2D {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut m_enabled = None;
        let mut m_collider_mask = None;
        let mut m_use_collider_mask = None;
        let mut m_speed = None;
        let mut m_speed_variation = None;
        let mut m_force_scale = None;
        let mut m_use_contact_force = None;
        let mut m_use_friction = None;
        let mut m_use_bounce = None;
        m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
        m_enabled = Some(rdr.read_u8()?);
        rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(5, 0, 2) {
            m_use_collider_mask = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
        }
        m_collider_mask = Some(BitField::parse(rdr, ver)?);
        m_speed = Some(rdr.read_f32()?);
        m_speed_variation = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(5, 0, 1) {
            m_force_scale = Some(rdr.read_f32()?);
            m_use_contact_force = Some(rdr.read_bool()?);
            m_use_friction = Some(rdr.read_bool()?);
            m_use_bounce = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
        }

        Ok(SurfaceEffector2D {
            m_game_object: m_game_object.unwrap(),
            m_enabled: m_enabled.unwrap(),
            m_collider_mask: m_collider_mask.unwrap(),
            m_use_collider_mask,
            m_speed: m_speed.unwrap(),
            m_speed_variation: m_speed_variation.unwrap(),
            m_force_scale,
            m_use_contact_force,
            m_use_friction,
            m_use_bounce,
        })
    }
}

impl crate::ToValue for SurfaceEffector2D {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_GameObject".to_string(), self.m_game_object.to_value())),
            Some(("m_Enabled".to_string(), self.m_enabled.to_value())),
            Some((
                "m_ColliderMask".to_string(),
                self.m_collider_mask.to_value(),
            )),
            crate::opt_field("m_UseColliderMask", &self.m_use_collider_mask),
            Some(("m_Speed".to_string(), self.m_speed.to_value())),
            Some((
                "m_SpeedVariation".to_string(),
                self.m_speed_variation.to_value(),
            )),
            crate::opt_field("m_ForceScale", &self.m_force_scale),
            crate::opt_field("m_UseContactForce", &self.m_use_contact_force),
            crate::opt_field("m_UseFriction", &self.m_use_friction),
            crate::opt_field("m_UseBounce", &self.m_use_bounce),
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
                "m_Enabled" => (self.m_enabled).inspect_value(path, offset, limit),
                "m_ColliderMask" => (self.m_collider_mask).inspect_value(path, offset, limit),
                "m_UseColliderMask" => {
                    (self.m_use_collider_mask).inspect_value(path, offset, limit)
                }
                "m_Speed" => (self.m_speed).inspect_value(path, offset, limit),
                "m_SpeedVariation" => (self.m_speed_variation).inspect_value(path, offset, limit),
                "m_ForceScale" => (self.m_force_scale).inspect_value(path, offset, limit),
                "m_UseContactForce" => {
                    (self.m_use_contact_force).inspect_value(path, offset, limit)
                }
                "m_UseFriction" => (self.m_use_friction).inspect_value(path, offset, limit),
                "m_UseBounce" => (self.m_use_bounce).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_game_object).inspect_value(path, offset, limit))
                .map(|v| ("m_GameObject".to_string(), v)),
            ((self.m_enabled).inspect_value(path, offset, limit))
                .map(|v| ("m_Enabled".to_string(), v)),
            ((self.m_collider_mask).inspect_value(path, offset, limit))
                .map(|v| ("m_ColliderMask".to_string(), v)),
            ((self.m_use_collider_mask).inspect_value(path, offset, limit))
                .map(|v| ("m_UseColliderMask".to_string(), v)),
            ((self.m_speed).inspect_value(path, offset, limit)).map(|v| ("m_Speed".to_string(), v)),
            ((self.m_speed_variation).inspect_value(path, offset, limit))
                .map(|v| ("m_SpeedVariation".to_string(), v)),
            ((self.m_force_scale).inspect_value(path, offset, limit))
                .map(|v| ("m_ForceScale".to_string(), v)),
            ((self.m_use_contact_force).inspect_value(path, offset, limit))
                .map(|v| ("m_UseContactForce".to_string(), v)),
            ((self.m_use_friction).inspect_value(path, offset, limit))
                .map(|v| ("m_UseFriction".to_string(), v)),
            ((self.m_use_bounce).inspect_value(path, offset, limit))
                .map(|v| ("m_UseBounce".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
