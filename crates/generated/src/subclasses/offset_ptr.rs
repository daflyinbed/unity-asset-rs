//! 自动生成。请勿手动修改。

use crate::subclasses::value_array_constant::ValueArrayConstant;

#[derive(Debug, Clone)]
pub struct OffsetPtr {
    pub data: ValueArrayConstant,
}

impl crate::Parse for OffsetPtr {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let data = ValueArrayConstant::parse(r, ver)?;

        Ok(OffsetPtr {
            data,
        })
    }
}
