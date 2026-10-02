//! 自动生成。请勿手动修改。

use crate::subclasses::hash128::Hash128;

#[derive(Debug, Clone)]
pub struct EnlightenSystemAtlasInformation {
    pub atlas_size: i32,
    pub atlas_hash: Hash128,
    pub first_system_id: i32,
}

impl crate::Parse for EnlightenSystemAtlasInformation {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let atlas_size = rdr.read_i32()?;
        let atlas_hash = Hash128::parse(rdr, ver)?;
        let first_system_id = rdr.read_i32()?;

        Ok(EnlightenSystemAtlasInformation {
            atlas_size,
            atlas_hash,
            first_system_id,
        })
    }
}

impl crate::ToValue for EnlightenSystemAtlasInformation {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("atlasSize".to_string(), self.atlas_size.to_value())),
            Some(("atlasHash".to_string(), self.atlas_hash.to_value())),
            Some(("firstSystemId".to_string(), self.first_system_id.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "atlasSize" => (self.atlas_size).inspect_value(path, offset, limit),
                "atlasHash" => (self.atlas_hash).inspect_value(path, offset, limit),
                "firstSystemId" => (self.first_system_id).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.atlas_size).inspect_value(path, offset, limit))
                .map(|v| ("atlasSize".to_string(), v)),
            ((self.atlas_hash).inspect_value(path, offset, limit))
                .map(|v| ("atlasHash".to_string(), v)),
            ((self.first_system_id).inspect_value(path, offset, limit))
                .map(|v| ("firstSystemId".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
