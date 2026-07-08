//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vector2f::Vector2f;

#[derive(Debug, Clone)]
pub struct RelativeJoint2D {
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
        let mut m_game_object = None;
        let mut m_enabled = None;
        let mut m_enable_collision = None;
        let mut m_connected_rigid_body = None;
        let mut m_break_force = None;
        let mut m_break_torque = None;
        let mut m_max_force = None;
        let mut m_break_action = None;
        let mut m_max_torque = None;
        let mut m_correction_scale = None;
        let mut m_auto_configure_offset = None;
        let mut m_linear_offset = None;
        let mut m_angular_offset = None;
            m_game_object = Some(crate::PPtr::parse(r, ver)?);
            m_enabled = Some(r.read_u8()?);
            r.align(4)?;
            m_enable_collision = Some(r.read_bool()?);
            r.align(4)?;
            m_connected_rigid_body = Some(crate::PPtr::parse(r, ver)?);
            m_break_force = Some(r.read_f32()?);
            m_break_torque = Some(r.read_f32()?);
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            m_break_action = Some(r.read_i32()?);
        } else {
        }
            m_max_force = Some(r.read_f32()?);
            m_max_torque = Some(r.read_f32()?);
            m_correction_scale = Some(r.read_f32()?);
            m_auto_configure_offset = Some(r.read_bool()?);
            r.align(4)?;
            m_linear_offset = Some(Vector2f::parse(r, ver)?);
            m_angular_offset = Some(r.read_f32()?);

        Ok(RelativeJoint2D {
            m_game_object: m_game_object.unwrap(),
            m_enabled: m_enabled.unwrap(),
            m_enable_collision: m_enable_collision.unwrap(),
            m_connected_rigid_body: m_connected_rigid_body.unwrap(),
            m_break_force: m_break_force.unwrap(),
            m_break_torque: m_break_torque.unwrap(),
            m_max_force: m_max_force.unwrap(),
            m_break_action,
            m_max_torque: m_max_torque.unwrap(),
            m_correction_scale: m_correction_scale.unwrap(),
            m_auto_configure_offset: m_auto_configure_offset.unwrap(),
            m_linear_offset: m_linear_offset.unwrap(),
            m_angular_offset: m_angular_offset.unwrap(),
        })
    }
}
