//! 自动生成。请勿手动修改。

use crate::subclasses::hash128::Hash128;

#[derive(Debug, Clone)]
pub struct EnlightenSystemInformation {
    pub renderer_index: u32,
    pub renderer_size: u32,
    pub atlas_index: i32,
    pub atlas_offset_x: i32,
    pub atlas_offset_y: i32,
    pub input_system_hash: Hash128,
    pub radiosity_system_hash: Hash128,
}

impl crate::Parse for EnlightenSystemInformation {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let renderer_index = rdr.read_u32()?;
        let renderer_size = rdr.read_u32()?;
        let atlas_index = rdr.read_i32()?;
        let atlas_offset_x = rdr.read_i32()?;
        let atlas_offset_y = rdr.read_i32()?;
        let input_system_hash = Hash128::parse(rdr, ver)?;
        let radiosity_system_hash = Hash128::parse(rdr, ver)?;

        Ok(EnlightenSystemInformation {
            renderer_index,
            renderer_size,
            atlas_index,
            atlas_offset_x,
            atlas_offset_y,
            input_system_hash,
            radiosity_system_hash,
        })
    }
}

impl crate::ToValue for EnlightenSystemInformation {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("rendererIndex".to_string(), self.renderer_index.to_value())),
            Some(("rendererSize".to_string(), self.renderer_size.to_value())),
            Some(("atlasIndex".to_string(), self.atlas_index.to_value())),
            Some(("atlasOffsetX".to_string(), self.atlas_offset_x.to_value())),
            Some(("atlasOffsetY".to_string(), self.atlas_offset_y.to_value())),
            Some((
                "inputSystemHash".to_string(),
                self.input_system_hash.to_value(),
            )),
            Some((
                "radiositySystemHash".to_string(),
                self.radiosity_system_hash.to_value(),
            )),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "rendererIndex" => (self.renderer_index).inspect_value(path, offset, limit),
                "rendererSize" => (self.renderer_size).inspect_value(path, offset, limit),
                "atlasIndex" => (self.atlas_index).inspect_value(path, offset, limit),
                "atlasOffsetX" => (self.atlas_offset_x).inspect_value(path, offset, limit),
                "atlasOffsetY" => (self.atlas_offset_y).inspect_value(path, offset, limit),
                "inputSystemHash" => (self.input_system_hash).inspect_value(path, offset, limit),
                "radiositySystemHash" => {
                    (self.radiosity_system_hash).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.renderer_index).inspect_value(path, offset, limit))
                .map(|v| ("rendererIndex".to_string(), v)),
            ((self.renderer_size).inspect_value(path, offset, limit))
                .map(|v| ("rendererSize".to_string(), v)),
            ((self.atlas_index).inspect_value(path, offset, limit))
                .map(|v| ("atlasIndex".to_string(), v)),
            ((self.atlas_offset_x).inspect_value(path, offset, limit))
                .map(|v| ("atlasOffsetX".to_string(), v)),
            ((self.atlas_offset_y).inspect_value(path, offset, limit))
                .map(|v| ("atlasOffsetY".to_string(), v)),
            ((self.input_system_hash).inspect_value(path, offset, limit))
                .map(|v| ("inputSystemHash".to_string(), v)),
            ((self.radiosity_system_hash).inspect_value(path, offset, limit))
                .map(|v| ("radiositySystemHash".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
