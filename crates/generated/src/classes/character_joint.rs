//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::soft_joint_limit::SoftJointLimit;
use crate::subclasses::soft_joint_limit_spring::SoftJointLimitSpring;
use crate::subclasses::vector3f::Vector3f;


/// 版本枚举：CharacterJointMEnablePreprocessingVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum CharacterJointMEnablePreprocessingVariant {
    Default(bool),
    V2017_1(bool),
}

impl crate::Parse for CharacterJointMEnablePreprocessingVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}
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
    pub m_enable_preprocessing: Option<CharacterJointMEnablePreprocessingVariant>,
    pub m_mass_scale: Option<f32>,
    pub m_connected_mass_scale: Option<f32>,
}

impl crate::Parse for CharacterJoint {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut m_connected_body = None;
        let mut m_anchor = None;
        let mut m_connected_articulation_body = None;
        let mut m_axis = None;
        let mut m_swing_axis = None;
        let mut m_auto_configure_connected_anchor = None;
        let mut m_low_twist_limit = None;
        let mut m_connected_anchor = None;
        let mut m_high_twist_limit = None;
        let mut m_swing1_limit = None;
        let mut m_twist_limit_spring = None;
        let mut m_swing2_limit = None;
        let mut m_break_force = None;
        let mut m_break_torque = None;
        let mut m_swing_limit_spring = None;
        let mut m_enable_collision = None;
        let mut m_enable_projection = None;
        let mut m_projection_distance = None;
        let mut m_projection_angle = None;
        let mut m_enable_preprocessing = None;
        let mut m_mass_scale = None;
        let mut m_connected_mass_scale = None;
            m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
            m_connected_body = Some(crate::PPtr::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            m_connected_articulation_body = Some(crate::PPtr::parse(rdr, ver)?);
        } else {
        }
            m_anchor = Some(Vector3f::parse(rdr, ver)?);
            m_axis = Some(Vector3f::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_auto_configure_connected_anchor = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_connected_anchor = Some(Vector3f::parse(rdr, ver)?);
        } else {
        }
            m_swing_axis = Some(Vector3f::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_twist_limit_spring = Some(SoftJointLimitSpring::parse(rdr, ver)?);
        } else {
        }
            m_low_twist_limit = Some(SoftJointLimit::parse(rdr, ver)?);
            m_high_twist_limit = Some(SoftJointLimit::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_swing_limit_spring = Some(SoftJointLimitSpring::parse(rdr, ver)?);
        } else {
        }
            m_swing1_limit = Some(SoftJointLimit::parse(rdr, ver)?);
            m_swing2_limit = Some(SoftJointLimit::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_enable_projection = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_projection_distance = Some(rdr.read_f32()?);
            m_projection_angle = Some(rdr.read_f32()?);
        } else {
        }
            m_break_force = Some(rdr.read_f32()?);
            m_break_torque = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_enable_collision = Some(rdr.read_bool()?);
            m_enable_preprocessing = Some(CharacterJointMEnablePreprocessingVariant::parse(rdr, ver)?);
            m_mass_scale = Some(rdr.read_f32()?);
            m_connected_mass_scale = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_enable_collision = Some(rdr.read_bool()?);
            m_enable_preprocessing = Some(CharacterJointMEnablePreprocessingVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 5, 0) {
            m_enable_collision = Some(rdr.read_bool()?);
        } else {
        }

        Ok(CharacterJoint {
            m_game_object: m_game_object.unwrap(),
            m_connected_body: m_connected_body.unwrap(),
            m_anchor: m_anchor.unwrap(),
            m_connected_articulation_body,
            m_axis: m_axis.unwrap(),
            m_swing_axis: m_swing_axis.unwrap(),
            m_auto_configure_connected_anchor,
            m_low_twist_limit: m_low_twist_limit.unwrap(),
            m_connected_anchor,
            m_high_twist_limit: m_high_twist_limit.unwrap(),
            m_swing1_limit: m_swing1_limit.unwrap(),
            m_twist_limit_spring,
            m_swing2_limit: m_swing2_limit.unwrap(),
            m_break_force: m_break_force.unwrap(),
            m_break_torque: m_break_torque.unwrap(),
            m_swing_limit_spring,
            m_enable_collision,
            m_enable_projection,
            m_projection_distance,
            m_projection_angle,
            m_enable_preprocessing,
            m_mass_scale,
            m_connected_mass_scale,
        })
    }
}
