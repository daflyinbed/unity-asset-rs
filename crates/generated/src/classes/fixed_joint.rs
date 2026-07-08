//! 自动生成。请勿手动修改。

use crate::PPtr;


/// 版本枚举：FixedJointMEnablePreprocessingVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum FixedJointMEnablePreprocessingVariant {
    Default(bool),
    V2017_1(bool),
}

impl crate::Parse for FixedJointMEnablePreprocessingVariant {
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
pub struct FixedJoint {
    pub m_game_object: PPtr,
    pub m_connected_body: PPtr,
    pub m_break_force: f32,
    pub m_connected_articulation_body: Option<PPtr>,
    pub m_break_torque: f32,
    pub m_enable_collision: Option<bool>,
    pub m_enable_preprocessing: Option<FixedJointMEnablePreprocessingVariant>,
    pub m_mass_scale: Option<f32>,
    pub m_connected_mass_scale: Option<f32>,
}

impl crate::Parse for FixedJoint {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut m_connected_body = None;
        let mut m_break_force = None;
        let mut m_connected_articulation_body = None;
        let mut m_break_torque = None;
        let mut m_enable_collision = None;
        let mut m_enable_preprocessing = None;
        let mut m_mass_scale = None;
        let mut m_connected_mass_scale = None;
            m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
            m_connected_body = Some(crate::PPtr::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            m_connected_articulation_body = Some(crate::PPtr::parse(rdr, ver)?);
        } else {
        }
            m_break_force = Some(rdr.read_f32()?);
            m_break_torque = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_enable_collision = Some(rdr.read_bool()?);
            m_enable_preprocessing = Some(FixedJointMEnablePreprocessingVariant::parse(rdr, ver)?);
            m_mass_scale = Some(rdr.read_f32()?);
            m_connected_mass_scale = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_enable_collision = Some(rdr.read_bool()?);
            m_enable_preprocessing = Some(FixedJointMEnablePreprocessingVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 5, 0) {
            m_enable_collision = Some(rdr.read_bool()?);
        } else {
        }

        Ok(FixedJoint {
            m_game_object: m_game_object.unwrap(),
            m_connected_body: m_connected_body.unwrap(),
            m_break_force: m_break_force.unwrap(),
            m_connected_articulation_body,
            m_break_torque: m_break_torque.unwrap(),
            m_enable_collision,
            m_enable_preprocessing,
            m_mass_scale,
            m_connected_mass_scale,
        })
    }
}
