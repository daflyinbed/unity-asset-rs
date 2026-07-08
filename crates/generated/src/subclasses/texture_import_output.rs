//! 自动生成。请勿手动修改。

use crate::subclasses::source_texture_information::SourceTextureInformation;
use crate::subclasses::texture_import_instructions::TextureImportInstructions;


/// 版本枚举：TextureImportOutputTextureImportInstructionsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum TextureImportOutputTextureImportInstructionsVariant {
    Default(Box<TextureImportInstructions>),
    V2017_3(Box<TextureImportInstructions>),
}

impl crate::Parse for TextureImportOutputTextureImportInstructionsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            let v = ( { Ok(Box::new(TextureImportInstructions::parse(rdr, ver)?)) } )?;
            rdr.align(4)?;
            Ok(Self::V2017_3(v))
        } else {
            let v = ( { Ok(Box::new(TextureImportInstructions::parse(rdr, ver)?)) } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：TextureImportOutputSourceTextureInformationVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum TextureImportOutputSourceTextureInformationVariant {
    Default(Box<SourceTextureInformation>),
    V5_0(Box<SourceTextureInformation>),
}

impl crate::Parse for TextureImportOutputSourceTextureInformationVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = ( { Ok(Box::new(SourceTextureInformation::parse(rdr, ver)?)) } )?;
            rdr.align(4)?;
            Ok(Self::V5_0(v))
        } else {
            let v = ( { Ok(Box::new(SourceTextureInformation::parse(rdr, ver)?)) } )?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct TextureImportOutput {
    pub texture_import_instructions: TextureImportOutputTextureImportInstructionsVariant,
    pub source_texture_information: TextureImportOutputSourceTextureInformationVariant,
    pub import_inspector_warnings: Option<String>,
}

impl crate::Parse for TextureImportOutput {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let texture_import_instructions = TextureImportOutputTextureImportInstructionsVariant::parse(rdr, ver)?;
        let source_texture_information = TextureImportOutputSourceTextureInformationVariant::parse(rdr, ver)?;
        let import_inspector_warnings = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = rdr.read_aligned_string()?;
            rdr.align(4)?;
            Some(v)
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
