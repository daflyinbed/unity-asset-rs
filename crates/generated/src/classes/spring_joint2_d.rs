//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vector2f::Vector2f;

#[derive(Debug, Clone)]
pub struct SpringJoint2D {
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
    pub m_damping_ratio: f32,
    pub m_frequency: f32,
    pub m_auto_configure_distance: Option<bool>,
}

impl crate::Parse for SpringJoint2D {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
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
        let m_distance = r.read_f32()?;
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
        let m_damping_ratio = r.read_f32()?;
        let m_frequency = r.read_f32()?;
        let m_auto_configure_distance = if *ver >= crate::UnityVersion::new(5, 3, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(SpringJoint2D {
            m_game_object,
            m_enabled,
            m_collide_connected,
            m_enable_collision,
            m_connected_rigid_body,
            m_anchor,
            m_break_force,
            m_connected_anchor,
            m_break_torque,
            m_distance,
            m_auto_configure_connected_anchor,
            m_break_action,
            m_damping_ratio,
            m_frequency,
            m_auto_configure_distance,
        })
    }
}
