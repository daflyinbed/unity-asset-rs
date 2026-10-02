//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct SceneObjectIdentifier {
    pub target_object: i64,
    pub target_prefab: i64,
}

impl crate::Parse for SceneObjectIdentifier {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let target_object = rdr.read_i64()?;
        let target_prefab = rdr.read_i64()?;

        Ok(SceneObjectIdentifier {
            target_object,
            target_prefab,
        })
    }
}

impl crate::ToValue for SceneObjectIdentifier {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("targetObject".to_string(), self.target_object.to_value())),
            Some(("targetPrefab".to_string(), self.target_prefab.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "targetObject" => (self.target_object).inspect_value(path, offset, limit),
                "targetPrefab" => (self.target_prefab).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.target_object).inspect_value(path, offset, limit))
                .map(|v| ("targetObject".to_string(), v)),
            ((self.target_prefab).inspect_value(path, offset, limit))
                .map(|v| ("targetPrefab".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
