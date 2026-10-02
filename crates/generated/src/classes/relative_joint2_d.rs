//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vector2f::Vector2f;

#[derive(Debug, Clone)]
pub struct RelativeJoint2D {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_enable_collision: bool,
    pub m_connected_rigid_body: PPtr,
    pub m_break_force: f32,
    pub m_break_torque: f32,
    pub m_max_force: f32,
    pub m_break_action: Option<i32>,
    pub m_max_torque: f32,
    pub m_correction_scale: f32,
    pub m_auto_configure_offset: bool,
    pub m_linear_offset: Vector2f,
    pub m_angular_offset: f32,
}

impl crate::Parse for RelativeJoint2D {
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
        let mut m_max_force = None;
        let mut m_break_action = None;
        let mut m_max_torque = None;
        let mut m_correction_scale = None;
        let mut m_auto_configure_offset = None;
        let mut m_linear_offset = None;
        let mut m_angular_offset = None;
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
        m_max_force = Some(rdr.read_f32()?);
        m_max_torque = Some(rdr.read_f32()?);
        m_correction_scale = Some(rdr.read_f32()?);
        m_auto_configure_offset = Some(rdr.read_bool()?);
        rdr.align(4)?;
        m_linear_offset = Some(Vector2f::parse(rdr, ver)?);
        m_angular_offset = Some(rdr.read_f32()?);

        Ok(RelativeJoint2D {
            m_game_object: m_game_object.unwrap(),
            m_enabled: m_enabled.unwrap(),
            m_enable_collision: m_enable_collision.unwrap(),
            m_connected_rigid_body: m_connected_rigid_body.unwrap(),
            m_break_force: m_break_force.unwrap(),
            m_break_torque: m_break_torque.unwrap(),
            m_max_force: m_max_force.unwrap(),
            m_break_action,
            m_max_torque: m_max_torque.unwrap(),
            m_correction_scale: m_correction_scale.unwrap(),
            m_auto_configure_offset: m_auto_configure_offset.unwrap(),
            m_linear_offset: m_linear_offset.unwrap(),
            m_angular_offset: m_angular_offset.unwrap(),
        })
    }
}

impl crate::ToValue for RelativeJoint2D {
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
            Some(("m_MaxForce".to_string(), self.m_max_force.to_value())),
            crate::opt_field("m_BreakAction", &self.m_break_action),
            Some(("m_MaxTorque".to_string(), self.m_max_torque.to_value())),
            Some((
                "m_CorrectionScale".to_string(),
                self.m_correction_scale.to_value(),
            )),
            Some((
                "m_AutoConfigureOffset".to_string(),
                self.m_auto_configure_offset.to_value(),
            )),
            Some((
                "m_LinearOffset".to_string(),
                self.m_linear_offset.to_value(),
            )),
            Some((
                "m_AngularOffset".to_string(),
                self.m_angular_offset.to_value(),
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
                "m_EnableCollision" => (self.m_enable_collision).inspect_value(path, offset, limit),
                "m_ConnectedRigidBody" => {
                    (self.m_connected_rigid_body).inspect_value(path, offset, limit)
                }
                "m_BreakForce" => (self.m_break_force).inspect_value(path, offset, limit),
                "m_BreakTorque" => (self.m_break_torque).inspect_value(path, offset, limit),
                "m_MaxForce" => (self.m_max_force).inspect_value(path, offset, limit),
                "m_BreakAction" => (self.m_break_action).inspect_value(path, offset, limit),
                "m_MaxTorque" => (self.m_max_torque).inspect_value(path, offset, limit),
                "m_CorrectionScale" => (self.m_correction_scale).inspect_value(path, offset, limit),
                "m_AutoConfigureOffset" => {
                    (self.m_auto_configure_offset).inspect_value(path, offset, limit)
                }
                "m_LinearOffset" => (self.m_linear_offset).inspect_value(path, offset, limit),
                "m_AngularOffset" => (self.m_angular_offset).inspect_value(path, offset, limit),
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
            ((self.m_max_force).inspect_value(path, offset, limit))
                .map(|v| ("m_MaxForce".to_string(), v)),
            ((self.m_break_action).inspect_value(path, offset, limit))
                .map(|v| ("m_BreakAction".to_string(), v)),
            ((self.m_max_torque).inspect_value(path, offset, limit))
                .map(|v| ("m_MaxTorque".to_string(), v)),
            ((self.m_correction_scale).inspect_value(path, offset, limit))
                .map(|v| ("m_CorrectionScale".to_string(), v)),
            ((self.m_auto_configure_offset).inspect_value(path, offset, limit))
                .map(|v| ("m_AutoConfigureOffset".to_string(), v)),
            ((self.m_linear_offset).inspect_value(path, offset, limit))
                .map(|v| ("m_LinearOffset".to_string(), v)),
            ((self.m_angular_offset).inspect_value(path, offset, limit))
                .map(|v| ("m_AngularOffset".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
