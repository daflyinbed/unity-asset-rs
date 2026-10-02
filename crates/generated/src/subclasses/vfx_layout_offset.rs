//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct VfxLayoutOffset {
    pub bucket: u32,
    pub structure: u32,
    pub element: u32,
}

impl crate::Parse for VfxLayoutOffset {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let bucket = rdr.read_u32()?;
        let structure = rdr.read_u32()?;
        let element = rdr.read_u32()?;

        Ok(VfxLayoutOffset {
            bucket,
            structure,
            element,
        })
    }
}

impl crate::ToValue for VfxLayoutOffset {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("bucket".to_string(), self.bucket.to_value())),
            Some(("structure".to_string(), self.structure.to_value())),
            Some(("element".to_string(), self.element.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "bucket" => (self.bucket).inspect_value(path, offset, limit),
                "structure" => (self.structure).inspect_value(path, offset, limit),
                "element" => (self.element).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.bucket).inspect_value(path, offset, limit)).map(|v| ("bucket".to_string(), v)),
            ((self.structure).inspect_value(path, offset, limit))
                .map(|v| ("structure".to_string(), v)),
            ((self.element).inspect_value(path, offset, limit)).map(|v| ("element".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
