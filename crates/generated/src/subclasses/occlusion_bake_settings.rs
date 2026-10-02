//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct OcclusionBakeSettings {
    pub view_cell_size: f32,
    pub bake_mode: u32,
    pub memory_usage: u32,
}

impl crate::Parse for OcclusionBakeSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let view_cell_size = rdr.read_f32()?;
        let bake_mode = rdr.read_u32()?;
        let memory_usage = rdr.read_u32()?;

        Ok(OcclusionBakeSettings {
            view_cell_size,
            bake_mode,
            memory_usage,
        })
    }
}

impl crate::ToValue for OcclusionBakeSettings {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("viewCellSize".to_string(), self.view_cell_size.to_value())),
            Some(("bakeMode".to_string(), self.bake_mode.to_value())),
            Some(("memoryUsage".to_string(), self.memory_usage.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "viewCellSize" => (self.view_cell_size).inspect_value(path, offset, limit),
                "bakeMode" => (self.bake_mode).inspect_value(path, offset, limit),
                "memoryUsage" => (self.memory_usage).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.view_cell_size).inspect_value(path, offset, limit))
                .map(|v| ("viewCellSize".to_string(), v)),
            ((self.bake_mode).inspect_value(path, offset, limit))
                .map(|v| ("bakeMode".to_string(), v)),
            ((self.memory_usage).inspect_value(path, offset, limit))
                .map(|v| ("memoryUsage".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
