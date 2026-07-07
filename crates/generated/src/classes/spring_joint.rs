//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::classes::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct SpringJoint {
    pub m_game_object: PPtr,
    pub m_connected_body: PPtr,
    pub m_anchor: Vector3f,
    pub m_connected_articulation_body: Option<PPtr>,
    pub m_spring: f32,
    pub m_auto_configure_connected_anchor: Option<bool>,
    pub m_damper: f32,
    pub m_connected_anchor: Option<Vector3f>,
    pub m_axis: Option<Vector3f>,
    pub m_min_distance: f32,
    pub m_max_distance: f32,
    pub m_break_force: f32,
    pub m_break_torque: f32,
    pub m_tolerance: Option<f32>,
    pub m_enable_collision: Option<bool>,
    pub m_enable_preprocessing: Option<bool>,
    pub m_mass_scale: Option<f32>,
    pub m_connected_mass_scale: Option<f32>,
    pub m_enabled: Option<bool>,
}

impl crate::Parse for SpringJoint {
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
        let m_spring = r.read_f32()?;
        let m_auto_configure_connected_anchor = if *ver >= crate::UnityVersion::new(4, 3, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_damper = r.read_f32()?;
        let m_connected_anchor = if *ver >= crate::UnityVersion::new(4, 3, 0) {
            Some(Vector3f::parse(r, ver)?)
        } else {
            None
        };
        let m_axis = if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            Some(Vector3f::parse(r, ver)?)
        } else {
            None
        };
        let m_min_distance = r.read_f32()?;
        let m_max_distance = r.read_f32()?;
        let m_break_force = r.read_f32()?;
        let m_break_torque = r.read_f32()?;
        let m_tolerance = if *ver >= crate::UnityVersion::new(5, 2, 3) {
            Some(r.read_f32()?)
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

        Ok(SpringJoint {
            m_game_object,
            m_connected_body,
            m_anchor,
            m_connected_articulation_body,
            m_spring,
            m_auto_configure_connected_anchor,
            m_damper,
            m_connected_anchor,
            m_axis,
            m_min_distance,
            m_max_distance,
            m_break_force,
            m_break_torque,
            m_tolerance,
            m_enable_collision,
            m_enable_preprocessing,
            m_mass_scale,
            m_connected_mass_scale,
            m_enabled,
        })
    }
}
