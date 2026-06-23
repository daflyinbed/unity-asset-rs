//! 自动生成。请勿手动修改。

use crate::subclasses::source_texture_information::SourceTextureInformation;
use crate::subclasses::texture_import_instructions::TextureImportInstructions;

#[derive(Debug, Clone)]
pub struct TextureImportOutput {
    pub texture_import_instructions: TextureImportInstructions,
    pub source_texture_information: SourceTextureInformation,
}

impl crate::Parse for TextureImportOutput {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let texture_import_instructions = TextureImportInstructions::parse(r, ver)?;
        let source_texture_information = SourceTextureInformation::parse(r, ver)?;

        Ok(TextureImportOutput {
            texture_import_instructions,
            source_texture_information,
        })
    }
}
