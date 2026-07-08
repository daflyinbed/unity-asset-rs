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
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let shader_platform = rdr.read_i32()?;
        let defines_tier1 = fixed_bitset::parse(rdr, ver)?;
        let defines_tier2 = fixed_bitset::parse(rdr, ver)?;
        let defines_tier3 = fixed_bitset::parse(rdr, ver)?;
        rdr.align(4)?;

        Ok(PlatformShaderDefines {
            shader_platform,
            defines_tier1,
            defines_tier2,
            defines_tier3,
        })
    }
}
