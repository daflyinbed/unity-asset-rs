//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct SerializedPackageRequirement {
    pub name: String,
    pub version: String,
    pub status: i8,
}

impl crate::Parse for SerializedPackageRequirement {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let name = rdr.read_aligned_string()?;
        let version = rdr.read_aligned_string()?;
        let status = rdr.read_i8()?;

        Ok(SerializedPackageRequirement {
            name,
            version,
            status,
        })
    }
}

impl crate::ToValue for SerializedPackageRequirement {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("name".to_string(), self.name.to_value())),
            Some(("version".to_string(), self.version.to_value())),
            Some(("status".to_string(), self.status.to_value())),
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
                "version" => (self.version).inspect_value(path, offset, limit),
                "status" => (self.status).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.name).inspect_value(path, offset, limit)).map(|v| ("name".to_string(), v)),
            ((self.version).inspect_value(path, offset, limit)).map(|v| ("version".to_string(), v)),
            ((self.status).inspect_value(path, offset, limit)).map(|v| ("status".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
