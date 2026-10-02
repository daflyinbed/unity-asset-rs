//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct TypeStats {
    pub class_id: i32,
    pub size: u64,
    pub object_count: i32,
    pub resource_count: i32,
}

impl crate::Parse for TypeStats {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let class_id = rdr.read_i32()?;
        let size = rdr.read_u64()?;
        let object_count = rdr.read_i32()?;
        let resource_count = rdr.read_i32()?;

        Ok(TypeStats {
            class_id,
            size,
            object_count,
            resource_count,
        })
    }
}

impl crate::ToValue for TypeStats {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("classID".to_string(), self.class_id.to_value())),
            Some(("size".to_string(), self.size.to_value())),
            Some(("objectCount".to_string(), self.object_count.to_value())),
            Some(("resourceCount".to_string(), self.resource_count.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "classID" => (self.class_id).inspect_value(path, offset, limit),
                "size" => (self.size).inspect_value(path, offset, limit),
                "objectCount" => (self.object_count).inspect_value(path, offset, limit),
                "resourceCount" => (self.resource_count).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.class_id).inspect_value(path, offset, limit))
                .map(|v| ("classID".to_string(), v)),
            ((self.size).inspect_value(path, offset, limit)).map(|v| ("size".to_string(), v)),
            ((self.object_count).inspect_value(path, offset, limit))
                .map(|v| ("objectCount".to_string(), v)),
            ((self.resource_count).inspect_value(path, offset, limit))
                .map(|v| ("resourceCount".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
