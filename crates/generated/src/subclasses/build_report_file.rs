//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct BuildReportFile {
    pub path: String,
    pub role: String,
    pub id: u32,
    pub total_size: u64,
    pub flags: Option<i32>,
}

impl crate::Parse for BuildReportFile {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let path = rdr.read_aligned_string()?;
        let role = rdr.read_aligned_string()?;
        let id = rdr.read_u32()?;
        let total_size = rdr.read_u64()?;
        let flags = if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            Some(rdr.read_i32()?)
        } else {
            None
        };

        Ok(BuildReportFile {
            path,
            role,
            id,
            total_size,
            flags,
        })
    }
}

impl crate::ToValue for BuildReportFile {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("path".to_string(), self.path.to_value())),
            Some(("role".to_string(), self.role.to_value())),
            Some(("id".to_string(), self.id.to_value())),
            Some(("totalSize".to_string(), self.total_size.to_value())),
            crate::opt_field("flags", &self.flags),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "path" => (self.path).inspect_value(path, offset, limit),
                "role" => (self.role).inspect_value(path, offset, limit),
                "id" => (self.id).inspect_value(path, offset, limit),
                "totalSize" => (self.total_size).inspect_value(path, offset, limit),
                "flags" => (self.flags).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.path).inspect_value(path, offset, limit)).map(|v| ("path".to_string(), v)),
            ((self.role).inspect_value(path, offset, limit)).map(|v| ("role".to_string(), v)),
            ((self.id).inspect_value(path, offset, limit)).map(|v| ("id".to_string(), v)),
            ((self.total_size).inspect_value(path, offset, limit))
                .map(|v| ("totalSize".to_string(), v)),
            ((self.flags).inspect_value(path, offset, limit)).map(|v| ("flags".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
