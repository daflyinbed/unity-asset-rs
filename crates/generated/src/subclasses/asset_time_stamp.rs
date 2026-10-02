//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct AssetTimeStamp {
    pub modification_date_0: u32,
    pub modification_date_1: u32,
    pub meta_modification_date_0: u32,
    pub meta_modification_date_1: u32,
}

impl crate::Parse for AssetTimeStamp {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let modification_date_0 = rdr.read_u32()?;
        let modification_date_1 = rdr.read_u32()?;
        let meta_modification_date_0 = rdr.read_u32()?;
        let meta_modification_date_1 = rdr.read_u32()?;

        Ok(AssetTimeStamp {
            modification_date_0,
            modification_date_1,
            meta_modification_date_0,
            meta_modification_date_1,
        })
    }
}

impl crate::ToValue for AssetTimeStamp {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "modificationDate[0]".to_string(),
                self.modification_date_0.to_value(),
            )),
            Some((
                "modificationDate[1]".to_string(),
                self.modification_date_1.to_value(),
            )),
            Some((
                "metaModificationDate[0]".to_string(),
                self.meta_modification_date_0.to_value(),
            )),
            Some((
                "metaModificationDate[1]".to_string(),
                self.meta_modification_date_1.to_value(),
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
                "modificationDate[0]" => {
                    (self.modification_date_0).inspect_value(path, offset, limit)
                }
                "modificationDate[1]" => {
                    (self.modification_date_1).inspect_value(path, offset, limit)
                }
                "metaModificationDate[0]" => {
                    (self.meta_modification_date_0).inspect_value(path, offset, limit)
                }
                "metaModificationDate[1]" => {
                    (self.meta_modification_date_1).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.modification_date_0).inspect_value(path, offset, limit))
                .map(|v| ("modificationDate[0]".to_string(), v)),
            ((self.modification_date_1).inspect_value(path, offset, limit))
                .map(|v| ("modificationDate[1]".to_string(), v)),
            ((self.meta_modification_date_0).inspect_value(path, offset, limit))
                .map(|v| ("metaModificationDate[0]".to_string(), v)),
            ((self.meta_modification_date_1).inspect_value(path, offset, limit))
                .map(|v| ("metaModificationDate[1]".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
