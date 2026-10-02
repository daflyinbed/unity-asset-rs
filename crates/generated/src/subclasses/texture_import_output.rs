//! 自动生成。请勿手动修改。

use crate::subclasses::source_texture_information::SourceTextureInformation;
use crate::subclasses::texture_import_instructions::TextureImportInstructions;

#[derive(Debug, Clone)]
pub struct TextureImportOutput {
    pub texture_import_instructions: TextureImportInstructions,
    pub source_texture_information: SourceTextureInformation,
    pub import_inspector_warnings: Option<String>,
}

impl crate::Parse for TextureImportOutput {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let texture_import_instructions = TextureImportInstructions::parse(rdr, ver)?;
        let source_texture_information = SourceTextureInformation::parse(rdr, ver)?;
        let import_inspector_warnings = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(rdr.read_aligned_string()?)
        } else {
            None
        };

        Ok(TextureImportOutput {
            texture_import_instructions,
            source_texture_information,
            import_inspector_warnings,
        })
    }
}

impl crate::ToValue for TextureImportOutput {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "textureImportInstructions".to_string(),
                self.texture_import_instructions.to_value(),
            )),
            Some((
                "sourceTextureInformation".to_string(),
                self.source_texture_information.to_value(),
            )),
            crate::opt_field("importInspectorWarnings", &self.import_inspector_warnings),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "textureImportInstructions" => {
                    (self.texture_import_instructions).inspect_value(path, offset, limit)
                }
                "sourceTextureInformation" => {
                    (self.source_texture_information).inspect_value(path, offset, limit)
                }
                "importInspectorWarnings" => {
                    (self.import_inspector_warnings).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.texture_import_instructions).inspect_value(path, offset, limit))
                .map(|v| ("textureImportInstructions".to_string(), v)),
            ((self.source_texture_information).inspect_value(path, offset, limit))
                .map(|v| ("sourceTextureInformation".to_string(), v)),
            ((self.import_inspector_warnings).inspect_value(path, offset, limit))
                .map(|v| ("importInspectorWarnings".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
