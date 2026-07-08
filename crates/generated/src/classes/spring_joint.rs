//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vector3f::Vector3f;


/// 版本枚举：SpringJointMEnablePreprocessingVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SpringJointMEnablePreprocessingVariant {
    Default(bool),
    V2017_1(bool),
}

impl crate::Parse for SpringJointMEnablePreprocessingVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = r.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}
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
    pub m_enable_preprocessing: Option<SpringJointMEnablePreprocessingVariant>,
    pub m_mass_scale: Option<f32>,
    pub m_connected_mass_scale: Option<f32>,
}

impl crate::Parse for SpringJoint {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut m_connected_body = None;
        let mut m_anchor = None;
        let mut m_connected_articulation_body = None;
        let mut m_spring = None;
        let mut m_auto_configure_connected_anchor = None;
        let mut m_damper = None;
        let mut m_connected_anchor = None;
        let mut m_axis = None;
        let mut m_min_distance = None;
        let mut m_max_distance = None;
        let mut m_break_force = None;
        let mut m_break_torque = None;
        let mut m_tolerance = None;
        let mut m_enable_collision = None;
        let mut m_enable_preprocessing = None;
        let mut m_mass_scale = None;
        let mut m_connected_mass_scale = None;
            m_game_object = Some(crate::PPtr::parse(r, ver)?);
            m_connected_body = Some(crate::PPtr::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            m_connected_articulation_body = Some(crate::PPtr::parse(r, ver)?);
        } else {
        }
            m_anchor = Some(Vector3f::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(6000, 0, 48) {
            m_auto_configure_connected_anchor = Some(r.read_bool()?);
            r.align(4)?;
            m_connected_anchor = Some(Vector3f::parse(r, ver)?);
        } else if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            m_axis = Some(Vector3f::parse(r, ver)?);
            m_auto_configure_connected_anchor = Some(r.read_bool()?);
            r.align(4)?;
            m_connected_anchor = Some(Vector3f::parse(r, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_auto_configure_connected_anchor = Some(r.read_bool()?);
            r.align(4)?;
            m_connected_anchor = Some(Vector3f::parse(r, ver)?);
        } else {
        }
            m_spring = Some(r.read_f32()?);
            m_damper = Some(r.read_f32()?);
            m_min_distance = Some(r.read_f32()?);
            m_max_distance = Some(r.read_f32()?);
        if *ver >= crate::UnityVersion::new(5, 2, 3) {
            m_tolerance = Some(r.read_f32()?);
        } else {
        }
            m_break_force = Some(r.read_f32()?);
            m_break_torque = Some(r.read_f32()?);
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_enable_collision = Some(r.read_bool()?);
            m_enable_preprocessing = Some(SpringJointMEnablePreprocessingVariant::parse(r, ver)?);
            m_mass_scale = Some(r.read_f32()?);
            m_connected_mass_scale = Some(r.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_enable_collision = Some(r.read_bool()?);
            m_enable_preprocessing = Some(SpringJointMEnablePreprocessingVariant::parse(r, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 5, 0) {
            m_enable_collision = Some(r.read_bool()?);
        } else {
        }

        Ok(SpringJoint {
            m_game_object: m_game_object.unwrap(),
            m_connected_body: m_connected_body.unwrap(),
            m_anchor: m_anchor.unwrap(),
            m_connected_articulation_body,
            m_spring: m_spring.unwrap(),
            m_auto_configure_connected_anchor,
            m_damper: m_damper.unwrap(),
            m_connected_anchor,
            m_axis,
            m_min_distance: m_min_distance.unwrap(),
            m_max_distance: m_max_distance.unwrap(),
            m_break_force: m_break_force.unwrap(),
            m_break_torque: m_break_torque.unwrap(),
            m_tolerance,
            m_enable_collision,
            m_enable_preprocessing,
            m_mass_scale,
            m_connected_mass_scale,
        })
    }
}
