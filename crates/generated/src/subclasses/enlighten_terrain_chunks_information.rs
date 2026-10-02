//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct EnlightenTerrainChunksInformation {
    pub first_system_id: i32,
    pub num_chunks_in_x: i32,
    pub num_chunks_in_y: i32,
}

impl crate::Parse for EnlightenTerrainChunksInformation {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let first_system_id = rdr.read_i32()?;
        let num_chunks_in_x = rdr.read_i32()?;
        let num_chunks_in_y = rdr.read_i32()?;

        Ok(EnlightenTerrainChunksInformation {
            first_system_id,
            num_chunks_in_x,
            num_chunks_in_y,
        })
    }
}

impl crate::ToValue for EnlightenTerrainChunksInformation {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("firstSystemId".to_string(), self.first_system_id.to_value())),
            Some(("numChunksInX".to_string(), self.num_chunks_in_x.to_value())),
            Some(("numChunksInY".to_string(), self.num_chunks_in_y.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "firstSystemId" => (self.first_system_id).inspect_value(path, offset, limit),
                "numChunksInX" => (self.num_chunks_in_x).inspect_value(path, offset, limit),
                "numChunksInY" => (self.num_chunks_in_y).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.first_system_id).inspect_value(path, offset, limit))
                .map(|v| ("firstSystemId".to_string(), v)),
            ((self.num_chunks_in_x).inspect_value(path, offset, limit))
                .map(|v| ("numChunksInX".to_string(), v)),
            ((self.num_chunks_in_y).inspect_value(path, offset, limit))
                .map(|v| ("numChunksInY".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
