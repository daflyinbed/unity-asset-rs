//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct BuildTextureStackReference {
    pub group_name: String,
    pub item_name: String,
}

impl crate::Parse for BuildTextureStackReference {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let group_name = rdr.read_aligned_string()?;
        let item_name = rdr.read_aligned_string()?;

        Ok(BuildTextureStackReference {
            group_name,
            item_name,
        })
    }
}

impl crate::ToValue for BuildTextureStackReference {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("groupName".to_string(), self.group_name.to_value())),
            Some(("itemName".to_string(), self.item_name.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "groupName" => (self.group_name).inspect_value(path, offset, limit),
                "itemName" => (self.item_name).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.group_name).inspect_value(path, offset, limit))
                .map(|v| ("groupName".to_string(), v)),
            ((self.item_name).inspect_value(path, offset, limit))
                .map(|v| ("itemName".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
