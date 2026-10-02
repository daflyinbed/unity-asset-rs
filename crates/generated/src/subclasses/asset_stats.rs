//! 自动生成。请勿手动修改。

use crate::subclasses::guid::Guid;

#[derive(Debug, Clone)]
pub struct AssetStats {
    pub source_asset_guid: Guid,
    pub source_asset_path: String,
    pub size: u64,
    pub object_count: i32,
    pub resource_count: i32,
}

impl crate::Parse for AssetStats {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let source_asset_guid = Guid::parse(rdr, ver)?;
        let source_asset_path = rdr.read_aligned_string()?;
        let size = rdr.read_u64()?;
        let object_count = rdr.read_i32()?;
        let resource_count = rdr.read_i32()?;

        Ok(AssetStats {
            source_asset_guid,
            source_asset_path,
            size,
            object_count,
            resource_count,
        })
    }
}

impl crate::ToValue for AssetStats {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "sourceAssetGUID".to_string(),
                self.source_asset_guid.to_value(),
            )),
            Some((
                "sourceAssetPath".to_string(),
                self.source_asset_path.to_value(),
            )),
            Some(("size".to_string(), self.size.to_value())),
            Some(("objectCount".to_string(), self.object_count.to_value())),
            Some(("resourceCount".to_string(), self.resource_count.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "sourceAssetGUID" => (self.source_asset_guid).inspect_value(path, offset, limit),
                "sourceAssetPath" => (self.source_asset_path).inspect_value(path, offset, limit),
                "size" => (self.size).inspect_value(path, offset, limit),
                "objectCount" => (self.object_count).inspect_value(path, offset, limit),
                "resourceCount" => (self.resource_count).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.source_asset_guid).inspect_value(path, offset, limit))
                .map(|v| ("sourceAssetGUID".to_string(), v)),
            ((self.source_asset_path).inspect_value(path, offset, limit))
                .map(|v| ("sourceAssetPath".to_string(), v)),
            ((self.size).inspect_value(path, offset, limit)).map(|v| ("size".to_string(), v)),
            ((self.object_count).inspect_value(path, offset, limit))
                .map(|v| ("objectCount".to_string(), v)),
            ((self.resource_count).inspect_value(path, offset, limit))
                .map(|v| ("resourceCount".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
