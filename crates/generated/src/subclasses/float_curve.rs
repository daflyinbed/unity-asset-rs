//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::animation_curve::AnimationCurve;

#[derive(Debug, Clone)]
pub struct FloatCurve {
    pub curve: AnimationCurve,
    pub attribute: String,
    pub path: String,
    pub class_id: i32,
    pub script: PPtr,
    pub flags: Option<i32>,
}

impl crate::Parse for FloatCurve {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let curve = AnimationCurve::parse(rdr, ver)?;
        let attribute = rdr.read_aligned_string()?;
        let path = rdr.read_aligned_string()?;
        let class_id = rdr.read_i32()?;
        let script = crate::PPtr::parse(rdr, ver)?;
        let flags = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(rdr.read_i32()?)
        } else {
            None
        };

        Ok(FloatCurve {
            curve,
            attribute,
            path,
            class_id,
            script,
            flags,
        })
    }
}

impl crate::ToValue for FloatCurve {
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
