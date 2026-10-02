//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct ConstantForce {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_force: Vector3f,
    pub m_relative_force: Vector3f,
    pub m_torque: Vector3f,
    pub m_relative_torque: Vector3f,
}

impl crate::Parse for ConstantForce {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(rdr, ver)?;
        let m_enabled = rdr.read_u8()?;
        rdr.align(4)?;
        let m_force = Vector3f::parse(rdr, ver)?;
        let m_relative_force = Vector3f::parse(rdr, ver)?;
        let m_torque = Vector3f::parse(rdr, ver)?;
        let m_relative_torque = Vector3f::parse(rdr, ver)?;

        Ok(ConstantForce {
            m_game_object,
            m_enabled,
            m_force,
            m_relative_force,
            m_torque,
            m_relative_torque,
        })
    }
}

impl crate::ToValue for ConstantForce {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_GameObject".to_string(), self.m_game_object.to_value())),
            Some(("m_Enabled".to_string(), self.m_enabled.to_value())),
            Some(("m_Force".to_string(), self.m_force.to_value())),
            Some((
                "m_RelativeForce".to_string(),
                self.m_relative_force.to_value(),
            )),
            Some(("m_Torque".to_string(), self.m_torque.to_value())),
            Some((
                "m_RelativeTorque".to_string(),
                self.m_relative_torque.to_value(),
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
                "m_Enabled" => (self.m_enabled).inspect_value(path, offset, limit),
                "m_Force" => (self.m_force).inspect_value(path, offset, limit),
                "m_RelativeForce" => (self.m_relative_force).inspect_value(path, offset, limit),
                "m_Torque" => (self.m_torque).inspect_value(path, offset, limit),
                "m_RelativeTorque" => (self.m_relative_torque).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_game_object).inspect_value(path, offset, limit))
                .map(|v| ("m_GameObject".to_string(), v)),
            ((self.m_enabled).inspect_value(path, offset, limit))
                .map(|v| ("m_Enabled".to_string(), v)),
            ((self.m_force).inspect_value(path, offset, limit)).map(|v| ("m_Force".to_string(), v)),
            ((self.m_relative_force).inspect_value(path, offset, limit))
                .map(|v| ("m_RelativeForce".to_string(), v)),
            ((self.m_torque).inspect_value(path, offset, limit))
                .map(|v| ("m_Torque".to_string(), v)),
            ((self.m_relative_torque).inspect_value(path, offset, limit))
                .map(|v| ("m_RelativeTorque".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
