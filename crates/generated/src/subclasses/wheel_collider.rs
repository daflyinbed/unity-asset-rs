//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::joint_spring::JointSpring;
use crate::subclasses::vector3f::Vector3f;
use crate::subclasses::wheel_friction_curve::WheelFrictionCurve;

#[derive(Debug, Clone)]
pub struct WheelCollider {
    pub m_object_hide_flags: u32,
    pub m_extension_ptr: PPtr,
    pub m_game_object: PPtr,
    pub m_center: Vector3f,
    pub m_radius: f32,
    pub m_suspension_distance: f32,
    pub m_suspension_spring: JointSpring,
    pub m_mass: f32,
    pub m_forward_friction: WheelFrictionCurve,
    pub m_sideways_friction: WheelFrictionCurve,
}

impl crate::Parse for WheelCollider {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_extension_ptr = crate::PPtr::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_center = Vector3f::parse(r, ver)?;
        let m_radius = r.read_f32()?;
        let m_suspension_distance = r.read_f32()?;
        let m_suspension_spring = JointSpring::parse(r, ver)?;
        let m_mass = r.read_f32()?;
        let m_forward_friction = WheelFrictionCurve::parse(r, ver)?;
        let m_sideways_friction = WheelFrictionCurve::parse(r, ver)?;

        Ok(WheelCollider {
            m_object_hide_flags,
            m_extension_ptr,
            m_game_object,
            m_center,
            m_radius,
            m_suspension_distance,
            m_suspension_spring,
            m_mass,
            m_forward_friction,
            m_sideways_friction,
        })
    }
}
