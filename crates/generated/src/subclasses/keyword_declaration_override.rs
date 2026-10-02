//! 自动生成。请勿手动修改。

use crate::subclasses::keyword_override_info::KeywordOverrideInfo;

#[derive(Debug, Clone)]
pub struct KeywordDeclarationOverride {
    pub keywords: Vec<KeywordOverrideInfo>,
    pub variant_generation_mode: i32,
}

impl crate::Parse for KeywordDeclarationOverride {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let keywords = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| KeywordOverrideInfo::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let variant_generation_mode = rdr.read_i32()?;

        Ok(KeywordDeclarationOverride {
            keywords,
            variant_generation_mode,
        })
    }
}

impl crate::ToValue for KeywordDeclarationOverride {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("keywords".to_string(), self.keywords.to_value())),
            Some((
                "variantGenerationMode".to_string(),
                self.variant_generation_mode.to_value(),
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
                "keywords" => (self.keywords).inspect_value(path, offset, limit),
                "variantGenerationMode" => {
                    (self.variant_generation_mode).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.keywords).inspect_value(path, offset, limit))
                .map(|v| ("keywords".to_string(), v)),
            ((self.variant_generation_mode).inspect_value(path, offset, limit))
                .map(|v| ("variantGenerationMode".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
