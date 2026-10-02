//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct LocalizationAsset {
    pub m_name: String,
    pub locale_iso_code: String,
    pub editor_asset: bool,
    pub string_table: Vec<(String, String)>,
}

impl crate::Parse for LocalizationAsset {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        let locale_iso_code = rdr.read_aligned_string()?;
        let editor_asset = rdr.read_bool()?;
        rdr.align(4)?;
        let string_table = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| {
                    ({
                        let a = rdr.read_aligned_string()?;
                        let b = rdr.read_aligned_string()?;
                        Ok((a, b))
                    })
                })
                .collect::<Result<Vec<_>, _>>()
        })?;

        Ok(LocalizationAsset {
            m_name,
            locale_iso_code,
            editor_asset,
            string_table,
        })
    }
}

impl crate::ToValue for LocalizationAsset {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some((
                "Locale ISO Code".to_string(),
                self.locale_iso_code.to_value(),
            )),
            Some(("Editor Asset".to_string(), self.editor_asset.to_value())),
            Some(("String Table".to_string(), self.string_table.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Name" => (self.m_name).inspect_value(path, offset, limit),
                "Locale ISO Code" => (self.locale_iso_code).inspect_value(path, offset, limit),
                "Editor Asset" => (self.editor_asset).inspect_value(path, offset, limit),
                "String Table" => (self.string_table).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.locale_iso_code).inspect_value(path, offset, limit))
                .map(|v| ("Locale ISO Code".to_string(), v)),
            ((self.editor_asset).inspect_value(path, offset, limit))
                .map(|v| ("Editor Asset".to_string(), v)),
            ((self.string_table).inspect_value(path, offset, limit))
                .map(|v| ("String Table".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
