//! 自动生成。请勿手动修改。

use crate::subclasses::animation_curve::AnimationCurve;

#[derive(Debug, Clone)]
pub struct ClipAnimationInfoCurve {
    pub name: String,
    pub curve: AnimationCurve,
}

impl crate::Parse for ClipAnimationInfoCurve {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let name = rdr.read_aligned_string()?;
        let curve = AnimationCurve::parse(rdr, ver)?;

        Ok(ClipAnimationInfoCurve { name, curve })
    }
}

impl crate::ToValue for ClipAnimationInfoCurve {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("name".to_string(), self.name.to_value())),
            Some(("curve".to_string(), self.curve.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "name" => (self.name).inspect_value(path, offset, limit),
                "curve" => (self.curve).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.name).inspect_value(path, offset, limit)).map(|v| ("name".to_string(), v)),
            ((self.curve).inspect_value(path, offset, limit)).map(|v| ("curve".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
