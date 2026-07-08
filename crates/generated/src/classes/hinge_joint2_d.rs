//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::joint_angle_limit2_d::JointAngleLimit2D;
use crate::subclasses::joint_angle_limits2_d::JointAngleLimits2D;
use crate::subclasses::joint_motor2_d::JointMotor2D;
use crate::subclasses::vector2f::Vector2f;


/// 版本枚举：HingeJoint2DMAngleLimitsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum HingeJoint2DMAngleLimitsVariant {
    Default(JointAngleLimit2D),
    V5_0(JointAngleLimits2D),
}

impl crate::Parse for HingeJoint2DMAngleLimitsVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = JointAngleLimits2D::parse(r, ver)?;
            Ok(Self::V5_0(v))
        } else {
            let v = JointAngleLimit2D::parse(r, ver)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct HingeJoint2D {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_collide_connected: Option<bool>,
    pub m_enable_collision: Option<bool>,
    pub m_connected_rigid_body: PPtr,
    pub m_anchor: Vector2f,
    pub m_break_force: Option<f32>,
    pub m_connected_anchor: Vector2f,
    pub m_break_torque: Option<f32>,
    pub m_use_motor: bool,
    pub m_auto_configure_connected_anchor: Option<bool>,
    pub m_break_action: Option<i32>,
    pub m_motor: JointMotor2D,
    pub m_use_limits: bool,
    pub m_angle_limits: HingeJoint2DMAngleLimitsVariant,
    pub m_use_connected_anchor: Option<bool>,
}

impl crate::Parse for HingeJoint2D {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
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
        let mut m_use_motor = None;
        let mut m_auto_configure_connected_anchor = None;
        let mut m_break_action = None;
        let mut m_motor = None;
        let mut m_use_limits = None;
        let mut m_angle_limits = None;
        let mut m_use_connected_anchor = None;
            m_game_object = Some(crate::PPtr::parse(r, ver)?);
            m_enabled = Some(r.read_u8()?);
            r.align(4)?;
        if *ver >= crate::UnityVersion::new(5, 0, 1) {
            m_enable_collision = Some(r.read_bool()?);
            r.align(4)?;
        } else {
            m_collide_connected = Some(r.read_bool()?);
            r.align(4)?;
        }
            m_connected_rigid_body = Some(crate::PPtr::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            m_break_force = Some(r.read_f32()?);
            m_break_torque = Some(r.read_f32()?);
            m_break_action = Some(r.read_i32()?);
            m_auto_configure_connected_anchor = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 3, 0) {
            m_break_force = Some(r.read_f32()?);
            m_break_torque = Some(r.read_f32()?);
            m_auto_configure_connected_anchor = Some(r.read_bool()?);
            r.align(4)?;
        } else {
        }
            m_anchor = Some(Vector2f::parse(r, ver)?);
            m_connected_anchor = Some(Vector2f::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            m_use_connected_anchor = Some(r.read_bool()?);
        } else {
        }
            m_use_motor = Some(r.read_bool()?);
            r.align(4)?;
            m_motor = Some(JointMotor2D::parse(r, ver)?);
            m_use_limits = Some(r.read_bool()?);
            r.align(4)?;
            m_angle_limits = Some(HingeJoint2DMAngleLimitsVariant::parse(r, ver)?);

        Ok(HingeJoint2D {
            m_game_object: m_game_object.unwrap(),
            m_enabled: m_enabled.unwrap(),
            m_collide_connected,
            m_enable_collision,
            m_connected_rigid_body: m_connected_rigid_body.unwrap(),
            m_anchor: m_anchor.unwrap(),
            m_break_force,
            m_connected_anchor: m_connected_anchor.unwrap(),
            m_break_torque,
            m_use_motor: m_use_motor.unwrap(),
            m_auto_configure_connected_anchor,
            m_break_action,
            m_motor: m_motor.unwrap(),
            m_use_limits: m_use_limits.unwrap(),
            m_angle_limits: m_angle_limits.unwrap(),
            m_use_connected_anchor,
        })
    }
}
