//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct ClothSphereColliderPair {
    pub first: PPtr,
    pub second: PPtr,
}

impl crate::Parse for ClothSphereColliderPair {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let first = crate::PPtr::parse(rdr, ver)?;
        let second = crate::PPtr::parse(rdr, ver)?;

        Ok(ClothSphereColliderPair { first, second })
    }
}

impl crate::ToValue for ClothSphereColliderPair {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("first".to_string(), self.first.to_value())),
            Some(("second".to_string(), self.second.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "first" => (self.first).inspect_value(path, offset, limit),
                "second" => (self.second).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.first).inspect_value(path, offset, limit)).map(|v| ("first".to_string(), v)),
            ((self.second).inspect_value(path, offset, limit)).map(|v| ("second".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
