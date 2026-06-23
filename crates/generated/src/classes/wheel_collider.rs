//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::bit_field::BitField;
use crate::subclasses::joint_spring::JointSpring;
use crate::subclasses::vector3f::Vector3f;
use crate::subclasses::wheel_friction_curve::WheelFrictionCurve;
use super::collider::Collider;

#[derive(Debug, Clone)]
pub struct WheelCollider {
    pub parent: Collider,
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
    pub m_enabled: Option<bool>,
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
        let parent = Collider::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_center = Vector3f::parse(r, ver)?;
        let m_radius = r.read_f32()?;
        let m_suspension_distance = r.read_f32()?;
        let m_suspension_spring = JointSpring::parse(r, ver)?;
        let m_mass = r.read_f32()?;
        let m_force_app_point_distance = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_forward_friction = WheelFrictionCurve::parse(r, ver)?;
        let m_sideways_friction = WheelFrictionCurve::parse(r, ver)?;
        let m_wheel_damping_rate = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_enabled = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_include_layers = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(BitField::parse(r, ver)?)
        } else {
            None
        };
        let m_exclude_layers = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(BitField::parse(r, ver)?)
        } else {
            None
        };
        let m_layer_override_priority = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_provides_contacts = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(WheelCollider {
            parent,
            m_game_object,
            m_center,
            m_radius,
            m_suspension_distance,
            m_suspension_spring,
            m_mass,
            m_force_app_point_distance,
            m_forward_friction,
            m_sideways_friction,
            m_wheel_damping_rate,
            m_enabled,
            m_include_layers,
            m_exclude_layers,
            m_layer_override_priority,
            m_provides_contacts,
        })
    }
}
