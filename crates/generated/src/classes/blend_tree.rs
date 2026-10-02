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
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| ChildMotion::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| ChildMotion::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for BlendTreeMChildsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for BlendTreeMUseAutomaticThresholdsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_0(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_0(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for BlendTreeMBlendTypeVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V4_2(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V4_2(v) => (v).inspect_value(path, offset, limit),
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
        m_use_automatic_thresholds =
            Some(BlendTreeMUseAutomaticThresholdsVariant::parse(rdr, ver)?);
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

impl crate::ToValue for BlendTree {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some(("m_Childs".to_string(), self.m_childs.to_value())),
            Some((
                "m_BlendParameter".to_string(),
                self.m_blend_parameter.to_value(),
            )),
            Some((
                "m_MinThreshold".to_string(),
                self.m_min_threshold.to_value(),
            )),
            crate::opt_field("m_BlendParameterY", &self.m_blend_parameter_y),
            Some((
                "m_MaxThreshold".to_string(),
                self.m_max_threshold.to_value(),
            )),
            Some((
                "m_UseAutomaticThresholds".to_string(),
                self.m_use_automatic_thresholds.to_value(),
            )),
            crate::opt_field("m_BlendType", &self.m_blend_type),
            crate::opt_field("m_NormalizedBlendValues", &self.m_normalized_blend_values),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Name" => (self.m_name).inspect_value(path, offset, limit),
                "m_Childs" => (self.m_childs).inspect_value(path, offset, limit),
                "m_BlendParameter" => (self.m_blend_parameter).inspect_value(path, offset, limit),
                "m_MinThreshold" => (self.m_min_threshold).inspect_value(path, offset, limit),
                "m_BlendParameterY" => {
                    (self.m_blend_parameter_y).inspect_value(path, offset, limit)
                }
                "m_MaxThreshold" => (self.m_max_threshold).inspect_value(path, offset, limit),
                "m_UseAutomaticThresholds" => {
                    (self.m_use_automatic_thresholds).inspect_value(path, offset, limit)
                }
                "m_BlendType" => (self.m_blend_type).inspect_value(path, offset, limit),
                "m_NormalizedBlendValues" => {
                    (self.m_normalized_blend_values).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_childs).inspect_value(path, offset, limit))
                .map(|v| ("m_Childs".to_string(), v)),
            ((self.m_blend_parameter).inspect_value(path, offset, limit))
                .map(|v| ("m_BlendParameter".to_string(), v)),
            ((self.m_min_threshold).inspect_value(path, offset, limit))
                .map(|v| ("m_MinThreshold".to_string(), v)),
            ((self.m_blend_parameter_y).inspect_value(path, offset, limit))
                .map(|v| ("m_BlendParameterY".to_string(), v)),
            ((self.m_max_threshold).inspect_value(path, offset, limit))
                .map(|v| ("m_MaxThreshold".to_string(), v)),
            ((self.m_use_automatic_thresholds).inspect_value(path, offset, limit))
                .map(|v| ("m_UseAutomaticThresholds".to_string(), v)),
            ((self.m_blend_type).inspect_value(path, offset, limit))
                .map(|v| ("m_BlendType".to_string(), v)),
            ((self.m_normalized_blend_values).inspect_value(path, offset, limit))
                .map(|v| ("m_NormalizedBlendValues".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
