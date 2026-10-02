//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct ConstraintSource {
    pub source_transform: PPtr,
    pub weight: f32,
}

impl crate::Parse for ConstraintSource {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let source_transform = crate::PPtr::parse(rdr, ver)?;
        let weight = rdr.read_f32()?;

        Ok(ConstraintSource {
            source_transform,
            weight,
        })
    }
}

impl crate::ToValue for ConstraintSource {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "sourceTransform".to_string(),
                self.source_transform.to_value(),
            )),
            Some(("weight".to_string(), self.weight.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "sourceTransform" => (self.source_transform).inspect_value(path, offset, limit),
                "weight" => (self.weight).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.source_transform).inspect_value(path, offset, limit))
                .map(|v| ("sourceTransform".to_string(), v)),
            ((self.weight).inspect_value(path, offset, limit)).map(|v| ("weight".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
