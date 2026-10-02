//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct StaticBatchInfo {
    pub first_sub_mesh: u16,
    pub sub_mesh_count: u16,
}

impl crate::Parse for StaticBatchInfo {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let first_sub_mesh = rdr.read_u16()?;
        let sub_mesh_count = rdr.read_u16()?;

        Ok(StaticBatchInfo {
            first_sub_mesh,
            sub_mesh_count,
        })
    }
}

impl crate::ToValue for StaticBatchInfo {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("firstSubMesh".to_string(), self.first_sub_mesh.to_value())),
            Some(("subMeshCount".to_string(), self.sub_mesh_count.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "firstSubMesh" => (self.first_sub_mesh).inspect_value(path, offset, limit),
                "subMeshCount" => (self.sub_mesh_count).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.first_sub_mesh).inspect_value(path, offset, limit))
                .map(|v| ("firstSubMesh".to_string(), v)),
            ((self.sub_mesh_count).inspect_value(path, offset, limit))
                .map(|v| ("subMeshCount".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
