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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            let v = r.read_bool()?;
            Ok(Self::V2022_2(v))
        } else {
            let v = r.read_bool()?;
            r.align(4)?;
            Ok(Self::Default(v))
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
        r: &mut R,
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
            m_game_object = Some(crate::PPtr::parse(r, ver)?);
            m_center = Some(Vector3f::parse(r, ver)?);
            m_radius = Some(r.read_f32()?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
        } else {
            m_suspension_distance = Some(r.read_f32()?);
        }
            m_suspension_spring = Some(JointSpring::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_suspension_distance = Some(r.read_f32()?);
            m_force_app_point_distance = Some(r.read_f32()?);
        } else {
        }
            m_mass = Some(r.read_f32()?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_wheel_damping_rate = Some(r.read_f32()?);
        } else {
        }
            m_forward_friction = Some(WheelFrictionCurve::parse(r, ver)?);
            m_sideways_friction = Some(WheelFrictionCurve::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            m_include_layers = Some(BitField::parse(r, ver)?);
            m_exclude_layers = Some(BitField::parse(r, ver)?);
            m_layer_override_priority = Some(r.read_i32()?);
            m_enabled = Some(WheelColliderMEnabledVariant::parse(r, ver)?);
            m_provides_contacts = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_enabled = Some(WheelColliderMEnabledVariant::parse(r, ver)?);
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
