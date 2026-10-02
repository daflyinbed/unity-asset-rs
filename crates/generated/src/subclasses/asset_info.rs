//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct AssetInfo {
    pub preload_index: i32,
    pub preload_size: i32,
    pub asset: PPtr,
}

impl crate::Parse for AssetInfo {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let preload_index = rdr.read_i32()?;
        let preload_size = rdr.read_i32()?;
        let asset = crate::PPtr::parse(rdr, ver)?;

        Ok(AssetInfo {
            preload_index,
            preload_size,
            asset,
        })
    }
}

impl crate::ToValue for AssetInfo {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("preloadIndex".to_string(), self.preload_index.to_value())),
            Some(("preloadSize".to_string(), self.preload_size.to_value())),
            Some(("asset".to_string(), self.asset.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "preloadIndex" => (self.preload_index).inspect_value(path, offset, limit),
                "preloadSize" => (self.preload_size).inspect_value(path, offset, limit),
                "asset" => (self.asset).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.preload_index).inspect_value(path, offset, limit))
                .map(|v| ("preloadIndex".to_string(), v)),
            ((self.preload_size).inspect_value(path, offset, limit))
                .map(|v| ("preloadSize".to_string(), v)),
            ((self.asset).inspect_value(path, offset, limit)).map(|v| ("asset".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
