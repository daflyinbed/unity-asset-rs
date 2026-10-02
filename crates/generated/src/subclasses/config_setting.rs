//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct ConfigSetting {
    pub value: String,
    pub flags: u32,
}

impl crate::Parse for ConfigSetting {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let value = rdr.read_aligned_string()?;
        let flags = rdr.read_u32()?;

        Ok(ConfigSetting { value, flags })
    }
}

impl crate::ToValue for ConfigSetting {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("value".to_string(), self.value.to_value())),
            Some(("flags".to_string(), self.flags.to_value())),
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
                "flags" => (self.flags).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.value).inspect_value(path, offset, limit)).map(|v| ("value".to_string(), v)),
            ((self.flags).inspect_value(path, offset, limit)).map(|v| ("flags".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
