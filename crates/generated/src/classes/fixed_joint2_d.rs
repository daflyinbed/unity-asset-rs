//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vector2f::Vector2f;

#[derive(Debug, Clone)]
pub struct FixedJoint2D {
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
    pub m_damping_ratio: f32,
    pub m_frequency: f32,
}

impl crate::Parse for FixedJoint2D {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut m_enabled = None;
        let mut m_enable_collision = None;
        let mut m_connected_rigid_body = None;
        let mut m_break_force = None;
        let mut m_break_torque = None;
        let mut m_auto_configure_connected_anchor = None;
        let mut m_break_action = None;
        let mut m_anchor = None;
        let mut m_connected_anchor = None;
        let mut m_damping_ratio = None;
        let mut m_frequency = None;
        m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
        m_enabled = Some(rdr.read_u8()?);
        rdr.align(4)?;
        m_enable_collision = Some(rdr.read_bool()?);
        rdr.align(4)?;
        m_connected_rigid_body = Some(crate::PPtr::parse(rdr, ver)?);
        m_break_force = Some(rdr.read_f32()?);
        m_break_torque = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            m_break_action = Some(rdr.read_i32()?);
        } else {
        }
        m_auto_configure_connected_anchor = Some(rdr.read_bool()?);
        rdr.align(4)?;
        m_anchor = Some(Vector2f::parse(rdr, ver)?);
        m_connected_anchor = Some(Vector2f::parse(rdr, ver)?);
        m_damping_ratio = Some(rdr.read_f32()?);
        m_frequency = Some(rdr.read_f32()?);

        Ok(FixedJoint2D {
            m_game_object: m_game_object.unwrap(),
            m_enabled: m_enabled.unwrap(),
            m_enable_collision: m_enable_collision.unwrap(),
            m_connected_rigid_body: m_connected_rigid_body.unwrap(),
            m_break_force: m_break_force.unwrap(),
            m_break_torque: m_break_torque.unwrap(),
            m_auto_configure_connected_anchor: m_auto_configure_connected_anchor.unwrap(),
            m_break_action,
            m_anchor: m_anchor.unwrap(),
            m_connected_anchor: m_connected_anchor.unwrap(),
            m_damping_ratio: m_damping_ratio.unwrap(),
            m_frequency: m_frequency.unwrap(),
        })
    }
}
