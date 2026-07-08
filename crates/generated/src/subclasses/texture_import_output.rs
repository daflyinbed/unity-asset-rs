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
