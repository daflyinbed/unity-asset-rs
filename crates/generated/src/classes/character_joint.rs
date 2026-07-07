//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::classes::vector3f::Vector3f;
use crate::subclasses::soft_joint_limit::SoftJointLimit;
use crate::subclasses::soft_joint_limit_spring::SoftJointLimitSpring;

#[derive(Debug, Clone)]
pub struct CharacterJoint {
    pub m_game_object: PPtr,
    pub m_connected_body: PPtr,
    pub m_anchor: Vector3f,
    pub m_connected_articulation_body: Option<PPtr>,
    pub m_axis: Vector3f,
    pub m_swing_axis: Vector3f,
    pub m_auto_configure_connected_anchor: Option<bool>,
    pub m_low_twist_limit: SoftJointLimit,
    pub m_connected_anchor: Option<Vector3f>,
    pub m_high_twist_limit: SoftJointLimit,
    pub m_swing1_limit: SoftJointLimit,
    pub m_twist_limit_spring: Option<SoftJointLimitSpring>,
    pub m_swing2_limit: SoftJointLimit,
    pub m_break_force: f32,
    pub m_break_torque: f32,
    pub m_swing_limit_spring: Option<SoftJointLimitSpring>,
    pub m_enable_collision: Option<bool>,
    pub m_enable_projection: Option<bool>,
    pub m_projection_distance: Option<f32>,
    pub m_projection_angle: Option<f32>,
    pub m_enable_preprocessing: Option<bool>,
    pub m_mass_scale: Option<f32>,
    pub m_connected_mass_scale: Option<f32>,
    pub m_enabled: Option<bool>,
}

impl crate::Parse for CharacterJoint {
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
        let m_swing_axis = Vector3f::parse(r, ver)?;
        let m_auto_configure_connected_anchor = if *ver >= crate::UnityVersion::new(4, 3, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_low_twist_limit = SoftJointLimit::parse(r, ver)?;
        let m_connected_anchor = if *ver >= crate::UnityVersion::new(4, 3, 0) {
            Some(Vector3f::parse(r, ver)?)
        } else {
            None
        };
        let m_high_twist_limit = SoftJointLimit::parse(r, ver)?;
        let m_swing1_limit = SoftJointLimit::parse(r, ver)?;
        let m_twist_limit_spring = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(SoftJointLimitSpring::parse(r, ver)?)
        } else {
            None
        };
        let m_swing2_limit = SoftJointLimit::parse(r, ver)?;
        let m_break_force = r.read_f32()?;
        let m_break_torque = r.read_f32()?;
        let m_swing_limit_spring = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(SoftJointLimitSpring::parse(r, ver)?)
        } else {
            None
        };
        let m_enable_collision = if *ver >= crate::UnityVersion::new(4, 5, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_enable_projection = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_projection_distance = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_projection_angle = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_f32()?)
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

        Ok(CharacterJoint {
            m_game_object,
            m_connected_body,
            m_anchor,
            m_connected_articulation_body,
            m_axis,
            m_swing_axis,
            m_auto_configure_connected_anchor,
            m_low_twist_limit,
            m_connected_anchor,
            m_high_twist_limit,
            m_swing1_limit,
            m_twist_limit_spring,
            m_swing2_limit,
            m_break_force,
            m_break_torque,
            m_swing_limit_spring,
            m_enable_collision,
            m_enable_projection,
            m_projection_distance,
            m_projection_angle,
            m_enable_preprocessing,
            m_mass_scale,
            m_connected_mass_scale,
            m_enabled,
        })
    }
}
