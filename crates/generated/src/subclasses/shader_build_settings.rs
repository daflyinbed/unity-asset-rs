//! 自动生成。请勿手动修改。

use crate::subclasses::keyword_declaration_override::KeywordDeclarationOverride;

#[derive(Debug, Clone)]
pub struct ShaderBuildSettings {
    pub keyword_declaration_overrides: Vec<KeywordDeclarationOverride>,
}

impl crate::Parse for ShaderBuildSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let keyword_declaration_overrides = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { KeywordDeclarationOverride::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(ShaderBuildSettings {
            keyword_declaration_overrides,
        })
    }
}
