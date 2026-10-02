//! 自动生成。请勿手动修改。

use crate::subclasses::native_type::NativeType;

#[derive(Debug, Clone)]
pub struct NativeObjectType {
    pub m_inner: NativeType,
}

impl crate::Parse for NativeObjectType {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_inner = NativeType::parse(rdr, ver)?;

        Ok(NativeObjectType { m_inner })
    }
}

impl crate::ToValue for NativeObjectType {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> =
            [Some(("m_Inner".to_string(), self.m_inner.to_value()))]
                .into_iter()
                .flatten()
                .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Inner" => (self.m_inner).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_inner).inspect_value(path, offset, limit)).map(|v| ("m_Inner".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
