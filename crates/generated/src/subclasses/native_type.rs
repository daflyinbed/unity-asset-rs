//! 自动生成。请勿手动修改。

use crate::subclasses::embedded_native_type::EmbeddedNativeType;

#[derive(Debug, Clone)]
pub struct NativeType {
    pub a: i32,
    pub b: f32,
    pub embedded: EmbeddedNativeType,
}

impl crate::Parse for NativeType {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let a = rdr.read_i32()?;
        let b = rdr.read_f32()?;
        let embedded = EmbeddedNativeType::parse(rdr, ver)?;

        Ok(NativeType { a, b, embedded })
    }
}

impl crate::ToValue for NativeType {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("a".to_string(), self.a.to_value())),
            Some(("b".to_string(), self.b.to_value())),
            Some(("embedded".to_string(), self.embedded.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "a" => (self.a).inspect_value(path, offset, limit),
                "b" => (self.b).inspect_value(path, offset, limit),
                "embedded" => (self.embedded).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.a).inspect_value(path, offset, limit)).map(|v| ("a".to_string(), v)),
            ((self.b).inspect_value(path, offset, limit)).map(|v| ("b".to_string(), v)),
            ((self.embedded).inspect_value(path, offset, limit))
                .map(|v| ("embedded".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
