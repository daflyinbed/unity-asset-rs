//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct MeshBlendShapeChannel {
    pub name: String,
    pub name_hash: u32,
    pub frame_index: i32,
    pub frame_count: i32,
}

impl crate::Parse for MeshBlendShapeChannel {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let name = rdr.read_aligned_string()?;
        let name_hash = rdr.read_u32()?;
        let frame_index = rdr.read_i32()?;
        let frame_count = rdr.read_i32()?;

        Ok(MeshBlendShapeChannel {
            name,
            name_hash,
            frame_index,
            frame_count,
        })
    }
}

impl crate::ToValue for MeshBlendShapeChannel {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("name".to_string(), self.name.to_value())),
            Some(("nameHash".to_string(), self.name_hash.to_value())),
            Some(("frameIndex".to_string(), self.frame_index.to_value())),
            Some(("frameCount".to_string(), self.frame_count.to_value())),
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
                "nameHash" => (self.name_hash).inspect_value(path, offset, limit),
                "frameIndex" => (self.frame_index).inspect_value(path, offset, limit),
                "frameCount" => (self.frame_count).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.name).inspect_value(path, offset, limit)).map(|v| ("name".to_string(), v)),
            ((self.name_hash).inspect_value(path, offset, limit))
                .map(|v| ("nameHash".to_string(), v)),
            ((self.frame_index).inspect_value(path, offset, limit))
                .map(|v| ("frameIndex".to_string(), v)),
            ((self.frame_count).inspect_value(path, offset, limit))
                .map(|v| ("frameCount".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
