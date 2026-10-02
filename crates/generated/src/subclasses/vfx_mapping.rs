//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct VfxMapping {
    pub name_id: String,
    pub index: i32,
}

impl crate::Parse for VfxMapping {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let name_id = rdr.read_aligned_string()?;
        let index = rdr.read_i32()?;

        Ok(VfxMapping { name_id, index })
    }
}

impl crate::ToValue for VfxMapping {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("nameId".to_string(), self.name_id.to_value())),
            Some(("index".to_string(), self.index.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "nameId" => (self.name_id).inspect_value(path, offset, limit),
                "index" => (self.index).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.name_id).inspect_value(path, offset, limit)).map(|v| ("nameId".to_string(), v)),
            ((self.index).inspect_value(path, offset, limit)).map(|v| ("index".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
