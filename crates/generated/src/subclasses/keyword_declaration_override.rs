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
        let keywords = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { KeywordOverrideInfo::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;
        let variant_generation_mode = rdr.read_i32()?;

        Ok(KeywordDeclarationOverride {
            keywords,
            variant_generation_mode,
        })
    }
}
