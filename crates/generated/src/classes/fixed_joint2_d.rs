//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vector2f::Vector2f;

#[derive(Debug, Clone)]
pub struct FixedJoint2D {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_enable_collision: bool,
    pub m_connected_rigid_body: PPtr,
    pub m_break_force: f32,
    pub m_break_torque: f32,
    pub m_auto_configure_connected_anchor: bool,
    pub m_break_action: Option<i32>,
    pub m_anchor: Vector2f,
    pub m_connected_anchor: Vector2f,
    pub m_damping_ratio: f32,
    pub m_frequency: f32,
}

impl crate::Parse for FixedJoint2D {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut m_enabled = None;
        let mut m_enable_collision = None;
        let mut m_connected_rigid_body = None;
        let mut m_break_force = None;
        let mut m_break_torque = None;
        let mut m_auto_configure_connected_anchor = None;
        let mut m_break_action = None;
        let mut m_anchor = None;
        let mut m_connected_anchor = None;
        let mut m_damping_ratio = None;
        let mut m_frequency = None;
        m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
        m_enabled = Some(rdr.read_u8()?);
        rdr.align(4)?;
        m_enable_collision = Some(rdr.read_bool()?);
        rdr.align(4)?;
        m_connected_rigid_body = Some(crate::PPtr::parse(rdr, ver)?);
        m_break_force = Some(rdr.read_f32()?);
        m_break_torque = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            m_break_action = Some(rdr.read_i32()?);
        } else {
        }
        m_auto_configure_connected_anchor = Some(rdr.read_bool()?);
        rdr.align(4)?;
        m_anchor = Some(Vector2f::parse(rdr, ver)?);
        m_connected_anchor = Some(Vector2f::parse(rdr, ver)?);
        m_damping_ratio = Some(rdr.read_f32()?);
        m_frequency = Some(rdr.read_f32()?);

        Ok(FixedJoint2D {
            m_game_object: m_game_object.unwrap(),
            m_enabled: m_enabled.unwrap(),
            m_enable_collision: m_enable_collision.unwrap(),
            m_connected_rigid_body: m_connected_rigid_body.unwrap(),
            m_break_force: m_break_force.unwrap(),
            m_break_torque: m_break_torque.unwrap(),
            m_auto_configure_connected_anchor: m_auto_configure_connected_anchor.unwrap(),
            m_break_action,
            m_anchor: m_anchor.unwrap(),
            m_connected_anchor: m_connected_anchor.unwrap(),
            m_damping_ratio: m_damping_ratio.unwrap(),
            m_frequency: m_frequency.unwrap(),
        })
    }
}

impl crate::ToValue for FixedJoint2D {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_GameObject".to_string(), self.m_game_object.to_value())),
            Some(("m_Enabled".to_string(), self.m_enabled.to_value())),
            Some((
                "m_EnableCollision".to_string(),
                self.m_enable_collision.to_value(),
            )),
            Some((
                "m_ConnectedRigidBody".to_string(),
                self.m_connected_rigid_body.to_value(),
            )),
            Some(("m_BreakForce".to_string(), self.m_break_force.to_value())),
            Some(("m_BreakTorque".to_string(), self.m_break_torque.to_value())),
            Some((
                "m_AutoConfigureConnectedAnchor".to_string(),
                self.m_auto_configure_connected_anchor.to_value(),
            )),
            crate::opt_field("m_BreakAction", &self.m_break_action),
            Some(("m_Anchor".to_string(), self.m_anchor.to_value())),
            Some((
                "m_ConnectedAnchor".to_string(),
                self.m_connected_anchor.to_value(),
            )),
            Some((
                "m_DampingRatio".to_string(),
                self.m_damping_ratio.to_value(),
            )),
            Some(("m_Frequency".to_string(), self.m_frequency.to_value())),
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
                "m_EnableCollision" => (self.m_enable_collision).inspect_value(path, offset, limit),
                "m_ConnectedRigidBody" => {
                    (self.m_connected_rigid_body).inspect_value(path, offset, limit)
                }
                "m_BreakForce" => (self.m_break_force).inspect_value(path, offset, limit),
                "m_BreakTorque" => (self.m_break_torque).inspect_value(path, offset, limit),
                "m_AutoConfigureConnectedAnchor" => {
                    (self.m_auto_configure_connected_anchor).inspect_value(path, offset, limit)
                }
                "m_BreakAction" => (self.m_break_action).inspect_value(path, offset, limit),
                "m_Anchor" => (self.m_anchor).inspect_value(path, offset, limit),
                "m_ConnectedAnchor" => (self.m_connected_anchor).inspect_value(path, offset, limit),
                "m_DampingRatio" => (self.m_damping_ratio).inspect_value(path, offset, limit),
                "m_Frequency" => (self.m_frequency).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_game_object).inspect_value(path, offset, limit))
                .map(|v| ("m_GameObject".to_string(), v)),
            ((self.m_enabled).inspect_value(path, offset, limit))
                .map(|v| ("m_Enabled".to_string(), v)),
            ((self.m_enable_collision).inspect_value(path, offset, limit))
                .map(|v| ("m_EnableCollision".to_string(), v)),
            ((self.m_connected_rigid_body).inspect_value(path, offset, limit))
                .map(|v| ("m_ConnectedRigidBody".to_string(), v)),
            ((self.m_break_force).inspect_value(path, offset, limit))
                .map(|v| ("m_BreakForce".to_string(), v)),
            ((self.m_break_torque).inspect_value(path, offset, limit))
                .map(|v| ("m_BreakTorque".to_string(), v)),
            ((self.m_auto_configure_connected_anchor).inspect_value(path, offset, limit))
                .map(|v| ("m_AutoConfigureConnectedAnchor".to_string(), v)),
            ((self.m_break_action).inspect_value(path, offset, limit))
                .map(|v| ("m_BreakAction".to_string(), v)),
            ((self.m_anchor).inspect_value(path, offset, limit))
                .map(|v| ("m_Anchor".to_string(), v)),
            ((self.m_connected_anchor).inspect_value(path, offset, limit))
                .map(|v| ("m_ConnectedAnchor".to_string(), v)),
            ((self.m_damping_ratio).inspect_value(path, offset, limit))
                .map(|v| ("m_DampingRatio".to_string(), v)),
            ((self.m_frequency).inspect_value(path, offset, limit))
                .map(|v| ("m_Frequency".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
