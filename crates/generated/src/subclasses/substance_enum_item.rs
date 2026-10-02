//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct SubstanceEnumItem {
    pub value: i32,
    pub text: String,
}

impl crate::Parse for SubstanceEnumItem {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let value = rdr.read_i32()?;
        let text = rdr.read_aligned_string()?;

        Ok(SubstanceEnumItem { value, text })
    }
}

impl crate::ToValue for SubstanceEnumItem {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("value".to_string(), self.value.to_value())),
            Some(("text".to_string(), self.text.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "value" => (self.value).inspect_value(path, offset, limit),
                "text" => (self.text).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.value).inspect_value(path, offset, limit)).map(|v| ("value".to_string(), v)),
            ((self.text).inspect_value(path, offset, limit)).map(|v| ("text".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
