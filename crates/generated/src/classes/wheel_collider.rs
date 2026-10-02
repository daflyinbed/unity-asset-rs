//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::bit_field::BitField;
use crate::subclasses::joint_spring::JointSpring;
use crate::subclasses::vector3f::Vector3f;
use crate::subclasses::wheel_friction_curve::WheelFrictionCurve;

/// 版本枚举：WheelColliderMEnabledVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum WheelColliderMEnabledVariant {
    Default(bool),
    V2022_2(bool),
}

impl crate::Parse for WheelColliderMEnabledVariant {
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

impl crate::ToValue for WheelColliderMEnabledVariant {
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
#[derive(Debug, Clone)]
pub struct WheelCollider {
    pub m_game_object: PPtr,
    pub m_center: Vector3f,
    pub m_radius: f32,
    pub m_suspension_distance: f32,
    pub m_suspension_spring: JointSpring,
    pub m_mass: f32,
    pub m_force_app_point_distance: Option<f32>,
    pub m_forward_friction: WheelFrictionCurve,
    pub m_sideways_friction: WheelFrictionCurve,
    pub m_wheel_damping_rate: Option<f32>,
    pub m_enabled: Option<WheelColliderMEnabledVariant>,
    pub m_include_layers: Option<BitField>,
    pub m_exclude_layers: Option<BitField>,
    pub m_layer_override_priority: Option<i32>,
    pub m_provides_contacts: Option<bool>,
}

impl crate::Parse for WheelCollider {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut m_center = None;
        let mut m_radius = None;
        let mut m_suspension_distance = None;
        let mut m_suspension_spring = None;
        let mut m_mass = None;
        let mut m_force_app_point_distance = None;
        let mut m_forward_friction = None;
        let mut m_sideways_friction = None;
        let mut m_wheel_damping_rate = None;
        let mut m_enabled = None;
        let mut m_include_layers = None;
        let mut m_exclude_layers = None;
        let mut m_layer_override_priority = None;
        let mut m_provides_contacts = None;
        m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
        m_center = Some(Vector3f::parse(rdr, ver)?);
        m_radius = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
        } else {
            m_suspension_distance = Some(rdr.read_f32()?);
        }
        m_suspension_spring = Some(JointSpring::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_suspension_distance = Some(rdr.read_f32()?);
            m_force_app_point_distance = Some(rdr.read_f32()?);
        } else {
        }
        m_mass = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_wheel_damping_rate = Some(rdr.read_f32()?);
        } else {
        }
        m_forward_friction = Some(WheelFrictionCurve::parse(rdr, ver)?);
        m_sideways_friction = Some(WheelFrictionCurve::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            m_include_layers = Some(BitField::parse(rdr, ver)?);
            m_exclude_layers = Some(BitField::parse(rdr, ver)?);
            m_layer_override_priority = Some(rdr.read_i32()?);
            m_enabled = Some(WheelColliderMEnabledVariant::parse(rdr, ver)?);
            m_provides_contacts = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_enabled = Some(WheelColliderMEnabledVariant::parse(rdr, ver)?);
        } else {
        }

        Ok(WheelCollider {
            m_game_object: m_game_object.unwrap(),
            m_center: m_center.unwrap(),
            m_radius: m_radius.unwrap(),
            m_suspension_distance: m_suspension_distance.unwrap(),
            m_suspension_spring: m_suspension_spring.unwrap(),
            m_mass: m_mass.unwrap(),
            m_force_app_point_distance,
            m_forward_friction: m_forward_friction.unwrap(),
            m_sideways_friction: m_sideways_friction.unwrap(),
            m_wheel_damping_rate,
            m_enabled,
            m_include_layers,
            m_exclude_layers,
            m_layer_override_priority,
            m_provides_contacts,
        })
    }
}

