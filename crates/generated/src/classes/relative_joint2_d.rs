//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vector2f::Vector2f;
use super::joint2_d::Joint2D;

#[derive(Debug, Clone)]
pub struct RelativeJoint2D {
    pub parent: Joint2D,
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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = Joint2D::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_u8()?;
        r.align(4)?;
        let m_enable_collision = r.read_bool()?;
        r.align(4)?;
        let m_connected_rigid_body = crate::PPtr::parse(r, ver)?;
        let m_break_force = r.read_f32()?;
        let m_break_torque = r.read_f32()?;
        let m_max_force = r.read_f32()?;
        let m_break_action = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_max_torque = r.read_f32()?;
        let m_correction_scale = r.read_f32()?;
        let m_auto_configure_offset = r.read_bool()?;
        r.align(4)?;
        let m_linear_offset = Vector2f::parse(r, ver)?;
        let m_angular_offset = r.read_f32()?;

        Ok(RelativeJoint2D {
            parent,
            m_game_object,
            m_enabled,
            m_enable_collision,
            m_connected_rigid_body,
            m_break_force,
            m_break_torque,
            m_max_force,
            m_break_action,
            m_max_torque,
            m_correction_scale,
            m_auto_configure_offset,
            m_linear_offset,
            m_angular_offset,
        })
    }
}
