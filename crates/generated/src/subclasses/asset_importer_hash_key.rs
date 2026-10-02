//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct AssetImporterHashKey {
    pub r#type: i32,
    pub script_class: String,
}

impl crate::Parse for AssetImporterHashKey {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let r#type = rdr.read_i32()?;
        let script_class = rdr.read_aligned_string()?;

        Ok(AssetImporterHashKey {
            r#type,
            script_class,
        })
    }
}

impl crate::ToValue for AssetImporterHashKey {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("type".to_string(), self.r#type.to_value())),
            Some(("ScriptClass".to_string(), self.script_class.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "type" => (self.r#type).inspect_value(path, offset, limit),
                "ScriptClass" => (self.script_class).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.r#type).inspect_value(path, offset, limit)).map(|v| ("type".to_string(), v)),
            ((self.script_class).inspect_value(path, offset, limit))
                .map(|v| ("ScriptClass".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