impl crate::ToValue for WheelCollider {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_GameObject".to_string(), self.m_game_object.to_value())),
            Some(("m_Center".to_string(), self.m_center.to_value())),
            Some(("m_Radius".to_string(), self.m_radius.to_value())),
            Some((
                "m_SuspensionDistance".to_string(),
                self.m_suspension_distance.to_value(),
            )),
            Some((
                "m_SuspensionSpring".to_string(),
                self.m_suspension_spring.to_value(),
            )),
            Some(("m_Mass".to_string(), self.m_mass.to_value())),
            crate::opt_field("m_ForceAppPointDistance", &self.m_force_app_point_distance),
            Some((
                "m_ForwardFriction".to_string(),
                self.m_forward_friction.to_value(),
            )),
            Some((
                "m_SidewaysFriction".to_string(),
                self.m_sideways_friction.to_value(),
            )),
            crate::opt_field("m_WheelDampingRate", &self.m_wheel_damping_rate),
            crate::opt_field("m_Enabled", &self.m_enabled),
            crate::opt_field("m_IncludeLayers", &self.m_include_layers),
            crate::opt_field("m_ExcludeLayers", &self.m_exclude_layers),
            crate::opt_field("m_LayerOverridePriority", &self.m_layer_override_priority),
            crate::opt_field("m_ProvidesContacts", &self.m_provides_contacts),
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
                "m_Center" => (self.m_center).inspect_value(path, offset, limit),
                "m_Radius" => (self.m_radius).inspect_value(path, offset, limit),
                "m_SuspensionDistance" => {
                    (self.m_suspension_distance).inspect_value(path, offset, limit)
                }
                "m_SuspensionSpring" => {
                    (self.m_suspension_spring).inspect_value(path, offset, limit)
                }
                "m_Mass" => (self.m_mass).inspect_value(path, offset, limit),
                "m_ForceAppPointDistance" => {
                    (self.m_force_app_point_distance).inspect_value(path, offset, limit)
                }
                "m_ForwardFriction" => (self.m_forward_friction).inspect_value(path, offset, limit),
                "m_SidewaysFriction" => {
                    (self.m_sideways_friction).inspect_value(path, offset, limit)
                }
                "m_WheelDampingRate" => {
                    (self.m_wheel_damping_rate).inspect_value(path, offset, limit)
                }
                "m_Enabled" => (self.m_enabled).inspect_value(path, offset, limit),
                "m_IncludeLayers" => (self.m_include_layers).inspect_value(path, offset, limit),
                "m_ExcludeLayers" => (self.m_exclude_layers).inspect_value(path, offset, limit),
                "m_LayerOverridePriority" => {
                    (self.m_layer_override_priority).inspect_value(path, offset, limit)
                }
                "m_ProvidesContacts" => {
                    (self.m_provides_contacts).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_game_object).inspect_value(path, offset, limit))
                .map(|v| ("m_GameObject".to_string(), v)),
            ((self.m_center).inspect_value(path, offset, limit))
                .map(|v| ("m_Center".to_string(), v)),
            ((self.m_radius).inspect_value(path, offset, limit))
                .map(|v| ("m_Radius".to_string(), v)),
            ((self.m_suspension_distance).inspect_value(path, offset, limit))
                .map(|v| ("m_SuspensionDistance".to_string(), v)),
            ((self.m_suspension_spring).inspect_value(path, offset, limit))
                .map(|v| ("m_SuspensionSpring".to_string(), v)),
            ((self.m_mass).inspect_value(path, offset, limit)).map(|v| ("m_Mass".to_string(), v)),
            ((self.m_force_app_point_distance).inspect_value(path, offset, limit))
                .map(|v| ("m_ForceAppPointDistance".to_string(), v)),
            ((self.m_forward_friction).inspect_value(path, offset, limit))
                .map(|v| ("m_ForwardFriction".to_string(), v)),
            ((self.m_sideways_friction).inspect_value(path, offset, limit))
                .map(|v| ("m_SidewaysFriction".to_string(), v)),
            ((self.m_wheel_damping_rate).inspect_value(path, offset, limit))
                .map(|v| ("m_WheelDampingRate".to_string(), v)),
            ((self.m_enabled).inspect_value(path, offset, limit))
                .map(|v| ("m_Enabled".to_string(), v)),
            ((self.m_include_layers).inspect_value(path, offset, limit))
                .map(|v| ("m_IncludeLayers".to_string(), v)),
            ((self.m_exclude_layers).inspect_value(path, offset, limit))
                .map(|v| ("m_ExcludeLayers".to_string(), v)),
            ((self.m_layer_override_priority).inspect_value(path, offset, limit))
                .map(|v| ("m_LayerOverridePriority".to_string(), v)),
            ((self.m_provides_contacts).inspect_value(path, offset, limit))
                .map(|v| ("m_ProvidesContacts".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
