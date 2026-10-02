//! 自动生成。请勿手动修改。

use crate::subclasses::none::None;

#[derive(Debug, Clone)]
pub struct VrEditorSettings {
    pub none: None,
}

impl crate::Parse for VrEditorSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let none = None::parse(rdr, ver)?;
        rdr.align(4)?;

        Ok(VrEditorSettings { none })
    }
}

impl crate::ToValue for VrEditorSettings {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> =
            [Some(("none".to_string(), self.none.to_value()))]
                .into_iter()
                .flatten()
                .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "none" => (self.none).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> =
            vec![((self.none).inspect_value(path, offset, limit)).map(|v| ("none".to_string(), v))]
                .into_iter()
                .flatten()
                .collect();
        Some(crate::Value::Struct(fields))
    }
}
