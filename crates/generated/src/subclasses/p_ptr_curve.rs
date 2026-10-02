//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::p_ptr_keyframe::PPtrKeyframe;

/// 版本枚举：PPtrCurveCurveVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PPtrCurveCurveVariant {
    Default(Vec<PPtrKeyframe>),
    V2017_1(Vec<PPtrKeyframe>),
}

impl crate::Parse for PPtrCurveCurveVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| PPtrKeyframe::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| PPtrKeyframe::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for PPtrCurveCurveVariant {
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
pub struct PPtrCurve {
    pub curve: PPtrCurveCurveVariant,
    pub attribute: String,
    pub path: String,
    pub class_id: i32,
    pub script: PPtr,
    pub flags: Option<i32>,
}

impl crate::Parse for PPtrCurve {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let curve = PPtrCurveCurveVariant::parse(rdr, ver)?;
        let attribute = rdr.read_aligned_string()?;
        let path = rdr.read_aligned_string()?;
        let class_id = rdr.read_i32()?;
        let script = crate::PPtr::parse(rdr, ver)?;
        let flags = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(rdr.read_i32()?)
        } else {
            None
        };

        Ok(PPtrCurve {
            curve,
            attribute,
            path,
            class_id,
            script,
            flags,
        })
    }
}

impl crate::ToValue for PPtrCurve {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("curve".to_string(), self.curve.to_value())),
            Some(("attribute".to_string(), self.attribute.to_value())),
            Some(("path".to_string(), self.path.to_value())),
            Some(("classID".to_string(), self.class_id.to_value())),
            Some(("script".to_string(), self.script.to_value())),
            crate::opt_field("flags", &self.flags),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "curve" => (self.curve).inspect_value(path, offset, limit),
                "attribute" => (self.attribute).inspect_value(path, offset, limit),
                "path" => (self.path).inspect_value(path, offset, limit),
                "classID" => (self.class_id).inspect_value(path, offset, limit),
                "script" => (self.script).inspect_value(path, offset, limit),
                "flags" => (self.flags).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.curve).inspect_value(path, offset, limit)).map(|v| ("curve".to_string(), v)),
            ((self.attribute).inspect_value(path, offset, limit))
                .map(|v| ("attribute".to_string(), v)),
            ((self.path).inspect_value(path, offset, limit)).map(|v| ("path".to_string(), v)),
            ((self.class_id).inspect_value(path, offset, limit))
                .map(|v| ("classID".to_string(), v)),
            ((self.script).inspect_value(path, offset, limit)).map(|v| ("script".to_string(), v)),
            ((self.flags).inspect_value(path, offset, limit)).map(|v| ("flags".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
