//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vector2f::Vector2f;
use super::anchored_joint2_d::AnchoredJoint2D;

#[derive(Debug, Clone)]
pub struct FrictionJoint2D {
    pub parent: AnchoredJoint2D,
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_enable_collision: bool,
    pub m_connected_rigid_body: PPtr,
    pub m_break_force: f32,
    pub m_break_torque: f32,
    pub m_auto_configure_connected_anchor: bool,
    pub m_break_action: Option<i32>,
    pub m_anchor: Vector2f,
    pub m_connected_anchor: Vector2f,
    pub m_max_force: f32,
    pub m_max_torque: f32,
}

impl crate::Parse for FrictionJoint2D {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = AnchoredJoint2D::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_u8()?;
        r.align(4)?;
        let m_enable_collision = r.read_bool()?;
        r.align(4)?;
        let m_connected_rigid_body = crate::PPtr::parse(r, ver)?;
        let m_break_force = r.read_f32()?;
        let m_break_torque = r.read_f32()?;
        let m_auto_configure_connected_anchor = r.read_bool()?;
        r.align(4)?;
        let m_break_action = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_anchor = Vector2f::parse(r, ver)?;
        let m_connected_anchor = Vector2f::parse(r, ver)?;
        let m_max_force = r.read_f32()?;
        let m_max_torque = r.read_f32()?;

        Ok(FrictionJoint2D {
            parent,
            m_game_object,
            m_enabled,
            m_enable_collision,
            m_connected_rigid_body,
            m_break_force,
            m_break_torque,
            m_auto_configure_connected_anchor,
            m_break_action,
            m_anchor,
            m_connected_anchor,
            m_max_force,
            m_max_torque,
        })
    }
}
