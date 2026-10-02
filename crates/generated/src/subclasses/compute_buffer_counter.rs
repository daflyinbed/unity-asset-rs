//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct ComputeBufferCounter {
    pub bindpoint: i32,
    pub offset: i32,
}

impl crate::Parse for ComputeBufferCounter {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let bindpoint = rdr.read_i32()?;
        let offset = rdr.read_i32()?;

        Ok(ComputeBufferCounter { bindpoint, offset })
    }
}

impl crate::ToValue for ComputeBufferCounter {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("bindpoint".to_string(), self.bindpoint.to_value())),
            Some(("offset".to_string(), self.offset.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "bindpoint" => (self.bindpoint).inspect_value(path, offset, limit),
                "offset" => (self.offset).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.bindpoint).inspect_value(path, offset, limit))
                .map(|v| ("bindpoint".to_string(), v)),
            ((self.offset).inspect_value(path, offset, limit)).map(|v| ("offset".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
