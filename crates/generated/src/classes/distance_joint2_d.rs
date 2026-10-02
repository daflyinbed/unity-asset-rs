//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vector2f::Vector2f;

#[derive(Debug, Clone)]
pub struct DistanceJoint2D {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_collide_connected: Option<bool>,
    pub m_enable_collision: Option<bool>,
    pub m_connected_rigid_body: PPtr,
    pub m_anchor: Vector2f,
    pub m_break_force: Option<f32>,
    pub m_connected_anchor: Vector2f,
    pub m_break_torque: Option<f32>,
    pub m_distance: f32,
    pub m_auto_configure_connected_anchor: Option<bool>,
    pub m_break_action: Option<i32>,
    pub m_max_distance_only: Option<bool>,
    pub m_auto_configure_distance: Option<bool>,
}

impl crate::Parse for DistanceJoint2D {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut m_enabled = None;
        let mut m_collide_connected = None;
        let mut m_enable_collision = None;
        let mut m_connected_rigid_body = None;
        let mut m_anchor = None;
        let mut m_break_force = None;
        let mut m_connected_anchor = None;
        let mut m_break_torque = None;
        let mut m_distance = None;
        let mut m_auto_configure_connected_anchor = None;
        let mut m_break_action = None;
        let mut m_max_distance_only = None;
        let mut m_auto_configure_distance = None;
        m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
        m_enabled = Some(rdr.read_u8()?);
        rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(5, 0, 1) {
            m_enable_collision = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
            m_collide_connected = Some(rdr.read_bool()?);
            rdr.align(4)?;
        }
        m_connected_rigid_body = Some(crate::PPtr::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            m_break_force = Some(rdr.read_f32()?);
            m_break_torque = Some(rdr.read_f32()?);
            m_break_action = Some(rdr.read_i32()?);
            m_auto_configure_connected_anchor = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 3, 0) {
            m_break_force = Some(rdr.read_f32()?);
            m_break_torque = Some(rdr.read_f32()?);
            m_auto_configure_connected_anchor = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
        }
        m_anchor = Some(Vector2f::parse(rdr, ver)?);
        m_connected_anchor = Some(Vector2f::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 3, 0) {
            m_auto_configure_distance = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
        }
        m_distance = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(4, 5, 0) {
            m_max_distance_only = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
        }

        Ok(DistanceJoint2D {
            m_game_object: m_game_object.unwrap(),
            m_enabled: m_enabled.unwrap(),
            m_collide_connected,
            m_enable_collision,
            m_connected_rigid_body: m_connected_rigid_body.unwrap(),
            m_anchor: m_anchor.unwrap(),
            m_break_force,
            m_connected_anchor: m_connected_anchor.unwrap(),
            m_break_torque,
            m_distance: m_distance.unwrap(),
            m_auto_configure_connected_anchor,
            m_break_action,
            m_max_distance_only,
            m_auto_configure_distance,
        })
    }
}

impl crate::ToValue for DistanceJoint2D {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_GameObject".to_string(), self.m_game_object.to_value())),
            Some(("m_Enabled".to_string(), self.m_enabled.to_value())),
            crate::opt_field("m_CollideConnected", &self.m_collide_connected),
            crate::opt_field("m_EnableCollision", &self.m_enable_collision),
            Some((
                "m_ConnectedRigidBody".to_string(),
                self.m_connected_rigid_body.to_value(),
            )),
            Some(("m_Anchor".to_string(), self.m_anchor.to_value())),
            crate::opt_field("m_BreakForce", &self.m_break_force),
            Some((
                "m_ConnectedAnchor".to_string(),
                self.m_connected_anchor.to_value(),
            )),
            crate::opt_field("m_BreakTorque", &self.m_break_torque),
            Some(("m_Distance".to_string(), self.m_distance.to_value())),
            crate::opt_field(
                "m_AutoConfigureConnectedAnchor",
                &self.m_auto_configure_connected_anchor,
            ),
            crate::opt_field("m_BreakAction", &self.m_break_action),
            crate::opt_field("m_MaxDistanceOnly", &self.m_max_distance_only),
            crate::opt_field("m_AutoConfigureDistance", &self.m_auto_configure_distance),
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
                "m_CollideConnected" => {
                    (self.m_collide_connected).inspect_value(path, offset, limit)
                }
                "m_EnableCollision" => (self.m_enable_collision).inspect_value(path, offset, limit),
                "m_ConnectedRigidBody" => {
                    (self.m_connected_rigid_body).inspect_value(path, offset, limit)
                }
                "m_Anchor" => (self.m_anchor).inspect_value(path, offset, limit),
                "m_BreakForce" => (self.m_break_force).inspect_value(path, offset, limit),
                "m_ConnectedAnchor" => (self.m_connected_anchor).inspect_value(path, offset, limit),
                "m_BreakTorque" => (self.m_break_torque).inspect_value(path, offset, limit),
                "m_Distance" => (self.m_distance).inspect_value(path, offset, limit),
                "m_AutoConfigureConnectedAnchor" => {
                    (self.m_auto_configure_connected_anchor).inspect_value(path, offset, limit)
                }
                "m_BreakAction" => (self.m_break_action).inspect_value(path, offset, limit),
                "m_MaxDistanceOnly" => {
                    (self.m_max_distance_only).inspect_value(path, offset, limit)
                }
                "m_AutoConfigureDistance" => {
                    (self.m_auto_configure_distance).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_game_object).inspect_value(path, offset, limit))
                .map(|v| ("m_GameObject".to_string(), v)),
            ((self.m_enabled).inspect_value(path, offset, limit))
                .map(|v| ("m_Enabled".to_string(), v)),
            ((self.m_collide_connected).inspect_value(path, offset, limit))
                .map(|v| ("m_CollideConnected".to_string(), v)),
            ((self.m_enable_collision).inspect_value(path, offset, limit))
                .map(|v| ("m_EnableCollision".to_string(), v)),
            ((self.m_connected_rigid_body).inspect_value(path, offset, limit))
                .map(|v| ("m_ConnectedRigidBody".to_string(), v)),
            ((self.m_anchor).inspect_value(path, offset, limit))
                .map(|v| ("m_Anchor".to_string(), v)),
            ((self.m_break_force).inspect_value(path, offset, limit))
                .map(|v| ("m_BreakForce".to_string(), v)),
            ((self.m_connected_anchor).inspect_value(path, offset, limit))
                .map(|v| ("m_ConnectedAnchor".to_string(), v)),
            ((self.m_break_torque).inspect_value(path, offset, limit))
                .map(|v| ("m_BreakTorque".to_string(), v)),
            ((self.m_distance).inspect_value(path, offset, limit))
                .map(|v| ("m_Distance".to_string(), v)),
            ((self.m_auto_configure_connected_anchor).inspect_value(path, offset, limit))
                .map(|v| ("m_AutoConfigureConnectedAnchor".to_string(), v)),
            ((self.m_break_action).inspect_value(path, offset, limit))
                .map(|v| ("m_BreakAction".to_string(), v)),
            ((self.m_max_distance_only).inspect_value(path, offset, limit))
                .map(|v| ("m_MaxDistanceOnly".to_string(), v)),
            ((self.m_auto_configure_distance).inspect_value(path, offset, limit))
                .map(|v| ("m_AutoConfigureDistance".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
