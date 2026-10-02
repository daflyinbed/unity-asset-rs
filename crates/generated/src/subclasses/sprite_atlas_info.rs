//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::guid::Guid;

#[derive(Debug, Clone)]
pub struct SpriteAtlasInfo {
    pub guid: Guid,
    pub atlas: PPtr,
}

impl crate::Parse for SpriteAtlasInfo {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let guid = Guid::parse(rdr, ver)?;
        let atlas = crate::PPtr::parse(rdr, ver)?;

        Ok(SpriteAtlasInfo { guid, atlas })
    }
}

impl crate::ToValue for SpriteAtlasInfo {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("guid".to_string(), self.guid.to_value())),
            Some(("atlas".to_string(), self.atlas.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "guid" => (self.guid).inspect_value(path, offset, limit),
                "atlas" => (self.atlas).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.guid).inspect_value(path, offset, limit)).map(|v| ("guid".to_string(), v)),
            ((self.atlas).inspect_value(path, offset, limit)).map(|v| ("atlas".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
