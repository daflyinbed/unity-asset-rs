//! 自动生成。请勿手动修改。

use crate::subclasses::keyword_declaration_override::KeywordDeclarationOverride;

#[derive(Debug, Clone)]
pub struct ShaderBuildSettings {
    pub keyword_declaration_overrides: Vec<KeywordDeclarationOverride>,
}

impl crate::Parse for ShaderBuildSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let keyword_declaration_overrides = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| KeywordDeclarationOverride::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;

        Ok(ShaderBuildSettings {
            keyword_declaration_overrides,
        })
    }
}

impl crate::ToValue for ShaderBuildSettings {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [Some((
            "keywordDeclarationOverrides".to_string(),
            self.keyword_declaration_overrides.to_value(),
        ))]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "keywordDeclarationOverrides" => {
                    (self.keyword_declaration_overrides).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.keyword_declaration_overrides).inspect_value(path, offset, limit))
                .map(|v| ("keywordDeclarationOverrides".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
