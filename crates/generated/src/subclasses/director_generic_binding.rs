//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct DirectorGenericBinding {
    pub key: PPtr,
    pub value: PPtr,
}

impl crate::Parse for DirectorGenericBinding {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let key = crate::PPtr::parse(rdr, ver)?;
        let value = crate::PPtr::parse(rdr, ver)?;

        Ok(DirectorGenericBinding { key, value })
    }
}

impl crate::ToValue for DirectorGenericBinding {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("key".to_string(), self.key.to_value())),
            Some(("value".to_string(), self.value.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "key" => (self.key).inspect_value(path, offset, limit),
                "value" => (self.value).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.key).inspect_value(path, offset, limit)).map(|v| ("key".to_string(), v)),
            ((self.value).inspect_value(path, offset, limit)).map(|v| ("value".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
