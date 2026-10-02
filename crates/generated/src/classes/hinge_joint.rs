//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::joint_limits::JointLimits;
use crate::subclasses::joint_motor::JointMotor;
use crate::subclasses::joint_spring::JointSpring;
use crate::subclasses::vector3f::Vector3f;

/// 版本枚举：HingeJointMUseLimitsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum HingeJointMUseLimitsVariant {
    Default(bool),
    V2022_2(bool),
}

impl crate::Parse for HingeJointMUseLimitsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2022_2(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for HingeJointMUseLimitsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2022_2(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2022_2(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：HingeJointMEnablePreprocessingVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum HingeJointMEnablePreprocessingVariant {
    Default(bool),
    V2017_1(bool),
}

impl crate::Parse for HingeJointMEnablePreprocessingVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for HingeJointMEnablePreprocessingVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct HingeJoint {
    pub m_game_object: PPtr,
    pub m_connected_body: PPtr,
    pub m_anchor: Vector3f,
    pub m_connected_articulation_body: Option<PPtr>,
    pub m_axis: Vector3f,
    pub m_use_spring: bool,
    pub m_auto_configure_connected_anchor: Option<bool>,
    pub m_spring: JointSpring,
    pub m_connected_anchor: Option<Vector3f>,
    pub m_use_motor: bool,
    pub m_motor: JointMotor,
    pub m_use_limits: HingeJointMUseLimitsVariant,
    pub m_limits: JointLimits,
    pub m_break_force: f32,
    pub m_break_torque: f32,
    pub m_extended_limits: Option<bool>,
    pub m_use_acceleration: Option<bool>,
    pub m_enable_collision: Option<bool>,
    pub m_enable_preprocessing: Option<HingeJointMEnablePreprocessingVariant>,
    pub m_mass_scale: Option<f32>,
    pub m_connected_mass_scale: Option<f32>,
}

impl crate::Parse for HingeJoint {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut m_connected_body = None;
        let mut m_anchor = None;
        let mut m_connected_articulation_body = None;
        let mut m_axis = None;
        let mut m_use_spring = None;
        let mut m_auto_configure_connected_anchor = None;
        let mut m_spring = None;
        let mut m_connected_anchor = None;
        let mut m_use_motor = None;
        let mut m_motor = None;
        let mut m_use_limits = None;
        let mut m_limits = None;
        let mut m_break_force = None;
        let mut m_break_torque = None;
        let mut m_extended_limits = None;
        let mut m_use_acceleration = None;
        let mut m_enable_collision = None;
        let mut m_enable_preprocessing = None;
        let mut m_mass_scale = None;
        let mut m_connected_mass_scale = None;
        m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
        m_connected_body = Some(crate::PPtr::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            m_connected_articulation_body = Some(crate::PPtr::parse(rdr, ver)?);
        } else {
        }
        m_anchor = Some(Vector3f::parse(rdr, ver)?);
        m_axis = Some(Vector3f::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_auto_configure_connected_anchor = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_connected_anchor = Some(Vector3f::parse(rdr, ver)?);
        } else {
        }
        m_use_spring = Some(rdr.read_bool()?);
        rdr.align(4)?;
        m_spring = Some(JointSpring::parse(rdr, ver)?);
        m_use_motor = Some(rdr.read_bool()?);
        rdr.align(4)?;
        m_motor = Some(JointMotor::parse(rdr, ver)?);
        m_use_limits = Some(HingeJointMUseLimitsVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            m_extended_limits = Some(rdr.read_bool()?);
            m_use_acceleration = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
        }
        m_limits = Some(JointLimits::parse(rdr, ver)?);
        m_break_force = Some(rdr.read_f32()?);
        m_break_torque = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_enable_collision = Some(rdr.read_bool()?);
            m_enable_preprocessing = Some(HingeJointMEnablePreprocessingVariant::parse(rdr, ver)?);
            m_mass_scale = Some(rdr.read_f32()?);
            m_connected_mass_scale = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_enable_collision = Some(rdr.read_bool()?);
            m_enable_preprocessing = Some(HingeJointMEnablePreprocessingVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 5, 0) {
            m_enable_collision = Some(rdr.read_bool()?);
        } else {
        }

        Ok(HingeJoint {
            m_game_object: m_game_object.unwrap(),
            m_connected_body: m_connected_body.unwrap(),
            m_anchor: m_anchor.unwrap(),
            m_connected_articulation_body,
            m_axis: m_axis.unwrap(),
            m_use_spring: m_use_spring.unwrap(),
            m_auto_configure_connected_anchor,
            m_spring: m_spring.unwrap(),
            m_connected_anchor,
            m_use_motor: m_use_motor.unwrap(),
            m_motor: m_motor.unwrap(),
            m_use_limits: m_use_limits.unwrap(),
            m_limits: m_limits.unwrap(),
            m_break_force: m_break_force.unwrap(),
            m_break_torque: m_break_torque.unwrap(),
            m_extended_limits,
            m_use_acceleration,
            m_enable_collision,
            m_enable_preprocessing,
            m_mass_scale,
            m_connected_mass_scale,
        })
    }
}

impl crate::ToValue for HingeJoint {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_GameObject".to_string(), self.m_game_object.to_value())),
            Some((
                "m_ConnectedBody".to_string(),
                self.m_connected_body.to_value(),
            )),
            Some(("m_Anchor".to_string(), self.m_anchor.to_value())),
            crate::opt_field(
                "m_ConnectedArticulationBody",
                &self.m_connected_articulation_body,
            ),
            Some(("m_Axis".to_string(), self.m_axis.to_value())),
            Some(("m_UseSpring".to_string(), self.m_use_spring.to_value())),
            crate::opt_field(
                "m_AutoConfigureConnectedAnchor",
                &self.m_auto_configure_connected_anchor,
            ),
            Some(("m_Spring".to_string(), self.m_spring.to_value())),
            crate::opt_field("m_ConnectedAnchor", &self.m_connected_anchor),
            Some(("m_UseMotor".to_string(), self.m_use_motor.to_value())),
            Some(("m_Motor".to_string(), self.m_motor.to_value())),
            Some(("m_UseLimits".to_string(), self.m_use_limits.to_value())),
            Some(("m_Limits".to_string(), self.m_limits.to_value())),
            Some(("m_BreakForce".to_string(), self.m_break_force.to_value())),
            Some(("m_BreakTorque".to_string(), self.m_break_torque.to_value())),
            crate::opt_field("m_ExtendedLimits", &self.m_extended_limits),
            crate::opt_field("m_UseAcceleration", &self.m_use_acceleration),
            crate::opt_field("m_EnableCollision", &self.m_enable_collision),
            crate::opt_field("m_EnablePreprocessing", &self.m_enable_preprocessing),
            crate::opt_field("m_MassScale", &self.m_mass_scale),
            crate::opt_field("m_ConnectedMassScale", &self.m_connected_mass_scale),
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
                "m_ConnectedBody" => (self.m_connected_body).inspect_value(path, offset, limit),
                "m_Anchor" => (self.m_anchor).inspect_value(path, offset, limit),
                "m_ConnectedArticulationBody" => {
                    (self.m_connected_articulation_body).inspect_value(path, offset, limit)
                }
                "m_Axis" => (self.m_axis).inspect_value(path, offset, limit),
                "m_UseSpring" => (self.m_use_spring).inspect_value(path, offset, limit),
                "m_AutoConfigureConnectedAnchor" => {
                    (self.m_auto_configure_connected_anchor).inspect_value(path, offset, limit)
                }
                "m_Spring" => (self.m_spring).inspect_value(path, offset, limit),
                "m_ConnectedAnchor" => (self.m_connected_anchor).inspect_value(path, offset, limit),
                "m_UseMotor" => (self.m_use_motor).inspect_value(path, offset, limit),
                "m_Motor" => (self.m_motor).inspect_value(path, offset, limit),
                "m_UseLimits" => (self.m_use_limits).inspect_value(path, offset, limit),
                "m_Limits" => (self.m_limits).inspect_value(path, offset, limit),
                "m_BreakForce" => (self.m_break_force).inspect_value(path, offset, limit),
                "m_BreakTorque" => (self.m_break_torque).inspect_value(path, offset, limit),
                "m_ExtendedLimits" => (self.m_extended_limits).inspect_value(path, offset, limit),
                "m_UseAcceleration" => (self.m_use_acceleration).inspect_value(path, offset, limit),
                "m_EnableCollision" => (self.m_enable_collision).inspect_value(path, offset, limit),
                "m_EnablePreprocessing" => {
                    (self.m_enable_preprocessing).inspect_value(path, offset, limit)
                }
                "m_MassScale" => (self.m_mass_scale).inspect_value(path, offset, limit),
                "m_ConnectedMassScale" => {
                    (self.m_connected_mass_scale).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_game_object).inspect_value(path, offset, limit))
                .map(|v| ("m_GameObject".to_string(), v)),
            ((self.m_connected_body).inspect_value(path, offset, limit))
                .map(|v| ("m_ConnectedBody".to_string(), v)),
            ((self.m_anchor).inspect_value(path, offset, limit))
                .map(|v| ("m_Anchor".to_string(), v)),
            ((self.m_connected_articulation_body).inspect_value(path, offset, limit))
                .map(|v| ("m_ConnectedArticulationBody".to_string(), v)),
            ((self.m_axis).inspect_value(path, offset, limit)).map(|v| ("m_Axis".to_string(), v)),
            ((self.m_use_spring).inspect_value(path, offset, limit))
                .map(|v| ("m_UseSpring".to_string(), v)),
            ((self.m_auto_configure_connected_anchor).inspect_value(path, offset, limit))
                .map(|v| ("m_AutoConfigureConnectedAnchor".to_string(), v)),
            ((self.m_spring).inspect_value(path, offset, limit))
                .map(|v| ("m_Spring".to_string(), v)),
            ((self.m_connected_anchor).inspect_value(path, offset, limit))
                .map(|v| ("m_ConnectedAnchor".to_string(), v)),
            ((self.m_use_motor).inspect_value(path, offset, limit))
                .map(|v| ("m_UseMotor".to_string(), v)),
            ((self.m_motor).inspect_value(path, offset, limit)).map(|v| ("m_Motor".to_string(), v)),
            ((self.m_use_limits).inspect_value(path, offset, limit))
                .map(|v| ("m_UseLimits".to_string(), v)),
            ((self.m_limits).inspect_value(path, offset, limit))
                .map(|v| ("m_Limits".to_string(), v)),
            ((self.m_break_force).inspect_value(path, offset, limit))
                .map(|v| ("m_BreakForce".to_string(), v)),
            ((self.m_break_torque).inspect_value(path, offset, limit))
                .map(|v| ("m_BreakTorque".to_string(), v)),
            ((self.m_extended_limits).inspect_value(path, offset, limit))
                .map(|v| ("m_ExtendedLimits".to_string(), v)),
            ((self.m_use_acceleration).inspect_value(path, offset, limit))
                .map(|v| ("m_UseAcceleration".to_string(), v)),
            ((self.m_enable_collision).inspect_value(path, offset, limit))
                .map(|v| ("m_EnableCollision".to_string(), v)),
            ((self.m_enable_preprocessing).inspect_value(path, offset, limit))
                .map(|v| ("m_EnablePreprocessing".to_string(), v)),
            ((self.m_mass_scale).inspect_value(path, offset, limit))
                .map(|v| ("m_MassScale".to_string(), v)),
            ((self.m_connected_mass_scale).inspect_value(path, offset, limit))
                .map(|v| ("m_ConnectedMassScale".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
