//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct ClothSphereColliderPair {
    pub first: PPtr,
    pub second: PPtr,
}

impl crate::Parse for ClothSphereColliderPair {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let first = crate::PPtr::parse(rdr, ver)?;
        let second = crate::PPtr::parse(rdr, ver)?;

        Ok(ClothSphereColliderPair {
            first,
            second,
        })
    }
}
