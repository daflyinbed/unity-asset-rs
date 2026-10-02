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

impl crate::ToValue for ChildMotionMIsAnimVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V4_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V4_1(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for ChildMotion {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Motion".to_string(), self.m_motion.to_value())),
            Some(("m_Threshold".to_string(), self.m_threshold.to_value())),
            Some(("m_TimeScale".to_string(), self.m_time_scale.to_value())),
            crate::opt_field("m_Position", &self.m_position),
            crate::opt_field("m_IsAnim", &self.m_is_anim),
            crate::opt_field("m_CycleOffset", &self.m_cycle_offset),
            crate::opt_field("m_DirectBlendParameter", &self.m_direct_blend_parameter),
            crate::opt_field("m_Mirror", &self.m_mirror),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Motion" => (self.m_motion).inspect_value(path, offset, limit),
                "m_Threshold" => (self.m_threshold).inspect_value(path, offset, limit),
                "m_TimeScale" => (self.m_time_scale).inspect_value(path, offset, limit),
                "m_Position" => (self.m_position).inspect_value(path, offset, limit),
                "m_IsAnim" => (self.m_is_anim).inspect_value(path, offset, limit),
                "m_CycleOffset" => (self.m_cycle_offset).inspect_value(path, offset, limit),
                "m_DirectBlendParameter" => {
                    (self.m_direct_blend_parameter).inspect_value(path, offset, limit)
                }
                "m_Mirror" => (self.m_mirror).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_motion).inspect_value(path, offset, limit))
                .map(|v| ("m_Motion".to_string(), v)),
            ((self.m_threshold).inspect_value(path, offset, limit))
                .map(|v| ("m_Threshold".to_string(), v)),
            ((self.m_time_scale).inspect_value(path, offset, limit))
                .map(|v| ("m_TimeScale".to_string(), v)),
            ((self.m_position).inspect_value(path, offset, limit))
                .map(|v| ("m_Position".to_string(), v)),
            ((self.m_is_anim).inspect_value(path, offset, limit))
                .map(|v| ("m_IsAnim".to_string(), v)),
            ((self.m_cycle_offset).inspect_value(path, offset, limit))
                .map(|v| ("m_CycleOffset".to_string(), v)),
            ((self.m_direct_blend_parameter).inspect_value(path, offset, limit))
                .map(|v| ("m_DirectBlendParameter".to_string(), v)),
            ((self.m_mirror).inspect_value(path, offset, limit))
                .map(|v| ("m_Mirror".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
