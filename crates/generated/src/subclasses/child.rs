//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vector2f::Vector2f;


/// 版本枚举：ChildMIsAnimVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ChildMIsAnimVariant {
    Default(bool),
    V4_1(bool),
}

impl crate::Parse for ChildMIsAnimVariant {
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
pub struct Child {
    pub m_motion: PPtr,
    pub m_threshold: f32,
    pub m_time_scale: f32,
    pub m_position: Option<Vector2f>,
    pub m_is_anim: ChildMIsAnimVariant,
    pub m_cycle_offset: Option<f32>,
    pub m_mirror: Option<bool>,
}

impl crate::Parse for Child {
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
        let mut m_mirror = None;
            m_motion = Some(crate::PPtr::parse(rdr, ver)?);
            m_threshold = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(4, 1, 0) {
            m_position = Some(Vector2f::parse(rdr, ver)?);
        } else {
        }
            m_time_scale = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(4, 1, 0) {
            m_cycle_offset = Some(rdr.read_f32()?);
        } else {
        }
            m_is_anim = Some(ChildMIsAnimVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(4, 1, 0) {
            m_mirror = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
        }

        Ok(Child {
            m_motion: m_motion.unwrap(),
            m_threshold: m_threshold.unwrap(),
            m_time_scale: m_time_scale.unwrap(),
            m_position,
            m_is_anim: m_is_anim.unwrap(),
            m_cycle_offset,
            m_mirror,
        })
    }
}
