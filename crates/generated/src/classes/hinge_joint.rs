//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::joint_limits::JointLimits;
use crate::subclasses::joint_motor::JointMotor;
use crate::subclasses::joint_spring::JointSpring;
use crate::subclasses::vector3f::Vector3f;

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
    pub m_use_limits: bool,
    pub m_limits: JointLimits,
    pub m_break_force: f32,
    pub m_break_torque: f32,
    pub m_extended_limits: Option<bool>,
    pub m_use_acceleration: Option<bool>,
    pub m_enable_collision: Option<bool>,
    pub m_enable_preprocessing: Option<bool>,
    pub m_mass_scale: Option<f32>,
    pub m_connected_mass_scale: Option<f32>,
    pub m_enabled: Option<bool>,
}

impl crate::Parse for HingeJoint {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_connected_body = crate::PPtr::parse(r, ver)?;
        let m_anchor = Vector3f::parse(r, ver)?;
        let m_connected_articulation_body = if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            Some(crate::PPtr::parse(r, ver)?)
        } else {
            None
        };
        let m_axis = Vector3f::parse(r, ver)?;
        let m_use_spring = r.read_bool()?;
        r.align(4)?;
        let m_auto_configure_connected_anchor = if *ver >= crate::UnityVersion::new(4, 3, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_spring = JointSpring::parse(r, ver)?;
        let m_connected_anchor = if *ver >= crate::UnityVersion::new(4, 3, 0) {
            Some(Vector3f::parse(r, ver)?)
        } else {
            None
        };
        let m_use_motor = r.read_bool()?;
        r.align(4)?;
        let m_motor = JointMotor::parse(r, ver)?;
        let m_use_limits = r.read_bool()?;
        let m_limits = JointLimits::parse(r, ver)?;
        let m_break_force = r.read_f32()?;
        let m_break_torque = r.read_f32()?;
        let m_extended_limits = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_use_acceleration = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_enable_collision = if *ver >= crate::UnityVersion::new(4, 5, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_enable_preprocessing = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_mass_scale = if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_connected_mass_scale = if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_enabled = if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };

        Ok(HingeJoint {
            m_game_object,
            m_connected_body,
            m_anchor,
            m_connected_articulation_body,
            m_axis,
            m_use_spring,
            m_auto_configure_connected_anchor,
            m_spring,
            m_connected_anchor,
            m_use_motor,
            m_motor,
            m_use_limits,
            m_limits,
            m_break_force,
            m_break_torque,
            m_extended_limits,
            m_use_acceleration,
            m_enable_collision,
            m_enable_preprocessing,
            m_mass_scale,
            m_connected_mass_scale,
            m_enabled,
        })
    }
}
