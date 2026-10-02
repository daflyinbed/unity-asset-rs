//! 自动生成。请勿手动修改。

use crate::subclasses::animation_curve::AnimationCurve;

#[derive(Debug, Clone)]
pub struct Channel {
    pub byte_offset: i32,
    pub curve: AnimationCurve,
    pub attribute_name: String,
}

impl crate::Parse for Channel {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let byte_offset = rdr.read_i32()?;
        let curve = AnimationCurve::parse(rdr, ver)?;
        let attribute_name = rdr.read_aligned_string()?;

        Ok(Channel {
            byte_offset,
            curve,
            attribute_name,
        })
    }
}

impl crate::ToValue for Channel {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("byteOffset".to_string(), self.byte_offset.to_value())),
            Some(("curve".to_string(), self.curve.to_value())),
            Some(("attributeName".to_string(), self.attribute_name.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "byteOffset" => (self.byte_offset).inspect_value(path, offset, limit),
                "curve" => (self.curve).inspect_value(path, offset, limit),
                "attributeName" => (self.attribute_name).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.byte_offset).inspect_value(path, offset, limit))
                .map(|v| ("byteOffset".to_string(), v)),
            ((self.curve).inspect_value(path, offset, limit)).map(|v| ("curve".to_string(), v)),
            ((self.attribute_name).inspect_value(path, offset, limit))
                .map(|v| ("attributeName".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
