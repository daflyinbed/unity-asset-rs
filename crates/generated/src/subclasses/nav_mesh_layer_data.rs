//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct NavMeshLayerData {
    pub name: String,
    pub cost: f32,
    pub edit_type: i32,
}

impl crate::Parse for NavMeshLayerData {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let name = rdr.read_aligned_string()?;
        let cost = rdr.read_f32()?;
        let edit_type = rdr.read_i32()?;

        Ok(NavMeshLayerData {
            name,
            cost,
            edit_type,
        })
    }
}

impl crate::ToValue for NavMeshLayerData {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("name".to_string(), self.name.to_value())),
            Some(("cost".to_string(), self.cost.to_value())),
            Some(("editType".to_string(), self.edit_type.to_value())),
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
                "cost" => (self.cost).inspect_value(path, offset, limit),
                "editType" => (self.edit_type).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.name).inspect_value(path, offset, limit)).map(|v| ("name".to_string(), v)),
            ((self.cost).inspect_value(path, offset, limit)).map(|v| ("cost".to_string(), v)),
            ((self.edit_type).inspect_value(path, offset, limit))
                .map(|v| ("editType".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
