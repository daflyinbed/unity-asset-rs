//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct AssetBundleScriptInfo {
    pub class_name: String,
    pub name_space: String,
    pub assembly_name: String,
    pub hash: u32,
}

impl crate::Parse for AssetBundleScriptInfo {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let class_name = rdr.read_aligned_string()?;
        let name_space = rdr.read_aligned_string()?;
        let assembly_name = rdr.read_aligned_string()?;
        let hash = rdr.read_u32()?;

        Ok(AssetBundleScriptInfo {
            class_name,
            name_space,
            assembly_name,
            hash,
        })
    }
}

impl crate::ToValue for AssetBundleScriptInfo {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("className".to_string(), self.class_name.to_value())),
            Some(("nameSpace".to_string(), self.name_space.to_value())),
            Some(("assemblyName".to_string(), self.assembly_name.to_value())),
            Some(("hash".to_string(), self.hash.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "className" => (self.class_name).inspect_value(path, offset, limit),
                "nameSpace" => (self.name_space).inspect_value(path, offset, limit),
                "assemblyName" => (self.assembly_name).inspect_value(path, offset, limit),
                "hash" => (self.hash).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.class_name).inspect_value(path, offset, limit))
                .map(|v| ("className".to_string(), v)),
            ((self.name_space).inspect_value(path, offset, limit))
                .map(|v| ("nameSpace".to_string(), v)),
            ((self.assembly_name).inspect_value(path, offset, limit))
                .map(|v| ("assemblyName".to_string(), v)),
            ((self.hash).inspect_value(path, offset, limit)).map(|v| ("hash".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
