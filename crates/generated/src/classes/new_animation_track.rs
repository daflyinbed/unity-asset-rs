//! 自动生成。请勿手动修改。

use crate::subclasses::channel::Channel;

/// 版本枚举：NewAnimationTrackMCurvesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum NewAnimationTrackMCurvesVariant {
    Default(Vec<Channel>),
    V2017_1(Vec<Channel>),
}

impl crate::Parse for NewAnimationTrackMCurvesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| Channel::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| Channel::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for NewAnimationTrackMCurvesVariant {
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
#[derive(Debug, Clone)]
pub struct NewAnimationTrack {
    pub m_name: String,
    pub m_curves: NewAnimationTrackMCurvesVariant,
    pub m_class_id: i32,
}

impl crate::Parse for NewAnimationTrack {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        let m_curves = NewAnimationTrackMCurvesVariant::parse(rdr, ver)?;
        let m_class_id = rdr.read_i32()?;

        Ok(NewAnimationTrack {
            m_name,
            m_curves,
            m_class_id,
        })
    }
}

impl crate::ToValue for NewAnimationTrack {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some(("m_Curves".to_string(), self.m_curves.to_value())),
            Some(("m_ClassID".to_string(), self.m_class_id.to_value())),
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
                "m_Curves" => (self.m_curves).inspect_value(path, offset, limit),
                "m_ClassID" => (self.m_class_id).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_curves).inspect_value(path, offset, limit))
                .map(|v| ("m_Curves".to_string(), v)),
            ((self.m_class_id).inspect_value(path, offset, limit))
                .map(|v| ("m_ClassID".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
