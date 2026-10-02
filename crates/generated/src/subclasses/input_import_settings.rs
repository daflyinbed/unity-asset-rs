//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct InputImportSettings {
    pub name: String,
    pub alpha_source: i32,
    pub filter_mode: i32,
    pub aniso: i32,
    pub wrap_mode: i32,
}

impl crate::Parse for InputImportSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let name = rdr.read_aligned_string()?;
        let alpha_source = rdr.read_i32()?;
        let filter_mode = rdr.read_i32()?;
        let aniso = rdr.read_i32()?;
        let wrap_mode = rdr.read_i32()?;

        Ok(InputImportSettings {
            name,
            alpha_source,
            filter_mode,
            aniso,
            wrap_mode,
        })
    }
}

impl crate::ToValue for InputImportSettings {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("name".to_string(), self.name.to_value())),
            Some(("alphaSource".to_string(), self.alpha_source.to_value())),
            Some(("filterMode".to_string(), self.filter_mode.to_value())),
            Some(("aniso".to_string(), self.aniso.to_value())),
            Some(("wrapMode".to_string(), self.wrap_mode.to_value())),
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
                "alphaSource" => (self.alpha_source).inspect_value(path, offset, limit),
                "filterMode" => (self.filter_mode).inspect_value(path, offset, limit),
                "aniso" => (self.aniso).inspect_value(path, offset, limit),
                "wrapMode" => (self.wrap_mode).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.name).inspect_value(path, offset, limit)).map(|v| ("name".to_string(), v)),
            ((self.alpha_source).inspect_value(path, offset, limit))
                .map(|v| ("alphaSource".to_string(), v)),
            ((self.filter_mode).inspect_value(path, offset, limit))
                .map(|v| ("filterMode".to_string(), v)),
            ((self.aniso).inspect_value(path, offset, limit)).map(|v| ("aniso".to_string(), v)),
            ((self.wrap_mode).inspect_value(path, offset, limit))
                .map(|v| ("wrapMode".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
