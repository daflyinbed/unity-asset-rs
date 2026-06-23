//! 自动生成。请勿手动修改。

use crate::subclasses::variant_info::VariantInfo;

#[derive(Debug, Clone)]
pub struct ShaderInfo {
    pub variants: Vec<VariantInfo>,
}

impl crate::Parse for ShaderInfo {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let variants = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { VariantInfo::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(ShaderInfo {
            variants,
        })
    }
}
