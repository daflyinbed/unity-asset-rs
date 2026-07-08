//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vector2f::Vector2f;

#[derive(Debug, Clone)]
pub struct DistanceJoint2D {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_collide_connected: Option<bool>,
    pub m_enable_collision: Option<bool>,
    pub m_connected_rigid_body: PPtr,
    pub m_anchor: Vector2f,
    pub m_break_force: Option<f32>,
    pub m_connected_anchor: Vector2f,
    pub m_break_torque: Option<f32>,
    pub m_distance: f32,
    pub m_auto_configure_connected_anchor: Option<bool>,
    pub m_break_action: Option<i32>,
    pub m_max_distance_only: Option<bool>,
    pub m_auto_configure_distance: Option<bool>,
}

impl crate::Parse for DistanceJoint2D {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
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
        let mut m_distance = None;
        let mut m_auto_configure_connected_anchor = None;
        let mut m_break_action = None;
        let mut m_max_distance_only = None;
        let mut m_auto_configure_distance = None;
            m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
            m_enabled = Some(rdr.read_u8()?);
            rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(5, 0, 1) {
            m_enable_collision = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
            m_collide_connected = Some(rdr.read_bool()?);
            rdr.align(4)?;
        }
            m_connected_rigid_body = Some(crate::PPtr::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            m_break_force = Some(rdr.read_f32()?);
            m_break_torque = Some(rdr.read_f32()?);
            m_break_action = Some(rdr.read_i32()?);
            m_auto_configure_connected_anchor = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 3, 0) {
            m_break_force = Some(rdr.read_f32()?);
            m_break_torque = Some(rdr.read_f32()?);
            m_auto_configure_connected_anchor = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
        }
            m_anchor = Some(Vector2f::parse(rdr, ver)?);
            m_connected_anchor = Some(Vector2f::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 3, 0) {
            m_auto_configure_distance = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
        }
            m_distance = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(4, 5, 0) {
            m_max_distance_only = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
        }

        Ok(DistanceJoint2D {
            m_game_object: m_game_object.unwrap(),
            m_enabled: m_enabled.unwrap(),
            m_collide_connected,
            m_enable_collision,
            m_connected_rigid_body: m_connected_rigid_body.unwrap(),
            m_anchor: m_anchor.unwrap(),
            m_break_force,
            m_connected_anchor: m_connected_anchor.unwrap(),
            m_break_torque,
            m_distance: m_distance.unwrap(),
            m_auto_configure_connected_anchor,
            m_break_action,
            m_max_distance_only,
            m_auto_configure_distance,
        })
    }
}
