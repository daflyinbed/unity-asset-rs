//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct SortingLayerEntry {
    pub name: String,
    pub user_id: Option<u32>,
    pub unique_id: u32,
}

impl crate::Parse for SortingLayerEntry {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut name = None;
        let mut user_id = None;
        let mut unique_id = None;
        name = Some(rdr.read_aligned_string()?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
        } else {
            user_id = Some(rdr.read_u32()?);
        }
        unique_id = Some(rdr.read_u32()?);
        rdr.align(4)?;

        Ok(SortingLayerEntry {
            name: name.unwrap(),
            user_id,
            unique_id: unique_id.unwrap(),
        })
    }
}

impl crate::ToValue for SortingLayerEntry {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("name".to_string(), self.name.to_value())),
            crate::opt_field("userID", &self.user_id),
            Some(("uniqueID".to_string(), self.unique_id.to_value())),
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
                "userID" => (self.user_id).inspect_value(path, offset, limit),
                "uniqueID" => (self.unique_id).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.name).inspect_value(path, offset, limit)).map(|v| ("name".to_string(), v)),
            ((self.user_id).inspect_value(path, offset, limit)).map(|v| ("userID".to_string(), v)),
            ((self.unique_id).inspect_value(path, offset, limit))
                .map(|v| ("uniqueID".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
