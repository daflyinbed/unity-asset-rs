//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::joint_angle_limit2_d::JointAngleLimit2D;
use crate::subclasses::joint_motor2_d::JointMotor2D;
use crate::subclasses::vector2f::Vector2f;
use super::anchored_joint2_d::AnchoredJoint2D;
use super::joint2_d::Joint2D;

#[derive(Debug, Clone)]
pub enum HingeJoint2DParent {
    Joint2D(Joint2D),
    AnchoredJoint2D(AnchoredJoint2D),
}

#[derive(Debug, Clone)]
pub struct HingeJoint2D {
    pub parent: HingeJoint2DParent,
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
    pub m_angle_limits: JointAngleLimit2D,
    pub m_use_connected_anchor: Option<bool>,
}

impl crate::Parse for HingeJoint2D {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = if *ver >= crate::UnityVersion::new(4, 3, 0) {
            HingeJoint2DParent::Joint2D(Joint2D::parse(r, ver)?)
        } else if *ver >= crate::UnityVersion::new(4, 5, 0) {
            HingeJoint2DParent::AnchoredJoint2D(AnchoredJoint2D::parse(r, ver)?)
        } else {
            panic!("无匹配的父类分支：version = {:?}", ver)
        };
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_u8()?;
        r.align(4)?;
        let m_collide_connected = if *ver >= crate::UnityVersion::new(4, 3, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_enable_collision = if *ver >= crate::UnityVersion::new(5, 0, 1) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_connected_rigid_body = crate::PPtr::parse(r, ver)?;
        let m_anchor = Vector2f::parse(r, ver)?;
        let m_break_force = if *ver >= crate::UnityVersion::new(5, 3, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_connected_anchor = Vector2f::parse(r, ver)?;
        let m_break_torque = if *ver >= crate::UnityVersion::new(5, 3, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_use_motor = r.read_bool()?;
        r.align(4)?;
        let m_auto_configure_connected_anchor = if *ver >= crate::UnityVersion::new(5, 3, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_break_action = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_motor = JointMotor2D::parse(r, ver)?;
        let m_use_limits = r.read_bool()?;
        r.align(4)?;
        let m_angle_limits = JointAngleLimit2D::parse(r, ver)?;
        let m_use_connected_anchor = if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };

        Ok(HingeJoint2D {
            parent,
            m_game_object,
            m_enabled,
            m_collide_connected,
            m_enable_collision,
            m_connected_rigid_body,
            m_anchor,
            m_break_force,
            m_connected_anchor,
            m_break_torque,
            m_use_motor,
            m_auto_configure_connected_anchor,
            m_break_action,
            m_motor,
            m_use_limits,
            m_angle_limits,
            m_use_connected_anchor,
        })
    }
}
