//! 自动生成。请勿手动修改。

use crate::subclasses::child_motion::ChildMotion;


/// 版本枚举：BlendTreeMChildsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum BlendTreeMChildsVariant {
    Default(Vec<ChildMotion>),
    V2017_1(Vec<ChildMotion>),
}

impl crate::Parse for BlendTreeMChildsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ChildMotion::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ChildMotion::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：BlendTreeMUseAutomaticThresholdsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum BlendTreeMUseAutomaticThresholdsVariant {
    Default(bool),
    V5_0(bool),
}

impl crate::Parse for BlendTreeMUseAutomaticThresholdsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V5_0(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：BlendTreeMBlendTypeVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum BlendTreeMBlendTypeVariant {
    Default(u32),
    V4_2(i32),
}

impl crate::Parse for BlendTreeMBlendTypeVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(4, 2, 0) {
            let v = rdr.read_i32()?;
            Ok(Self::V4_2(v))
        } else {
            let v = rdr.read_u32()?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct BlendTree {
    pub m_name: String,
    pub m_childs: BlendTreeMChildsVariant,
    pub m_blend_parameter: String,
    pub m_min_threshold: f32,
    pub m_blend_parameter_y: Option<String>,
    pub m_max_threshold: f32,
    pub m_use_automatic_thresholds: BlendTreeMUseAutomaticThresholdsVariant,
    pub m_blend_type: Option<BlendTreeMBlendTypeVariant>,
    pub m_normalized_blend_values: Option<bool>,
}

impl crate::Parse for BlendTree {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut m_childs = None;
        let mut m_blend_parameter = None;
        let mut m_min_threshold = None;
        let mut m_blend_parameter_y = None;
        let mut m_max_threshold = None;
        let mut m_use_automatic_thresholds = None;
        let mut m_blend_type = None;
        let mut m_normalized_blend_values = None;
            m_name = Some(rdr.read_aligned_string()?);
            m_childs = Some(BlendTreeMChildsVariant::parse(rdr, ver)?);
            m_blend_parameter = Some(rdr.read_aligned_string()?);
        if *ver >= crate::UnityVersion::new(4, 1, 0) {
            m_blend_parameter_y = Some(rdr.read_aligned_string()?);
        } else {
        }
            m_min_threshold = Some(rdr.read_f32()?);
            m_max_threshold = Some(rdr.read_f32()?);
            m_use_automatic_thresholds = Some(BlendTreeMUseAutomaticThresholdsVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_normalized_blend_values = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_blend_type = Some(BlendTreeMBlendTypeVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 1, 0) {
            m_blend_type = Some(BlendTreeMBlendTypeVariant::parse(rdr, ver)?);
        } else {
        }

        Ok(BlendTree {
            m_name: m_name.unwrap(),
            m_childs: m_childs.unwrap(),
            m_blend_parameter: m_blend_parameter.unwrap(),
            m_min_threshold: m_min_threshold.unwrap(),
            m_blend_parameter_y,
            m_max_threshold: m_max_threshold.unwrap(),
            m_use_automatic_thresholds: m_use_automatic_thresholds.unwrap(),
            m_blend_type,
            m_normalized_blend_values,
        })
    }
}
