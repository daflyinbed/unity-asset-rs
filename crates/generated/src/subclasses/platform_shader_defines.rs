//! 自动生成。请勿手动修改。

use crate::subclasses::fixed_bitset::fixed_bitset;

#[derive(Debug, Clone)]
pub struct PlatformShaderDefines {
    pub shader_platform: i32,
    pub defines_tier1: fixed_bitset,
    pub defines_tier2: fixed_bitset,
    pub defines_tier3: fixed_bitset,
}

impl crate::Parse for PlatformShaderDefines {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let shader_platform = r.read_i32()?;
        let defines_tier1 = fixed_bitset::parse(r, ver)?;
        r.align(4)?;
        let defines_tier2 = fixed_bitset::parse(r, ver)?;
        r.align(4)?;
        let defines_tier3 = fixed_bitset::parse(r, ver)?;
        r.align(4)?;

        Ok(PlatformShaderDefines {
            shader_platform,
            defines_tier1,
            defines_tier2,
            defines_tier3,
        })
    }
}
