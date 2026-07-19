//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vector2f::Vector2f;

/// 版本枚举：ChildMotionMIsAnimVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ChildMotionMIsAnimVariant {
    Default(bool),
    V4_1(bool),
}

impl crate::Parse for ChildMotionMIsAnimVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(4, 1, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V4_1(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct ChildMotion {
    pub m_motion: PPtr,
    pub m_threshold: f32,
    pub m_time_scale: f32,
    pub m_position: Option<Vector2f>,
    pub m_is_anim: Option<ChildMotionMIsAnimVariant>,
    pub m_cycle_offset: Option<f32>,
    pub m_direct_blend_parameter: Option<String>,
    pub m_mirror: Option<bool>,
}

impl crate::Parse for ChildMotion {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_motion = None;
        let mut m_threshold = None;
        let mut m_time_scale = None;
        let mut m_position = None;
        let mut m_is_anim = None;
        let mut m_cycle_offset = None;
        let mut m_direct_blend_parameter = None;
        let mut m_mirror = None;
        m_motion = Some(crate::PPtr::parse(rdr, ver)?);
        m_threshold = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(4, 1, 0) {
            m_position = Some(Vector2f::parse(rdr, ver)?);
        } else {
        }
        m_time_scale = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_cycle_offset = Some(rdr.read_f32()?);
            m_direct_blend_parameter = Some(rdr.read_aligned_string()?);
            m_mirror = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(4, 1, 0) {
            m_cycle_offset = Some(rdr.read_f32()?);
            m_is_anim = Some(ChildMotionMIsAnimVariant::parse(rdr, ver)?);
            m_mirror = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
            m_is_anim = Some(ChildMotionMIsAnimVariant::parse(rdr, ver)?);
        }

        Ok(ChildMotion {
            m_motion: m_motion.unwrap(),
            m_threshold: m_threshold.unwrap(),
            m_time_scale: m_time_scale.unwrap(),
            m_position,
            m_is_anim,
            m_cycle_offset,
            m_direct_blend_parameter,
            m_mirror,
        })
    }
}
