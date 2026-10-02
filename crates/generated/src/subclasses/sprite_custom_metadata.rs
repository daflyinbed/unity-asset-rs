//! 自动生成。请勿手动修改。

use crate::subclasses::sprite_custom_data_entry::SpriteCustomDataEntry;

#[derive(Debug, Clone)]
pub struct SpriteCustomMetadata {
    pub m_entries: Vec<SpriteCustomDataEntry>,
}

impl crate::Parse for SpriteCustomMetadata {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_entries = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| SpriteCustomDataEntry::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;

        Ok(SpriteCustomMetadata { m_entries })
    }
}

impl crate::ToValue for SpriteCustomMetadata {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> =
            [Some(("m_Entries".to_string(), self.m_entries.to_value()))]
                .into_iter()
                .flatten()
                .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Entries" => (self.m_entries).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_entries).inspect_value(path, offset, limit))
                .map(|v| ("m_Entries".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
